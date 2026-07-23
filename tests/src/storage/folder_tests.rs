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

/// Test for create folder.
#[tokio::test]
async fn folder_create_folder() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";

    let request =
        CreateFolderRequest::new((remote_data_folder.clone() + "/TestCreateFolder").into());

    context.api().create_folder(request).await?;
    Ok(())
}

/// Test for delete folder.
#[tokio::test]
async fn folder_delete_folder() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let test_delete_folder = remote_data_folder.clone() + "/TestDeleteFolder";

    context
        .upload_file(
            local_file.clone(),
            test_delete_folder.clone() + "/TestDeleteFolder.docx",
        )
        .await?;

    let request =
        DeleteFolderRequest::new((test_delete_folder.clone()).into()).with_recursive((true).into());

    context.api().delete_folder(request).await?;
    Ok(())
}

/// Test for get file list of folder.
#[tokio::test]
async fn folder_get_files_list() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";

    let request = GetFilesListRequest::new((remote_data_folder.clone()).into());

    let result = context.api().get_files_list(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Value")?;
    Ok(())
}

/// Test for copy folder.
#[tokio::test]
async fn folder_copy_folder() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let folder_to_copy = remote_data_folder.clone() + "/TestCopyFolder";

    context
        .upload_file(
            local_file.clone(),
            folder_to_copy.clone() + "Src/TestCopyFolderSrc.docx",
        )
        .await?;

    let request = CopyFolderRequest::new(
        (folder_to_copy.clone() + "Dest").into(),
        (folder_to_copy.clone() + "Src").into(),
    );

    context.api().copy_folder(request).await?;
    Ok(())
}

/// Test for move folder.
#[tokio::test]
async fn folder_move_folder() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Storage";
    let local_file = "Common/test_multi_pages.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/TestMoveFolderSrc/TestMoveFolderSrc.docx",
        )
        .await?;

    let request = MoveFolderRequest::new(
        (base_test_out_path.clone() + "/TestMoveFolderDest_" + &create_random_guid()).into(),
        (remote_data_folder.clone() + "/TestMoveFolderSrc").into(),
    );

    context.api().move_folder(request).await?;
    Ok(())
}
