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

/// Allows to set advanced compare options.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdvancedCompareOptions {
        /// Gets or sets the value indicating whether list definition contents are compared instead of list definition Ids.
            /// Default value is false.
        #[serde(rename = "CompareListDefinitions", skip_serializing_if = "Option::is_none")]
        pub compare_list_definitions: Option<bool>,


        /// Gets or sets the value indicating whether to ignore difference in DrawingML unique Id.
            /// Default value is false.
        #[serde(rename = "IgnoreDmlUniqueId", skip_serializing_if = "Option::is_none")]
        pub ignore_dml_unique_id: Option<bool>,


        /// Gets or sets the value indicating whether to ignore difference in StructuredDocumentTag store item Id.
            /// Default value is false.
        #[serde(rename = "IgnoreStoreItemId", skip_serializing_if = "Option::is_none")]
        pub ignore_store_item_id: Option<bool>,

}

impl Default for AdvancedCompareOptions {
    fn default() -> Self {
        Self {
            compare_list_definitions: None,
            ignore_dml_unique_id: None,
            ignore_store_item_id: None,
        }
    }
}

impl Model for AdvancedCompareOptions {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

