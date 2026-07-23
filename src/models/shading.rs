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

/// DTO container with a paragraph format shading element.
#[derive(Debug, Deserialize, Serialize)]
pub struct Shading {
    /// Gets or sets the color that's applied to the background of the Shading object.
    #[serde(
        rename = "BackgroundPatternColor",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_pattern_color: Option<XmlColor>,

    /// Gets or sets the color that's applied to the foreground of the Shading object.
    #[serde(
        rename = "ForegroundPatternColor",
        skip_serializing_if = "Option::is_none"
    )]
    pub foreground_pattern_color: Option<XmlColor>,

    /// Gets or sets the shading texture.
    #[serde(rename = "Texture", skip_serializing_if = "Option::is_none")]
    pub texture: Option<ShadingTextureEnum>,
}

impl Default for Shading {
    fn default() -> Self {
        Self {
            background_pattern_color: None,
            foreground_pattern_color: None,
            texture: None,
        }
    }
}

impl Model for Shading {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.background_pattern_color {
            value.validate()?;
        }
        if let Some(value) = &self.foreground_pattern_color {
            value.validate()?;
        }

        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the shading texture.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ShadingTextureEnum {
    #[serde(rename = "TextureNone")]
    TextureNone,
    #[serde(rename = "TextureSolid")]
    TextureSolid,
    #[serde(rename = "Texture5Percent")]
    Texture5Percent,
    #[serde(rename = "Texture10Percent")]
    Texture10Percent,
    #[serde(rename = "Texture20Percent")]
    Texture20Percent,
    #[serde(rename = "Texture25Percent")]
    Texture25Percent,
    #[serde(rename = "Texture30Percent")]
    Texture30Percent,
    #[serde(rename = "Texture40Percent")]
    Texture40Percent,
    #[serde(rename = "Texture50Percent")]
    Texture50Percent,
    #[serde(rename = "Texture60Percent")]
    Texture60Percent,
    #[serde(rename = "Texture70Percent")]
    Texture70Percent,
    #[serde(rename = "Texture75Percent")]
    Texture75Percent,
    #[serde(rename = "Texture80Percent")]
    Texture80Percent,
    #[serde(rename = "Texture90Percent")]
    Texture90Percent,
    #[serde(rename = "TextureDarkHorizontal")]
    TextureDarkHorizontal,
    #[serde(rename = "TextureDarkVertical")]
    TextureDarkVertical,
    #[serde(rename = "TextureDarkDiagonalDown")]
    TextureDarkDiagonalDown,
    #[serde(rename = "TextureDarkDiagonalUp")]
    TextureDarkDiagonalUp,
    #[serde(rename = "TextureDarkCross")]
    TextureDarkCross,
    #[serde(rename = "TextureDarkDiagonalCross")]
    TextureDarkDiagonalCross,
    #[serde(rename = "TextureHorizontal")]
    TextureHorizontal,
    #[serde(rename = "TextureVertical")]
    TextureVertical,
    #[serde(rename = "TextureDiagonalDown")]
    TextureDiagonalDown,
    #[serde(rename = "TextureDiagonalUp")]
    TextureDiagonalUp,
    #[serde(rename = "TextureCross")]
    TextureCross,
    #[serde(rename = "TextureDiagonalCross")]
    TextureDiagonalCross,
    #[serde(rename = "Texture2Pt5Percent")]
    Texture2Pt5Percent,
    #[serde(rename = "Texture7Pt5Percent")]
    Texture7Pt5Percent,
    #[serde(rename = "Texture12Pt5Percent")]
    Texture12Pt5Percent,
    #[serde(rename = "Texture15Percent")]
    Texture15Percent,
    #[serde(rename = "Texture17Pt5Percent")]
    Texture17Pt5Percent,
    #[serde(rename = "Texture22Pt5Percent")]
    Texture22Pt5Percent,
    #[serde(rename = "Texture27Pt5Percent")]
    Texture27Pt5Percent,
    #[serde(rename = "Texture32Pt5Percent")]
    Texture32Pt5Percent,
    #[serde(rename = "Texture35Percent")]
    Texture35Percent,
    #[serde(rename = "Texture37Pt5Percent")]
    Texture37Pt5Percent,
    #[serde(rename = "Texture42Pt5Percent")]
    Texture42Pt5Percent,
    #[serde(rename = "Texture45Percent")]
    Texture45Percent,
    #[serde(rename = "Texture47Pt5Percent")]
    Texture47Pt5Percent,
    #[serde(rename = "Texture52Pt5Percent")]
    Texture52Pt5Percent,
    #[serde(rename = "Texture55Percent")]
    Texture55Percent,
    #[serde(rename = "Texture57Pt5Percent")]
    Texture57Pt5Percent,
    #[serde(rename = "Texture62Pt5Percent")]
    Texture62Pt5Percent,
    #[serde(rename = "Texture65Percent")]
    Texture65Percent,
    #[serde(rename = "Texture67Pt5Percent")]
    Texture67Pt5Percent,
    #[serde(rename = "Texture72Pt5Percent")]
    Texture72Pt5Percent,
    #[serde(rename = "Texture77Pt5Percent")]
    Texture77Pt5Percent,
    #[serde(rename = "Texture82Pt5Percent")]
    Texture82Pt5Percent,
    #[serde(rename = "Texture85Percent")]
    Texture85Percent,
    #[serde(rename = "Texture87Pt5Percent")]
    Texture87Pt5Percent,
    #[serde(rename = "Texture92Pt5Percent")]
    Texture92Pt5Percent,
    #[serde(rename = "Texture95Percent")]
    Texture95Percent,
    #[serde(rename = "Texture97Pt5Percent")]
    Texture97Pt5Percent,
    #[serde(rename = "TextureNil")]
    TextureNil,
}
