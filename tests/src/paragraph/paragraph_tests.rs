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

/// Test for getting paragraph.
#[tokio::test]
async fn paragraph_get_document_paragraph_by_index() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphByIndex.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", "0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting paragraph online.
#[tokio::test]
async fn paragraph_get_document_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetParagraphOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().get_paragraph_online(request).await?;
    Ok(())
}

/// Test for getting paragraph without node path.
#[tokio::test]
async fn paragraph_get_document_paragraph_by_index_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphByIndexWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", "0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting all paragraphs.
#[tokio::test]
async fn paragraph_get_document_paragraphs() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphs.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraphs(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraphs")?;
    assert_not_null(&result_json, "Paragraphs.ParagraphLinkList")?;
    assert_length(&result_json, "Paragraphs.ParagraphLinkList", 15)?;
    assert_string(&result_json, "Paragraphs.ParagraphLinkList[0].Text", "Page 1 of 3".to_owned())?;
    Ok(())
}

/// Test for getting all paragraphs online.
#[tokio::test]
async fn paragraph_get_document_paragraphs_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetParagraphsOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().get_paragraphs_online(request).await?;
    Ok(())
}

/// Test for getting all paragraphs without node path.
#[tokio::test]
async fn paragraph_get_document_paragraphs_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphsWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraphs(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraphs")?;
    assert_not_null(&result_json, "Paragraphs.ParagraphLinkList")?;
    assert_length(&result_json, "Paragraphs.ParagraphLinkList", 15)?;
    assert_string(&result_json, "Paragraphs.ParagraphLinkList[0].Text", "Page 1 of 3".to_owned())?;
    Ok(())
}

/// Test for getting paragraph run.
#[tokio::test]
async fn paragraph_get_document_paragraph_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphRun.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetRunRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/0".to_owned()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_run(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Run")?;
    assert_string(&result_json, "Run.Text", "Page ".to_owned())?;
    Ok(())
}

/// Test for getting paragraph run online.
#[tokio::test]
async fn paragraph_get_document_paragraph_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetRunOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/0".to_owned()).into(),
        (0).into()
    );

    context.api().get_run_online(request).await?;
    Ok(())
}

/// Test for getting paragraph run font.
#[tokio::test]
async fn paragraph_get_document_paragraph_run_font() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphRunFont.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetRunFontRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/0".to_owned()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_run_font(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Font")?;
    assert_string(&result_json, "Font.Name", "Times New Roman".to_owned())?;
    Ok(())
}

/// Test for getting paragraph run font online.
#[tokio::test]
async fn paragraph_get_document_paragraph_run_font_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetRunFontOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/0".to_owned()).into(),
        (0).into()
    );

    context.api().get_run_font_online(request).await?;
    Ok(())
}

/// Test for getting paragraph runs.
#[tokio::test]
async fn paragraph_get_paragraph_runs() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetParagraphRuns.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetRunsRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/paragraphs/0".to_owned()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_runs(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Runs")?;
    assert_not_null(&result_json, "Runs.List")?;
    assert_length(&result_json, "Runs.List", 6)?;
    assert_string(&result_json, "Runs.List[0].Text", "Page ".to_owned())?;
    Ok(())
}

/// Test for getting paragraph runs online.
#[tokio::test]
async fn paragraph_get_paragraph_runs_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetRunsOnlineRequest::new(
        (request_document).into(),
        ("sections/0/paragraphs/0".to_owned()).into()
    );

    context.api().get_runs_online(request).await?;
    Ok(())
}

/// Test for updating paragraph run font.
#[tokio::test]
async fn paragraph_update_run_font() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateRunFont.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_font_dto = Font::default();
    request_font_dto.bold = Some((true).into());

    let request = UpdateRunFontRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/0".to_owned()).into(),
        (0).into(),
        (request_font_dto).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().update_run_font(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Font")?;
    assert_bool(&result_json, "Font.Bold", true)?;
    Ok(())
}

/// Test for updating paragraph run font online.
#[tokio::test]
async fn paragraph_update_run_font_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_font_dto = Font::default();
    request_font_dto.bold = Some((true).into());

    let request = UpdateRunFontOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/0".to_owned()).into(),
        (request_font_dto).into(),
        (0).into()
    );

    context.api().update_run_font_online(request).await?;
    Ok(())
}

/// Test for adding paragraph.
#[tokio::test]
async fn paragraph_insert_paragraph() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertParagraph.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_paragraph = ParagraphInsert::default();
    request_paragraph.text = Some(("This is a new paragraph for your document".to_owned()).into());

    let request = InsertParagraphRequest::new(
        (remote_file_name.clone()).into(),
        (request_paragraph).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", "0.3.8".to_owned())?;
    Ok(())
}

