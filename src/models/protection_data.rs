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

/// Container for the data about protection of the document.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectionData {
    /// Gets or sets type of the protection.
    #[serde(rename = "ProtectionType", skip_serializing_if = "Option::is_none")]
    pub protection_type: Option<ProtectionDataProtectionTypeEnum>,
}

impl Default for ProtectionData {
    fn default() -> Self {
        Self {
            protection_type: None,
        }
    }
}

impl Model for ProtectionData {
    fn validate(&self) -> SdkResult<()> {
        if self.protection_type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property ProtectionType in ProtectionData is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets type of the protection.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProtectionDataProtectionTypeEnum {
    #[serde(rename = "AllowOnlyRevisions")]
    AllowOnlyRevisions,
    #[serde(rename = "AllowOnlyComments")]
    AllowOnlyComments,
    #[serde(rename = "AllowOnlyFormFields")]
    AllowOnlyFormFields,
    #[serde(rename = "ReadOnly")]
    ReadOnly,
    #[serde(rename = "NoProtection")]
    NoProtection,
}
