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

/// Request parameters for the InsertStructuredDocumentTag operation.
pub struct InsertStructuredDocumentTagRequest {
    /// The filename of the input document.
    pub name: String,
    /// Structured document tag parameters.
    pub structured_document_tag: StructuredDocumentTagInsert,
    /// The path to the node in the document tree.
    pub node_path: Option<String>,
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
    /// Result path of the document after the operation. If this parameter is omitted then result of the operation will be saved as the source document.
    pub dest_file_name: Option<String>,
    /// Initials of the author to use for revisions.If you set this parameter and then make some changes to the document programmatically, save the document and later open the document in MS Word you will see these changes as revisions.
    pub revision_author: Option<String>,
    /// The date and time to use for revisions.
    pub revision_date_time: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl InsertStructuredDocumentTagRequest {
    pub fn new(name: String, structured_document_tag: StructuredDocumentTagInsert) -> Self {
        Self {
            name,
            structured_document_tag,
            node_path: None,
            folder: None,
            storage: None,
            load_encoding: None,
            password: None,
            encrypted_password: None,
            open_type_support: None,
            dest_file_name: None,
            revision_author: None,
            revision_date_time: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_node_path(mut self, value: String) -> Self {
        self.node_path = Some(value);
        self
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

    pub fn with_dest_file_name(mut self, value: String) -> Self {
        self.dest_file_name = Some(value);
        self
    }

    pub fn with_revision_author(mut self, value: String) -> Self {
        self.revision_author = Some(value);
        self
    }

    pub fn with_revision_date_time(mut self, value: String) -> Self {
        self.revision_date_time = Some(value);
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

    async fn parse_response_data(
        response: ResponseData,
    ) -> SdkResult<StructuredDocumentTagResponse> {
        Ok(serde_json::from_slice::<StructuredDocumentTagResponse>(
            &response.body,
        )?)
    }
}

#[async_trait]
impl TypedRequest for InsertStructuredDocumentTagRequest {
    type Response = StructuredDocumentTagResponse;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for InsertStructuredDocumentTagRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/{name}/{nodePath}/sdt".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        let value = client.query_value(&self.name)?;
        path = path.replace("{name}", &value);
        if let Some(value) = &self.node_path {
            path = path.replace("{nodePath}", &client.query_value(value)?);
        }
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
        if let Some(value) = &self.dest_file_name {
            query.push(("destFileName".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.revision_author {
            query.push(("revisionAuthor".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.revision_date_time {
            query.push(("revisionDateTime".to_owned(), client.query_value(value)?));
        }
        client
            .add_model_part(&mut body_parts, "Body", &self.structured_document_tag)
            .await?;

        let url = client.build_url(&path, &query)?;
        let body = client.request_body_from_parts(&mut headers, body_parts);
        Ok(ApiRequestData {
            method: reqwest::Method::POST,
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
