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

/// Represents a single document style to insert.
#[derive(Debug, Deserialize, Serialize)]
pub struct StyleInsert {
        /// Gets or sets the case sensitive name of the style to create.
        #[serde(rename = "StyleName", skip_serializing_if = "Option::is_none")]
        pub style_name: Option<String>,


        /// Gets or sets the StyleType value that specifies the type of the style to create.
        #[serde(rename = "StyleType", skip_serializing_if = "Option::is_none")]
        pub style_type: Option<StyleInsertStyleTypeEnum>,

}

impl Default for StyleInsert {
    fn default() -> Self {
        Self {
            style_name: None,
            style_type: None,
        }
    }
}

impl Model for StyleInsert {
    fn validate(&self) -> SdkResult<()> {
        if self.style_name.is_none() {
            return Err(SdkError::InvalidRequest(
                "property StyleName in StyleInsert is required".to_owned(),
            ));
        }
        if self.style_type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property StyleType in StyleInsert is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the StyleType value that specifies the type of the style to create.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StyleInsertStyleTypeEnum {
    #[serde(rename = "Paragraph")]
        Paragraph,
    #[serde(rename = "Character")]
        Character,
    #[serde(rename = "Table")]
        Table,
    #[serde(rename = "List")]
        List,
}