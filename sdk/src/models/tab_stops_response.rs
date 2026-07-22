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

/// The REST response with an array of tab stops.
/// This response is returned by the Service when handling "GET https://api.aspose.cloud/v4.0/words/Test.doc/paragraphs/{0}/tabstops" REST API requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct TabStopsResponse {
    #[serde(flatten)]
    pub parent: WordsResponse,
        /// Gets or sets the array of tab stops.
        #[serde(rename = "TabStops", skip_serializing_if = "Option::is_none")]
        pub tab_stops: Option<Vec<TabStop>>,

}

impl Default for TabStopsResponse {
    fn default() -> Self {
        let mut parent = WordsResponse::default();
        Self {
            parent,
            tab_stops: None,
        }
    }
}

impl Deref for TabStopsResponse {
    type Target = WordsResponse;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for TabStopsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for TabStopsResponse {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(values) = &self.tab_stops {
        for value in values {
        value.validate()?;
        }
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

