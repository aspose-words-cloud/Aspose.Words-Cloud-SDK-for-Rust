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

/// Revision Dto.
#[derive(Debug, Deserialize, Serialize)]
pub struct Revision {
    /// Gets or sets the revision author.
    #[serde(rename = "RevisionAuthor", skip_serializing_if = "Option::is_none")]
    pub revision_author: Option<String>,

    /// Gets or sets the revision date time.
    #[serde(rename = "RevisionDateTime", skip_serializing_if = "Option::is_none")]
    pub revision_date_time: Option<chrono::DateTime<chrono::Utc>>,

    /// Gets or sets the revision text.
    #[serde(rename = "RevisionText", skip_serializing_if = "Option::is_none")]
    pub revision_text: Option<String>,

    /// Gets or sets the revision type.
    #[serde(rename = "RevisionType", skip_serializing_if = "Option::is_none")]
    pub revision_type: Option<String>,
}

impl Default for Revision {
    fn default() -> Self {
        Self {
            revision_author: None,
            revision_date_time: None,
            revision_text: None,
            revision_type: None,
        }
    }
}

impl Model for Revision {
    fn validate(&self) -> SdkResult<()> {
        if self.revision_date_time.is_none() {
            return Err(SdkError::InvalidRequest(
                "property RevisionDateTime in Revision is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
