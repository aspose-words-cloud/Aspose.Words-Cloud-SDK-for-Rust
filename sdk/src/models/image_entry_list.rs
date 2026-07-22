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

/// Represents a list of images which will be appended to the original resource document or image.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageEntryList {
    #[serde(flatten)]
    pub parent: BaseEntryList,
        /// Gets or sets a value indicating whether each image should be added to a new page in the document.
            /// This value only has an effect when adding images to a document that supports pagination.
        #[serde(rename = "AppendEachImageOnNewPage", skip_serializing_if = "Option::is_none")]
        pub r#append_each_image_on_new_page: Option<bool>,


        /// Gets or sets the list of images.
        #[serde(rename = "ImageEntries", skip_serializing_if = "Option::is_none")]
        pub r#image_entries: Option<Vec<ImageEntry>>,

}

impl Default for ImageEntryList {
    fn default() -> Self {
        let mut parent = BaseEntryList::default();
        Self {
            parent,
            r#append_each_image_on_new_page: None,
            r#image_entries: None,
        }
    }
}

impl Deref for ImageEntryList {
    type Target = BaseEntryList;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for ImageEntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for ImageEntryList {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#image_entries.is_none() {
            return Err(SdkError::InvalidRequest(
                "property ImageEntries in ImageEntryList is required".to_owned(),
            ));
        }
        if let Some(values) = &self.r#image_entries {
        for value in values {
        value.validate()?;
        }
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
        if let Some(values) = &self.r#image_entries {
        for value in values {
        value.collect_file_references(output);
        }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

