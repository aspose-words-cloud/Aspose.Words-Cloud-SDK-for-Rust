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
pub struct PdfDigitalSignatureDetailsData {
        /// Gets or sets the certificate's filename using for signing.
        #[serde(rename = "CertificateFilename", skip_serializing_if = "Option::is_none")]
        pub certificate_filename: Option<String>,


        /// Gets or sets the hash algorithm.
        #[serde(rename = "HashAlgorithm", skip_serializing_if = "Option::is_none")]
        pub hash_algorithm: Option<PdfDigitalSignatureDetailsDataHashAlgorithmEnum>,


        /// Gets or sets the location of the signing.
        #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,


        /// Gets or sets the reason for the signing.
        #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,


        /// Gets or sets the date of the signing.
        #[serde(rename = "SignatureDate", skip_serializing_if = "Option::is_none")]
        pub signature_date: Option<chrono::DateTime<chrono::Utc>>,

}

impl Default for PdfDigitalSignatureDetailsData {
    fn default() -> Self {
        Self {
            certificate_filename: None,
            hash_algorithm: None,
            location: None,
            reason: None,
            signature_date: None,
        }
    }
}

impl Model for PdfDigitalSignatureDetailsData {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the hash algorithm.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfDigitalSignatureDetailsDataHashAlgorithmEnum {
    #[serde(rename = "Sha256")]
        Sha256,
    #[serde(rename = "Sha384")]
        Sha384,
    #[serde(rename = "Sha512")]
        Sha512,
    #[serde(rename = "RipeMD160")]
        RipeMd160,
}