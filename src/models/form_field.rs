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

/// FromField.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormField {
    #[serde(flatten)]
    pub parent: NodeLink,
    /// Gets or sets the form field name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Gets or sets a value indicating whether a form field is enabled.
    /// If a form field is enabled, its contents can be changed as the form is filled in.
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Gets or sets text, displayed in the status bar when a form field has the focus.
    /// If the OwnStatus property is set to true, the StatusText property specifies the status bar text. If the OwnStatus property is set to false, the StatusText property specifies the name of an AutoText entry that contains status bar text for the form field.
    #[serde(rename = "StatusText", skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,

    /// Gets or sets a value indicating whether the source of the text that's displayed in the status bar when a form field has the focus.
    /// If true, the text specified by the StatusText property is displayed. If false, the text of the AutoText entry specified by the StatusText property is displayed.
    #[serde(rename = "OwnStatus", skip_serializing_if = "Option::is_none")]
    pub own_status: Option<bool>,

    /// Gets or sets text, displayed in a message box when the form field has the focus and the user presses F1.
    /// If the OwnHelp property is set to True, HelpText specifies the text string value. If OwnHelp is set to False, HelpText specifies the name of an AutoText entry that contains help text for the form field.
    #[serde(rename = "HelpText", skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,

    /// Gets or sets a value indicating whether the source of the text that's displayed in a message box when a form field has the focus and the user presses F1.
    /// If true, the text specified by the HelpText property is displayed. If False, the text in the AutoText entry specified by the HelpText property is displayed.
    #[serde(rename = "OwnHelp", skip_serializing_if = "Option::is_none")]
    pub own_help: Option<bool>,

    /// Gets or sets a value indicating whether references to the specified form field are automatically updated whenever the field is exited.
    /// Setting CalculateOnExit only affects the behavior of the form field when the document is opened in Microsoft Word. Aspose.Words never updates references to the form field.
    #[serde(rename = "CalculateOnExit", skip_serializing_if = "Option::is_none")]
    pub calculate_on_exit: Option<bool>,

    /// Gets or sets the entry macro name for the form field.
    /// The entry macro runs when the form field gets the focus in Microsoft Word.
    #[serde(rename = "EntryMacro", skip_serializing_if = "Option::is_none")]
    pub entry_macro: Option<String>,

    /// Gets or sets the exit macro name for the form field.
    /// The exit macro runs when the form field loses the focus in Microsoft Word.
    #[serde(rename = "ExitMacro", skip_serializing_if = "Option::is_none")]
    pub exit_macro: Option<String>,
}

impl Default for FormField {
    fn default() -> Self {
        let mut parent = NodeLink::default();
        Self {
            parent,
            name: None,
            enabled: None,
            status_text: None,
            own_status: None,
            help_text: None,
            own_help: None,
            calculate_on_exit: None,
            entry_macro: None,
            exit_macro: None,
        }
    }
}

impl Deref for FormField {
    type Target = NodeLink;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FormField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for FormField {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.name.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Name in FormField is required".to_owned(),
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
