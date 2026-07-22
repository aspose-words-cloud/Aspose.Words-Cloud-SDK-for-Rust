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

/// Container class for docx/docm/dotx/dotm/flatopc save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct OoxmlSaveOptionsData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
        /// Gets or sets the oOXML version for the output document.
        #[serde(rename = "Compliance", skip_serializing_if = "Option::is_none")]
        pub r#compliance: Option<OoxmlSaveOptionsData_ComplianceEnum>,


        /// Gets or sets the compression level.
        #[serde(rename = "CompressionLevel", skip_serializing_if = "Option::is_none")]
        pub r#compression_level: Option<OoxmlSaveOptionsData_CompressionLevelEnum>,


        /// Gets or sets the password to encrypt document using ECMA376 Standard encryption algorithm.
        #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
        pub r#password: Option<String>,


        /// Gets or sets a value indicating whether to use pretty formats output.
        #[serde(rename = "PrettyFormat", skip_serializing_if = "Option::is_none")]
        pub r#pretty_format: Option<bool>,

}

impl Default for OoxmlSaveOptionsData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        Self {
            parent,
            r#compliance: None,
            r#compression_level: None,
            r#password: None,
            r#pretty_format: None,
        }
    }
}

impl Deref for OoxmlSaveOptionsData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for OoxmlSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for OoxmlSaveOptionsData {
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

/// Gets or sets the oOXML version for the output document.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OoxmlSaveOptionsData_ComplianceEnum {
    #[serde(rename = "Ecma376_2006")]
        Ecma376_2006,
    #[serde(rename = "Iso29500_2008_Transitional")]
        Iso29500_2008_Transitional,
    #[serde(rename = "Iso29500_2008_Strict")]
        Iso29500_2008_Strict,
}

/// Gets or sets the compression level.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OoxmlSaveOptionsData_CompressionLevelEnum {
    #[serde(rename = "Normal")]
        Normal,
    #[serde(rename = "Maximum")]
        Maximum,
    #[serde(rename = "Fast")]
        Fast,
    #[serde(rename = "SuperFast")]
        SuperFast,
}