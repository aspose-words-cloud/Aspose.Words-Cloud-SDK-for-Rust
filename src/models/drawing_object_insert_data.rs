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

/// Drawing object element for insert with the image passed by reference.
#[derive(Debug, Deserialize, Serialize)]
pub struct DrawingObjectInsertData {
    #[serde(flatten)]
    pub parent: DrawingObjectInsert,
        /// Gets or sets the reference to the image file.
            /// Source "Storage": the reference is either a storage path to the image located in the folder of the document or in one of its subfolders,
            /// or an absolute HTTP(S) URL to download the image from. For online methods any storage path is allowed.
            /// Source "Request": the reference is the name of the multipart request part that contains the image.
        #[serde(rename = "ImageReference", skip_serializing_if = "Option::is_none")]
        pub image_reference: Option<FileReference>,

}

impl Default for DrawingObjectInsertData {
    fn default() -> Self {
        let mut parent = DrawingObjectInsert::default();
        Self {
            parent,
            image_reference: None,
        }
    }
}

impl Deref for DrawingObjectInsertData {
    type Target = DrawingObjectInsert;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for DrawingObjectInsertData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for DrawingObjectInsertData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.image_reference.is_none() {
            return Err(SdkError::InvalidRequest(
                "property ImageReference in DrawingObjectInsertData is required".to_owned(),
            ));
        }
        if let Some(value) = &self.image_reference {
        value.validate()?;
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
        if let Some(value) = &self.image_reference {
        value.collect_file_references(_output);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

