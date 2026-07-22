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

/// Container class for details of encryption.
#[derive(Debug, Deserialize, Serialize)]
pub struct PdfEncryptionDetailsData {
        /// Gets or sets the owner password for the encrypted PDF document.
        #[serde(rename = "OwnerPassword", skip_serializing_if = "Option::is_none")]
        pub owner_password: Option<String>,


        /// Gets or sets the operations that are allowed to a user on the encrypted PDF document.
        #[serde(rename = "Permissions", skip_serializing_if = "Option::is_none")]
        pub permissions: Option<Vec<PdfPermissionsEnum>>,


        /// Gets or sets the user password required for opening the encrypted PDF document.
        #[serde(rename = "UserPassword", skip_serializing_if = "Option::is_none")]
        pub user_password: Option<String>,

}

impl Default for PdfEncryptionDetailsData {
    fn default() -> Self {
        Self {
            owner_password: None,
            permissions: None,
            user_password: None,
        }
    }
}

impl Model for PdfEncryptionDetailsData {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

