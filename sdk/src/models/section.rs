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

/// DTO container with a section element.
#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    #[serde(flatten)]
    pub parent: LinkElement,
        /// Gets or sets the list of child nodes.
        #[serde(rename = "ChildNodes", skip_serializing_if = "Option::is_none")]
        pub r#child_nodes: Option<Vec<NodeLink>>,


        /// Gets or sets the link to Paragraphs resource.
        #[serde(rename = "Paragraphs", skip_serializing_if = "Option::is_none")]
        pub r#paragraphs: Option<LinkElement>,


        /// Gets or sets the link to PageSetup resource.
        #[serde(rename = "PageSetup", skip_serializing_if = "Option::is_none")]
        pub r#page_setup: Option<LinkElement>,


        /// Gets or sets the link to HeaderFooters resource.
        #[serde(rename = "HeaderFooters", skip_serializing_if = "Option::is_none")]
        pub r#header_footers: Option<LinkElement>,


        /// Gets or sets the link to Tables resource.
        #[serde(rename = "Tables", skip_serializing_if = "Option::is_none")]
        pub r#tables: Option<LinkElement>,

}

impl Default for Section {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#child_nodes: None,
            r#paragraphs: None,
            r#page_setup: None,
            r#header_footers: None,
            r#tables: None,
        }
    }
}

impl Deref for Section {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Section {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for Section {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(values) = &self.r#child_nodes {
        for value in values {
        value.validate()?;
        }
        }
        if let Some(value) = &self.r#paragraphs {
        value.validate()?;
        }
        if let Some(value) = &self.r#page_setup {
        value.validate()?;
        }
        if let Some(value) = &self.r#header_footers {
        value.validate()?;
        }
        if let Some(value) = &self.r#tables {
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

