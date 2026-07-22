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

/// Container class for compare documents.
#[derive(Debug, Deserialize, Serialize)]
pub struct CompareData {
        /// Gets or sets the initials of the author to use for revisions.
        #[serde(rename = "Author", skip_serializing_if = "Option::is_none")]
        pub r#author: Option<String>,


        /// Gets or sets the compare options.
        #[serde(rename = "CompareOptions", skip_serializing_if = "Option::is_none")]
        pub r#compare_options: Option<CompareOptions>,


        /// Gets or sets the path to document to compare at the server.
        #[serde(rename = "ComparingWithDocument", skip_serializing_if = "Option::is_none")]
        pub r#comparing_with_document: Option<String>,


        /// Gets or sets the date and time to use for revisions.
        #[serde(rename = "DateTime", skip_serializing_if = "Option::is_none")]
        pub r#date_time: Option<DateTime<Utc>>,


        /// Gets or sets the file reference.
        #[serde(rename = "FileReference", skip_serializing_if = "Option::is_none")]
        pub r#file_reference: Option<FileReference>,


        /// Gets or sets the result document format.
        #[serde(rename = "ResultDocumentFormat", skip_serializing_if = "Option::is_none")]
        pub r#result_document_format: Option<String>,

}

impl Default for CompareData {
    fn default() -> Self {
        Self {
            r#author: None,
            r#compare_options: None,
            r#comparing_with_document: None,
            r#date_time: None,
            r#file_reference: None,
            r#result_document_format: None,
        }
    }
}

impl Model for CompareData {
    fn validate(&self) -> SdkResult<()> {
        if self.r#author.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Author in CompareData is required".to_owned(),
            ));
        }
        if self.r#file_reference.is_none() {
            return Err(SdkError::InvalidRequest(
                "property FileReference in CompareData is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#compare_options {
        value.validate()?;
        }


        if let Some(value) = &self.r#file_reference {
        value.validate()?;
        }

        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        if let Some(value) = &self.r#file_reference {
        value.collect_file_references(output);
        }

    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

