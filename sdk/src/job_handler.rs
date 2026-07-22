// --------------------------------------------------------------------------------
// Copyright (c) 2026 Aspose.Words for Cloud
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// --------------------------------------------------------------------------------

use std::time::Duration;

use async_trait::async_trait;

use crate::api_client::{parse_http_part, parse_multipart};
use crate::models::{JobInfo, JobInfoStatusEnum, Model};
use crate::request::{
    ApiRequestData, DynamicResponse, Request, RequestBody, ResponseData, TypedRequest,
};
use crate::{ApiClient, SdkError, SdkResult};

/// Polls and deserializes a queued Words Cloud job without erasing its result type.
pub struct JobHandler<R>
where
    R: TypedRequest,
{
    client: ApiClient,
    request: R,
    info: JobInfo,
    result: Option<R::Response>,
}

impl<R> JobHandler<R>
where
    R: TypedRequest,
{
    pub(crate) fn new(client: ApiClient, request: R, info: JobInfo) -> Self {
        Self {
            client,
            request,
            info,
            result: None,
        }
    }

    pub fn message(&self) -> &str {
        self.info.r#message.as_deref().unwrap_or_default()
    }

    pub fn status(&self) -> JobInfoStatusEnum {
        self.info.r#status.unwrap_or(JobInfoStatusEnum::Unknown)
    }

    pub fn result(&self) -> Option<&R::Response> {
        self.result.as_ref()
    }

    pub async fn update(&mut self) -> SdkResult<()> {
        let job_id = self.info.r#job_id.as_ref().ok_or_else(|| {
            SdkError::InvalidResponse("job response does not contain a job id".to_owned())
        })?;
        let response = self
            .client
            .execute(&JobResultRequest {
                job_id: job_id.clone(),
            })
            .await?;
        let content_type = response.headers.get("content-type").ok_or_else(|| {
            SdkError::InvalidResponse("job response content type is missing".to_owned())
        })?;
        let parts = parse_multipart(content_type, response.body).await?;
        let info_part = parts.first().ok_or_else(|| {
            SdkError::InvalidResponse("job response has no status part".to_owned())
        })?;
        let info: JobInfo = serde_json::from_slice(&info_part.data)?;
        info.validate()?;
        let succeeded = info.r#status == Some(JobInfoStatusEnum::Succeded);
        self.info = info;
        if succeeded {
            if let Some(result_part) = parts.get(1) {
                let (status, response) = parse_http_part(result_part.data.clone()).await?;
                if !(200..300).contains(&status) {
                    return Err(SdkError::Http {
                        status,
                        message: String::from_utf8_lossy(&response.body).into_owned(),
                    });
                }
                self.result = Some(self.request.parse_response(response).await?);
            }
        }
        Ok(())
    }

    pub async fn wait_result(mut self, update_interval: Duration) -> SdkResult<R::Response> {
        while matches!(
            self.status(),
            JobInfoStatusEnum::Queued | JobInfoStatusEnum::Processing
        ) {
            tokio::time::sleep(update_interval).await;
            self.update().await?;
        }
        if self.status() != JobInfoStatusEnum::Succeded {
            return Err(SdkError::InvalidResponse(format!(
                "job failed with status {:?}: {}",
                self.status(),
                self.message()
            )));
        }
        self.result.ok_or_else(|| {
            SdkError::InvalidResponse("successful job response has no result".to_owned())
        })
    }
}

struct JobResultRequest {
    job_id: String,
}

#[async_trait]
impl Request for JobResultRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let query = vec![("id".to_owned(), self.job_id.clone())];
        Ok(ApiRequestData {
            method: reqwest::Method::GET,
            url: client.build_url("/words/job", &query)?,
            headers: Vec::new(),
            body: RequestBody::Empty,
            send_progress: None,
            receive_progress: None,
        })
    }

    async fn parse_any(&self, response: ResponseData) -> SdkResult<DynamicResponse> {
        Ok(Box::new(response))
    }
}