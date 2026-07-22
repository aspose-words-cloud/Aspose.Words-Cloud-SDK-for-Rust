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

/// Class for insert watermark text request building.
#[derive(Debug, Deserialize, Serialize)]
pub struct WatermarkDataText {
    #[serde(flatten)]
    pub parent: WatermarkDataBase,
        /// Gets or sets font color. The default value is System.Drawing.Color.Silver.
        #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
        pub r#color: Option<XmlColor>,


        /// Gets or sets font family name. The default value is "Calibri".
        #[serde(rename = "FontFamily", skip_serializing_if = "Option::is_none")]
        pub r#font_family: Option<String>,


        /// Gets or sets a font size. The default value is 0 - auto.
            /// Valid values range from 0 to 65.5 inclusive. Auto font size means that the watermark will be scaled to its max width and max height relative to the page margins.
        #[serde(rename = "FontSize", skip_serializing_if = "Option::is_none")]
        pub r#font_size: Option<f64>,


        /// Gets or sets a boolean value which is responsible for opacity of the watermark. The default value is true.
        #[serde(rename = "IsSemitrasparent", skip_serializing_if = "Option::is_none")]
        pub r#is_semitrasparent: Option<bool>,


        /// Gets or sets layout of the watermark. The default value is Aspose.Words.WatermarkLayout.Diagonal.
        #[serde(rename = "Layout", skip_serializing_if = "Option::is_none")]
        pub r#layout: Option<WatermarkDataText_LayoutEnum>,


        /// Gets or sets the watermark text.
        #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
        pub r#text: Option<String>,

}

impl Default for WatermarkDataText {
    fn default() -> Self {
        let mut parent = WatermarkDataBase::default();
        Self {
            parent,
            r#color: None,
            r#font_family: None,
            r#font_size: None,
            r#is_semitrasparent: None,
            r#layout: None,
            r#text: None,
        }
    }
}

impl Deref for WatermarkDataText {
    type Target = WatermarkDataBase;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WatermarkDataText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for WatermarkDataText {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#text.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Text in WatermarkDataText is required".to_owned(),
            ));
        }
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

/// Gets or sets layout of the watermark. The default value is Aspose.Words.WatermarkLayout.Diagonal.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WatermarkDataText_LayoutEnum {
    #[serde(rename = "Horizontal")]
        Horizontal,
    #[serde(rename = "Diagonal")]
        Diagonal,
}