/// Test for adding paragraph online.
#[tokio::test]
async fn paragraph_insert_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_paragraph = ParagraphInsert::default();
    request_paragraph.text = Some(("This is a new paragraph for your document".to_owned()).into());

    let request = InsertParagraphOnlineRequest::new(
        (request_document).into(),
        (request_paragraph).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().insert_paragraph_online(request).await?;
    Ok(())
}

/// Test for adding paragraph without node path.
#[tokio::test]
async fn paragraph_insert_paragraph_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertParagraphWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_paragraph = ParagraphInsert::default();
    request_paragraph.text = Some(("This is a new paragraph for your document".to_owned()).into());

    let request = InsertParagraphRequest::new(
        (remote_file_name.clone()).into(),
        (request_paragraph).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", "0.3.8".to_owned())?;
    Ok(())
}

/// Test for paragraph rendering.
#[tokio::test]
async fn paragraph_render_paragraph() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestRenderParagraph.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = RenderParagraphRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().render_paragraph(request).await?;
    Ok(())
}

/// Test for paragraph rendering.
#[tokio::test]
async fn paragraph_render_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = RenderParagraphOnlineRequest::new(
        (request_document).into(),
        ("png".to_owned()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().render_paragraph_online(request).await?;
    Ok(())
}

/// Test for paragraph rendering without node path.
#[tokio::test]
async fn paragraph_render_paragraph_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestRenderParagraphWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = RenderParagraphRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().render_paragraph(request).await?;
    Ok(())
}

/// Test for getting paragraph format settings.
#[tokio::test]
async fn paragraph_get_paragraph_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphs.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ParagraphFormat")?;
    assert_string(&result_json, "ParagraphFormat.StyleName", "Normal".to_owned())?;
    Ok(())
}

/// Test for getting paragraph format settings online.
#[tokio::test]
async fn paragraph_get_paragraph_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetParagraphFormatOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_paragraph_format_online(request).await?;
    Ok(())
}

/// Test for getting paragraph format settings without node path.
#[tokio::test]
async fn paragraph_get_paragraph_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphsWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ParagraphFormat")?;
    assert_string(&result_json, "ParagraphFormat.StyleName", "Normal".to_owned())?;
    Ok(())
}

/// Test for updating  paragraph format settings.
#[tokio::test]
async fn paragraph_update_paragraph_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentParagraphs.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_paragraph_format_dto = ParagraphFormatUpdate::default();
    request_paragraph_format_dto.alignment = Some((ParagraphFormatBaseAlignmentEnum::Right).into());

    let request = UpdateParagraphFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_paragraph_format_dto).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_paragraph_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ParagraphFormat")?;

    Ok(())
}

/// Test for updating  paragraph format settings online.
#[tokio::test]
async fn paragraph_update_paragraph_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_paragraph_format_dto = ParagraphFormatUpdate::default();
    request_paragraph_format_dto.alignment = Some((ParagraphFormatBaseAlignmentEnum::Right).into());

    let request = UpdateParagraphFormatOnlineRequest::new(
        (request_document).into(),
        (request_paragraph_format_dto).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().update_paragraph_format_online(request).await?;
    Ok(())
}

/// Test for deleting  a paragraph.
#[tokio::test]
async fn paragraph_delete_paragraph() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteParagraph.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteParagraphRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_paragraph(request).await?;
    Ok(())
}

/// Test for deleting  a paragraph online.
#[tokio::test]
async fn paragraph_delete_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteParagraphOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_paragraph_online(request).await?;
    Ok(())
}

/// Test for deleting  a paragraph without node path.
#[tokio::test]
async fn paragraph_delete_paragraph_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteParagraphWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteParagraphRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_paragraph(request).await?;
    Ok(())
}

/// Test for getting paragraph list format.
#[tokio::test]
async fn paragraph_get_paragraph_list_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();
    let remote_file_name = "TestParagraphGetListFormat.docx".to_owned();

    context.upload_file(list_folder.clone() + "/ParagraphGetListFormat.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphListFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph_list_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ListFormat")?;
    assert_integer(&result_json, "ListFormat.ListId", 1)?;
    Ok(())
}

/// Test for getting paragraph list format online.
#[tokio::test]
async fn paragraph_get_paragraph_list_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();

    let request_document = context.load_binary_file(list_folder.clone() + "/ParagraphGetListFormat.doc").await?;

    let request = GetParagraphListFormatOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_paragraph_list_format_online(request).await?;
    Ok(())
}

