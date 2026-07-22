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

/// Result of saving.
#[derive(Debug, Deserialize, Serialize)]
pub struct SaveResult {
        /// Gets or sets the link to destination document.
        #[serde(rename = "DestDocument", skip_serializing_if = "Option::is_none")]
        pub r#dest_document: Option<FileLink>,


        /// Gets or sets the link to source document.
        #[serde(rename = "SourceDocument", skip_serializing_if = "Option::is_none")]
        pub r#source_document: Option<FileLink>,


        /// Gets or sets the list of links to additional items (css, images etc).
        #[serde(rename = "AdditionalItems", skip_serializing_if = "Option::is_none")]
        pub r#additional_items: Option<Vec<FileLink>>,

}

impl Default for SaveResult {
    fn default() -> Self {
        Self {
            r#dest_document: None,
            r#source_document: None,
            r#additional_items: None,
        }
    }
}

impl Model for SaveResult {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.r#dest_document {
        value.validate()?;
        }
        if let Some(value) = &self.r#source_document {
        value.validate()?;
        }
        if let Some(values) = &self.r#additional_items {
        for value in values {
        value.validate()?;
        }
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

