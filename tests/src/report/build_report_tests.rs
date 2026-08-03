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

#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::sync::Arc;
use std::time::Duration;

use aspose_words_cloud::*;
use chrono::{TimeZone, Utc};

use crate::test_context::*;

/// Test for build report online.
#[tokio::test]
async fn build_report_build_report_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let reporting_folder = "DocumentActions/Reporting".to_owned();
    let local_document_file = "ReportTemplate.docx".to_owned();
    let local_data_file = read_text_file(reporting_folder.clone() + "/ReportData.json").await?;

    let request_template = context.load_binary_file(reporting_folder.clone() + "/" + &local_document_file).await?;
    let mut request_report_engine_settings = ReportEngineSettings::default();
    request_report_engine_settings.data_source_type = Some((ReportEngineSettingsDataSourceTypeEnum::Json).into());
    request_report_engine_settings.data_source_name = Some(("persons".to_owned()).into());

    let request = BuildReportOnlineRequest::new(
        (request_template).into(),
        (local_data_file.clone()).into(),
        (request_report_engine_settings).into()
    );

    context.api().build_report_online(request).await?;
    Ok(())
}

/// Test for build report.
#[tokio::test]
async fn build_report_build_report() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Reporting";
    let reporting_folder = "DocumentActions/Reporting".to_owned();
    let local_document_file = "ReportTemplate.docx".to_owned();
    let remote_file_name = "TestBuildReport.docx".to_owned();
    let local_data_file = read_text_file(reporting_folder.clone() + "/ReportData.json").await?;

    context.upload_file(reporting_folder.clone() + "/" + &local_document_file, remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let request_report_engine_settings_report_build_options = vec![
    ReportBuildOptionsEnum::AllowMissingMembers,
    ReportBuildOptionsEnum::RemoveEmptyParagraphs
    ];
    let mut request_report_engine_settings = ReportEngineSettings::default();
    request_report_engine_settings.data_source_type = Some((ReportEngineSettingsDataSourceTypeEnum::Json).into());
    request_report_engine_settings.report_build_options = Some((request_report_engine_settings_report_build_options).into());

    let request = BuildReportRequest::new(
        (remote_file_name.clone()).into(),
        (local_data_file.clone()).into(),
        (request_report_engine_settings).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().build_report(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestBuildReport.docx".to_owned())?;
    Ok(())
}