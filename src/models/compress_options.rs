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

/// Options of document compress.
#[derive(Debug, Deserialize, Serialize)]
pub struct CompressOptions {
        /// Gets or sets the quality level of images from 0 to 100. The default value is 75.
        #[serde(rename = "ImagesQuality", skip_serializing_if = "Option::is_none")]
        pub images_quality: Option<i32>,


        /// Gets or sets the resize factor of images.
            /// This value determines how many times the size of the images in the document will be reduced.
            /// The parameter value must be greater than 1 for resizing. The default value is 1 and has no effect on images size.
        #[serde(rename = "ImagesReduceSizeFactor", skip_serializing_if = "Option::is_none")]
        pub images_reduce_size_factor: Option<i32>,

}

impl Default for CompressOptions {
    fn default() -> Self {
        Self {
            images_quality: None,
            images_reduce_size_factor: None,
        }
    }
}

impl Model for CompressOptions {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

