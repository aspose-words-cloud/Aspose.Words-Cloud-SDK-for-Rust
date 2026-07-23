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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{SdkError, SdkResult};

use super::*;

/// Api error.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    /// Gets or sets the API error code.
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Gets or sets the server DateTime.
    #[serde(rename = "DateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<chrono::DateTime<chrono::Utc>>,

    /// Gets or sets the error description.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Gets or sets the inner error.
    #[serde(rename = "InnerError", skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<Box<ApiError>>,

    /// Gets or sets the error message.
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Default for ApiError {
    fn default() -> Self {
        Self {
            code: None,
            date_time: None,
            description: None,
            inner_error: None,
            message: None,
        }
    }
}

impl Model for ApiError {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.inner_error {
            value.validate()?;
        }

        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
