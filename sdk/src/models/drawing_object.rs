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

/// DTO container with a DrawingObject.
#[derive(Debug, Deserialize, Serialize)]
pub struct DrawingObject {
    #[serde(flatten)]
    pub parent: DrawingObjectLink,
        /// Gets or sets the list of links that originate from this DrawingObjectDto.
        #[serde(rename = "RenderLinks", skip_serializing_if = "Option::is_none")]
        pub r#render_links: Option<Vec<WordsApiLink>>,


        /// Gets or sets the width of the DrawingObjects in points.
        #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
        pub r#width: Option<f64>,


        /// Gets or sets the height of the DrawingObject in points.
        #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
        pub r#height: Option<f64>,


        /// Gets or sets the link to OLE object. Can be null if shape does not have OLE data.
        #[serde(rename = "OleDataLink", skip_serializing_if = "Option::is_none")]
        pub r#ole_data_link: Option<WordsApiLink>,


        /// Gets or sets the link to image data. Can be null if shape does not have an image.
        #[serde(rename = "ImageDataLink", skip_serializing_if = "Option::is_none")]
        pub r#image_data_link: Option<WordsApiLink>,


        /// Gets or sets the relative horizontal position, from which the distance to the image is measured.
        #[serde(rename = "RelativeHorizontalPosition", skip_serializing_if = "Option::is_none")]
        pub r#relative_horizontal_position: Option<DrawingObject_RelativeHorizontalPositionEnum>,


        /// Gets or sets the distance in points from the origin to the left side of the image.
        #[serde(rename = "Left", skip_serializing_if = "Option::is_none")]
        pub r#left: Option<f64>,


        /// Gets or sets the relative vertical position, from which the distance to the image is measured.
        #[serde(rename = "RelativeVerticalPosition", skip_serializing_if = "Option::is_none")]
        pub r#relative_vertical_position: Option<DrawingObject_RelativeVerticalPositionEnum>,


        /// Gets or sets the distance in points from the origin to the top side of the image.
        #[serde(rename = "Top", skip_serializing_if = "Option::is_none")]
        pub r#top: Option<f64>,


        /// Gets or sets the option that controls how to wrap text around the image.
        #[serde(rename = "WrapType", skip_serializing_if = "Option::is_none")]
        pub r#wrap_type: Option<DrawingObject_WrapTypeEnum>,

}

impl Default for DrawingObject {
    fn default() -> Self {
        let mut parent = DrawingObjectLink::default();
        Self {
            parent,
            r#render_links: None,
            r#width: None,
            r#height: None,
            r#ole_data_link: None,
            r#image_data_link: None,
            r#relative_horizontal_position: None,
            r#left: None,
            r#relative_vertical_position: None,
            r#top: None,
            r#wrap_type: None,
        }
    }
}

impl Deref for DrawingObject {
    type Target = DrawingObjectLink;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for DrawingObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for DrawingObject {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(values) = &self.r#render_links {
        for value in values {
        value.validate()?;
        }
        }


        if let Some(value) = &self.r#ole_data_link {
        value.validate()?;
        }
        if let Some(value) = &self.r#image_data_link {
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

/// Gets or sets the relative horizontal position, from which the distance to the image is measured.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DrawingObject_RelativeHorizontalPositionEnum {
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
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DrawingObject_RelativeVerticalPositionEnum {
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

/// Gets or sets the option that controls how to wrap text around the image.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DrawingObject_WrapTypeEnum {
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