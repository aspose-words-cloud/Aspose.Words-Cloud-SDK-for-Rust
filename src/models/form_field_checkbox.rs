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

/// FormField checkbox element.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormFieldCheckbox {
    #[serde(flatten)]
    pub parent: FormField,
        /// Gets or sets a value indicating whether the size of the textbox is automatic or specified explicitly.
        #[serde(rename = "IsCheckBoxExactSize", skip_serializing_if = "Option::is_none")]
        pub is_check_box_exact_size: Option<bool>,


        /// Gets or sets the size of the checkbox in points. Has effect only when IsCheckBoxExactSize is true.
        #[serde(rename = "CheckBoxSize", skip_serializing_if = "Option::is_none")]
        pub check_box_size: Option<f64>,


        /// Gets or sets the checked status of the check box form field.
        #[serde(rename = "Checked", skip_serializing_if = "Option::is_none")]
        pub checked: Option<bool>,

}

impl Default for FormFieldCheckbox {
    fn default() -> Self {
        let mut parent = FormField::default();
        Self {
            parent,
            is_check_box_exact_size: None,
            check_box_size: None,
            checked: None,
        }
    }
}

impl Deref for FormFieldCheckbox {
    type Target = FormField;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FormFieldCheckbox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for FormFieldCheckbox {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.checked.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Checked in FormFieldCheckbox is required".to_owned(),
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

