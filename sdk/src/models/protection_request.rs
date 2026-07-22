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

/// Request on changing of protection.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectionRequest {
    #[serde(flatten)]
    pub parent: ProtectionRequestBase,
        /// Gets or sets the new password.
        #[serde(rename = "NewPassword", skip_serializing_if = "Option::is_none")]
        pub r#new_password: Option<String>,


        /// Gets or sets the current password.
        #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
        pub r#password: Option<String>,


        /// Gets or sets the new type of protection.
        #[serde(rename = "ProtectionType", skip_serializing_if = "Option::is_none")]
        pub r#protection_type: Option<String>,

}

impl Default for ProtectionRequest {
    fn default() -> Self {
        let mut parent = ProtectionRequestBase::default();
        Self {
            parent,
            r#new_password: None,
            r#password: None,
            r#protection_type: None,
        }
    }
}

impl Deref for ProtectionRequest {
    type Target = ProtectionRequestBase;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for ProtectionRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for ProtectionRequest {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#password.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Password in ProtectionRequest is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