/// Test for getting paragraph list format without node path.
#[tokio::test]
async fn paragraph_get_paragraph_list_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();
    let remote_file_name = "TestParagraphGetListFormatWithoutNodePath.docx".to_owned();

    context.upload_file(list_folder.clone() + "/ParagraphGetListFormat.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphListFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph_list_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ListFormat")?;
    assert_integer(&result_json, "ListFormat.ListId", 1)?;
    Ok(())
}

/// Test for updating paragraph list format.
#[tokio::test]
async fn paragraph_update_paragraph_list_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();
    let remote_file_name = "TestUpdateParagraphListFormat.docx".to_owned();

    context.upload_file(list_folder.clone() + "/ParagraphUpdateListFormat.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_list_format_dto = ListFormatUpdate::default();
    request_list_format_dto.list_id = Some((2).into());

    let request = UpdateParagraphListFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_list_format_dto).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_paragraph_list_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ListFormat")?;
    assert_integer(&result_json, "ListFormat.ListId", 2)?;
    Ok(())
}

/// Test for updating paragraph list format online.
#[tokio::test]
async fn paragraph_update_paragraph_list_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();

    let request_document = context.load_binary_file(list_folder.clone() + "/ParagraphUpdateListFormat.doc").await?;
    let mut request_list_format_dto = ListFormatUpdate::default();
    request_list_format_dto.list_id = Some((2).into());

    let request = UpdateParagraphListFormatOnlineRequest::new(
        (request_document).into(),
        (request_list_format_dto).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().update_paragraph_list_format_online(request).await?;
    Ok(())
}

/// Test for updating paragraph list format without node path.
#[tokio::test]
async fn paragraph_update_paragraph_list_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();
    let remote_file_name = "TestUpdateParagraphListFormatWithoutNodePath.docx".to_owned();

    context.upload_file(list_folder.clone() + "/ParagraphUpdateListFormat.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_list_format_dto = ListFormatUpdate::default();
    request_list_format_dto.list_id = Some((2).into());

    let request = UpdateParagraphListFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_list_format_dto).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_paragraph_list_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ListFormat")?;
    assert_integer(&result_json, "ListFormat.ListId", 2)?;
    Ok(())
}

/// Test for deleting paragraph list format.
#[tokio::test]
async fn paragraph_delete_paragraph_list_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();
    let remote_file_name = "TestDeleteParagraphListFormat.docx".to_owned();

    context.upload_file(list_folder.clone() + "/ParagraphDeleteListFormat.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteParagraphListFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_paragraph_list_format(request).await?;
    Ok(())
}

/// Test for deleting paragraph list format online.
#[tokio::test]
async fn paragraph_delete_paragraph_list_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();

    let request_document = context.load_binary_file(list_folder.clone() + "/ParagraphDeleteListFormat.doc").await?;

    let request = DeleteParagraphListFormatOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_paragraph_list_format_online(request).await?;
    Ok(())
}

/// Test for deleting paragraph list format without node path.
#[tokio::test]
async fn paragraph_delete_paragraph_list_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let list_folder = "DocumentElements/ParagraphListFormat".to_owned();
    let remote_file_name = "TestDeleteParagraphListFormatWithoutNodePath.docx".to_owned();

    context.upload_file(list_folder.clone() + "/ParagraphDeleteListFormat.doc", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteParagraphListFormatRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_paragraph_list_format(request).await?;
    Ok(())
}

/// Test for getting paragraph tab stops.
#[tokio::test]
async fn paragraph_get_paragraph_tab_stops() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestGetParagraphTabStops.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphTabStopsRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph_tab_stops(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 2)?;
    assert_float(&result_json, "TabStops[0].Position", (72.0) as f64)?;
    Ok(())
}

/// Test for getting paragraph tab stops online.
#[tokio::test]
async fn paragraph_get_paragraph_tab_stops_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();

    let request_document = context.load_binary_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx").await?;

    let request = GetParagraphTabStopsOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_paragraph_tab_stops_online(request).await?;
    Ok(())
}

/// Test for getting paragraph tab stops without node path.
#[tokio::test]
async fn paragraph_get_paragraph_tab_stops_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestGetParagraphTabStopsWithoutNodePath.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetParagraphTabStopsRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_paragraph_tab_stops(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 2)?;
    assert_float(&result_json, "TabStops[0].Position", (72.0) as f64)?;
    Ok(())
}

