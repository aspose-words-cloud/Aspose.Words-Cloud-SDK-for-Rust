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

/// Test for updating run.
#[tokio::test]
async fn run_update_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Runs")?;
    let localFile = test_string!("DocumentElements/Runs/Run.doc")?;
    let remoteFileName = test_string!("TestUpdateRun.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestRun = RunUpdate::default();
    requestRun.r#text = Some((test_string!("run with text")?).into());

    let request = UpdateRunRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/1")?).into(),
        (0).into(),
        (requestRun).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_run(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Run")?;
    assert_string(&result_json, "Run.Text", test_string!("run with text")?)?;
    Ok(())
}

/// Test for updating run online.
#[tokio::test]
async fn run_update_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Runs/Run.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestRun = RunUpdate::default();
    requestRun.r#text = Some((test_string!("run with text")?).into());

    let request = UpdateRunOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/1")?).into(),
        (requestRun).into(),
        (0).into()
    );

    context.api().update_run_online(request).await?;
    Ok(())
}

/// Test for adding run.
#[tokio::test]
async fn run_insert_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Runs")?;
    let localFile = test_string!("DocumentElements/Runs/Run.doc")?;
    let remoteFileName = test_string!("TestInsertRun.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestRun = RunInsert::default();
    requestRun.r#text = Some((test_string!("run with text")?).into());

    let request = InsertRunRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestRun).into()
    ).with_paragraph_path((test_string!("paragraphs/1")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_run(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Run")?;
    assert_string(&result_json, "Run.Text", test_string!("run with text")?)?;
    assert_string(&result_json, "Run.NodeId", test_string!("0.0.1.3")?)?;
    Ok(())
}

/// Test for adding run online.
#[tokio::test]
async fn run_insert_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Runs/Run.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestRun = RunInsert::default();
    requestRun.r#text = Some((test_string!("run with text")?).into());

    let request = InsertRunOnlineRequest::new(
        (requestDocument).into(),
        (requestRun).into()
    ).with_paragraph_path((test_string!("paragraphs/1")?).into());

    context.api().insert_run_online(request).await?;
    Ok(())
}

/// Test for deleting run.
#[tokio::test]
async fn run_delete_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Runs")?;
    let localFile = test_string!("DocumentElements/Runs/Run.doc")?;
    let remoteFileName = test_string!("TestDeleteRun.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteRunRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/1")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_run(request).await?;
    Ok(())
}

/// Test for deleting run online.
#[tokio::test]
async fn run_delete_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Runs/Run.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteRunOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/1")?).into(),
        (0).into()
    );

    context.api().delete_run_online(request).await?;
    Ok(())
}