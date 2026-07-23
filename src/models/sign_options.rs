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

/// Container class for digital signature options.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignOptions {
    /// Gets or sets comments on the digital signature. The default value is an empty string.
    #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,

    /// Gets or sets the password to decrypt source document. The default value is an empty string.
    #[serde(rename = "DecryptionPassword", skip_serializing_if = "Option::is_none")]
    pub decryption_password: Option<String>,

    /// Gets or sets the class Guid of the signature cryptography provider. The default value is Empty (all zeroes) Guid.
    #[serde(rename = "ProviderId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Gets or sets user defined signature line Guid. The default value is Empty (all zeroes) Guid.
    #[serde(rename = "SignatureLineId", skip_serializing_if = "Option::is_none")]
    pub signature_line_id: Option<String>,

    /// Gets or sets the image that will be shown in associated SignatureLine. The default value is an empty string.
    #[serde(
        rename = "SignatureLineImageFilename",
        skip_serializing_if = "Option::is_none"
    )]
    pub signature_line_image_filename: Option<String>,

    /// Gets or sets the date of signing. The default value is current time (Now).
    #[serde(
        rename = "SignTime",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_optional_date_time"
    )]
    pub sign_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SignOptions {
    fn default() -> Self {
        Self {
            comments: None,
            decryption_password: None,
            provider_id: None,
            signature_line_id: None,
            signature_line_image_filename: None,
            sign_time: None,
        }
    }
}

impl Model for SignOptions {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
