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

/// Base class for save options of text formats.
#[derive(Debug, Deserialize, Serialize)]
pub struct TxtSaveOptionsBaseData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
        /// Gets or sets the character encoding to use when exporting in plain text format.
        #[serde(rename = "Encoding", skip_serializing_if = "Option::is_none")]
        pub r#encoding: Option<String>,


        /// Gets or sets the option that controls whether to output headers and footers when exporting in plain text format.
            /// The default value is TxtExportHeadersFootersMode.PrimaryOnly.
        #[serde(rename = "ExportHeadersFootersMode", skip_serializing_if = "Option::is_none")]
        pub r#export_headers_footers_mode: Option<TxtSaveOptionsBaseData_ExportHeadersFootersModeEnum>,


        /// Gets or sets a value indicating whether the page breaks should be preserved during export.
            /// The default value is false.
        #[serde(rename = "ForcePageBreaks", skip_serializing_if = "Option::is_none")]
        pub r#force_page_breaks: Option<bool>,


        /// Gets or sets the string to use as a paragraph break when exporting in plain text format.
        #[serde(rename = "ParagraphBreak", skip_serializing_if = "Option::is_none")]
        pub r#paragraph_break: Option<String>,

}

impl Default for TxtSaveOptionsBaseData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        Self {
            parent,
            r#encoding: None,
            r#export_headers_footers_mode: None,
            r#force_page_breaks: None,
            r#paragraph_break: None,
        }
    }
}

impl Deref for TxtSaveOptionsBaseData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TxtSaveOptionsBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TxtSaveOptionsBaseData {
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

/// Gets or sets the option that controls whether to output headers and footers when exporting in plain text format.
/// The default value is TxtExportHeadersFootersMode.PrimaryOnly.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TxtSaveOptionsBaseData_ExportHeadersFootersModeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "PrimaryOnly")]
        PrimaryOnly,
    #[serde(rename = "AllAtEnd")]
        AllAtEnd,
}