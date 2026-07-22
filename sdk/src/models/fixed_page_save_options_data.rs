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

/// Contains common options that can be specified when saving a document into fixed page formats (PDF, XPS, images etc).
#[derive(Debug, Deserialize, Serialize)]
pub struct FixedPageSaveOptionsData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
        /// Gets or sets the value determining how colors are rendered.
            /// { Normal | Grayscale}.
            /// The default value is Normal.
            /// This property is used when the document is exported to fixed page formats.
        #[serde(rename = "ColorMode", skip_serializing_if = "Option::is_none")]
        pub color_mode: Option<FixedPageSaveOptionsDataColorModeEnum>,


        /// Gets or sets the quality of the JPEG images inside PDF document.
        #[serde(rename = "JpegQuality", skip_serializing_if = "Option::is_none")]
        pub jpeg_quality: Option<i32>,


        /// Gets or sets the metafile rendering options.
        #[serde(rename = "MetafileRenderingOptions", skip_serializing_if = "Option::is_none")]
        pub metafile_rendering_options: Option<MetafileRenderingOptionsData>,


        /// Gets or sets the symbol set, that is used to represent numbers while rendering to fixed page formats.
        #[serde(rename = "NumeralFormat", skip_serializing_if = "Option::is_none")]
        pub numeral_format: Option<FixedPageSaveOptionsDataNumeralFormatEnum>,


        /// Gets or sets a value indicating whether it is required to optimize output of XPS.
            /// If this flag is set redundant nested canvases and empty canvases are removed, also neighbor glyphs with the same formatting are concatenated.
            /// Note: The accuracy of the content display may be affected if this property is set to true.. The default value is false.
        #[serde(rename = "OptimizeOutput", skip_serializing_if = "Option::is_none")]
        pub optimize_output: Option<bool>,


        /// Gets or sets the number of pages to render.
        #[serde(rename = "PageCount", skip_serializing_if = "Option::is_none")]
        pub page_count: Option<i32>,


        /// Gets or sets the 0-based index of the first page to render.
        #[serde(rename = "PageIndex", skip_serializing_if = "Option::is_none")]
        pub page_index: Option<i32>,

}

impl Default for FixedPageSaveOptionsData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        Self {
            parent,
            color_mode: None,
            jpeg_quality: None,
            metafile_rendering_options: None,
            numeral_format: None,
            optimize_output: None,
            page_count: None,
            page_index: None,
        }
    }
}

impl Deref for FixedPageSaveOptionsData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FixedPageSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for FixedPageSaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.metafile_rendering_options {
        value.validate()?;
        }




        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the value determining how colors are rendered.
/// { Normal | Grayscale}.
/// The default value is Normal.
/// This property is used when the document is exported to fixed page formats.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FixedPageSaveOptionsDataColorModeEnum {
    #[serde(rename = "Normal")]
        Normal,
    #[serde(rename = "Grayscale")]
        Grayscale,
}

/// Gets or sets the symbol set, that is used to represent numbers while rendering to fixed page formats.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FixedPageSaveOptionsDataNumeralFormatEnum {
    #[serde(rename = "European")]
        European,
    #[serde(rename = "ArabicIndic")]
        ArabicIndic,
    #[serde(rename = "EasternArabicIndic")]
        EasternArabicIndic,
    #[serde(rename = "Context")]
        Context,
    #[serde(rename = "System")]
        System,
}