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

/// DTO container with font info.
#[derive(Debug, Deserialize, Serialize)]
pub struct FontInfo {
        /// Gets or sets the path to the font file if any.
        #[serde(rename = "FilePath", skip_serializing_if = "Option::is_none")]
        pub r#file_path: Option<String>,


        /// Gets or sets the family name of the font.
        #[serde(rename = "FontFamilyName", skip_serializing_if = "Option::is_none")]
        pub r#font_family_name: Option<String>,


        /// Gets or sets the full name of the font.
        #[serde(rename = "FullFontName", skip_serializing_if = "Option::is_none")]
        pub r#full_font_name: Option<String>,


        /// Gets or sets the version string of the font.
        #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
        pub r#version: Option<String>,

}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            r#file_path: None,
            r#font_family_name: None,
            r#full_font_name: None,
            r#version: None,
        }
    }
}

impl Model for FontInfo {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

