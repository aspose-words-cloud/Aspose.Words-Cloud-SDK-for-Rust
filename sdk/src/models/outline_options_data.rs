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

/// Container class for outline options.
#[derive(Debug, Deserialize, Serialize)]
pub struct OutlineOptionsData {
        /// Gets or sets a value indicating whether to create missing outline levels when the document is exported. The default value is false.
        #[serde(rename = "CreateMissingOutlineLevels", skip_serializing_if = "Option::is_none")]
        pub r#create_missing_outline_levels: Option<bool>,


        /// Gets or sets a value indicating whether to create outlines for headings (paragraphs formatted with the Heading styles) inside tables.
            /// The default value is false.
        #[serde(rename = "CreateOutlinesForHeadingsInTables", skip_serializing_if = "Option::is_none")]
        pub r#create_outlines_for_headings_in_tables: Option<bool>,


        /// Gets or sets the default level in the document outline at which to display Word bookmarks.
        #[serde(rename = "DefaultBookmarksOutlineLevel", skip_serializing_if = "Option::is_none")]
        pub r#default_bookmarks_outline_level: Option<i32>,


        /// Gets or sets the number of levels in the document outline to show expanded when the file is viewed.
        #[serde(rename = "ExpandedOutlineLevels", skip_serializing_if = "Option::is_none")]
        pub r#expanded_outline_levels: Option<i32>,


        /// Gets or sets the number of levels of headings (paragraphs formatted with the Heading styles) to include in the document outline.
        #[serde(rename = "HeadingsOutlineLevels", skip_serializing_if = "Option::is_none")]
        pub r#headings_outline_levels: Option<i32>,


        /// Gets or sets the individual bookmarks outline level.
        #[serde(rename = "BookmarksOutlineLevels", skip_serializing_if = "Option::is_none")]
        pub r#bookmarks_outline_levels: Option<Vec<BookmarksOutlineLevelData>>,

}

impl Default for OutlineOptionsData {
    fn default() -> Self {
        Self {
            r#create_missing_outline_levels: None,
            r#create_outlines_for_headings_in_tables: None,
            r#default_bookmarks_outline_level: None,
            r#expanded_outline_levels: None,
            r#headings_outline_levels: None,
            r#bookmarks_outline_levels: None,
        }
    }
}

impl Model for OutlineOptionsData {
    fn validate(&self) -> SdkResult<()> {
        if let Some(values) = &self.r#bookmarks_outline_levels {
        for value in values {
        value.validate()?;
        }
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

