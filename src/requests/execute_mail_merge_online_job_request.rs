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

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::*;
use crate::models::*;
use crate::request::{
    ApiRequestData, DynamicResponse, ProgressCallback, Request, ResponseData, TypedRequest,
};
use crate::responses::*;
use crate::{ApiClient, SdkResult};

/// Request parameters for the ExecuteMailMergeOnlineJob operation.
pub struct ExecuteMailMergeOnlineJobRequest {
    /// File with template.
    pub template: Vec<u8>,
    /// File with mailmerge data.
    pub data: Vec<u8>,
    /// Field options.
    pub options: Option<FieldOptions>,
    /// The flag indicating whether to execute Mail Merge operation with regions.
    pub with_regions: Option<bool>,
    /// The flag indicating whether fields in whole document are updated while executing of a mail merge with regions.
    pub merge_whole_document: Option<bool>,
    /// The cleanup options.
    pub cleanup: Option<String>,
    /// The filename of the output document, that will be used when the resulting document has a dynamic field {filename}. If it is not set, the "template" will be used instead.
    pub document_file_name: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl ExecuteMailMergeOnlineJobRequest {
    pub fn new(template: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            template,
            data,
            options: None,
            with_regions: None,
            merge_whole_document: None,
            cleanup: None,
            document_file_name: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_options(mut self, value: FieldOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn with_with_regions(mut self, value: bool) -> Self {
        self.with_regions = Some(value);
        self
    }

    pub fn with_merge_whole_document(mut self, value: bool) -> Self {
        self.merge_whole_document = Some(value);
        self
    }

    pub fn with_cleanup(mut self, value: String) -> Self {
        self.cleanup = Some(value);
        self
    }

    pub fn with_document_file_name(mut self, value: String) -> Self {
        self.document_file_name = Some(value);
        self
    }

    pub fn with_send_progress(mut self, callback: ProgressCallback) -> Self {
        self.send_progress = Some(callback);
        self
    }

    pub fn with_receive_progress(mut self, callback: ProgressCallback) -> Self {
        self.receive_progress = Some(callback);
        self
    }

    async fn parse_response_data(response: ResponseData) -> SdkResult<JobInfo> {
        Ok(serde_json::from_slice::<JobInfo>(&response.body)?)
    }

    pub(crate) fn into_original(self) -> ExecuteMailMergeOnlineRequest {
        ExecuteMailMergeOnlineRequest {
            template: self.template,
            data: self.data,
            options: self.options,
            with_regions: self.with_regions,
            merge_whole_document: self.merge_whole_document,
            cleanup: self.cleanup,
            document_file_name: self.document_file_name,
            send_progress: self.send_progress,
            receive_progress: self.receive_progress,
        }
    }
}

#[async_trait]
impl TypedRequest for ExecuteMailMergeOnlineJobRequest {
    type Response = JobInfo;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for ExecuteMailMergeOnlineJobRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/job/put/MailMerge".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        if let Some(value) = &self.with_regions {
            query.push(("withRegions".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.merge_whole_document {
            query.push(("mergeWholeDocument".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.cleanup {
            query.push(("cleanup".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.document_file_name {
            query.push(("documentFileName".to_owned(), client.query_value(value)?));
        }
        client.add_binary_part(&mut body_parts, "Template", &self.template);
        client.add_binary_part(&mut body_parts, "Data", &self.data);
        if let Some(value) = &self.options {
            client
                .add_model_part(&mut body_parts, "Options", value)
                .await?;
        }

        let url = client.build_url(&path, &query)?;
        let body = client.request_body_from_parts(&mut headers, body_parts);
        Ok(ApiRequestData {
            method: reqwest::Method::PUT,
            url,
            headers,
            body,
            send_progress: self.send_progress.clone(),
            receive_progress: self.receive_progress.clone(),
        })
    }

    async fn parse_any(&self, response: ResponseData) -> SdkResult<DynamicResponse> {
        Ok(Box::new(Self::parse_response_data(response).await?))
    }
}
