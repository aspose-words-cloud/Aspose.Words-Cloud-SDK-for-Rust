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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let reportingFolder = test_string!("DocumentActions/Reporting")?;
    let localDocumentFile = test_string!("ReportTemplate.docx")?;
    let localDataFile = test_string!(ReadFile(t, reportingFolder + "/ReportData.json"))?;

    let requestTemplate = context.load_binary_file(test_string!(reportingFolder + "/" + localDocumentFile)?).await?;
    let mut requestReportEngineSettings = ReportEngineSettings::default();
    requestReportEngineSettings.r#data_source_type = Some((ReportEngineSettings_DataSourceTypeEnum::Json).into());
    requestReportEngineSettings.r#data_source_name = Some((test_string!("persons")?).into());

    let request = BuildReportOnlineRequest::new(
        (requestTemplate).into(),
        (test_string!(localDataFile)?).into(),
        (requestReportEngineSettings).into()
    );

    context.api().build_report_online(request).await?;
    Ok(())
}

/// Test for build report.
#[tokio::test]
async fn build_report_build_report() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Reporting")?;
    let reportingFolder = test_string!("DocumentActions/Reporting")?;
    let localDocumentFile = test_string!("ReportTemplate.docx")?;
    let remoteFileName = test_string!("TestBuildReport.docx")?;
    let localDataFile = test_string!(ReadFile(t, reportingFolder + "/ReportData.json"))?;

    context.upload_file(test_string!(reportingFolder + "/" + localDocumentFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let requestReportEngineSettingsReportBuildOptions = vec![
    ReportBuildOptionsEnum::AllowMissingMembers,
    ReportBuildOptionsEnum::RemoveEmptyParagraphs
    ];
    let mut requestReportEngineSettings = ReportEngineSettings::default();
    requestReportEngineSettings.r#data_source_type = Some((ReportEngineSettings_DataSourceTypeEnum::Json).into());
    requestReportEngineSettings.r#report_build_options = Some((requestReportEngineSettingsReportBuildOptions).into());

    let request = BuildReportRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!(localDataFile)?).into(),
        (requestReportEngineSettings).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().build_report(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestBuildReport.docx")?)?;
    Ok(())
}