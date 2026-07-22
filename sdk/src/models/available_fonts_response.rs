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

/// The REST response with data on system, additional and custom fonts, available for document processing.
#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableFontsResponse {
    #[serde(flatten)]
    pub parent: WordsResponse,
        /// Gets or sets the list of additional fonts, provided by Aspose team.
        #[serde(rename = "AdditionalFonts", skip_serializing_if = "Option::is_none")]
        pub r#additional_fonts: Option<Vec<FontInfo>>,


        /// Gets or sets the list of custom user fonts from user cloud storage. To use them, you should specify "fontsLocation" parameter in any request.
        #[serde(rename = "CustomFonts", skip_serializing_if = "Option::is_none")]
        pub r#custom_fonts: Option<Vec<FontInfo>>,


        /// Gets or sets the list of system fonts, available on the server.
        #[serde(rename = "SystemFonts", skip_serializing_if = "Option::is_none")]
        pub r#system_fonts: Option<Vec<FontInfo>>,

}

impl Default for AvailableFontsResponse {
    fn default() -> Self {
        let mut parent = WordsResponse::default();
        Self {
            parent,
            r#additional_fonts: None,
            r#custom_fonts: None,
            r#system_fonts: None,
        }
    }
}

impl Deref for AvailableFontsResponse {
    type Target = WordsResponse;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for AvailableFontsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for AvailableFontsResponse {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(values) = &self.r#additional_fonts {
        for value in values {
        value.validate()?;
        }
        }
        if let Some(values) = &self.r#custom_fonts {
        for value in values {
        value.validate()?;
        }
        }
        if let Some(values) = &self.r#system_fonts {
        for value in values {
        value.validate()?;
        }
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

