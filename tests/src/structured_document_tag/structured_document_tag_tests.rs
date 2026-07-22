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

/// Test for getting SDT objects from document.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tags() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/StructuredDocumentTag")?;
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;
    let remoteFileName = test_string!("TestGetStructuredDocumentTags.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetStructuredDocumentTagsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_structured_document_tags(request).await?;
    Ok(())
}

/// Test for getting SDT objects from document online.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tags_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetStructuredDocumentTagsOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into());

    context.api().get_structured_document_tags_online(request).await?;
    Ok(())
}

/// Test for getting SDT object from document.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/StructuredDocumentTag")?;
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;
    let remoteFileName = test_string!("TestGetStructuredDocumentTag.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetStructuredDocumentTagRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_structured_document_tag(request).await?;
    Ok(())
}

/// Test for getting SDT object from document online.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetStructuredDocumentTagOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into());

    context.api().get_structured_document_tag_online(request).await?;
    Ok(())
}

/// Test for adding SDT object.
#[tokio::test]
async fn structured_document_tag_insert_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/StructuredDocumentTag")?;
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;
    let remoteFileName = test_string!("TestInsetStructuredDocumentTag.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestStructuredDocumentTag = StructuredDocumentTagInsert::default();
    requestStructuredDocumentTag.r#sdt_type = Some((StructuredDocumentTagInsert_SdtTypeEnum::ComboBox).into());
    requestStructuredDocumentTag.r#level = Some((StructuredDocumentTagInsert_LevelEnum::Inline).into());

    let request = InsertStructuredDocumentTagRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestStructuredDocumentTag).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().insert_structured_document_tag(request).await?;
    Ok(())
}

/// Test for adding SDT object online.
#[tokio::test]
async fn structured_document_tag_insert_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestStructuredDocumentTag = StructuredDocumentTagInsert::default();
    requestStructuredDocumentTag.r#sdt_type = Some((StructuredDocumentTagInsert_SdtTypeEnum::ComboBox).into());
    requestStructuredDocumentTag.r#level = Some((StructuredDocumentTagInsert_LevelEnum::Inline).into());

    let request = InsertStructuredDocumentTagOnlineRequest::new(
        (requestDocument).into(),
        (requestStructuredDocumentTag).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into());

    context.api().insert_structured_document_tag_online(request).await?;
    Ok(())
}

/// Test for deleting SDT object.
#[tokio::test]
async fn structured_document_tag_delete_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/StructuredDocumentTag")?;
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;
    let remoteFileName = test_string!("TestDeleteStructuredDocumentTag.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteStructuredDocumentTagRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_structured_document_tag(request).await?;
    Ok(())
}

/// Test for deleting SDT object online.
#[tokio::test]
async fn structured_document_tag_delete_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteStructuredDocumentTagOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into());

    context.api().delete_structured_document_tag_online(request).await?;
    Ok(())
}

/// Test for updating SDT object.
#[tokio::test]
async fn structured_document_tag_update_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/StructuredDocumentTag")?;
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;
    let remoteFileName = test_string!("TestUpdateStructuredDocumentTag.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestStructuredDocumentTagListItems0 = StructuredDocumentTagListItem::default();
    requestStructuredDocumentTagListItems0.r#display_text = Some((test_string!("Aspose Words")?).into());
    requestStructuredDocumentTagListItems0.r#value = Some((test_string!("1")?).into());
    let mut requestStructuredDocumentTagListItems1 = StructuredDocumentTagListItem::default();
    requestStructuredDocumentTagListItems1.r#display_text = Some((test_string!("Hello world")?).into());
    requestStructuredDocumentTagListItems1.r#value = Some((test_string!("2")?).into());
    let requestStructuredDocumentTagListItems = vec![
    requestStructuredDocumentTagListItems0,
    requestStructuredDocumentTagListItems1
    ];
    let mut requestStructuredDocumentTag = StructuredDocumentTagUpdate::default();
    requestStructuredDocumentTag.r#list_items = Some((requestStructuredDocumentTagListItems).into());

    let request = UpdateStructuredDocumentTagRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestStructuredDocumentTag).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().update_structured_document_tag(request).await?;
    Ok(())
}

/// Test for updating SDT object online.
#[tokio::test]
async fn structured_document_tag_update_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestStructuredDocumentTagListItems0 = StructuredDocumentTagListItem::default();
    requestStructuredDocumentTagListItems0.r#display_text = Some((test_string!("Aspose Words")?).into());
    requestStructuredDocumentTagListItems0.r#value = Some((test_string!("1")?).into());
    let mut requestStructuredDocumentTagListItems1 = StructuredDocumentTagListItem::default();
    requestStructuredDocumentTagListItems1.r#display_text = Some((test_string!("Hello world")?).into());
    requestStructuredDocumentTagListItems1.r#value = Some((test_string!("2")?).into());
    let requestStructuredDocumentTagListItems = vec![
    requestStructuredDocumentTagListItems0,
    requestStructuredDocumentTagListItems1
    ];
    let mut requestStructuredDocumentTag = StructuredDocumentTagUpdate::default();
    requestStructuredDocumentTag.r#list_items = Some((requestStructuredDocumentTagListItems).into());

    let request = UpdateStructuredDocumentTagOnlineRequest::new(
        (requestDocument).into(),
        (requestStructuredDocumentTag).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/body/paragraphs/0")?).into());

    context.api().update_structured_document_tag_online(request).await?;
    Ok(())
}