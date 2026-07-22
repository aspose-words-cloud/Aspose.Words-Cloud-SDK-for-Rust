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

/// Test for executing mail merge online.
#[tokio::test]
async fn execute_mail_merge_execute_mail_merge_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let mailMergeFolder = test_string!("DocumentActions/MailMerge")?;
    let localDocumentFile = test_string!("SampleExecuteTemplate.docx")?;
    let localDataFile = test_string!("SampleExecuteTemplateData.txt")?;

    let requestTemplate = context.load_binary_file(test_string!(mailMergeFolder + "/" + localDocumentFile)?).await?;
    let requestData = context.load_binary_file(test_string!(mailMergeFolder + "/" + localDataFile)?).await?;

    let request = ExecuteMailMergeOnlineRequest::new(
        (requestTemplate).into(),
        (requestData).into()
    ).with_with_regions((true).into());

    context.api().execute_mail_merge_online(request).await?;
    Ok(())
}

/// Test for executing mail merge online job.
#[tokio::test]
async fn execute_mail_merge_execute_mail_merge_online_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let mailMergeFolder = test_string!("DocumentActions/MailMerge")?;
    let localDocumentFile = test_string!("SampleExecuteTemplate.docx")?;
    let localDataFile = test_string!("SampleExecuteTemplateData.txt")?;

    let requestTemplate = context.load_binary_file(test_string!(mailMergeFolder + "/" + localDocumentFile)?).await?;
    let requestData = context.load_binary_file(test_string!(mailMergeFolder + "/" + localDataFile)?).await?;

    let request = ExecuteMailMergeOnlineJobRequest::new(
        (requestTemplate).into(),
        (requestData).into()
    ).with_with_regions((true).into());

    let job_handler = context.api().execute_mail_merge_online_job(request).await?;
    job_handler.wait_result(Duration::from_secs(3)).await?;
    Ok(())
}

/// Test for executing mail merge.
#[tokio::test]
async fn execute_mail_merge_execute_mail_merge() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/MailMerge")?;
    let mailMergeFolder = test_string!("DocumentActions/MailMerge")?;
    let localDocumentFile = test_string!("SampleExecuteTemplate.docx")?;
    let remoteFileName = test_string!("TestExecuteMailMerge.docx")?;
    let localDataFile = test_string!(ReadFile(t, mailMergeFolder + "/SampleMailMergeTemplateData.txt"))?;

    context.upload_file(test_string!(mailMergeFolder + "/" + localDocumentFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = ExecuteMailMergeRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_data((test_string!(localDataFile)?).into())
.with_folder((test_string!(remoteDataFolder)?).into())
.with_with_regions((true).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().execute_mail_merge(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestExecuteMailMerge.docx")?)?;
    Ok(())
}

/// Test for executing mail merge job.
#[tokio::test]
async fn execute_mail_merge_execute_mail_merge_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/MailMerge")?;
    let mailMergeFolder = test_string!("DocumentActions/MailMerge")?;
    let localDocumentFile = test_string!("SampleExecuteTemplate.docx")?;
    let remoteFileName = test_string!("TestExecuteMailMerge.docx")?;
    let localDataFile = test_string!(ReadFile(t, mailMergeFolder + "/SampleMailMergeTemplateData.txt"))?;

    context.upload_file(test_string!(mailMergeFolder + "/" + localDocumentFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = ExecuteMailMergeJobRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_data((test_string!(localDataFile)?).into())
.with_folder((test_string!(remoteDataFolder)?).into())
.with_with_regions((true).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let job_handler = context.api().execute_mail_merge_job(request).await?;
    let result = job_handler.wait_result(Duration::from_secs(3)).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestExecuteMailMerge.docx")?)?;
    Ok(())
}