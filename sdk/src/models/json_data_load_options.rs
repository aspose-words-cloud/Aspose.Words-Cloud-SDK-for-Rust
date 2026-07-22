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

/// Represents options for parsing JSON data.
/// An instance of this class can be passed into constructors of Aspose.Words.Reporting.JsonDataSource.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonDataLoadOptions {
        /// Gets or sets a value indicating whether a generated data source will always contain
            /// an object for a JSON root element. If a JSON root element contains a single complex
            /// property, such an object is not created by default.
            /// The default value is false.
        #[serde(rename = "AlwaysGenerateRootObject", skip_serializing_if = "Option::is_none")]
        pub r#always_generate_root_object: Option<bool>,


        /// Gets or sets exact formats for parsing JSON date-time values while loading JSON.
            /// The default is null.
            /// Strings encoded using Microsoft® JSON date-time format (for example, "/Date(1224043200000)/")
            /// are always recognized as date-time values regardless of a value of this property.
            /// The property defines additional formats to be used while parsing date-time values
            /// from strings in the following way:
            /// • When Aspose.Words.Reporting.JsonDataLoadOptions.ExactDateTimeParseFormats is
            /// null, the ISO-8601 format and all date-time formats supported for the current,
            /// English USA, and English New Zealand cultures are used additionally in the mentioned
            /// order.
            /// • When Aspose.Words.Reporting.JsonDataLoadOptions.ExactDateTimeParseFormats contains
            /// strings, they are used as additional date-time formats utilizing the current
            /// culture.
            /// • When Aspose.Words.Reporting.JsonDataLoadOptions.ExactDateTimeParseFormats is
            /// empty, no additional date-time formats are used.
        #[serde(rename = "ExactDateTimeParseFormats", skip_serializing_if = "Option::is_none")]
        pub r#exact_date_time_parse_formats: Option<Vec<String>>,


        /// Gets or sets a mode for parsing JSON simple values (null, boolean, number, integer,
            /// and string) while loading JSON. Such a mode does not affect parsing of date-time
            /// values. The default is Aspose.Words.Reporting.JsonSimpleValueParseMode.Loose.
        #[serde(rename = "SimpleValueParseMode", skip_serializing_if = "Option::is_none")]
        pub r#simple_value_parse_mode: Option<JsonDataLoadOptions_SimpleValueParseModeEnum>,

}

impl Default for JsonDataLoadOptions {
    fn default() -> Self {
        Self {
            r#always_generate_root_object: None,
            r#exact_date_time_parse_formats: None,
            r#simple_value_parse_mode: None,
        }
    }
}

impl Model for JsonDataLoadOptions {
    fn validate(&self) -> SdkResult<()> {
        if self.r#always_generate_root_object.is_none() {
            return Err(SdkError::InvalidRequest(
                "property AlwaysGenerateRootObject in JsonDataLoadOptions is required".to_owned(),
            ));
        }
        if self.r#simple_value_parse_mode.is_none() {
            return Err(SdkError::InvalidRequest(
                "property SimpleValueParseMode in JsonDataLoadOptions is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets a mode for parsing JSON simple values (null, boolean, number, integer,
/// and string) while loading JSON. Such a mode does not affect parsing of date-time
/// values. The default is Aspose.Words.Reporting.JsonSimpleValueParseMode.Loose.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum JsonDataLoadOptions_SimpleValueParseModeEnum {
    #[serde(rename = "Loose")]
        Loose,
    #[serde(rename = "Strict")]
        Strict,
}