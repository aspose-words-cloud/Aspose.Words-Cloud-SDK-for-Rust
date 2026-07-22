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

/// DTO container with a comment.
#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    #[serde(flatten)]
    pub parent: CommentLink,
        /// Gets or sets the link to comment range start node.
        #[serde(rename = "RangeStart", skip_serializing_if = "Option::is_none")]
        pub r#range_start: Option<DocumentPosition>,


        /// Gets or sets the link to comment range end node.
        #[serde(rename = "RangeEnd", skip_serializing_if = "Option::is_none")]
        pub r#range_end: Option<DocumentPosition>,


        /// Gets or sets the author name for a comment.
            /// Cannot be null.Default is an empty string.
        #[serde(rename = "Author", skip_serializing_if = "Option::is_none")]
        pub r#author: Option<String>,


        /// Gets or sets the initials of the user associated with a specific comment.
            /// Cannot be null.Default is an empty string.
        #[serde(rename = "Initial", skip_serializing_if = "Option::is_none")]
        pub r#initial: Option<String>,


        /// Gets or sets the date and time that the comment was made.
        #[serde(rename = "DateTime", skip_serializing_if = "Option::is_none")]
        pub r#date_time: Option<DateTime<Utc>>,


        /// Gets or sets text of the comment.
            /// This method allows to quickly set text of a comment from a string. The string can contain paragraph breaks, this will create paragraphs of text in the comment accordingly.
        #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
        pub r#text: Option<String>,


        /// Gets or sets the content of the comment.
        #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
        pub r#content: Option<StoryChildNodes>,

}

impl Default for Comment {
    fn default() -> Self {
        let mut parent = CommentLink::default();
        Self {
            parent,
            r#range_start: None,
            r#range_end: None,
            r#author: None,
            r#initial: None,
            r#date_time: None,
            r#text: None,
            r#content: None,
        }
    }
}

impl Deref for Comment {
    type Target = CommentLink;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for Comment {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.r#range_start {
        value.validate()?;
        }
        if let Some(value) = &self.r#range_end {
        value.validate()?;
        }




        if let Some(value) = &self.r#content {
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

