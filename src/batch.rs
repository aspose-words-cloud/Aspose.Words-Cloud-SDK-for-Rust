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

use std::any::Any;

use async_trait::async_trait;
use uuid::Uuid;

use crate::api_client::{parse_http_part, parse_multipart, prepare_batch_part};
use crate::request::{
    ApiRequestData, BodyPart, DynamicResponse, Request, RequestBody, ResponseData,
};
use crate::{ApiClient, SdkError, SdkResult};

/// One operation in a heterogeneous batch request.
pub struct BatchRequest {
    request_id: String,
    depends_on: Option<String>,
    request: Box<dyn Request>,
}

impl BatchRequest {
    pub fn new<R>(request: R) -> Self
    where
        R: Request + 'static,
    {
        Self {
            request_id: Uuid::new_v4().to_string(),
            depends_on: None,
            request: Box::new(request),
        }
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn set_depends_on(&mut self, request: &BatchRequest) {
        self.depends_on = Some(request.request_id.clone());
    }

    pub fn result_of(&self) -> Vec<u8> {
        format!("resultOf({})", self.request_id).into_bytes()
    }
}

/// A typed error or a dynamically typed value returned by one batch operation.
pub enum BatchResult {
    Success(DynamicResponse),
    Error(SdkError),
}

impl BatchResult {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        match self {
            Self::Success(value) => value.downcast_ref(),
            Self::Error(_) => None,
        }
    }

    pub fn error(&self) -> Option<&SdkError> {
        match self {
            Self::Success(_) => None,
            Self::Error(error) => Some(error),
        }
    }
}

pub(crate) async fn execute_batch(
    client: &ApiClient,
    requests: &[BatchRequest],
    display_intermediate_results: bool,
) -> SdkResult<Vec<BatchResult>> {
    let mut parts = Vec::new();
    for request in requests {
        let mut data = request.request.build(client).await?;
        data.headers
            .push(("RequestId".to_owned(), request.request_id.clone()));
        if let Some(depends_on) = &request.depends_on {
            data.headers
                .push(("DependsOn".to_owned(), depends_on.clone()));
        }
        parts.push(BodyPart {
            name: request.request_id.clone(),
            filename: None,
            content_type: "application/http; msgtype=request".to_owned(),
            data: prepare_batch_part(data, &client.configuration().api_root())?,
        });
    }

    let envelope = BatchEnvelope {
        parts,
        display_intermediate_results,
    };
    let response = client.execute(&envelope).await?;
    let content_type = response.headers.get("content-type").ok_or_else(|| {
        SdkError::InvalidResponse("batch response content type is missing".to_owned())
    })?;
    let response_parts = parse_multipart(content_type, response.body).await?;
    let mut results = Vec::new();
    for part in response_parts {
        let request_id = part
            .headers
            .get("requestid")
            .or_else(|| part.headers.get("request-id"))
            .cloned()
            .or(part.name)
            .ok_or_else(|| {
                SdkError::InvalidResponse("batch response request id is missing".to_owned())
            })?;
        let request = requests
            .iter()
            .find(|request| request.request_id == request_id)
            .ok_or_else(|| {
                SdkError::InvalidResponse(format!(
                    "batch response refers to unknown request id {request_id}"
                ))
            })?;
        let (status, response) = parse_http_part(part.data).await?;
        if (200..300).contains(&status) {
            results.push(BatchResult::Success(
                request.request.parse_any(response).await?,
            ));
        } else {
            results.push(BatchResult::Error(SdkError::Http {
                status,
                message: String::from_utf8_lossy(&response.body).into_owned(),
            }));
        }
    }
    Ok(results)
}

struct BatchEnvelope {
    parts: Vec<BodyPart>,
    display_intermediate_results: bool,
}

#[async_trait]
impl Request for BatchEnvelope {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let query = vec![(
            "displayIntermediateResults".to_owned(),
            self.display_intermediate_results.to_string(),
        )];
        Ok(ApiRequestData {
            method: reqwest::Method::PUT,
            url: client.build_url("/words/batch", &query)?,
            headers: Vec::new(),
            body: RequestBody::Multipart(self.parts.clone()),
            send_progress: None,
            receive_progress: None,
        })
    }

    async fn parse_any(&self, _response: ResponseData) -> SdkResult<DynamicResponse> {
        Ok(Box::new(()))
    }
}
