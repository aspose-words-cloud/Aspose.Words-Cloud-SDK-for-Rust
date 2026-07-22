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

/// Class for document replace text request building.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceTextParameters {
        /// Gets or sets a value indicating whether apply superscript to font or not.
        #[serde(rename = "ApplySuperscript", skip_serializing_if = "Option::is_none")]
        pub apply_superscript: Option<bool>,


        /// Gets or sets a value indicating whether flag, true means the search is case-sensitive; false means the search is not case-sensitive.
        #[serde(rename = "IsMatchCase", skip_serializing_if = "Option::is_none")]
        pub is_match_case: Option<bool>,


        /// Gets or sets a value indicating whether flag, means that only whole word matched are replaced.
        #[serde(rename = "IsMatchWholeWord", skip_serializing_if = "Option::is_none")]
        pub is_match_whole_word: Option<bool>,


        /// Gets or sets a value indicating whether flag, means that OldValue contains regex expression.
        #[serde(rename = "IsOldValueRegex", skip_serializing_if = "Option::is_none")]
        pub is_old_value_regex: Option<bool>,


        /// Gets or sets the new text value to replace by.
        #[serde(rename = "NewValue", skip_serializing_if = "Option::is_none")]
        pub new_value: Option<String>,


        /// Gets or sets the old text value (or regex pattern IsOldValueRegex) to replace.
        #[serde(rename = "OldValue", skip_serializing_if = "Option::is_none")]
        pub old_value: Option<String>,

}

impl Default for ReplaceTextParameters {
    fn default() -> Self {
        Self {
            apply_superscript: None,
            is_match_case: None,
            is_match_whole_word: None,
            is_old_value_regex: None,
            new_value: None,
            old_value: None,
        }
    }
}

impl Model for ReplaceTextParameters {
    fn validate(&self) -> SdkResult<()> {
        if self.is_match_case.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsMatchCase in ReplaceTextParameters is required".to_owned(),
            ));
        }
        if self.is_match_whole_word.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsMatchWholeWord in ReplaceTextParameters is required".to_owned(),
            ));
        }
        if self.is_old_value_regex.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsOldValueRegex in ReplaceTextParameters is required".to_owned(),
            ));
        }
        if self.new_value.is_none() {
            return Err(SdkError::InvalidRequest(
                "property NewValue in ReplaceTextParameters is required".to_owned(),
            ));
        }
        if self.old_value.is_none() {
            return Err(SdkError::InvalidRequest(
                "property OldValue in ReplaceTextParameters is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

