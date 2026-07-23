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

/// DTO container with a range element.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceRange {
    /// Gets or sets the range's text.
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Gets or sets the range's text type.
    #[serde(rename = "TextType", skip_serializing_if = "Option::is_none")]
    pub text_type: Option<ReplaceRangeTextTypeEnum>,
}

impl Default for ReplaceRange {
    fn default() -> Self {
        Self {
            text: None,
            text_type: None,
        }
    }
}

impl Model for ReplaceRange {
    fn validate(&self) -> SdkResult<()> {
        if self.text.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Text in ReplaceRange is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the range's text type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReplaceRangeTextTypeEnum {
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "Html")]
    Html,
}
