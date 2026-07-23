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

/// Container class for pcl save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct PclSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
    /// Gets or sets the font name, that will be used if no expected font is found in printer and built-in fonts collections.
    /// If no fallback is found, "Arial" font is used.
    #[serde(rename = "FalllbackFontName", skip_serializing_if = "Option::is_none")]
    pub falllback_font_name: Option<String>,

    /// Gets or sets a value indicating whether complex transformed elements should be rasterized before saving to PCL document.. The default value is true.
    /// PCL doesn't support some kind of transformations that are used by Aspose Words.  E.g. rotated, skewed images and texture brushes. To properly render such elements rasterization process is used, i.e. saving to image and clipping.  This process can take additional time and memory.  If flag is set to false, some content in output may be different as compared with the source document.
    #[serde(
        rename = "RasterizeTransformedElements",
        skip_serializing_if = "Option::is_none"
    )]
    pub rasterize_transformed_elements: Option<bool>,
}

impl Default for PclSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        parent.save_format = Some("pcl".to_owned());
        Self {
            parent,
            falllback_font_name: None,
            rasterize_transformed_elements: None,
        }
    }
}

impl Deref for PclSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for PclSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for PclSaveOptionsData {
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
