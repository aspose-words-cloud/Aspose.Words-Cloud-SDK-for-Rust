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

/// DTO container with compare documents options.
#[derive(Debug, Deserialize, Serialize)]
pub struct CompareOptions {
    /// Gets or sets whether accept revisions before comparison or not.
    #[serde(
        rename = "AcceptAllRevisionsBeforeComparison",
        skip_serializing_if = "Option::is_none"
    )]
    pub accept_all_revisions_before_comparison: Option<bool>,

    /// Gets or sets the option indicating whether changes are tracked by character or by word.
    #[serde(rename = "Granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<CompareOptionsGranularityEnum>,

    /// Gets or sets a value indicating whether documents comparison is case insensitive. By default comparison is case sensitive.
    #[serde(rename = "IgnoreCaseChanges", skip_serializing_if = "Option::is_none")]
    pub ignore_case_changes: Option<bool>,

    /// Gets or sets a value indicating whether comments content is ignored. By default comments are not ignored.
    #[serde(rename = "IgnoreComments", skip_serializing_if = "Option::is_none")]
    pub ignore_comments: Option<bool>,

    /// Gets or sets a value indicating whether fields content is ignored. By default fields are not ignored.
    #[serde(rename = "IgnoreFields", skip_serializing_if = "Option::is_none")]
    pub ignore_fields: Option<bool>,

    /// Gets or sets a value indicating whether footnotes/endnotes content is ignored. By default footnotes/endnotes are not ignored.
    #[serde(rename = "IgnoreFootnotes", skip_serializing_if = "Option::is_none")]
    pub ignore_footnotes: Option<bool>,

    /// Gets or sets a value indicating whether formatting is ignored. By default document formatting is not ignored.
    #[serde(rename = "IgnoreFormatting", skip_serializing_if = "Option::is_none")]
    pub ignore_formatting: Option<bool>,

    /// Gets or sets a value indicating whether headers and footers content is ignored. By default headers and footers are not ignored.
    #[serde(
        rename = "IgnoreHeadersAndFooters",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_headers_and_footers: Option<bool>,

    /// Gets or sets a value indicating whether tables content is ignored. By default tables are not ignored.
    #[serde(rename = "IgnoreTables", skip_serializing_if = "Option::is_none")]
    pub ignore_tables: Option<bool>,

    /// Gets or sets a value indicating whether textboxes content is ignored. By default textboxes are not ignored.
    #[serde(rename = "IgnoreTextboxes", skip_serializing_if = "Option::is_none")]
    pub ignore_textboxes: Option<bool>,

    /// Gets or sets the option that controls which document shall be used as a target during comparison.
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<CompareOptionsTargetEnum>,
}

impl Default for CompareOptions {
    fn default() -> Self {
        Self {
            accept_all_revisions_before_comparison: None,
            granularity: None,
            ignore_case_changes: None,
            ignore_comments: None,
            ignore_fields: None,
            ignore_footnotes: None,
            ignore_formatting: None,
            ignore_headers_and_footers: None,
            ignore_tables: None,
            ignore_textboxes: None,
            target: None,
        }
    }
}

impl Model for CompareOptions {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the option indicating whether changes are tracked by character or by word.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CompareOptionsGranularityEnum {
    #[serde(rename = "CharLevel")]
    CharLevel,
    #[serde(rename = "WordLevel")]
    WordLevel,
}

/// Gets or sets the option that controls which document shall be used as a target during comparison.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CompareOptionsTargetEnum {
    #[serde(rename = "Current")]
    Current,
    #[serde(rename = "New")]
    New,
}
