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

/// Container class for text save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct TextSaveOptionsData {
    #[serde(flatten)]
    pub parent: TxtSaveOptionsBaseData,
        /// Gets or sets a value indicating whether to add bi-directional marks before each BiDi run when exporting in plain text format.
            /// The default value is true.
        #[serde(rename = "AddBidiMarks", skip_serializing_if = "Option::is_none")]
        pub add_bidi_marks: Option<bool>,


        /// Gets or sets an integer value that specifies the maximum number of characters per one line.
            /// The default value is 0, that means no limit.
        #[serde(rename = "MaxCharactersPerLine", skip_serializing_if = "Option::is_none")]
        pub max_characters_per_line: Option<i32>,


        /// Gets or sets a value that specifies how OfficeMath will be written to the output file.
            /// The default value is Text.
        #[serde(rename = "OfficeMathExportMode", skip_serializing_if = "Option::is_none")]
        pub office_math_export_mode: Option<TextSaveOptionsDataOfficeMathExportModeEnum>,


        /// Gets or sets a value indicating whether the program should attempt to preserve layout of tables when saving in the plain text format.
        #[serde(rename = "PreserveTableLayout", skip_serializing_if = "Option::is_none")]
        pub preserve_table_layout: Option<bool>,


        /// Gets or sets a value indicating whether the program should simplify list labels in case of complex label formatting not being adequately represented by plain text.
        #[serde(rename = "SimplifyListLabels", skip_serializing_if = "Option::is_none")]
        pub simplify_list_labels: Option<bool>,


}

impl Default for TextSaveOptionsData {
    fn default() -> Self {
        let mut parent = TxtSaveOptionsBaseData::default();
        parent.save_format = Some("txt".to_owned());
        Self {
            parent,
            add_bidi_marks: None,
            max_characters_per_line: None,
            office_math_export_mode: None,
            preserve_table_layout: None,
            simplify_list_labels: None,

        }
    }
}

impl Deref for TextSaveOptionsData {
    type Target = TxtSaveOptionsBaseData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TextSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TextSaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.max_characters_per_line.is_none() {
            return Err(SdkError::InvalidRequest(
                "property MaxCharactersPerLine in TextSaveOptionsData is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets a value that specifies how OfficeMath will be written to the output file.
/// The default value is Text.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TextSaveOptionsDataOfficeMathExportModeEnum {
    #[serde(rename = "Text")]
        Text,
    #[serde(rename = "Latex")]
        Latex,
}