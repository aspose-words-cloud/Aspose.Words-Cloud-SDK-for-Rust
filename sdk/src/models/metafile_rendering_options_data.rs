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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{SdkError, SdkResult};

use super::*;

/// Container class for options of metafile rendering.
#[derive(Debug, Deserialize, Serialize)]
pub struct MetafileRenderingOptionsData {
        /// Gets or sets the option that controls how EMF+ Dual metafiles should be rendered.
        #[serde(rename = "EmfPlusDualRenderingMode", skip_serializing_if = "Option::is_none")]
        pub r#emf_plus_dual_rendering_mode: Option<MetafileRenderingOptionsData_EmfPlusDualRenderingModeEnum>,


        /// Gets or sets a value indicating whether the raster operations should be emulated.
            /// Specific raster operations could be used in metafiles. They can not be rendered directly to vector graphics. Emulating raster operations requires partial rasterization of the resulting vector graphics which may affect the metafile rendering performance. When this value is set to true, Aspose.Words emulates the raster operations. The resulting output maybe partially rasterized and performance might be slower. When this value is set to false, Aspose.Words does not emulate the raster operations. When Aspose.Words encounters a raster operation in a metafile it fallbacks to rendering the metafile into a bitmap by using the operating system. This option is used only when metafile is rendered as vector graphics. The default value is true.
        #[serde(rename = "EmulateRasterOperations", skip_serializing_if = "Option::is_none")]
        pub r#emulate_raster_operations: Option<bool>,


        /// Gets or sets a value determining whether metafile rendering emulates the display of the metafile according to the size on page
            /// or the display of the metafile in its default size.
        #[serde(rename = "EmulateRenderingToSizeOnPage", skip_serializing_if = "Option::is_none")]
        pub r#emulate_rendering_to_size_on_page: Option<bool>,


        /// Gets or sets the resolution in pixels per inch for the emulation of metafile rendering to the size on page.
            /// This option is used only when EmulateRenderingToSizeOnPage is set to true.The default value is 96. This is a default display resolution. I.e. metafile rendering will emulate the display of
            /// the metafile in MS Word with a 100% zoom factor.
        #[serde(rename = "EmulateRenderingToSizeOnPageResolution", skip_serializing_if = "Option::is_none")]
        pub r#emulate_rendering_to_size_on_page_resolution: Option<i32>,


        /// Gets or sets the option that controls how metafile images should be rendered.
        #[serde(rename = "RenderingMode", skip_serializing_if = "Option::is_none")]
        pub r#rendering_mode: Option<MetafileRenderingOptionsData_RenderingModeEnum>,


        /// Gets or sets the flag, that controls how WMF metafiles with embedded EMF metafiles should be rendered.
        #[serde(rename = "UseEmfEmbeddedToWmf", skip_serializing_if = "Option::is_none")]
        pub r#use_emf_embedded_to_wmf: Option<bool>,

}

impl Default for MetafileRenderingOptionsData {
    fn default() -> Self {
        Self {
            r#emf_plus_dual_rendering_mode: None,
            r#emulate_raster_operations: None,
            r#emulate_rendering_to_size_on_page: None,
            r#emulate_rendering_to_size_on_page_resolution: None,
            r#rendering_mode: None,
            r#use_emf_embedded_to_wmf: None,
        }
    }
}

impl Model for MetafileRenderingOptionsData {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the option that controls how EMF+ Dual metafiles should be rendered.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MetafileRenderingOptionsData_EmfPlusDualRenderingModeEnum {
    #[serde(rename = "EmfPlusWithFallback")]
        EmfPlusWithFallback,
    #[serde(rename = "EmfPlus")]
        EmfPlus,
    #[serde(rename = "Emf")]
        Emf,
}

/// Gets or sets the option that controls how metafile images should be rendered.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MetafileRenderingOptionsData_RenderingModeEnum {
    #[serde(rename = "VectorWithFallback")]
        VectorWithFallback,
    #[serde(rename = "Vector")]
        Vector,
    #[serde(rename = "Bitmap")]
        Bitmap,
}