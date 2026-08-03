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

/// FormField text input element.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormFieldTextInput {
    #[serde(flatten)]
    pub parent: FormField,
        /// Gets or sets text formatting for the text form field.
            /// If the text form field contains regular text, then valid format strings are "", "UPPERCASE", "LOWERCASE", "FIRST CAPITAL" and "TITLE CASE". The strings are case-insensitive.If the text form field contains a number or a date/time value, then valid format strings are number or date and time format strings.
        #[serde(rename = "TextInputFormat", skip_serializing_if = "Option::is_none")]
        pub text_input_format: Option<String>,


        /// Gets or sets the type of the text form field.
        #[serde(rename = "TextInputType", skip_serializing_if = "Option::is_none")]
        pub text_input_type: Option<FormFieldTextInputTextInputTypeEnum>,


        /// Gets or sets the default string or a calculation expression of the text form field.
            /// The meaning of this property depends on the value of the TextInputType property.When TextInputType is Regular or Number, this string specifies the default string for the text form field. This string is the content that Microsoft Word will display in the document when the form field is empty.When TextInputType is Calculated, then this string holds the expression to be calculated. The expression needs to be a formula valid according to Microsoft Word formula field requirements. When you set a new expression using this property, Aspose.Words calculates the formula result automatically and inserts it into the form field.
        #[serde(rename = "TextInputDefault", skip_serializing_if = "Option::is_none")]
        pub text_input_default: Option<String>,


        /// Gets or sets the maximum length for the text field. Zero when the length is not limited.
        #[serde(rename = "MaxLength", skip_serializing_if = "Option::is_none")]
        pub max_length: Option<i32>,

}

impl Default for FormFieldTextInput {
    fn default() -> Self {
        let mut parent = FormField::default();
        Self {
            parent,
            text_input_format: None,
            text_input_type: None,
            text_input_default: None,
            max_length: None,
        }
    }
}

impl Deref for FormFieldTextInput {
    type Target = FormField;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FormFieldTextInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for FormFieldTextInput {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.text_input_format.is_none() {
            return Err(SdkError::InvalidRequest(
                "property TextInputFormat in FormFieldTextInput is required".to_owned(),
            ));
        }
        if self.text_input_default.is_none() {
            return Err(SdkError::InvalidRequest(
                "property TextInputDefault in FormFieldTextInput is required".to_owned(),
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

/// Gets or sets the type of the text form field.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FormFieldTextInputTextInputTypeEnum {
    #[serde(rename = "Regular")]
        Regular,
    #[serde(rename = "Number")]
        Number,
    #[serde(rename = "Date")]
        Date,
    #[serde(rename = "CurrentDate")]
        CurrentDate,
    #[serde(rename = "CurrentTime")]
        CurrentTime,
    #[serde(rename = "Calculated")]
        Calculated,
}