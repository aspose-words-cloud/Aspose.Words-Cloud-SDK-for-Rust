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

/// Container class for tiff save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct TiffSaveOptionsData {
    #[serde(flatten)]
    pub parent: ImageSaveOptionsData,
        /// Gets or sets the threshold that determines the value of the binarization error in the Floyd-Steinberg method. when ImageBinarizationMethod is ImageBinarizationMethod.FloydSteinbergDithering.
            /// The default value is 128.
        #[serde(rename = "ThresholdForFloydSteinbergDithering", skip_serializing_if = "Option::is_none")]
        pub r#threshold_for_floyd_steinberg_dithering: Option<i32>,


        /// Gets or sets the method used while converting images to 1 bpp format.
        #[serde(rename = "TiffBinarizationMethod", skip_serializing_if = "Option::is_none")]
        pub r#tiff_binarization_method: Option<TiffSaveOptionsData_TiffBinarizationMethodEnum>,


        /// Gets or sets the type of compression.
        #[serde(rename = "TiffCompression", skip_serializing_if = "Option::is_none")]
        pub r#tiff_compression: Option<TiffSaveOptionsData_TiffCompressionEnum>,


}

impl Default for TiffSaveOptionsData {
    fn default() -> Self {
        let mut parent = ImageSaveOptionsData::default();
        parent.r#save_format = Some("tiff".to_owned());
        Self {
            parent,
            r#threshold_for_floyd_steinberg_dithering: None,
            r#tiff_binarization_method: None,
            r#tiff_compression: None,

        }
    }
}

impl Deref for TiffSaveOptionsData {
    type Target = ImageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TiffSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TiffSaveOptionsData {
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

/// Gets or sets the method used while converting images to 1 bpp format.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TiffSaveOptionsData_TiffBinarizationMethodEnum {
    #[serde(rename = "Threshold")]
        Threshold,
    #[serde(rename = "FloydSteinbergDithering")]
        FloydSteinbergDithering,
}

/// Gets or sets the type of compression.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TiffSaveOptionsData_TiffCompressionEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Rle")]
        Rle,
    #[serde(rename = "Lzw")]
        Lzw,
    #[serde(rename = "Ccitt3")]
        Ccitt3,
    #[serde(rename = "Ccitt4")]
        Ccitt4,
}