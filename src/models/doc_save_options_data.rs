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

/// Container class for doc/dot save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocSaveOptionsData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
        /// Gets or sets a value indicating when False, that small metafiles are not compressed for performance reason.
            /// The default value is true, all metafiles are compressed regardless of its size.
        #[serde(rename = "AlwaysCompressMetafiles", skip_serializing_if = "Option::is_none")]
        pub always_compress_metafiles: Option<bool>,


        /// Gets or sets the password.
        #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,


        /// Gets or sets a value indicating when False, that PictureBullet data is not saved to the output document.
            /// The default value is true.
        #[serde(rename = "SavePictureBullet", skip_serializing_if = "Option::is_none")]
        pub save_picture_bullet: Option<bool>,


        /// Gets or sets a value indicating whether to save RoutingSlip data to output document.
        #[serde(rename = "SaveRoutingSlip", skip_serializing_if = "Option::is_none")]
        pub save_routing_slip: Option<bool>,


}

impl Default for DocSaveOptionsData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        parent.save_format = Some("doc".to_owned());
        Self {
            parent,
            always_compress_metafiles: None,
            password: None,
            save_picture_bullet: None,
            save_routing_slip: None,

        }
    }
}

impl Deref for DocSaveOptionsData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for DocSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for DocSaveOptionsData {
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

