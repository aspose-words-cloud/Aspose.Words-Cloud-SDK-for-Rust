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

/// Result of splitting document.
#[derive(Debug, Deserialize, Serialize)]
pub struct SplitDocumentResult {
        /// Gets or sets the link to the source document.
        #[serde(rename = "SourceDocument", skip_serializing_if = "Option::is_none")]
        pub r#source_document: Option<FileLink>,


        /// Gets or sets the link to the file archive with pages.
        #[serde(rename = "ZippedPages", skip_serializing_if = "Option::is_none")]
        pub r#zipped_pages: Option<FileLink>,


        /// Gets or sets the list of pages.
        #[serde(rename = "Pages", skip_serializing_if = "Option::is_none")]
        pub r#pages: Option<Vec<FileLink>>,

}

impl Default for SplitDocumentResult {
    fn default() -> Self {
        Self {
            r#source_document: None,
            r#zipped_pages: None,
            r#pages: None,
        }
    }
}

impl Model for SplitDocumentResult {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.r#source_document {
        value.validate()?;
        }
        if let Some(value) = &self.r#zipped_pages {
        value.validate()?;
        }
        if let Some(values) = &self.r#pages {
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

