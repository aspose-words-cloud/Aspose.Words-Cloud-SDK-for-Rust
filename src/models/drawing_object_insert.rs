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

/// Drawing object element for insert.
#[derive(Debug, Deserialize, Serialize)]
pub struct DrawingObjectInsert {
    /// Gets or sets the position, where the DrawingObject will be inserted.
    #[serde(
        rename = "Position",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_optional_model"
    )]
    pub position: Option<ModelBox>,

    /// Gets or sets the relative horizontal position, from which the distance to the image is measured.
    #[serde(
        rename = "RelativeHorizontalPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_horizontal_position: Option<DrawingObjectInsertRelativeHorizontalPositionEnum>,

    /// Gets or sets the distance in points from the origin to the left side of the image.
    #[serde(rename = "Left", skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,

    /// Gets or sets the relative vertical position, from which the distance to the image is measured.
    #[serde(
        rename = "RelativeVerticalPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_vertical_position: Option<DrawingObjectInsertRelativeVerticalPositionEnum>,

    /// Gets or sets the distance in points from the origin to the top side of the image.
    #[serde(rename = "Top", skip_serializing_if = "Option::is_none")]
    pub top: Option<f64>,

    /// Gets or sets the width of the DrawingObjects in points.
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Gets or sets the height of the DrawingObject in points.
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// Gets or sets the option indicating how to wrap text around the image.
    #[serde(rename = "WrapType", skip_serializing_if = "Option::is_none")]
    pub wrap_type: Option<DrawingObjectInsertWrapTypeEnum>,

    /// Gets or sets a value indicating whether AspectRatioLocked option on or off.
    #[serde(rename = "AspectRatioLocked", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio_locked: Option<bool>,
}

impl Default for DrawingObjectInsert {
    fn default() -> Self {
        Self {
            position: None,
            relative_horizontal_position: None,
            left: None,
            relative_vertical_position: None,
            top: None,
            width: None,
            height: None,
            wrap_type: None,
            aspect_ratio_locked: None,
        }
    }
}

impl Model for DrawingObjectInsert {
    fn validate(&self) -> SdkResult<()> {
        if self.relative_horizontal_position.is_none() {
            return Err(SdkError::InvalidRequest(
                "property RelativeHorizontalPosition in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if self.left.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Left in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if self.relative_vertical_position.is_none() {
            return Err(SdkError::InvalidRequest(
                "property RelativeVerticalPosition in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if self.top.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Top in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if self.width.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Width in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if self.height.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Height in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if self.wrap_type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property WrapType in DrawingObjectInsert is required".to_owned(),
            ));
        }
        if let Some(value) = &self.position {
            value.validate()?;
        }

        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the relative horizontal position, from which the distance to the image is measured.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DrawingObjectInsertRelativeHorizontalPositionEnum {
    #[serde(rename = "Margin")]
    Margin,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Column")]
    Column,
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Character")]
    Character,
    #[serde(rename = "LeftMargin")]
    LeftMargin,
    #[serde(rename = "RightMargin")]
    RightMargin,
    #[serde(rename = "InsideMargin")]
    InsideMargin,
    #[serde(rename = "OutsideMargin")]
    OutsideMargin,
}

/// Gets or sets the relative vertical position, from which the distance to the image is measured.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DrawingObjectInsertRelativeVerticalPositionEnum {
    #[serde(rename = "Margin")]
    Margin,
    #[serde(rename = "TableDefault")]
    TableDefault,
    #[serde(rename = "Page")]
    Page,
    #[serde(rename = "Paragraph")]
    Paragraph,
    #[serde(rename = "TextFrameDefault")]
    TextFrameDefault,
    #[serde(rename = "Line")]
    Line,
    #[serde(rename = "TopMargin")]
    TopMargin,
    #[serde(rename = "BottomMargin")]
    BottomMargin,
    #[serde(rename = "InsideMargin")]
    InsideMargin,
    #[serde(rename = "OutsideMargin")]
    OutsideMargin,
}

/// Gets or sets the option indicating how to wrap text around the image.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DrawingObjectInsertWrapTypeEnum {
    #[serde(rename = "Inline")]
    Inline,
    #[serde(rename = "TopBottom")]
    TopBottom,
    #[serde(rename = "Square")]
    Square,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Tight")]
    Tight,
    #[serde(rename = "Through")]
    Through,
}
