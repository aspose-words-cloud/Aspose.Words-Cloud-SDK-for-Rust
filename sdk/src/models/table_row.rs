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

/// DTO container with a table row element.
#[derive(Debug, Deserialize, Serialize)]
pub struct TableRow {
    #[serde(flatten)]
    pub parent: NodeLink,
        /// Gets or sets the collection of rows.
        #[serde(rename = "TableCellList", skip_serializing_if = "Option::is_none")]
        pub r#table_cell_list: Option<Vec<TableCell>>,


        /// Gets or sets the formatting properties of a row.
        #[serde(rename = "RowFormat", skip_serializing_if = "Option::is_none")]
        pub r#row_format: Option<TableRowFormat>,

}

impl Default for TableRow {
    fn default() -> Self {
        let mut parent = NodeLink::default();
        Self {
            parent,
            r#table_cell_list: None,
            r#row_format: None,
        }
    }
}

impl Deref for TableRow {
    type Target = NodeLink;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TableRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TableRow {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(values) = &self.r#table_cell_list {
        for value in values {
        value.validate()?;
        }
        }
        if let Some(value) = &self.r#row_format {
        value.validate()?;
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

