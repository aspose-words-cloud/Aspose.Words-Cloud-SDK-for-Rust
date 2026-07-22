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

/// Container class for Downsample options.
#[derive(Debug, Deserialize, Serialize)]
pub struct DownsampleOptionsData {
        /// Gets or sets a value indicating whether images should be downsampled.
            /// The default value is true.
        #[serde(rename = "DownsampleImages", skip_serializing_if = "Option::is_none")]
        pub r#downsample_images: Option<bool>,


        /// Gets or sets the resolution in pixels per inch which the images should be downsampled to.
            /// The default value is 220 ppi.
        #[serde(rename = "Resolution", skip_serializing_if = "Option::is_none")]
        pub r#resolution: Option<i32>,


        /// Gets or sets the threshold resolution in pixels per inch. If resolution of an image in the document is less than threshold value, the downsampling algorithm will not be applied. A value of 0 means the threshold check is not used and all images that can be reduced in size are downsampled.
            /// The default value is 0.
        #[serde(rename = "ResolutionThreshold", skip_serializing_if = "Option::is_none")]
        pub r#resolution_threshold: Option<i32>,

}

impl Default for DownsampleOptionsData {
    fn default() -> Self {
        Self {
            r#downsample_images: None,
            r#resolution: None,
            r#resolution_threshold: None,
        }
    }
}

impl Model for DownsampleOptionsData {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

