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

use crate::models::*;
use crate::responses::*;
use crate::request::{
    ApiRequestData, DynamicResponse, ProgressCallback, Request, ResponseData, TypedRequest,
};
use crate::{ApiClient, SdkResult};
use super::*;

/// Request parameters for the DeleteComment operation.
pub struct DeleteCommentRequest {
    /// The filename of the input document.
    pub r#name: String,
    /// The index of the comment.
    pub r#comment_index: i32,
    /// Original document folder.
    pub r#folder: Option<String>,
    /// Original document storage.
    pub r#storage: Option<String>,
    /// Encoding that will be used to load an HTML (or TXT) document if the encoding is not specified in HTML.
    pub r#load_encoding: Option<String>,
    /// Password of protected Word document. Use the parameter to pass a password via SDK. SDK encrypts it automatically. We don't recommend to use the parameter to pass a plain password for direct call of API.
    pub r#password: Option<String>,
    /// Password of protected Word document. Use the parameter to pass an encrypted password for direct calls of API. See SDK code for encyption details.
    pub r#encrypted_password: Option<String>,
    /// The value indicates whether OpenType support is on.
    pub r#open_type_support: Option<bool>,
    /// Result path of the document after the operation. If this parameter is omitted then result of the operation will be saved as the source document.
    pub r#dest_file_name: Option<String>,
    /// Initials of the author to use for revisions.If you set this parameter and then make some changes to the document programmatically, save the document and later open the document in MS Word you will see these changes as revisions.
    pub r#revision_author: Option<String>,
    /// The date and time to use for revisions.
    pub r#revision_date_time: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl DeleteCommentRequest {
    pub fn new(r#name: String, r#comment_index: i32) -> Self {
        Self {
            r#name,
            r#comment_index,
            r#folder: None,
            r#storage: None,
            r#load_encoding: None,
            r#password: None,
            r#encrypted_password: None,
            r#open_type_support: None,
            r#dest_file_name: None,
            r#revision_author: None,
            r#revision_date_time: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_folder(mut self, value: String) -> Self {
        self.r#folder = Some(value);
        self
    }

    pub fn with_storage(mut self, value: String) -> Self {
        self.r#storage = Some(value);
        self
    }

    pub fn with_load_encoding(mut self, value: String) -> Self {
        self.r#load_encoding = Some(value);
        self
    }

    pub fn with_password(mut self, value: String) -> Self {
        self.r#password = Some(value);
        self
    }

    pub fn with_encrypted_password(mut self, value: String) -> Self {
        self.r#encrypted_password = Some(value);
        self
    }

    pub fn with_open_type_support(mut self, value: bool) -> Self {
        self.r#open_type_support = Some(value);
        self
    }

    pub fn with_dest_file_name(mut self, value: String) -> Self {
        self.r#dest_file_name = Some(value);
        self
    }

    pub fn with_revision_author(mut self, value: String) -> Self {
        self.r#revision_author = Some(value);
        self
    }

    pub fn with_revision_date_time(mut self, value: String) -> Self {
        self.r#revision_date_time = Some(value);
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
impl TypedRequest for DeleteCommentRequest {
    type Response = ();

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for DeleteCommentRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/{name}/comments/{commentIndex}".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        let value = client.query_value(&self.r#name)?;
        path = path.replace("{name}", &value);
        let value = client.query_value(&self.r#comment_index)?;
        path = path.replace("{commentIndex}", &value);
        if let Some(value) = &self.r#folder {
        query.push(("folder".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#storage {
        query.push(("storage".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#load_encoding {
        query.push(("loadEncoding".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#password {
        query.push(("encryptedPassword".to_owned(), client.encrypt_password(value).await?));
        }
        if let Some(value) = &self.r#encrypted_password {
        query.push(("encryptedPassword".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#open_type_support {
        query.push(("openTypeSupport".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#dest_file_name {
        query.push(("destFileName".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#revision_author {
        query.push(("revisionAuthor".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#revision_date_time {
        query.push(("revisionDateTime".to_owned(), client.query_value(value)?));
        }

        let url = client.build_url(&path, &query)?;
        let body = client.request_body_from_parts(&mut headers, body_parts);
        Ok(ApiRequestData {
            method: reqwest::Method::DELETE,
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