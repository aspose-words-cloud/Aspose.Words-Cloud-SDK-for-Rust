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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Runs";
    let local_file = "DocumentElements/Runs/Run.doc".to_owned();
    let remote_file_name = "TestUpdateRun.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_run = RunUpdate::default();
    request_run.text = Some(("run with text".to_owned()).into());

    let request = UpdateRunRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/1".to_owned()).into(),
        (0).into(),
        (request_run).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_run(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Run")?;
    assert_string(&result_json, "Run.Text", "run with text".to_owned())?;
    Ok(())
}

/// Test for updating run online.
#[tokio::test]
async fn run_update_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Runs/Run.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_run = RunUpdate::default();
    request_run.text = Some(("run with text".to_owned()).into());

    let request = UpdateRunOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/1".to_owned()).into(),
        (request_run).into(),
        (0).into()
    );

    context.api().update_run_online(request).await?;
    Ok(())
}

/// Test for adding run.
#[tokio::test]
async fn run_insert_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Runs";
    let local_file = "DocumentElements/Runs/Run.doc".to_owned();
    let remote_file_name = "TestInsertRun.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_run = RunInsert::default();
    request_run.text = Some(("run with text".to_owned()).into());

    let request = InsertRunRequest::new(
        (remote_file_name.clone()).into(),
        (request_run).into()
    ).with_paragraph_path(("paragraphs/1".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_run(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Run")?;
    assert_string(&result_json, "Run.Text", "run with text".to_owned())?;
    assert_string(&result_json, "Run.NodeId", "0.0.1.3".to_owned())?;
    Ok(())
}

/// Test for adding run online.
#[tokio::test]
async fn run_insert_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Runs/Run.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_run = RunInsert::default();
    request_run.text = Some(("run with text".to_owned()).into());

    let request = InsertRunOnlineRequest::new(
        (request_document).into(),
        (request_run).into()
    ).with_paragraph_path(("paragraphs/1".to_owned()).into());

    context.api().insert_run_online(request).await?;
    Ok(())
}

/// Test for deleting run.
#[tokio::test]
async fn run_delete_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Runs";
    let local_file = "DocumentElements/Runs/Run.doc".to_owned();
    let remote_file_name = "TestDeleteRun.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteRunRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/1".to_owned()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_run(request).await?;
    Ok(())
}

/// Test for deleting run online.
#[tokio::test]
async fn run_delete_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Runs/Run.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteRunOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/1".to_owned()).into(),
        (0).into()
    );

    context.api().delete_run_online(request).await?;
    Ok(())
}