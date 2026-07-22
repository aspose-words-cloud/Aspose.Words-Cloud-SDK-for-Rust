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

/// Class for insert watermark image request building.
#[derive(Debug, Deserialize, Serialize)]
pub struct WatermarkDataImage {
    #[serde(flatten)]
    pub parent: WatermarkDataBase,
        /// Gets or sets the watermark image.
        #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
        pub r#image: Option<FileReference>,


        /// Gets or sets a boolean value which is responsible for washout effect of the watermark. The default value is true.
        #[serde(rename = "IsWashout", skip_serializing_if = "Option::is_none")]
        pub r#is_washout: Option<bool>,


        /// Gets or sets the scale factor expressed as a fraction of the image. The default value is 0 - auto.
            /// Valid values range from 0 to 65.5 inclusive. Auto scale means that the watermark will be scaled to its max width and max height relative to the page margins.
        #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
        pub r#scale: Option<f64>,

}

impl Default for WatermarkDataImage {
    fn default() -> Self {
        let mut parent = WatermarkDataBase::default();
        Self {
            parent,
            r#image: None,
            r#is_washout: None,
            r#scale: None,
        }
    }
}

impl Deref for WatermarkDataImage {
    type Target = WatermarkDataBase;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WatermarkDataImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for WatermarkDataImage {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#image.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Image in WatermarkDataImage is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#image {
        value.validate()?;
        }


        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
        if let Some(value) = &self.r#image {
        value.collect_file_references(output);
        }


    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

