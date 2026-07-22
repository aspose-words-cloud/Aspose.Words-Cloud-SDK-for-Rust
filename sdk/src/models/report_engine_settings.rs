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

/// Report engine settings.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReportEngineSettings {
        /// Gets or sets the options for parsing CSV data.
        #[serde(rename = "CsvDataLoadOptions", skip_serializing_if = "Option::is_none")]
        pub r#csv_data_load_options: Option<CsvDataLoadOptions>,


        /// Gets or sets the name to reference the data source object in the template.
        #[serde(rename = "DataSourceName", skip_serializing_if = "Option::is_none")]
        pub r#data_source_name: Option<String>,


        /// Gets or sets type of datasource.
        #[serde(rename = "DataSourceType", skip_serializing_if = "Option::is_none")]
        pub r#data_source_type: Option<ReportEngineSettings_DataSourceTypeEnum>,


        /// Gets or sets the options for parsing JSON data.
        #[serde(rename = "JsonDataLoadOptions", skip_serializing_if = "Option::is_none")]
        pub r#json_data_load_options: Option<JsonDataLoadOptions>,


        /// Gets or sets type of options to build report.
        #[serde(rename = "ReportBuildOptions", skip_serializing_if = "Option::is_none")]
        pub r#report_build_options: Option<Vec<ReportBuildOptionsEnum>>,


        /// Gets or sets the options for parsing XML data.
        #[serde(rename = "XmlDataLoadOptions", skip_serializing_if = "Option::is_none")]
        pub r#xml_data_load_options: Option<XmlDataLoadOptions>,

}

impl Default for ReportEngineSettings {
    fn default() -> Self {
        Self {
            r#csv_data_load_options: None,
            r#data_source_name: None,
            r#data_source_type: None,
            r#json_data_load_options: None,
            r#report_build_options: None,
            r#xml_data_load_options: None,
        }
    }
}

impl Model for ReportEngineSettings {
    fn validate(&self) -> SdkResult<()> {
        if self.r#data_source_type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property DataSourceType in ReportEngineSettings is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#csv_data_load_options {
        value.validate()?;
        }


        if let Some(value) = &self.r#json_data_load_options {
        value.validate()?;
        }

        if let Some(value) = &self.r#xml_data_load_options {
        value.validate()?;
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets type of datasource.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReportEngineSettings_DataSourceTypeEnum {
    #[serde(rename = "Xml")]
        Xml,
    #[serde(rename = "Json")]
        Json,
    #[serde(rename = "Csv")]
        Csv,
}