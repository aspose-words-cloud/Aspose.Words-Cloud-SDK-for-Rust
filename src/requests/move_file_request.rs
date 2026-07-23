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

/// Request parameters for the MoveFile operation.
pub struct MoveFileRequest {
    /// Destination file path e.g. '/dest.ext'.
    pub dest_path: String,
    /// Source file's path e.g. '/Folder 1/file.ext' or '/Bucket/Folder 1/file.ext'.
    pub src_path: String,
    /// Source storage name.
    pub src_storage_name: Option<String>,
    /// Destination storage name.
    pub dest_storage_name: Option<String>,
    /// File version ID to move.
    pub version_id: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl MoveFileRequest {
    pub fn new(dest_path: String, src_path: String) -> Self {
        Self {
            dest_path,
            src_path,
            src_storage_name: None,
            dest_storage_name: None,
            version_id: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_src_storage_name(mut self, value: String) -> Self {
        self.src_storage_name = Some(value);
        self
    }

    pub fn with_dest_storage_name(mut self, value: String) -> Self {
        self.dest_storage_name = Some(value);
        self
    }

    pub fn with_version_id(mut self, value: String) -> Self {
        self.version_id = Some(value);
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

    async fn parse_response_data(response: ResponseData) -> SdkResult<()> {
        let _response = response;
        Ok(())
    }
}

#[async_trait]
impl TypedRequest for MoveFileRequest {
    type Response = ();

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for MoveFileRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/storage/file/move/{srcPath}".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        let value = client.query_value(&self.src_path)?;
        path = path.replace("{srcPath}", &value);
        query.push(("destPath".to_owned(), client.query_value(&self.dest_path)?));
        if let Some(value) = &self.src_storage_name {
            query.push(("srcStorageName".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.dest_storage_name {
            query.push(("destStorageName".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.version_id {
            query.push(("versionId".to_owned(), client.query_value(value)?));
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
