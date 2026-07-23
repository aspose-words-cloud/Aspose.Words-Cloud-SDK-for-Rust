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

/// Container class for odt/ott save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct OdtSaveOptionsData {
    #[serde(flatten)]
    pub parent: SaveOptionsData,
    /// Gets or sets a value indicating whether export should correspond to ODT specification 1.1 strictly.
    #[serde(rename = "IsStrictSchema11", skip_serializing_if = "Option::is_none")]
    pub is_strict_schema11: Option<bool>,

    /// Gets or sets the units of measure to apply to document content. The default value is Aspose.Words.Saving.OdtSaveMeasureUnit.Centimeters.
    /// Open Office uses centimeters when specifying lengths, widths and other measurable formatting and content properties in documents whereas MS Office uses inches.
    #[serde(rename = "MeasureUnit", skip_serializing_if = "Option::is_none")]
    pub measure_unit: Option<OdtSaveOptionsDataMeasureUnitEnum>,

    /// Gets or sets the password to encrypt document.
    /// In order to save document without encryption this property should be null or empty string.
    #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Gets or sets a value indicating whether to use pretty formats output.
    #[serde(rename = "PrettyFormat", skip_serializing_if = "Option::is_none")]
    pub pretty_format: Option<bool>,
}

impl Default for OdtSaveOptionsData {
    fn default() -> Self {
        let mut parent = SaveOptionsData::default();
        parent.save_format = Some("odt".to_owned());
        Self {
            parent,
            is_strict_schema11: None,
            measure_unit: None,
            password: None,
            pretty_format: None,
        }
    }
}

impl Deref for OdtSaveOptionsData {
    type Target = SaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for OdtSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for OdtSaveOptionsData {
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

/// Gets or sets the units of measure to apply to document content. The default value is Aspose.Words.Saving.OdtSaveMeasureUnit.Centimeters.
/// Open Office uses centimeters when specifying lengths, widths and other measurable formatting and content properties in documents whereas MS Office uses inches.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OdtSaveOptionsDataMeasureUnitEnum {
    #[serde(rename = "Centimeters")]
    Centimeters,
    #[serde(rename = "Inches")]
    Inches,
}
