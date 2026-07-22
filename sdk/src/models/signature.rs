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

/// The REST response with a document signature collection.
/// This response is returned by the Service when handling any "https://api.aspose.cloud/v4.0/words/Test.doc/signatures" REST API requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
        /// Gets or sets the signing purpose comment.
        #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
        pub comments: Option<String>,


        /// Gets or sets the subject distinguished name of the certificate issuer.
        #[serde(rename = "IssuerName", skip_serializing_if = "Option::is_none")]
        pub issuer_name: Option<String>,


        /// Gets or sets a value indicating whether this digital signature is valid.
        #[serde(rename = "IsValid", skip_serializing_if = "Option::is_none")]
        pub is_valid: Option<bool>,


        /// Gets or sets the type of the digital signature.
        #[serde(rename = "SignatureType", skip_serializing_if = "Option::is_none")]
        pub signature_type: Option<String>,


        /// Gets or sets an array of bytes representing a signature value as base64 string.
        #[serde(rename = "SignatureValue", skip_serializing_if = "Option::is_none")]
        pub signature_value: Option<String>,


        /// Gets or sets the time the document was signed.
        #[serde(rename = "SignTime", skip_serializing_if = "Option::is_none")]
        pub sign_time: Option<chrono::DateTime<chrono::Utc>>,


        /// Gets or sets the subject distinguished name of the certificate that was used to sign the document.
        #[serde(rename = "SubjectName", skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,

}

impl Default for Signature {
    fn default() -> Self {
        Self {
            comments: None,
            issuer_name: None,
            is_valid: None,
            signature_type: None,
            signature_value: None,
            sign_time: None,
            subject_name: None,
        }
    }
}

impl Model for Signature {
    fn validate(&self) -> SdkResult<()> {
        if self.is_valid.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsValid in Signature is required".to_owned(),
            ));
        }
        if self.sign_time.is_none() {
            return Err(SdkError::InvalidRequest(
                "property SignTime in Signature is required".to_owned(),
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

