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

/// result of the operation which modifies the original document and saves the result.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModificationOperationResult {
        /// Gets or sets the link to the dest document (result of the modification operation).
        #[serde(rename = "Dest", skip_serializing_if = "Option::is_none")]
        pub dest: Option<FileLink>,


        /// Gets or sets the link to the source document (source for the modification operation).
        #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
        pub source: Option<FileLink>,

}

impl Default for ModificationOperationResult {
    fn default() -> Self {
        Self {
            dest: None,
            source: None,
        }
    }
}

impl Model for ModificationOperationResult {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.dest {
        value.validate()?;
        }
        if let Some(value) = &self.source {
        value.validate()?;
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

