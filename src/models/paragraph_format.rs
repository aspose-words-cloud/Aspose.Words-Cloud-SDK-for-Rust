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

/// Paragraph format element.
#[derive(Debug, Deserialize, Serialize)]
pub struct ParagraphFormat {
    #[serde(flatten)]
    pub parent: ParagraphFormatBase,
        /// Gets or sets a value indicating whether the paragraph is an item in a bulleted or numbered list.
        #[serde(rename = "IsListItem", skip_serializing_if = "Option::is_none")]
        pub is_list_item: Option<bool>,


        /// Gets or sets a value indicating whether the paragraph style is one of the built-in Heading styles.
        #[serde(rename = "IsHeading", skip_serializing_if = "Option::is_none")]
        pub is_heading: Option<bool>,

}

impl Default for ParagraphFormat {
    fn default() -> Self {
        let mut parent = ParagraphFormatBase::default();
        Self {
            parent,
            is_list_item: None,
            is_heading: None,
        }
    }
}

impl Deref for ParagraphFormat {
    type Target = ParagraphFormatBase;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for ParagraphFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for ParagraphFormat {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

