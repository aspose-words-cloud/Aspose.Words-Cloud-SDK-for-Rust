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

/// Represents a bookmark to insert.
#[derive(Debug, Deserialize, Serialize)]
pub struct BookmarkInsert {
        /// Gets or sets the name of the bookmark.
        #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
        pub r#name: Option<String>,


        /// Gets or sets text, enclosed in the bookmark.
        #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
        pub r#text: Option<String>,


        /// Gets or sets the link to start bookmark node.
        #[serde(rename = "StartRange", skip_serializing_if = "Option::is_none", default, deserialize_with = "deserialize_optional_model")]
        pub r#start_range: Option<ModelBox>,


        /// Gets or sets the link to end bookmark node.
        #[serde(rename = "EndRange", skip_serializing_if = "Option::is_none", default, deserialize_with = "deserialize_optional_model")]
        pub r#end_range: Option<ModelBox>,

}

impl Default for BookmarkInsert {
    fn default() -> Self {
        Self {
            r#name: None,
            r#text: None,
            r#start_range: None,
            r#end_range: None,
        }
    }
}

impl Model for BookmarkInsert {
    fn validate(&self) -> SdkResult<()> {
        if self.r#name.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Name in BookmarkInsert is required".to_owned(),
            ));
        }
        if self.r#text.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Text in BookmarkInsert is required".to_owned(),
            ));
        }
        if self.r#start_range.is_none() {
            return Err(SdkError::InvalidRequest(
                "property StartRange in BookmarkInsert is required".to_owned(),
            ));
        }
        if self.r#end_range.is_none() {
            return Err(SdkError::InvalidRequest(
                "property EndRange in BookmarkInsert is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#start_range {
        value.validate()?;
        }
        if let Some(value) = &self.r#end_range {
        value.validate()?;
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

