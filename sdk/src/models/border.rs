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

/// Represents a border of an object.
/// Borders can be applied to various document elements including paragraph, run of text inside a paragraph or a table cell.
#[derive(Debug, Deserialize, Serialize)]
pub struct Border {
    #[serde(flatten)]
    pub parent: LinkElement,
        /// Gets or sets the border type.
        #[serde(rename = "BorderType", skip_serializing_if = "Option::is_none")]
        pub r#border_type: Option<Border_BorderTypeEnum>,


        /// Gets or sets the border color.
        #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
        pub r#color: Option<XmlColor>,


        /// Gets or sets the distance of the border from text or from the page edge in points.
            /// Has no effect and will be automatically reset to zero for borders of table cells.
        #[serde(rename = "DistanceFromText", skip_serializing_if = "Option::is_none")]
        pub r#distance_from_text: Option<f64>,


        /// Gets or sets the border style.
            /// If you set line style to none, then line width is automatically changed to zero.
        #[serde(rename = "LineStyle", skip_serializing_if = "Option::is_none")]
        pub r#line_style: Option<Border_LineStyleEnum>,


        /// Gets or sets the border width in points.
            /// If you set line width greater than zero when line style is none, the line style is automatically changed to single line.
        #[serde(rename = "LineWidth", skip_serializing_if = "Option::is_none")]
        pub r#line_width: Option<f64>,


        /// Gets or sets a value indicating whether the border has a shadow.
            /// In Microsoft Word, for a border to have a shadow, the borders on all four sides (left, top, right and bottom) should be of the same type, width, color and all should have the Shadow property set to true.
        #[serde(rename = "Shadow", skip_serializing_if = "Option::is_none")]
        pub r#shadow: Option<bool>,

}

impl Default for Border {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#border_type: None,
            r#color: None,
            r#distance_from_text: None,
            r#line_style: None,
            r#line_width: None,
            r#shadow: None,
        }
    }
}

impl Deref for Border {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Border {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for Border {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.r#color {
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

/// Gets or sets the border type.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Border_BorderTypeEnum {
    #[serde(rename = "Bottom")]
        Bottom,
    #[serde(rename = "Left")]
        Left,
    #[serde(rename = "Right")]
        Right,
    #[serde(rename = "Top")]
        Top,
    #[serde(rename = "Horizontal")]
        Horizontal,
    #[serde(rename = "Vertical")]
        Vertical,
    #[serde(rename = "DiagonalDown")]
        DiagonalDown,
    #[serde(rename = "DiagonalUp")]
        DiagonalUp,
    #[serde(rename = "None")]
        None,
}

/// Gets or sets the border style.
/// If you set line style to none, then line width is automatically changed to zero.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Border_LineStyleEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Single")]
        Single,
    #[serde(rename = "Thick")]
        Thick,
    #[serde(rename = "Double")]
        Double,
    #[serde(rename = "Hairline")]
        Hairline,
    #[serde(rename = "Dot")]
        Dot,
    #[serde(rename = "DashLargeGap")]
        DashLargeGap,
    #[serde(rename = "DotDash")]
        DotDash,
    #[serde(rename = "DotDotDash")]
        DotDotDash,
    #[serde(rename = "Triple")]
        Triple,
    #[serde(rename = "ThinThickSmallGap")]
        ThinThickSmallGap,
    #[serde(rename = "ThickThinSmallGap")]
        ThickThinSmallGap,
    #[serde(rename = "ThinThickThinSmallGap")]
        ThinThickThinSmallGap,
    #[serde(rename = "ThinThickMediumGap")]
        ThinThickMediumGap,
    #[serde(rename = "ThickThinMediumGap")]
        ThickThinMediumGap,
    #[serde(rename = "ThinThickThinMediumGap")]
        ThinThickThinMediumGap,
    #[serde(rename = "ThinThickLargeGap")]
        ThinThickLargeGap,
    #[serde(rename = "ThickThinLargeGap")]
        ThickThinLargeGap,
    #[serde(rename = "ThinThickThinLargeGap")]
        ThinThickThinLargeGap,
    #[serde(rename = "Wave")]
        Wave,
    #[serde(rename = "DoubleWave")]
        DoubleWave,
    #[serde(rename = "DashSmallGap")]
        DashSmallGap,
    #[serde(rename = "DashDotStroker")]
        DashDotStroker,
    #[serde(rename = "Emboss3D")]
        Emboss3D,
    #[serde(rename = "Engrave3D")]
        Engrave3D,
    #[serde(rename = "Outset")]
        Outset,
    #[serde(rename = "Inset")]
        Inset,
}