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

/// The error details.
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorDetails {
        /// Gets or sets ErrorDateTime.
        #[serde(rename = "ErrorDateTime", skip_serializing_if = "Option::is_none")]
        pub r#error_date_time: Option<DateTime<Utc>>,


        /// Gets or sets RequestId.
        #[serde(rename = "RequestId", skip_serializing_if = "Option::is_none")]
        pub r#request_id: Option<String>,

}

impl Default for ErrorDetails {
    fn default() -> Self {
        Self {
            r#error_date_time: None,
            r#request_id: None,
        }
    }
}

impl Model for ErrorDetails {
    fn validate(&self) -> SdkResult<()> {
        if self.r#error_date_time.is_none() {
            return Err(SdkError::InvalidRequest(
                "property ErrorDateTime in ErrorDetails is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

