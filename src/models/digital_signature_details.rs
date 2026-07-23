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

/// Container class for details of digital signature.
#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalSignatureDetails {
    /// Gets or sets the certificate's filename using for signing.
    #[serde(
        rename = "CertificateFilename",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_filename: Option<String>,

    /// Gets or sets signing options.
    #[serde(rename = "SignOptions", skip_serializing_if = "Option::is_none")]
    pub sign_options: Option<SignOptions>,
}

impl Default for DigitalSignatureDetails {
    fn default() -> Self {
        Self {
            certificate_filename: None,
            sign_options: None,
        }
    }
}

impl Model for DigitalSignatureDetails {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.sign_options {
            value.validate()?;
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
