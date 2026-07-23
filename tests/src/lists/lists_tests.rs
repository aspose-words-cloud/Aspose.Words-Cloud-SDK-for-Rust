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

/// Test for getting lists from document.
#[tokio::test]
async fn lists_get_lists() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Lists";
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();
    let remote_file_name = "TestGetLists.doc".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetListsRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_lists(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Lists")?;
    assert_not_null(&result_json, "Lists.ListInfo")?;
    assert_length(&result_json, "Lists.ListInfo", 2)?;
    assert_integer(&result_json, "Lists.ListInfo[0].ListId", 1)?;
    Ok(())
}

/// Test for getting lists from document online.
#[tokio::test]
async fn lists_get_lists_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetListsOnlineRequest::new((request_document).into());

    context.api().get_lists_online(request).await?;
    Ok(())
}

/// Test for getting list from document.
#[tokio::test]
async fn lists_get_list() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Lists";
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();
    let remote_file_name = "TestGetList.doc".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetListRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_list(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "List")?;
    assert_integer(&result_json, "List.ListId", 1)?;
    Ok(())
}

/// Test for getting list from document online.
#[tokio::test]
async fn lists_get_list_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetListOnlineRequest::new((request_document).into(), (1).into());

    context.api().get_list_online(request).await?;
    Ok(())
}

/// Test for updating list from document.
#[tokio::test]
async fn lists_update_list() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Lists";
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();
    let remote_file_name = "TestUpdateList.doc".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_list_update = ListUpdate::default();
    request_list_update.is_restart_at_each_section = Some((true).into());

    let request = UpdateListRequest::new(
        (remote_file_name.clone()).into(),
        (1).into(),
        (request_list_update).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().update_list(request).await?;
    Ok(())
}

/// Test for updating list from document online.
#[tokio::test]
async fn lists_update_list_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_list_update = ListUpdate::default();
    request_list_update.is_restart_at_each_section = Some((true).into());

    let request = UpdateListOnlineRequest::new(
        (request_document).into(),
        (1).into(),
        (request_list_update).into(),
    );

    let result = context.api().update_list_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.List")?;
    assert_integer(&result_json, "Model.List.ListId", 1)?;
    assert_bool(&result_json, "Model.List.IsRestartAtEachSection", true)?;
    Ok(())
}

/// Test for updating list level from document.
#[tokio::test]
async fn lists_update_list_level() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Lists";
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();
    let remote_file_name = "TestUpdateListLevel.doc".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_list_update = ListLevelUpdate::default();
    request_list_update.alignment = Some((ListLevelUpdateAlignmentEnum::Right).into());

    let request = UpdateListLevelRequest::new(
        (remote_file_name.clone()).into(),
        (1).into(),
        (1).into(),
        (request_list_update).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().update_list_level(request).await?;
    Ok(())
}

/// Test for updating list level from document online.
#[tokio::test]
async fn lists_update_list_level_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_list_update = ListLevelUpdate::default();
    request_list_update.alignment = Some((ListLevelUpdateAlignmentEnum::Right).into());

    let request = UpdateListLevelOnlineRequest::new(
        (request_document).into(),
        (1).into(),
        (request_list_update).into(),
        (1).into(),
    );

    let result = context.api().update_list_level_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.List")?;
    assert_not_null(&result_json, "Model.List.ListLevels")?;
    assert_not_null(&result_json, "Model.List.ListLevels.ListLevel")?;
    assert_length(&result_json, "Model.List.ListLevels.ListLevel", 9)?;

    Ok(())
}

/// Test for inserting list from document.
#[tokio::test]
async fn lists_insert_list() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Lists";
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();
    let remote_file_name = "TestInsertList.doc".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_list_insert = ListInsert::default();
    request_list_insert.template = Some((ListInsertTemplateEnum::OutlineLegal).into());

    let request = InsertListRequest::new(
        (remote_file_name.clone()).into(),
        (request_list_insert).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_list(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "List")?;
    assert_integer(&result_json, "List.ListId", 3)?;
    Ok(())
}

/// Test for inserting list from document online.
#[tokio::test]
async fn lists_insert_list_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Lists/ListsGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_list_insert = ListInsert::default();
    request_list_insert.template = Some((ListInsertTemplateEnum::OutlineLegal).into());

    let request =
        InsertListOnlineRequest::new((request_document).into(), (request_list_insert).into());

    context.api().insert_list_online(request).await?;
    Ok(())
}
