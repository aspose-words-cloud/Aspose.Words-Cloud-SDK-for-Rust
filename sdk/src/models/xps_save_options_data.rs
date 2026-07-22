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

/// Container class for xps save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct XpsSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
        /// Gets or sets the level in the XPS document outline at which to display Word bookmarks.
        #[serde(rename = "BookmarksOutlineLevel", skip_serializing_if = "Option::is_none")]
        pub r#bookmarks_outline_level: Option<i32>,


        /// Gets or sets the details for signing the output document.
        #[serde(rename = "DigitalSignatureDetails", skip_serializing_if = "Option::is_none")]
        pub r#digital_signature_details: Option<DigitalSignatureDetails>,


        /// Gets or sets the number of heading levels (paragraphs formatted with the Heading styles) to include in the XPS document outline.
        #[serde(rename = "HeadingsOutlineLevels", skip_serializing_if = "Option::is_none")]
        pub r#headings_outline_levels: Option<i32>,


        /// Gets or sets the outline options.
        #[serde(rename = "OutlineOptions", skip_serializing_if = "Option::is_none")]
        pub r#outline_options: Option<OutlineOptionsData>,


        /// Gets or sets a value indicating whether the document should be saved using a booklet printing layout.
        #[serde(rename = "UseBookFoldPrintingSettings", skip_serializing_if = "Option::is_none")]
        pub r#use_book_fold_printing_settings: Option<bool>,


}

impl Default for XpsSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        parent.r#save_format = Some("xps".to_owned());
        Self {
            parent,
            r#bookmarks_outline_level: None,
            r#digital_signature_details: None,
            r#headings_outline_levels: None,
            r#outline_options: None,
            r#use_book_fold_printing_settings: None,

        }
    }
}

impl Deref for XpsSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for XpsSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for XpsSaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.r#digital_signature_details {
        value.validate()?;
        }

        if let Some(value) = &self.r#outline_options {
        value.validate()?;
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

