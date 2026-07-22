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

/// Container for the document's statistical data.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentStatData {
        /// Gets or sets the detailed statistics on footnotes.
        #[serde(rename = "FootnotesStatData", skip_serializing_if = "Option::is_none")]
        pub r#footnotes_stat_data: Option<FootnotesStatData>,


        /// Gets or sets the total count of pages in the document.
        #[serde(rename = "PageCount", skip_serializing_if = "Option::is_none")]
        pub r#page_count: Option<i32>,


        /// Gets or sets the total count of paragraphs in the document.
        #[serde(rename = "ParagraphCount", skip_serializing_if = "Option::is_none")]
        pub r#paragraph_count: Option<i32>,


        /// Gets or sets the total count of words in the document.
        #[serde(rename = "WordCount", skip_serializing_if = "Option::is_none")]
        pub r#word_count: Option<i32>,


        /// Gets or sets the detailed statistics on all pages.
        #[serde(rename = "PageStatData", skip_serializing_if = "Option::is_none")]
        pub r#page_stat_data: Option<Vec<PageStatData>>,

}

impl Default for DocumentStatData {
    fn default() -> Self {
        Self {
            r#footnotes_stat_data: None,
            r#page_count: None,
            r#paragraph_count: None,
            r#word_count: None,
            r#page_stat_data: None,
        }
    }
}

impl Model for DocumentStatData {
    fn validate(&self) -> SdkResult<()> {
        if self.r#page_count.is_none() {
            return Err(SdkError::InvalidRequest(
                "property PageCount in DocumentStatData is required".to_owned(),
            ));
        }
        if self.r#paragraph_count.is_none() {
            return Err(SdkError::InvalidRequest(
                "property ParagraphCount in DocumentStatData is required".to_owned(),
            ));
        }
        if self.r#word_count.is_none() {
            return Err(SdkError::InvalidRequest(
                "property WordCount in DocumentStatData is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#footnotes_stat_data {
        value.validate()?;
        }



        if let Some(values) = &self.r#page_stat_data {
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

