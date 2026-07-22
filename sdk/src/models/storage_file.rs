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

/// File or folder information.
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageFile {
        /// True if it is a folder.
        #[serde(rename = "IsFolder", skip_serializing_if = "Option::is_none")]
        pub r#is_folder: Option<bool>,


        /// File or folder last modified DateTime.
        #[serde(rename = "ModifiedDate", skip_serializing_if = "Option::is_none")]
        pub r#modified_date: Option<DateTime<Utc>>,


        /// File or folder name.
        #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
        pub r#name: Option<String>,


        /// File or folder path.
        #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
        pub r#path: Option<String>,


        /// File or folder size.
        #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
        pub r#size: Option<i32>,

}

impl Default for StorageFile {
    fn default() -> Self {
        Self {
            r#is_folder: None,
            r#modified_date: None,
            r#name: None,
            r#path: None,
            r#size: None,
        }
    }
}

impl Model for StorageFile {
    fn validate(&self) -> SdkResult<()> {
        if self.r#is_folder.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsFolder in StorageFile is required".to_owned(),
            ));
        }
        if self.r#size.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Size in StorageFile is required".to_owned(),
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

