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

/// Container class for xaml flow save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct XamlFlowSaveOptionsData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
    /// Gets or sets the physical folder where images are saved when exporting.
    #[serde(rename = "ImagesFolder", skip_serializing_if = "Option::is_none")]
    pub images_folder: Option<String>,

    /// Gets or sets the name of the folder used to construct image URIs.
    #[serde(rename = "ImagesFolderAlias", skip_serializing_if = "Option::is_none")]
    pub images_folder_alias: Option<String>,

    /// Gets or sets the flag that indicates whether backslash characters should be replaced with yen signs.
    /// The default value is false.
    /// By default, Aspose.Words mimics MS Word's behavior and doesn't replace backslash characters with yen signs in
    /// generated HTML documents. However, previous versions of Aspose.Words performed such replacements in certain
    /// scenarios. This flag enables backward compatibility with previous versions of Aspose.Words.
    #[serde(
        rename = "ReplaceBackslashWithYenSign",
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_backslash_with_yen_sign: Option<bool>,
}

impl Default for XamlFlowSaveOptionsData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        parent.save_format = Some("xamlflow".to_owned());
        Self {
            parent,
            images_folder: None,
            images_folder_alias: None,
            replace_backslash_with_yen_sign: None,
        }
    }
}

impl Deref for XamlFlowSaveOptionsData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for XamlFlowSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for XamlFlowSaveOptionsData {
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
