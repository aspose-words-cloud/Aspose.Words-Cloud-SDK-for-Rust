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

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::api_client::{parse_files_collection, parse_multipart};
use crate::models::*;
use crate::request::ResponseData;
use crate::{SdkError, SdkResult};

/// Typed multipart response for the SaveAsTiffOnline operation.
#[derive(Debug, Serialize)]
pub struct SaveAsTiffOnlineResponse {
    #[serde(rename = "Model", skip_serializing_if = "Option::is_none")]
    pub model: Option<SaveResponse>,
    #[serde(rename = "Document", skip_serializing_if = "Option::is_none")]
    pub document: Option<std::collections::HashMap<String, Vec<u8>>>,
}

impl SaveAsTiffOnlineResponse {
    pub(crate) async fn from_response(response: ResponseData) -> SdkResult<Self> {
        let content_type = response.headers.get("content-type").ok_or_else(|| {
            SdkError::InvalidResponse("multipart response content type is missing".to_owned())
        })?;
        let mut parts = parse_multipart(content_type, response.body).await?;
        Ok(Self {
            model: async {
                let position = parts.iter().position(|part| {
                    part.name
                        .as_deref()
                        .is_some_and(|name| name.eq_ignore_ascii_case("Model"))
                });
                let Some(position) = position else {
                    return Ok::<Option<SaveResponse>, SdkError>(None);
                };
                let part = parts.remove(position);
                Ok::<Option<SaveResponse>, SdkError>(Some(serde_json::from_slice::<SaveResponse>(
                    &part.data,
                )?))
            }
            .await?,
            document: async {
                let position = parts.iter().position(|part| {
                    part.name
                        .as_deref()
                        .is_some_and(|name| name.eq_ignore_ascii_case("Document"))
                });
                let Some(position) = position else {
                    return Ok::<Option<std::collections::HashMap<String, Vec<u8>>>, SdkError>(
                        None,
                    );
                };
                let part = parts.remove(position);
                Ok::<Option<std::collections::HashMap<String, Vec<u8>>>, SdkError>(Some(
                    parse_files_collection(
                        part.content_type.as_deref(),
                        part.filename.as_deref(),
                        part.data,
                    )
                    .await?,
                ))
            }
            .await?,
        })
    }
}
