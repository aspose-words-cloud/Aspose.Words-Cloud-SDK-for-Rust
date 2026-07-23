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

/// Class is used for insert page number request building.
#[derive(Debug, Deserialize, Serialize)]
pub struct PageNumber {
    /// Gets or sets text alignment, possible values are left, right, center or justify.
    #[serde(rename = "Alignment", skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,

    /// Gets or sets the page number format, e.g. "{PAGE} of {NUMPAGES}".
    #[serde(rename = "Format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Gets or sets a value indicating whether if true the page number is added at the top of the page, else at the bottom.
    #[serde(rename = "IsTop", skip_serializing_if = "Option::is_none")]
    pub is_top: Option<bool>,

    /// Gets or sets the starting page number of the document.
    #[serde(rename = "PageStartingNumber", skip_serializing_if = "Option::is_none")]
    pub page_starting_number: Option<i32>,

    /// Gets or sets a value indicating whether if true the page number is added on first page too.
    #[serde(
        rename = "SetPageNumberOnFirstPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub set_page_number_on_first_page: Option<bool>,
}

impl Default for PageNumber {
    fn default() -> Self {
        Self {
            alignment: None,
            format: None,
            is_top: None,
            page_starting_number: None,
            set_page_number_on_first_page: None,
        }
    }
}

impl Model for PageNumber {
    fn validate(&self) -> SdkResult<()> {
        if self.is_top.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsTop in PageNumber is required".to_owned(),
            ));
        }
        if self.set_page_number_on_first_page.is_none() {
            return Err(SdkError::InvalidRequest(
                "property SetPageNumberOnFirstPage in PageNumber is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
