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

/// Represents options for parsing CSV data.
/// To learn more, visit the LINQ Reporting Engine documentation article.
/// An instance of this class can be passed into constructors of CsvDataSource.
#[derive(Debug, Deserialize, Serialize)]
pub struct CsvDataLoadOptions {
    /// Gets or sets the character that is used to comment lines of CSV data.
    /// The default value is '#' (number sign).
    #[serde(rename = "CommentChar", skip_serializing_if = "Option::is_none")]
    pub comment_char: Option<String>,

    /// Gets or sets the character to be used as a column delimiter.
    /// The default value is ',' (comma).
    #[serde(rename = "Delimiter", skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,

    /// Gets or sets a value indicating whether the first record of CSV data contains column names.
    /// The default value is false.
    #[serde(rename = "HasHeaders", skip_serializing_if = "Option::is_none")]
    pub has_headers: Option<bool>,

    /// Gets or sets the character that is used to quote field values.
    /// The default value is '"' (quotation mark).Double the character to place it into quoted text.
    #[serde(rename = "QuoteChar", skip_serializing_if = "Option::is_none")]
    pub quote_char: Option<String>,
}

impl Default for CsvDataLoadOptions {
    fn default() -> Self {
        Self {
            comment_char: None,
            delimiter: None,
            has_headers: None,
            quote_char: None,
        }
    }
}

impl Model for CsvDataLoadOptions {
    fn validate(&self) -> SdkResult<()> {
        if self.comment_char.is_none() {
            return Err(SdkError::InvalidRequest(
                "property CommentChar in CsvDataLoadOptions is required".to_owned(),
            ));
        }
        if self.delimiter.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Delimiter in CsvDataLoadOptions is required".to_owned(),
            ));
        }
        if self.has_headers.is_none() {
            return Err(SdkError::InvalidRequest(
                "property HasHeaders in CsvDataLoadOptions is required".to_owned(),
            ));
        }
        if self.quote_char.is_none() {
            return Err(SdkError::InvalidRequest(
                "property QuoteChar in CsvDataLoadOptions is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
