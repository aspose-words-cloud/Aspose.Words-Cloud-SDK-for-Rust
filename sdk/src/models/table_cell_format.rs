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

/// DTO container with all formatting for a table row.
#[derive(Debug, Deserialize, Serialize)]
pub struct TableCellFormat {
    #[serde(flatten)]
    pub parent: LinkElement,
        /// Gets or sets the amount of space (in points) to add below the contents of the cell.
        #[serde(rename = "BottomPadding", skip_serializing_if = "Option::is_none")]
        pub r#bottom_padding: Option<f64>,


        /// Gets or sets a value indicating whether to fit text in the cell, compress each paragraph to the width of the cell.
        #[serde(rename = "FitText", skip_serializing_if = "Option::is_none")]
        pub r#fit_text: Option<bool>,


        /// Gets or sets the option that controls how the cell is merged horizontally with other cells in the row.
        #[serde(rename = "HorizontalMerge", skip_serializing_if = "Option::is_none")]
        pub r#horizontal_merge: Option<TableCellFormat_HorizontalMergeEnum>,


        /// Gets or sets the amount of space (in points) to add to the left of the contents of the cell.
        #[serde(rename = "LeftPadding", skip_serializing_if = "Option::is_none")]
        pub r#left_padding: Option<f64>,


        /// Gets or sets the orientation of text in a table cell.
        #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
        pub r#orientation: Option<TableCellFormat_OrientationEnum>,


        /// Gets or sets the preferred width of the cell.
            /// The preferred width (along with the table's Auto Fit option) determines how the actual width of the cell is calculated by the table layout algorithm. Table layout can be performed by Aspose.Words when it saves the document or by Microsoft Word when it displays the document.The preferred width can be specified in points or in percent. The preferred width can also be specified as "auto", which means no preferred width is specified.The default value is Auto.
        #[serde(rename = "PreferredWidth", skip_serializing_if = "Option::is_none")]
        pub r#preferred_width: Option<PreferredWidth>,


        /// Gets or sets the amount of space (in points) to add to the right of the contents of the cell.
        #[serde(rename = "RightPadding", skip_serializing_if = "Option::is_none")]
        pub r#right_padding: Option<f64>,


        /// Gets or sets the amount of space (in points) to add above the contents of the cell.
        #[serde(rename = "TopPadding", skip_serializing_if = "Option::is_none")]
        pub r#top_padding: Option<f64>,


        /// Gets or sets the vertical alignment of text in the cell.
        #[serde(rename = "VerticalAlignment", skip_serializing_if = "Option::is_none")]
        pub r#vertical_alignment: Option<TableCellFormat_VerticalAlignmentEnum>,


        /// Gets or sets the option that controls how the cell is merged with other cells vertically.
            /// Cells can only be merged vertically if their left and right boundaries are identical.When cells are vertically merged, the display areas of the merged cells are consolidated. The consolidated area is used to display the contents of the first vertically merged cell and all other vertically merged cells must be empty.
        #[serde(rename = "VerticalMerge", skip_serializing_if = "Option::is_none")]
        pub r#vertical_merge: Option<TableCellFormat_VerticalMergeEnum>,


        /// Gets or sets the width of the cell in points.
            /// The width is calculated by Aspose.Words on document loading and saving. Currently, not every combination of table, cell and document properties is supported. The returned value may not be accurate for some documents. It may not exactly match the cell width as calculated by MS Word when the document is opened in MS Word.Setting this property is not recommended. There is no guarantee that the cell will actually have the set width. The width may be adjusted to accommodate cell contents in an auto-fit table layout. Cells in other rows may have conflicting width settings. The table may be resized to fit into the container or to meet table width settings. Consider using PreferredWidth for setting the cell width. Setting this property sets PreferredWidth implicitly since version 15.8.
        #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
        pub r#width: Option<f64>,


        /// Gets or sets a value indicating whether to wrap text in the cell.
        #[serde(rename = "WrapText", skip_serializing_if = "Option::is_none")]
        pub r#wrap_text: Option<bool>,

}

impl Default for TableCellFormat {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#bottom_padding: None,
            r#fit_text: None,
            r#horizontal_merge: None,
            r#left_padding: None,
            r#orientation: None,
            r#preferred_width: None,
            r#right_padding: None,
            r#top_padding: None,
            r#vertical_alignment: None,
            r#vertical_merge: None,
            r#width: None,
            r#wrap_text: None,
        }
    }
}

impl Deref for TableCellFormat {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TableCellFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TableCellFormat {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.r#preferred_width {
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

/// Gets or sets the option that controls how the cell is merged horizontally with other cells in the row.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TableCellFormat_HorizontalMergeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "First")]
        First,
    #[serde(rename = "Previous")]
        Previous,
}

/// Gets or sets the orientation of text in a table cell.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TableCellFormat_OrientationEnum {
    #[serde(rename = "Horizontal")]
        Horizontal,
    #[serde(rename = "Downward")]
        Downward,
    #[serde(rename = "Upward")]
        Upward,
    #[serde(rename = "HorizontalRotatedFarEast")]
        HorizontalRotatedFarEast,
    #[serde(rename = "VerticalFarEast")]
        VerticalFarEast,
    #[serde(rename = "VerticalRotatedFarEast")]
        VerticalRotatedFarEast,
}

/// Gets or sets the vertical alignment of text in the cell.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TableCellFormat_VerticalAlignmentEnum {
    #[serde(rename = "Top")]
        Top,
    #[serde(rename = "Center")]
        Center,
    #[serde(rename = "Bottom")]
        Bottom,
}

/// Gets or sets the option that controls how the cell is merged with other cells vertically.
/// Cells can only be merged vertically if their left and right boundaries are identical.When cells are vertically merged, the display areas of the merged cells are consolidated. The consolidated area is used to display the contents of the first vertically merged cell and all other vertically merged cells must be empty.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TableCellFormat_VerticalMergeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "First")]
        First,
    #[serde(rename = "Previous")]
        Previous,
}