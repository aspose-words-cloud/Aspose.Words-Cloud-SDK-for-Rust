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

/// DTO container with formatting for a table row.
#[derive(Debug, Deserialize, Serialize)]
pub struct TableRowFormat {
    #[serde(flatten)]
    pub parent: LinkElement,
        /// Gets or sets the height of the table row in points.
        #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
        pub r#height: Option<f64>,


        /// Gets or sets the rule for determining the height of the table row.
        #[serde(rename = "HeightRule", skip_serializing_if = "Option::is_none")]
        pub r#height_rule: Option<TableRowFormat_HeightRuleEnum>,


        /// Gets or sets a value indicating whether the text in a table row is allowed to split across a page break.
        #[serde(rename = "AllowBreakAcrossPages", skip_serializing_if = "Option::is_none")]
        pub r#allow_break_across_pages: Option<bool>,


        /// Gets or sets a value indicating whether the row is repeated as a table heading on every page when the table spans more than one page.
        #[serde(rename = "HeadingFormat", skip_serializing_if = "Option::is_none")]
        pub r#heading_format: Option<bool>,

}

impl Default for TableRowFormat {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#height: None,
            r#height_rule: None,
            r#allow_break_across_pages: None,
            r#heading_format: None,
        }
    }
}

impl Deref for TableRowFormat {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TableRowFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TableRowFormat {
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

/// Gets or sets the rule for determining the height of the table row.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TableRowFormat_HeightRuleEnum {
    #[serde(rename = "AtLeast")]
        AtLeast,
    #[serde(rename = "Exactly")]
        Exactly,
    #[serde(rename = "Auto")]
        Auto,
}