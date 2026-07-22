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

/// FormField dropdownlist element.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormFieldDropDown {
    #[serde(flatten)]
    pub parent: FormField,
        /// Gets or sets the items array of a dropdown form field.
            /// Microsoft Word allows maximum 25 items in a dropdown form field.
        #[serde(rename = "DropDownItems", skip_serializing_if = "Option::is_none")]
        pub r#drop_down_items: Option<Vec<String>>,


        /// Gets or sets the index specifying the currently selected item in a dropdown form field.
        #[serde(rename = "DropDownSelectedIndex", skip_serializing_if = "Option::is_none")]
        pub r#drop_down_selected_index: Option<i32>,

}

impl Default for FormFieldDropDown {
    fn default() -> Self {
        let mut parent = FormField::default();
        Self {
            parent,
            r#drop_down_items: None,
            r#drop_down_selected_index: None,
        }
    }
}

impl Deref for FormFieldDropDown {
    type Target = FormField;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FormFieldDropDown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for FormFieldDropDown {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#drop_down_items.is_none() {
            return Err(SdkError::InvalidRequest(
                "property DropDownItems in FormFieldDropDown is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

