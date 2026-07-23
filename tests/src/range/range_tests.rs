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

/// Test for getting the text from range.
#[tokio::test]
async fn range_get_range_text() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Range";
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();
    let remote_file_name = "TestGetRangeText.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetRangeTextRequest::new(
        (remote_file_name.clone()).into(),
        ("id0.0.0".to_owned()).into(),
    )
    .with_range_end_identifier(("id0.0.1".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_range_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_string(&result_json, "Text", "This is HEADER ".to_owned())?;
    Ok(())
}

/// Test for getting the text from range online.
#[tokio::test]
async fn range_get_range_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        GetRangeTextOnlineRequest::new((request_document).into(), ("id0.0.0".to_owned()).into())
            .with_range_end_identifier(("id0.0.1".to_owned()).into());

    context.api().get_range_text_online(request).await?;
    Ok(())
}

/// Test for removing the text for range.
#[tokio::test]
async fn range_remove_range() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Range";
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();
    let remote_file_name = "TestRemoveRange.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = RemoveRangeRequest::new(
        (remote_file_name.clone()).into(),
        ("id0.0.0".to_owned()).into(),
    )
    .with_range_end_identifier(("id0.0.1".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context.api().remove_range(request).await?;
    Ok(())
}

/// Test for removing the text for range online.
#[tokio::test]
async fn range_remove_range_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        RemoveRangeOnlineRequest::new((request_document).into(), ("id0.0.0".to_owned()).into())
            .with_range_end_identifier(("id0.0.1".to_owned()).into());

    context.api().remove_range_online(request).await?;
    Ok(())
}

/// Test for saving a range as a new document.
#[tokio::test]
async fn range_save_as_range() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Range";
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();
    let remote_file_name = "TestSaveAsRange.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_document_parameters = RangeDocument::default();
    request_document_parameters.document_name =
        Some((remote_data_folder.clone() + "/NewDoc.docx").into());

    let request = SaveAsRangeRequest::new(
        (remote_file_name.clone()).into(),
        ("id0.0.0".to_owned()).into(),
        (request_document_parameters).into(),
    )
    .with_range_end_identifier(("id0.0.1".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().save_as_range(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "NewDoc.docx".to_owned())?;
    Ok(())
}

/// Test for saving a range as a new document online.
#[tokio::test]
async fn range_save_as_range_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Range";
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_document_parameters = RangeDocument::default();
    request_document_parameters.document_name =
        Some((remote_data_folder.clone() + "/NewDoc.docx").into());

    let request = SaveAsRangeOnlineRequest::new(
        (request_document).into(),
        ("id0.0.0".to_owned()).into(),
        (request_document_parameters).into(),
    )
    .with_range_end_identifier(("id0.0.1".to_owned()).into());

    context.api().save_as_range_online(request).await?;
    Ok(())
}

/// Test for replacing text in range.
#[tokio::test]
async fn range_replace_with_text() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Range";
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();
    let remote_file_name = "TestReplaceWithText.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_range_text = ReplaceRange::default();
    request_range_text.text = Some(("Replaced header".to_owned()).into());

    let request = ReplaceWithTextRequest::new(
        (remote_file_name.clone()).into(),
        ("id0.0.0".to_owned()).into(),
        (request_range_text).into(),
    )
    .with_range_end_identifier(("id0.0.1".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().replace_with_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(
        &result_json,
        "Document.FileName",
        "TestReplaceWithText.docx".to_owned(),
    )?;
    Ok(())
}

/// Test for replacing text in range online.
#[tokio::test]
async fn range_replace_with_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_range_text = ReplaceRange::default();
    request_range_text.text = Some(("Replaced header".to_owned()).into());

    let request = ReplaceWithTextOnlineRequest::new(
        (request_document).into(),
        ("id0.0.0".to_owned()).into(),
        (request_range_text).into(),
    )
    .with_range_end_identifier(("id0.0.1".to_owned()).into());

    context.api().replace_with_text_online(request).await?;
    Ok(())
}

/// Test to translate node id to node path.
#[tokio::test]
async fn range_translate_node_id() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Range";
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();
    let remote_file_name = "TestTranslateNodeId.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = TranslateNodeIdRequest::new(
        (remote_file_name.clone()).into(),
        ("id0.0.0".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().translate_node_id(request).await?;
    let result_json = serialize_result(&result)?;
    assert_string(
        &result_json,
        "Path",
        "sections/0/body/paragraphs/0".to_owned(),
    )?;
    Ok(())
}

/// Test to translate node id to node path online.
#[tokio::test]
async fn range_translate_node_id_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Range/RangeGet.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        TranslateNodeIdOnlineRequest::new((request_document).into(), ("id0.0.0".to_owned()).into());

    context.api().translate_node_id_online(request).await?;
    Ok(())
}
