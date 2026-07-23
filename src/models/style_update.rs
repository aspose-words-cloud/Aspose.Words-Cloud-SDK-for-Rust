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

/// Represents a single document style properties to update.
#[derive(Debug, Deserialize, Serialize)]
pub struct StyleUpdate {
    /// Gets or sets the name of the style to be applied automatically to a new paragraph inserted after a paragraph formatted with the specified style.
    /// This property is not used by Aspose.Words. The next paragraph style will only be applied automatically when you edit the document in MS Word.
    #[serde(
        rename = "NextParagraphStyleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_paragraph_style_name: Option<String>,

    /// Gets or sets the name of the style this style is based on.
    /// This will be an empty string if the style is not based on any other style and it can be set to an empty string.
    #[serde(rename = "BaseStyleName", skip_serializing_if = "Option::is_none")]
    pub base_style_name: Option<String>,

    /// Gets or sets a value indicating whether this style is shown in the Quick Style gallery inside MS Word UI.
    #[serde(rename = "IsQuickStyle", skip_serializing_if = "Option::is_none")]
    pub is_quick_style: Option<bool>,

    /// Gets or sets the name of the style.
    /// Cannot be an empty string. If there already is a style with such name in the collection, than this style will override it. All affected nodes will reference new style.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for StyleUpdate {
    fn default() -> Self {
        Self {
            next_paragraph_style_name: None,
            base_style_name: None,
            is_quick_style: None,
            name: None,
        }
    }
}

impl Model for StyleUpdate {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
