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

/// Container abstract class for image save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
        /// Gets or sets the horizontal resolution in dots per inch for the generated images.
            /// This property has effect only when saving to raster image formats.
            /// The default value is 96.
        #[serde(rename = "HorizontalResolution", skip_serializing_if = "Option::is_none")]
        pub r#horizontal_resolution: Option<f64>,


        /// Gets or sets the brightness level of the image.
        #[serde(rename = "ImageBrightness", skip_serializing_if = "Option::is_none")]
        pub r#image_brightness: Option<f64>,


        /// Gets or sets the color mode of the image.
        #[serde(rename = "ImageColorMode", skip_serializing_if = "Option::is_none")]
        pub r#image_color_mode: Option<ImageSaveOptionsData_ImageColorModeEnum>,


        /// Gets or sets the contrast level of the image.
        #[serde(rename = "ImageContrast", skip_serializing_if = "Option::is_none")]
        pub r#image_contrast: Option<f64>,


        /// Gets or sets the background (paper) color of the image.
        #[serde(rename = "PaperColor", skip_serializing_if = "Option::is_none")]
        pub r#paper_color: Option<String>,


        /// Gets or sets the pixel format of the image.
        #[serde(rename = "PixelFormat", skip_serializing_if = "Option::is_none")]
        pub r#pixel_format: Option<ImageSaveOptionsData_PixelFormatEnum>,


        /// Gets or sets both horizontal and vertical resolution in dots per inch for the generated images.
            /// This property has effect only when saving to raster image formats.
            /// The default value is 96.
        #[serde(rename = "Resolution", skip_serializing_if = "Option::is_none")]
        pub r#resolution: Option<f64>,


        /// Gets or sets the zoom factor of the image.
        #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
        pub r#scale: Option<f64>,


        /// Gets or sets a value indicating whether to use anti-aliasing for rendering.
        #[serde(rename = "UseAntiAliasing", skip_serializing_if = "Option::is_none")]
        pub r#use_anti_aliasing: Option<bool>,


        /// Gets or sets a value indicating whether to use high quality (i.e. slow) rendering algorithms.
        #[serde(rename = "UseHighQualityRendering", skip_serializing_if = "Option::is_none")]
        pub r#use_high_quality_rendering: Option<bool>,


        /// Gets or sets the vertical resolution in dots per inch for the generated images.
            /// This property has effect only when saving to raster image formats.
            /// The default value is 96.
        #[serde(rename = "VerticalResolution", skip_serializing_if = "Option::is_none")]
        pub r#vertical_resolution: Option<f64>,


        /// Gets or sets the height of a generated image in pixels.
            /// This property has effect only when saving to raster image formats
            /// and used in pair with ImageWidth.
        #[serde(rename = "ImageHeight", skip_serializing_if = "Option::is_none")]
        pub r#image_height: Option<i32>,


        /// Gets or sets the width of a generated image in pixels.
            /// This property has effect only when saving to raster image formats
            /// and used in pair with ImageHeight.
        #[serde(rename = "ImageWidth", skip_serializing_if = "Option::is_none")]
        pub r#image_width: Option<i32>,


        /// Gets or sets a value indicating whether to use GDI+ or Aspose.Words metafile renderer when saving to EMF.
            /// If set to true - GDI+ metafile renderer is used. I.e. content is written to GDI+ graphics object and saved to metafile.If set to false - Aspose.Words metafile renderer is used. I.e. content is written directly to the metafile format with Aspose.Words.The default value is true.Has effect only when saving to EMF.
        #[serde(rename = "UseGdiEmfRenderer", skip_serializing_if = "Option::is_none")]
        pub r#use_gdi_emf_renderer: Option<bool>,

}

impl Default for ImageSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        Self {
            parent,
            r#horizontal_resolution: None,
            r#image_brightness: None,
            r#image_color_mode: None,
            r#image_contrast: None,
            r#paper_color: None,
            r#pixel_format: None,
            r#resolution: None,
            r#scale: None,
            r#use_anti_aliasing: None,
            r#use_high_quality_rendering: None,
            r#vertical_resolution: None,
            r#image_height: None,
            r#image_width: None,
            r#use_gdi_emf_renderer: None,
        }
    }
}

impl Deref for ImageSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for ImageSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for ImageSaveOptionsData {
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

/// Gets or sets the color mode of the image.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ImageSaveOptionsData_ImageColorModeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Grayscale")]
        Grayscale,
    #[serde(rename = "BlackAndWhite")]
        BlackAndWhite,
}

/// Gets or sets the pixel format of the image.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ImageSaveOptionsData_PixelFormatEnum {
    #[serde(rename = "Format16BppRgb555")]
        Format16BppRgb555,
    #[serde(rename = "Format16BppRgb565")]
        Format16BppRgb565,
    #[serde(rename = "Format16BppArgb1555")]
        Format16BppArgb1555,
    #[serde(rename = "Format24BppRgb")]
        Format24BppRgb,
    #[serde(rename = "Format32BppRgb")]
        Format32BppRgb,
    #[serde(rename = "Format32BppArgb")]
        Format32BppArgb,
    #[serde(rename = "Format32BppPArgb")]
        Format32BppPArgb,
    #[serde(rename = "Format48BppRgb")]
        Format48BppRgb,
    #[serde(rename = "Format64BppArgb")]
        Format64BppArgb,
    #[serde(rename = "Format64BppPArgb")]
        Format64BppPArgb,
    #[serde(rename = "Format1bppIndexed")]
        Format1bppIndexed,
}