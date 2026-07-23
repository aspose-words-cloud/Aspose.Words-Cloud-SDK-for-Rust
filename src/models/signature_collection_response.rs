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
use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{SdkError, SdkResult};

use super::*;

/// The REST response with a document signature collection.
/// This response is returned by the Service when handling any "https://api.aspose.cloud/v4.0/words/Test.doc/signatures" REST API requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureCollectionResponse {
    #[serde(flatten)]
    pub parent: WordsResponse,
    /// Gets or sets a value indicating whether all signatures are valid. Returns true if there is no signatures.
    #[serde(rename = "IsValid", skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,

    /// Gets or sets signatures.
    #[serde(rename = "Signatures", skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<Signature>>,
}

impl Default for SignatureCollectionResponse {
    fn default() -> Self {
        let mut parent = WordsResponse::default();
        Self {
            parent,
            is_valid: None,
            signatures: None,
        }
    }
}

impl Deref for SignatureCollectionResponse {
    type Target = WordsResponse;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for SignatureCollectionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for SignatureCollectionResponse {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.is_valid.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsValid in SignatureCollectionResponse is required".to_owned(),
            ));
        }
        if let Some(values) = &self.signatures {
            for value in values {
                value.validate()?;
            }
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
