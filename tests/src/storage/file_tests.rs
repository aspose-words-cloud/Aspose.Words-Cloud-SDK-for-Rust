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

/// Test for uploading file.
#[tokio::test]
async fn file_upload_file() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUploadFile.docx".to_owned();

    let request_file_content = context.load_binary_file(local_file.clone()).await?;

    let request = UploadFileRequest::new(
        (request_file_content).into(),
        (remote_data_folder.clone() + "/" + &remote_file_name).into(),
    );

    let result = context.api().upload_file(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Uploaded")?;
    assert_length(&result_json, "Uploaded", 1)?;
    assert_string(
        &result_json,
        "Uploaded[0]",
        "TestUploadFile.docx".to_owned(),
    )?;
    Ok(())
}

/// Test for copy file.
#[tokio::test]
async fn file_copy_file() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestCopyFileSrc.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = CopyFileRequest::new(
        (remote_data_folder.clone() + "/TestCopyFileDest.docx").into(),
        (remote_data_folder.clone() + "/" + &remote_file_name).into(),
    );

    context.api().copy_file(request).await?;
    Ok(())
}

/// Test for move file.
#[tokio::test]
async fn file_move_file() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestMoveFileSrc.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = MoveFileRequest::new(
        (base_test_out_path.clone() + "/TestMoveFileDest_" + &create_random_guid() + ".docx")
            .into(),
        (remote_data_folder.clone() + "/" + &remote_file_name).into(),
    );

    context.api().move_file(request).await?;
    Ok(())
}

/// Test for delete file.
#[tokio::test]
async fn file_delete_file() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteFile.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        DeleteFileRequest::new((remote_data_folder.clone() + "/" + &remote_file_name).into());

    context.api().delete_file(request).await?;
    Ok(())
}

/// Test for download file.
#[tokio::test]
async fn file_download_file() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDownloadFile.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        DownloadFileRequest::new((remote_data_folder.clone() + "/" + &remote_file_name).into());

    context.api().download_file(request).await?;
    Ok(())
}
