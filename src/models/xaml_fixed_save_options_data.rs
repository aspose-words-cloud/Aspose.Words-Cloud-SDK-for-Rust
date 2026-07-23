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

/// Container class for xaml fixed save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct XamlFixedSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
    /// Gets or sets the physical folder where resources (images and fonts) are saved when exporting a document to fixed page Xaml format.
    /// The default value is null.
    /// When you save a Document in fixed page Xaml format, Aspose.Words needs to save all images embedded in the document as standalone files. ResourcesFolder allows you to specify where the images will be saved and ResourcesFolderAlias allows to specify how the image URIs will be constructed.If you save a document into a file and provide a file name, Aspose.Words, by default, saves the images in the same folder where the document file is saved. Use ResourcesFolder to override this behavior.If you save a document into a stream, Aspose.Words does not have a folder where to save the images, but still needs to save the images somewhere. In this case, you need to specify an accessible folder by using the ResourcesFolder property.
    #[serde(rename = "ResourcesFolder", skip_serializing_if = "Option::is_none")]
    pub resources_folder: Option<String>,

    /// Gets or sets the name of the folder used to construct image URIs written into an fixed page Xaml document. The default value is null.
    /// When you save a Document in fixed page Xaml format, Aspose.Words needs to save all images embedded in the document as standalone files. ResourcesFolder allows you to specify where the images will be saved and ResourcesFolderAlias allows to specify how the image URIs will be constructed.
    #[serde(
        rename = "ResourcesFolderAlias",
        skip_serializing_if = "Option::is_none"
    )]
    pub resources_folder_alias: Option<String>,
}

impl Default for XamlFixedSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        parent.save_format = Some("xamlfixed".to_owned());
        Self {
            parent,
            resources_folder: None,
            resources_folder_alias: None,
        }
    }
}

impl Deref for XamlFixedSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for XamlFixedSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for XamlFixedSaveOptionsData {
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