/// Test for inserting paragraph tab stop.
#[tokio::test]
async fn paragraph_insert_paragraph_tab_stops() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestInsertOrUpdateParagraphTabStop.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_tab_stop_insert_dto = TabStopInsert::default();
    request_tab_stop_insert_dto.alignment = Some((TabStopBaseAlignmentEnum::Left).into());
    request_tab_stop_insert_dto.leader = Some((TabStopBaseLeaderEnum::None).into());
    request_tab_stop_insert_dto.position = Some(((100.0) as f64).into());

    let request = InsertOrUpdateParagraphTabStopRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_tab_stop_insert_dto).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_or_update_paragraph_tab_stop(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 3)?;
    assert_float(&result_json, "TabStops[1].Position", (100.0) as f64)?;


    Ok(())
}

/// Test for inserting paragraph tab stop online.
#[tokio::test]
async fn paragraph_insert_paragraph_tab_stops_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();

    let request_document = context.load_binary_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx").await?;
    let mut request_tab_stop_insert_dto = TabStopInsert::default();
    request_tab_stop_insert_dto.alignment = Some((TabStopBaseAlignmentEnum::Left).into());
    request_tab_stop_insert_dto.leader = Some((TabStopBaseLeaderEnum::None).into());
    request_tab_stop_insert_dto.position = Some(((72) as f64).into());

    let request = InsertOrUpdateParagraphTabStopOnlineRequest::new(
        (request_document).into(),
        (request_tab_stop_insert_dto).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().insert_or_update_paragraph_tab_stop_online(request).await?;
    Ok(())
}

/// Test for inserting paragraph tab stop without node path.
#[tokio::test]
async fn paragraph_insert_paragraph_tab_stops_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestInsertOrUpdateParagraphTabStopWithoutNodePath.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_tab_stop_insert_dto = TabStopInsert::default();
    request_tab_stop_insert_dto.alignment = Some((TabStopBaseAlignmentEnum::Left).into());
    request_tab_stop_insert_dto.leader = Some((TabStopBaseLeaderEnum::None).into());
    request_tab_stop_insert_dto.position = Some(((100.0) as f64).into());

    let request = InsertOrUpdateParagraphTabStopRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_tab_stop_insert_dto).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_or_update_paragraph_tab_stop(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 3)?;
    assert_float(&result_json, "TabStops[1].Position", (100.0) as f64)?;


    Ok(())
}

/// Test for deleting all paragraph tab stops.
#[tokio::test]
async fn paragraph_delete_all_paragraph_tab_stops() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestDeleteAllParagraphTabStops.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteAllParagraphTabStopsRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().delete_all_paragraph_tab_stops(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 0)?;
    Ok(())
}

/// Test for deleting all paragraph tab stops online.
#[tokio::test]
async fn paragraph_delete_all_paragraph_tab_stops_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();

    let request_document = context.load_binary_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx").await?;

    let request = DeleteAllParagraphTabStopsOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_all_paragraph_tab_stops_online(request).await?;
    Ok(())
}

/// Test for deleting all paragraph tab stops without node path.
#[tokio::test]
async fn paragraph_delete_all_paragraph_tab_stops_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestDeleteAllParagraphTabStopsWithoutNodePath.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteAllParagraphTabStopsRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().delete_all_paragraph_tab_stops(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 0)?;
    Ok(())
}

/// Test for deleting a tab stops.
#[tokio::test]
async fn paragraph_delete_paragraph_tab_stop() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestDeleteParagraphTabStop.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteParagraphTabStopRequest::new(
        (remote_file_name.clone()).into(),
        ((72.0) as f64).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().delete_paragraph_tab_stop(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 1)?;
    Ok(())
}

/// Test for deleting a tab stops online.
#[tokio::test]
async fn paragraph_delete_paragraph_tab_stop_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();

    let request_document = context.load_binary_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx").await?;

    let request = DeleteParagraphTabStopOnlineRequest::new(
        (request_document).into(),
        ((72.0) as f64).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_paragraph_tab_stop_online(request).await?;
    Ok(())
}

/// Test for deleting a tab stops without node path.
#[tokio::test]
async fn paragraph_delete_paragraph_tab_stop_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Paragraphs";
    let tab_stop_folder = "DocumentElements/Paragraphs".to_owned();
    let remote_file_name = "TestDeleteParagraphTabStopWithoutNodePath.docx".to_owned();

    context.upload_file(tab_stop_folder.clone() + "/ParagraphTabStops.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteParagraphTabStopRequest::new(
        (remote_file_name.clone()).into(),
        ((72.0) as f64).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().delete_paragraph_tab_stop(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 1)?;
    Ok(())
}