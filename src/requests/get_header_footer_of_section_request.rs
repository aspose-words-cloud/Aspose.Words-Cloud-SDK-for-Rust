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

/// Request parameters for the GetHeaderFooterOfSection operation.
pub struct GetHeaderFooterOfSectionRequest {
    /// The filename of the input document.
    pub name: String,
    /// The index of the HeaderFooter object.
    pub header_footer_index: i32,
    /// The index of the section.
    pub section_index: i32,
    /// Original document folder.
    pub folder: Option<String>,
    /// Original document storage.
    pub storage: Option<String>,
    /// Encoding that will be used to load an HTML (or TXT) document if the encoding is not specified in HTML.
    pub load_encoding: Option<String>,
    /// Password of protected Word document. Use the parameter to pass a password via SDK. SDK encrypts it automatically. We don't recommend to use the parameter to pass a plain password for direct call of API.
    pub password: Option<String>,
    /// Password of protected Word document. Use the parameter to pass an encrypted password for direct calls of API. See SDK code for encyption details.
    pub encrypted_password: Option<String>,
    /// The value indicates whether OpenType support is on.
    pub open_type_support: Option<bool>,
    /// The list of HeaderFooter types.
    pub filter_by_type: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl GetHeaderFooterOfSectionRequest {
    pub fn new(name: String, header_footer_index: i32, section_index: i32) -> Self {
        Self {
            name,
            header_footer_index,
            section_index,
            folder: None,
            storage: None,
            load_encoding: None,
            password: None,
            encrypted_password: None,
            open_type_support: None,
            filter_by_type: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_folder(mut self, value: String) -> Self {
        self.folder = Some(value);
        self
    }

    pub fn with_storage(mut self, value: String) -> Self {
        self.storage = Some(value);
        self
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

    pub fn with_filter_by_type(mut self, value: String) -> Self {
        self.filter_by_type = Some(value);
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

    async fn parse_response_data(response: ResponseData) -> SdkResult<HeaderFooterResponse> {
        Ok(serde_json::from_slice::<HeaderFooterResponse>(
            &response.body,
        )?)
    }
}

#[async_trait]
impl TypedRequest for GetHeaderFooterOfSectionRequest {
    type Response = HeaderFooterResponse;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for GetHeaderFooterOfSectionRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path =
            "/words/{name}/sections/{sectionIndex}/headersfooters/{headerFooterIndex}".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        let value = client.query_value(&self.name)?;
        path = path.replace("{name}", &value);
        let value = client.query_value(&self.header_footer_index)?;
        path = path.replace("{headerFooterIndex}", &value);
        let value = client.query_value(&self.section_index)?;
        path = path.replace("{sectionIndex}", &value);
        if let Some(value) = &self.folder {
            query.push(("folder".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.storage {
            query.push(("storage".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.load_encoding {
            query.push(("loadEncoding".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.password {
            query.push((
                "encryptedPassword".to_owned(),
                client.encrypt_password(value).await?,
            ));
        }
        if let Some(value) = &self.encrypted_password {
            query.push(("encryptedPassword".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.open_type_support {
            query.push(("openTypeSupport".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.filter_by_type {
            query.push(("filterByType".to_owned(), client.query_value(value)?));
        }

        let url = client.build_url(&path, &query)?;
        let body = client.request_body_from_parts(&mut headers, body_parts);
        Ok(ApiRequestData {
            method: reqwest::Method::GET,
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
