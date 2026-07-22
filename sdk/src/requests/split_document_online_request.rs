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

/// Request parameters for the SplitDocumentOnline operation.
pub struct SplitDocumentOnlineRequest {
    /// The document.
    pub document: Vec<u8>,
    /// The format to split.
    pub format: String,
    /// Encoding that will be used to load an HTML (or TXT) document if the encoding is not specified in HTML.
    pub load_encoding: Option<String>,
    /// Password of protected Word document. Use the parameter to pass a password via SDK. SDK encrypts it automatically. We don't recommend to use the parameter to pass a plain password for direct call of API.
    pub password: Option<String>,
    /// Password of protected Word document. Use the parameter to pass an encrypted password for direct calls of API. See SDK code for encyption details.
    pub encrypted_password: Option<String>,
    /// The value indicates whether OpenType support is on.
    pub open_type_support: Option<bool>,
    /// Result path of the document after the operation. If this parameter is omitted then result of the operation will be saved as the source document.
    pub dest_file_name: Option<String>,
    /// The start page.
    pub from: Option<i32>,
    /// The end page.
    pub to: Option<i32>,
    /// The flag indicating whether to ZIP the output.
    pub zip_output: Option<bool>,
    /// Folder in filestorage with custom fonts.
    pub fonts_location: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl SplitDocumentOnlineRequest {
    pub fn new(document: Vec<u8>, format: String) -> Self {
        Self {
            document,
            format,
            load_encoding: None,
            password: None,
            encrypted_password: None,
            open_type_support: None,
            dest_file_name: None,
            from: None,
            to: None,
            zip_output: None,
            fonts_location: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_load_encoding(mut self, value: String) -> Self {
        self.load_encoding = Some(value);
        self
    }

    pub fn with_password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    pub fn with_encrypted_password(mut self, value: String) -> Self {
        self.encrypted_password = Some(value);
        self
    }

    pub fn with_open_type_support(mut self, value: bool) -> Self {
        self.open_type_support = Some(value);
        self
    }

    pub fn with_dest_file_name(mut self, value: String) -> Self {
        self.dest_file_name = Some(value);
        self
    }

    pub fn with_from(mut self, value: i32) -> Self {
        self.from = Some(value);
        self
    }

    pub fn with_to(mut self, value: i32) -> Self {
        self.to = Some(value);
        self
    }

    pub fn with_zip_output(mut self, value: bool) -> Self {
        self.zip_output = Some(value);
        self
    }

    pub fn with_fonts_location(mut self, value: String) -> Self {
        self.fonts_location = Some(value);
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

    async fn parse_response_data(response: ResponseData) -> SdkResult<SplitDocumentOnlineResponse> {
        SplitDocumentOnlineResponse::from_response(response).await
    }

}

#[async_trait]
impl TypedRequest for SplitDocumentOnlineRequest {
    type Response = SplitDocumentOnlineResponse;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for SplitDocumentOnlineRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/online/put/split".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        query.push(("format".to_owned(), client.query_value(&self.format)?));
        if let Some(value) = &self.load_encoding {
        query.push(("loadEncoding".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.password {
        query.push(("encryptedPassword".to_owned(), client.encrypt_password(value).await?));
        }
        if let Some(value) = &self.encrypted_password {
        query.push(("encryptedPassword".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.open_type_support {
        query.push(("openTypeSupport".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.dest_file_name {
        query.push(("destFileName".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.from {
        query.push(("from".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.to {
        query.push(("to".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.zip_output {
        query.push(("zipOutput".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.fonts_location {
        query.push(("fontsLocation".to_owned(), client.query_value(value)?));
        }
        client.add_binary_part(&mut body_parts, "Document", &self.document);

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