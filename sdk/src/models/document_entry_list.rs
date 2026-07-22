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

/// Represents a list of documents which will be appended to the original resource document.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentEntryList {
    #[serde(flatten)]
    pub parent: BaseEntryList,
        /// Gets or sets a value indicating whether to append all documents to the same section.
        #[serde(rename = "AppendAllEntriesToOneSection", skip_serializing_if = "Option::is_none")]
        pub r#append_all_entries_to_one_section: Option<bool>,


        /// Gets or sets a value indicating whether to apply headers and footers from base document to appending documents. The default value is true.
        #[serde(rename = "ApplyBaseDocumentHeadersAndFootersToAppendingDocuments", skip_serializing_if = "Option::is_none")]
        pub r#apply_base_document_headers_and_footers_to_appending_documents: Option<bool>,


        /// Gets or sets the list of documents.
        #[serde(rename = "DocumentEntries", skip_serializing_if = "Option::is_none")]
        pub r#document_entries: Option<Vec<DocumentEntry>>,

}

impl Default for DocumentEntryList {
    fn default() -> Self {
        let mut parent = BaseEntryList::default();
        Self {
            parent,
            r#append_all_entries_to_one_section: None,
            r#apply_base_document_headers_and_footers_to_appending_documents: None,
            r#document_entries: None,
        }
    }
}

impl Deref for DocumentEntryList {
    type Target = BaseEntryList;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for DocumentEntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for DocumentEntryList {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#document_entries.is_none() {
            return Err(SdkError::InvalidRequest(
                "property DocumentEntries in DocumentEntryList is required".to_owned(),
            ));
        }
        if let Some(values) = &self.r#document_entries {
        for value in values {
        value.validate()?;
        }
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
        if let Some(values) = &self.r#document_entries {
        for value in values {
        value.collect_file_references(output);
        }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

