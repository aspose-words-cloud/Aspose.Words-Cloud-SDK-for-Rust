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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Revisions";
    let local_file = "DocumentElements/Revisions/TestRevisions.doc".to_owned();
    let remote_file_name = "TestAcceptAllRevisions.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = AcceptAllRevisionsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Revisions/TestRevisions.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = AcceptAllRevisionsOnlineRequest::new(
        (request_document).into()
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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Revisions";
    let local_file = "DocumentElements/Revisions/TestRevisions.doc".to_owned();
    let remote_file_name = "TestRejectAllRevisions.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = RejectAllRevisionsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Revisions/TestRevisions.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = RejectAllRevisionsOnlineRequest::new(
        (request_document).into()
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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Revisions";
    let local_file = "DocumentElements/Revisions/TestRevisions.doc".to_owned();
    let remote_file_name = "TestAcceptAllRevisions.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetAllRevisionsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Revisions/TestRevisions.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetAllRevisionsOnlineRequest::new(
        (request_document).into()
    );

    let result = context.api().get_all_revisions_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Revisions")?;
    assert_length(&result_json, "Revisions.Revisions", 6)?;
    Ok(())
}