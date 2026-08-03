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

/// Container class for svg save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct SvgSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
        /// Gets or sets a value indicating whether images should be embedded into SVG document as base64.
        #[serde(rename = "ExportEmbeddedImages", skip_serializing_if = "Option::is_none")]
        pub export_embedded_images: Option<bool>,


        /// Gets or sets a value indicating whether the output SVG should fill the available viewport area (browser window or container). When set to true width and height of output SVG are set to 100%.
        #[serde(rename = "FitToViewPort", skip_serializing_if = "Option::is_none")]
        pub fit_to_view_port: Option<bool>,


        /// Gets or sets specifies a prefix that is prepended to all generated element IDs in the output document.
            /// The default value is null and no prefix is prepended.
            /// If the prefix is specified, it can contain only letters, digits, underscores, and hyphens,
            /// and must start with a letter.
        #[serde(rename = "IdPrefix", skip_serializing_if = "Option::is_none")]
        pub id_prefix: Option<String>,


        /// Gets or sets a value in pixels per inch that limits resolution of exported raster images.
            /// If the value of this property is non-zero, it limits resolution of exported raster images.
            /// That is, higher-resolution images are resampled down to the limit and lower-resolution images are exported as is.
        #[serde(rename = "MaxImageResolution", skip_serializing_if = "Option::is_none")]
        pub max_image_resolution: Option<i32>,


        /// Gets or sets the physical folder where resources (images) are saved when exporting.
        #[serde(rename = "ResourcesFolder", skip_serializing_if = "Option::is_none")]
        pub resources_folder: Option<String>,


        /// Gets or sets the name of the folder used to construct image URIs.
        #[serde(rename = "ResourcesFolderAlias", skip_serializing_if = "Option::is_none")]
        pub resources_folder_alias: Option<String>,


        /// Gets or sets a value indicating whether to show or hide page stepper.
        #[serde(rename = "ShowPageBorder", skip_serializing_if = "Option::is_none")]
        pub show_page_border: Option<bool>,


        /// Gets or sets the option that controls how text should be rendered.
        #[serde(rename = "TextOutputMode", skip_serializing_if = "Option::is_none")]
        pub text_output_mode: Option<SvgSaveOptionsDataTextOutputModeEnum>,


}

impl Default for SvgSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        parent.save_format = Some("svg".to_owned());
        Self {
            parent,
            export_embedded_images: None,
            fit_to_view_port: None,
            id_prefix: None,
            max_image_resolution: None,
            resources_folder: None,
            resources_folder_alias: None,
            show_page_border: None,
            text_output_mode: None,

        }
    }
}

impl Deref for SvgSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for SvgSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for SvgSaveOptionsData {
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

/// Gets or sets the option that controls how text should be rendered.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SvgSaveOptionsDataTextOutputModeEnum {
    #[serde(rename = "UseSvgFonts")]
        UseSvgFonts,
    #[serde(rename = "UseTargetMachineFonts")]
        UseTargetMachineFonts,
    #[serde(rename = "UsePlacedGlyphs")]
        UsePlacedGlyphs,
}