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

/// Container class for rtf save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct RtfSaveOptionsData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
        /// Gets or sets a value indicating whether to make output RTF documents smaller in size, but if they contain RTL (right-to-left) text, it will not be displayed correctly.
        #[serde(rename = "ExportCompactSize", skip_serializing_if = "Option::is_none")]
        pub r#export_compact_size: Option<bool>,


        /// Gets or sets a value indicating whether the keywords for "old readers" are written to RTF or not.
        #[serde(rename = "ExportImagesForOldReaders", skip_serializing_if = "Option::is_none")]
        pub r#export_images_for_old_readers: Option<bool>,


        /// Gets or sets a value indicating whether to use pretty formats output.
        #[serde(rename = "PrettyFormat", skip_serializing_if = "Option::is_none")]
        pub r#pretty_format: Option<bool>,


        /// Gets or sets a value indicating whether when true all images will be saved as WMF. This option might help to avoid WordPad warning messages.
        #[serde(rename = "SaveImagesAsWmf", skip_serializing_if = "Option::is_none")]
        pub r#save_images_as_wmf: Option<bool>,


}

impl Default for RtfSaveOptionsData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        parent.r#save_format = Some("rtf".to_owned());
        Self {
            parent,
            r#export_compact_size: None,
            r#export_images_for_old_readers: None,
            r#pretty_format: None,
            r#save_images_as_wmf: None,

        }
    }
}

impl Deref for RtfSaveOptionsData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for RtfSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for RtfSaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

