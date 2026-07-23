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

/// HeaderFooter link element.
#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderFooterLink {
    #[serde(flatten)]
    pub parent: LinkElement,
    /// Gets or sets the paragraph's text.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<HeaderFooterLinkTypeEnum>,
}

impl Default for HeaderFooterLink {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#type: None,
        }
    }
}

impl Deref for HeaderFooterLink {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for HeaderFooterLink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for HeaderFooterLink {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Type in HeaderFooterLink is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the paragraph's text.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HeaderFooterLinkTypeEnum {
    #[serde(rename = "HeaderEven")]
    HeaderEven,
    #[serde(rename = "HeaderPrimary")]
    HeaderPrimary,
    #[serde(rename = "FooterEven")]
    FooterEven,
    #[serde(rename = "FooterPrimary")]
    FooterPrimary,
    #[serde(rename = "HeaderFirst")]
    HeaderFirst,
    #[serde(rename = "FooterFirst")]
    FooterFirst,
}
