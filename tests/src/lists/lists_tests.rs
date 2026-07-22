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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Lists")?;
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;
    let remoteFileName = test_string!("TestGetLists.doc")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetListsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetListsOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().get_lists_online(request).await?;
    Ok(())
}

/// Test for getting list from document.
#[tokio::test]
async fn lists_get_list() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Lists")?;
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;
    let remoteFileName = test_string!("TestGetList.doc")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetListRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetListOnlineRequest::new(
        (requestDocument).into(),
        (1).into()
    );

    context.api().get_list_online(request).await?;
    Ok(())
}

/// Test for updating list from document.
#[tokio::test]
async fn lists_update_list() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Lists")?;
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;
    let remoteFileName = test_string!("TestUpdateList.doc")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestListUpdate = ListUpdate::default();
    requestListUpdate.r#is_restart_at_each_section = Some((true).into());

    let request = UpdateListRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into(),
        (requestListUpdate).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().update_list(request).await?;
    Ok(())
}

/// Test for updating list from document online.
#[tokio::test]
async fn lists_update_list_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestListUpdate = ListUpdate::default();
    requestListUpdate.r#is_restart_at_each_section = Some((true).into());

    let request = UpdateListOnlineRequest::new(
        (requestDocument).into(),
        (1).into(),
        (requestListUpdate).into()
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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Lists")?;
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;
    let remoteFileName = test_string!("TestUpdateListLevel.doc")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestListUpdate = ListLevelUpdate::default();
    requestListUpdate.r#alignment = Some((ListLevelUpdate_AlignmentEnum::Right).into());

    let request = UpdateListLevelRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into(),
        (1).into(),
        (requestListUpdate).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().update_list_level(request).await?;
    Ok(())
}

/// Test for updating list level from document online.
#[tokio::test]
async fn lists_update_list_level_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestListUpdate = ListLevelUpdate::default();
    requestListUpdate.r#alignment = Some((ListLevelUpdate_AlignmentEnum::Right).into());

    let request = UpdateListLevelOnlineRequest::new(
        (requestDocument).into(),
        (1).into(),
        (requestListUpdate).into(),
        (1).into()
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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Lists")?;
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;
    let remoteFileName = test_string!("TestInsertList.doc")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestListInsert = ListInsert::default();
    requestListInsert.r#template = Some((ListInsert_TemplateEnum::OutlineLegal).into());

    let request = InsertListRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestListInsert).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Lists/ListsGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestListInsert = ListInsert::default();
    requestListInsert.r#template = Some((ListInsert_TemplateEnum::OutlineLegal).into());

    let request = InsertListOnlineRequest::new(
        (requestDocument).into(),
        (requestListInsert).into()
    );

    context.api().insert_list_online(request).await?;
    Ok(())
}