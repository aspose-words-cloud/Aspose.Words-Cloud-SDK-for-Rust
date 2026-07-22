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

/// Footnote base class.
#[derive(Debug, Deserialize, Serialize)]
pub struct FootnoteBase {
        /// Gets or sets the link to range start node.
        #[serde(rename = "Position", skip_serializing_if = "Option::is_none", default, deserialize_with = "deserialize_optional_model")]
        pub position: Option<ModelBox>,


        /// Gets or sets the option, that specifies whether this is a footnote or endnote.
        #[serde(rename = "FootnoteType", skip_serializing_if = "Option::is_none")]
        pub footnote_type: Option<FootnoteBaseFootnoteTypeEnum>,


        /// Gets or sets the custom reference mark to be used for this footnote.
            /// The default value is Empty, meaning auto-numbered footnotes are used.
            /// RTF-format can only store 1 symbol as custom reference mark, so upon export only the first symbol will be written others will be discard.
        #[serde(rename = "ReferenceMark", skip_serializing_if = "Option::is_none")]
        pub reference_mark: Option<String>,


        /// Gets or sets text of the footnote.
            /// This method allows to quickly set text of a footnote from a string. The string can contain paragraph breaks, this will create paragraphs of text in the footnote accordingly.
        #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,

}

impl Default for FootnoteBase {
    fn default() -> Self {
        Self {
            position: None,
            footnote_type: None,
            reference_mark: None,
            text: None,
        }
    }
}

impl Model for FootnoteBase {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.position {
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

/// Gets or sets the option, that specifies whether this is a footnote or endnote.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FootnoteBaseFootnoteTypeEnum {
    #[serde(rename = "Footnote")]
        Footnote,
    #[serde(rename = "Endnote")]
        Endnote,
}