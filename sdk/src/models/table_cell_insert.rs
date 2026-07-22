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

/// DTO container with a table cell.
#[derive(Debug, Deserialize, Serialize)]
pub struct TableCellInsert {
        /// Gets or sets the position of the table cell that will be used to determine the placement of a new cell.
        #[serde(rename = "ExistingCellPosition", skip_serializing_if = "Option::is_none", default, deserialize_with = "deserialize_optional_model")]
        pub existing_cell_position: Option<ModelBox>,


        /// Gets or sets the 0-based index, the table cell will be inserted after.
        #[serde(rename = "InsertAfter", skip_serializing_if = "Option::is_none")]
        pub insert_after: Option<i32>,

}

impl Default for TableCellInsert {
    fn default() -> Self {
        Self {
            existing_cell_position: None,
            insert_after: None,
        }
    }
}

impl Model for TableCellInsert {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.existing_cell_position {
        value.validate()?;
        }

        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

