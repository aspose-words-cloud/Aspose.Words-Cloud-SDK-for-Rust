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

/// Test for adding footnote.
#[tokio::test]
async fn footnote_insert_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestInsertFootnote.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_footnote_dto = FootnoteInsert::default();
    request_footnote_dto.footnote_type = Some((FootnoteBaseFootnoteTypeEnum::Endnote).into());
    request_footnote_dto.text = Some(("test endnote".to_owned()).into());

    let request = InsertFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (request_footnote_dto).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.NodeId", "0.1.7.1".to_owned())?;
    assert_string(&result_json, "Footnote.Text", " test endnote".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for adding footnote online.
#[tokio::test]
async fn footnote_insert_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let footnote_folder = "DocumentElements/Footnotes".to_owned();

    let request_document = context.load_binary_file(footnote_folder.clone() + "/Footnote.doc").await?;
    let mut request_footnote_dto = FootnoteInsert::default();
    request_footnote_dto.footnote_type = Some((FootnoteBaseFootnoteTypeEnum::Endnote).into());
    request_footnote_dto.text = Some(("test endnote".to_owned()).into());

    let request = InsertFootnoteOnlineRequest::new(
        (request_document).into(),
        (request_footnote_dto).into()
    ).with_node_path(("".to_owned()).into());

    context.api().insert_footnote_online(request).await?;
    Ok(())
}

/// Test for adding footnote without node path.
#[tokio::test]
async fn footnote_insert_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestInsertFootnoteWithoutNodePath.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_footnote_dto = FootnoteInsert::default();
    request_footnote_dto.footnote_type = Some((FootnoteBaseFootnoteTypeEnum::Endnote).into());
    request_footnote_dto.text = Some(("test endnote".to_owned()).into());

    let request = InsertFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (request_footnote_dto).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.NodeId", "0.1.7.1".to_owned())?;
    assert_string(&result_json, "Footnote.Text", " test endnote".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for deleting footnote.
#[tokio::test]
async fn footnote_delete_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestDeleteFootnote.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_footnote(request).await?;
    Ok(())
}

/// Test for deleting footnote online.
#[tokio::test]
async fn footnote_delete_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let footnote_folder = "DocumentElements/Footnotes".to_owned();

    let request_document = context.load_binary_file(footnote_folder.clone() + "/Footnote.doc").await?;

    let request = DeleteFootnoteOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_footnote_online(request).await?;
    Ok(())
}

/// Test for deleting footnote without node path.
#[tokio::test]
async fn footnote_delete_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestDeleteFootnoteWithoutNodePath.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_footnote(request).await?;
    Ok(())
}

/// Test for getting footnotes.
#[tokio::test]
async fn footnote_get_footnotes() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestGetFootnotes.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFootnotesRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_footnotes(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnotes")?;
    assert_not_null(&result_json, "Footnotes.List")?;
    assert_length(&result_json, "Footnotes.List", 6)?;
    assert_string(&result_json, "Footnotes.List[0].Text", " Footnote 1.".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for getting footnotes online.
#[tokio::test]
async fn footnote_get_footnotes_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let footnote_folder = "DocumentElements/Footnotes".to_owned();

    let request_document = context.load_binary_file(footnote_folder.clone() + "/Footnote.doc").await?;

    let request = GetFootnotesOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_footnotes_online(request).await?;
    Ok(())
}

/// Test for getting footnotes without node path.
#[tokio::test]
async fn footnote_get_footnotes_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestGetFootnotesWithoutNodePath.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFootnotesRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_footnotes(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnotes")?;
    assert_not_null(&result_json, "Footnotes.List")?;
    assert_length(&result_json, "Footnotes.List", 6)?;
    assert_string(&result_json, "Footnotes.List[0].Text", " Footnote 1.".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for getting footnote.
#[tokio::test]
async fn footnote_get_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestGetFootnote.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", " Footnote 1.".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for getting footnote online.
#[tokio::test]
async fn footnote_get_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let footnote_folder = "DocumentElements/Footnotes".to_owned();

    let request_document = context.load_binary_file(footnote_folder.clone() + "/Footnote.doc").await?;

    let request = GetFootnoteOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_footnote_online(request).await?;
    Ok(())
}

/// Test for getting footnote without node path.
#[tokio::test]
async fn footnote_get_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestGetFootnoteWithoutNodePath.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", " Footnote 1.".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for updating footnote.
#[tokio::test]
async fn footnote_update_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestUpdateFootnote.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_footnote_dto = FootnoteUpdate::default();
    request_footnote_dto.text = Some(("new text is here".to_owned()).into());

    let request = UpdateFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_footnote_dto).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", " new text is here".to_owned() + "\r\n")?;
    Ok(())
}

/// Test for updating footnote online.
#[tokio::test]
async fn footnote_update_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let footnote_folder = "DocumentElements/Footnotes".to_owned();

    let request_document = context.load_binary_file(footnote_folder.clone() + "/Footnote.doc").await?;
    let mut request_footnote_dto = FootnoteUpdate::default();
    request_footnote_dto.text = Some(("new text is here".to_owned()).into());

    let request = UpdateFootnoteOnlineRequest::new(
        (request_document).into(),
        (request_footnote_dto).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().update_footnote_online(request).await?;
    Ok(())
}

/// Test for updating footnote without node path.
#[tokio::test]
async fn footnote_update_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Footnotes";
    let footnote_folder = "DocumentElements/Footnotes".to_owned();
    let remote_file_name = "TestUpdateFootnoteWithoutNodePath.docx".to_owned();

    context.upload_file(footnote_folder.clone() + "/Footnote.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_footnote_dto = FootnoteUpdate::default();
    request_footnote_dto.text = Some(("new text is here".to_owned()).into());

    let request = UpdateFootnoteRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_footnote_dto).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", " new text is here".to_owned() + "\r\n")?;
    Ok(())
}