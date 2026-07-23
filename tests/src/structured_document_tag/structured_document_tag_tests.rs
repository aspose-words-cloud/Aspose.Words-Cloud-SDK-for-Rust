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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/StructuredDocumentTag";
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();
    let remote_file_name = "TestGetStructuredDocumentTags.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetStructuredDocumentTagsRequest::new((remote_file_name.clone()).into())
        .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().get_structured_document_tags(request).await?;
    Ok(())
}

/// Test for getting SDT objects from document online.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tags_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetStructuredDocumentTagsOnlineRequest::new((request_document).into())
        .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into());

    context
        .api()
        .get_structured_document_tags_online(request)
        .await?;
    Ok(())
}

/// Test for getting SDT object from document.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/StructuredDocumentTag";
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();
    let remote_file_name = "TestGetStructuredDocumentTag.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        GetStructuredDocumentTagRequest::new((remote_file_name.clone()).into(), (0).into())
            .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

    context.api().get_structured_document_tag(request).await?;
    Ok(())
}

/// Test for getting SDT object from document online.
#[tokio::test]
async fn structured_document_tag_get_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetStructuredDocumentTagOnlineRequest::new((request_document).into(), (0).into())
        .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into());

    context
        .api()
        .get_structured_document_tag_online(request)
        .await?;
    Ok(())
}

/// Test for adding SDT object.
#[tokio::test]
async fn structured_document_tag_insert_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/StructuredDocumentTag";
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();
    let remote_file_name = "TestInsetStructuredDocumentTag.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_structured_document_tag = StructuredDocumentTagInsert::default();
    request_structured_document_tag.sdt_type =
        Some((StructuredDocumentTagInsertSdtTypeEnum::ComboBox).into());
    request_structured_document_tag.level =
        Some((StructuredDocumentTagInsertLevelEnum::Inline).into());

    let request = InsertStructuredDocumentTagRequest::new(
        (remote_file_name.clone()).into(),
        (request_structured_document_tag).into(),
    )
    .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .insert_structured_document_tag(request)
        .await?;
    Ok(())
}

/// Test for adding SDT object online.
#[tokio::test]
async fn structured_document_tag_insert_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_structured_document_tag = StructuredDocumentTagInsert::default();
    request_structured_document_tag.sdt_type =
        Some((StructuredDocumentTagInsertSdtTypeEnum::ComboBox).into());
    request_structured_document_tag.level =
        Some((StructuredDocumentTagInsertLevelEnum::Inline).into());

    let request = InsertStructuredDocumentTagOnlineRequest::new(
        (request_document).into(),
        (request_structured_document_tag).into(),
    )
    .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into());

    context
        .api()
        .insert_structured_document_tag_online(request)
        .await?;
    Ok(())
}

/// Test for deleting SDT object.
#[tokio::test]
async fn structured_document_tag_delete_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/StructuredDocumentTag";
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();
    let remote_file_name = "TestDeleteStructuredDocumentTag.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        DeleteStructuredDocumentTagRequest::new((remote_file_name.clone()).into(), (0).into())
            .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .delete_structured_document_tag(request)
        .await?;
    Ok(())
}

/// Test for deleting SDT object online.
#[tokio::test]
async fn structured_document_tag_delete_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        DeleteStructuredDocumentTagOnlineRequest::new((request_document).into(), (0).into())
            .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into());

    context
        .api()
        .delete_structured_document_tag_online(request)
        .await?;
    Ok(())
}

/// Test for updating SDT object.
#[tokio::test]
async fn structured_document_tag_update_structured_document_tag() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/StructuredDocumentTag";
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();
    let remote_file_name = "TestUpdateStructuredDocumentTag.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_structured_document_tag_list_items0 = StructuredDocumentTagListItem::default();
    request_structured_document_tag_list_items0.display_text =
        Some(("Aspose Words".to_owned()).into());
    request_structured_document_tag_list_items0.value = Some(("1".to_owned()).into());
    let mut request_structured_document_tag_list_items1 = StructuredDocumentTagListItem::default();
    request_structured_document_tag_list_items1.display_text =
        Some(("Hello world".to_owned()).into());
    request_structured_document_tag_list_items1.value = Some(("2".to_owned()).into());
    let request_structured_document_tag_list_items = vec![
        request_structured_document_tag_list_items0,
        request_structured_document_tag_list_items1,
    ];
    let mut request_structured_document_tag = StructuredDocumentTagUpdate::default();
    request_structured_document_tag.list_items =
        Some((request_structured_document_tag_list_items).into());

    let request = UpdateStructuredDocumentTagRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_structured_document_tag).into(),
    )
    .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .update_structured_document_tag(request)
        .await?;
    Ok(())
}

/// Test for updating SDT object online.
#[tokio::test]
async fn structured_document_tag_update_structured_document_tag_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/StructuredDocumentTag/StructuredDocumentTag.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_structured_document_tag_list_items0 = StructuredDocumentTagListItem::default();
    request_structured_document_tag_list_items0.display_text =
        Some(("Aspose Words".to_owned()).into());
    request_structured_document_tag_list_items0.value = Some(("1".to_owned()).into());
    let mut request_structured_document_tag_list_items1 = StructuredDocumentTagListItem::default();
    request_structured_document_tag_list_items1.display_text =
        Some(("Hello world".to_owned()).into());
    request_structured_document_tag_list_items1.value = Some(("2".to_owned()).into());
    let request_structured_document_tag_list_items = vec![
        request_structured_document_tag_list_items0,
        request_structured_document_tag_list_items1,
    ];
    let mut request_structured_document_tag = StructuredDocumentTagUpdate::default();
    request_structured_document_tag.list_items =
        Some((request_structured_document_tag_list_items).into());

    let request = UpdateStructuredDocumentTagOnlineRequest::new(
        (request_document).into(),
        (request_structured_document_tag).into(),
        (0).into(),
    )
    .with_node_path(("sections/0/body/paragraphs/0".to_owned()).into());

    context
        .api()
        .update_structured_document_tag_online(request)
        .await?;
    Ok(())
}
