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

/// Test for accepting revisions in document.
#[tokio::test]
async fn revisions_accept_all_revisions() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Revisions")?;
    let localFile = test_string!("DocumentElements/Revisions/TestRevisions.doc")?;
    let remoteFileName = test_string!("TestAcceptAllRevisions.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = AcceptAllRevisionsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().accept_all_revisions(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Result")?;
    assert_not_null(&result_json, "Result.Dest")?;
    Ok(())
}

/// Test for accepting revisions in document online.
#[tokio::test]
async fn revisions_accept_all_revisions_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Revisions/TestRevisions.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = AcceptAllRevisionsOnlineRequest::new(
        (requestDocument).into()
    );

    let result = context.api().accept_all_revisions_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_not_null(&result_json, "Model")?;
    assert_not_null(&result_json, "Model.Result")?;
    assert_not_null(&result_json, "Model.Result.Dest")?;
    Ok(())
}

/// Test for rejecting revisions in document.
#[tokio::test]
async fn revisions_reject_all_revisions() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Revisions")?;
    let localFile = test_string!("DocumentElements/Revisions/TestRevisions.doc")?;
    let remoteFileName = test_string!("TestRejectAllRevisions.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RejectAllRevisionsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().reject_all_revisions(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Result")?;
    assert_not_null(&result_json, "Result.Dest")?;
    Ok(())
}

/// Test for rejecting revisions in document online.
#[tokio::test]
async fn revisions_reject_all_revisions_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Revisions/TestRevisions.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = RejectAllRevisionsOnlineRequest::new(
        (requestDocument).into()
    );

    let result = context.api().reject_all_revisions_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_not_null(&result_json, "Model")?;
    assert_not_null(&result_json, "Model.Result")?;
    assert_not_null(&result_json, "Model.Result.Dest")?;
    Ok(())
}

/// Test for getting revisions from document.
#[tokio::test]
async fn revisions_get_all_revisions() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Revisions")?;
    let localFile = test_string!("DocumentElements/Revisions/TestRevisions.doc")?;
    let remoteFileName = test_string!("TestAcceptAllRevisions.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetAllRevisionsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_all_revisions(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Revisions")?;
    assert_length(&result_json, "Revisions.Revisions", 6)?;
    Ok(())
}

/// Test for getting revisions online from document.
#[tokio::test]
async fn revisions_get_all_revisions_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Revisions/TestRevisions.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetAllRevisionsOnlineRequest::new(
        (requestDocument).into()
    );

    let result = context.api().get_all_revisions_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Revisions")?;
    assert_length(&result_json, "Revisions.Revisions", 6)?;
    Ok(())
}