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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphByIndex.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", test_string!("0.0.0")?)?;
    Ok(())
}

/// Test for getting paragraph online.
#[tokio::test]
async fn paragraph_get_document_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetParagraphOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_paragraph_online(request).await?;
    Ok(())
}

/// Test for getting paragraph without node path.
#[tokio::test]
async fn paragraph_get_document_paragraph_by_index_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphByIndexWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", test_string!("0.0.0")?)?;
    Ok(())
}

/// Test for getting all paragraphs.
#[tokio::test]
async fn paragraph_get_document_paragraphs() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphs.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_paragraphs(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraphs")?;
    assert_not_null(&result_json, "Paragraphs.ParagraphLinkList")?;
    assert_length(&result_json, "Paragraphs.ParagraphLinkList", 15)?;
    assert_string(&result_json, "Paragraphs.ParagraphLinkList[0].Text", test_string!("Page 1 of 3")?)?;
    Ok(())
}

/// Test for getting all paragraphs online.
#[tokio::test]
async fn paragraph_get_document_paragraphs_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetParagraphsOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_paragraphs_online(request).await?;
    Ok(())
}

/// Test for getting all paragraphs without node path.
#[tokio::test]
async fn paragraph_get_document_paragraphs_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphsWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_paragraphs(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraphs")?;
    assert_not_null(&result_json, "Paragraphs.ParagraphLinkList")?;
    assert_length(&result_json, "Paragraphs.ParagraphLinkList", 15)?;
    assert_string(&result_json, "Paragraphs.ParagraphLinkList[0].Text", test_string!("Page 1 of 3")?)?;
    Ok(())
}

/// Test for getting paragraph run.
#[tokio::test]
async fn paragraph_get_document_paragraph_run() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphRun.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetRunRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/0")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_run(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Run")?;
    assert_string(&result_json, "Run.Text", test_string!("Page ")?)?;
    Ok(())
}

/// Test for getting paragraph run online.
#[tokio::test]
async fn paragraph_get_document_paragraph_run_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetRunOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/0")?).into(),
        (0).into()
    );

    context.api().get_run_online(request).await?;
    Ok(())
}

/// Test for getting paragraph run font.
#[tokio::test]
async fn paragraph_get_document_paragraph_run_font() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphRunFont.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetRunFontRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/0")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_run_font(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Font")?;
    assert_string(&result_json, "Font.Name", test_string!("Times New Roman")?)?;
    Ok(())
}

/// Test for getting paragraph run font online.
#[tokio::test]
async fn paragraph_get_document_paragraph_run_font_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetRunFontOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/0")?).into(),
        (0).into()
    );

    context.api().get_run_font_online(request).await?;
    Ok(())
}

/// Test for getting paragraph runs.
#[tokio::test]
async fn paragraph_get_paragraph_runs() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetParagraphRuns.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetRunsRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/paragraphs/0")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_runs(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Runs")?;
    assert_not_null(&result_json, "Runs.List")?;
    assert_length(&result_json, "Runs.List", 6)?;
    assert_string(&result_json, "Runs.List[0].Text", test_string!("Page ")?)?;
    Ok(())
}

/// Test for getting paragraph runs online.
#[tokio::test]
async fn paragraph_get_paragraph_runs_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetRunsOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/paragraphs/0")?).into()
    );

    context.api().get_runs_online(request).await?;
    Ok(())
}

/// Test for updating paragraph run font.
#[tokio::test]
async fn paragraph_update_run_font() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateRunFont.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFontDto = Font::default();
    requestFontDto.r#bold = Some((true).into());

    let request = UpdateRunFontRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/0")?).into(),
        (0).into(),
        (requestFontDto).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestFontDto = Font::default();
    requestFontDto.r#bold = Some((true).into());

    let request = UpdateRunFontOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/0")?).into(),
        (requestFontDto).into(),
        (0).into()
    );

    context.api().update_run_font_online(request).await?;
    Ok(())
}

/// Test for adding paragraph.
#[tokio::test]
async fn paragraph_insert_paragraph() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertParagraph.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestParagraph = ParagraphInsert::default();
    requestParagraph.r#text = Some((test_string!("This is a new paragraph for your document")?).into());

    let request = InsertParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestParagraph).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", test_string!("0.3.8")?)?;
    Ok(())
}

/// Test for adding paragraph online.
#[tokio::test]
async fn paragraph_insert_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestParagraph = ParagraphInsert::default();
    requestParagraph.r#text = Some((test_string!("This is a new paragraph for your document")?).into());

    let request = InsertParagraphOnlineRequest::new(
        (requestDocument).into(),
        (requestParagraph).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().insert_paragraph_online(request).await?;
    Ok(())
}

/// Test for adding paragraph without node path.
#[tokio::test]
async fn paragraph_insert_paragraph_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertParagraphWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestParagraph = ParagraphInsert::default();
    requestParagraph.r#text = Some((test_string!("This is a new paragraph for your document")?).into());

    let request = InsertParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestParagraph).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_paragraph(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Paragraph")?;
    assert_string(&result_json, "Paragraph.NodeId", test_string!("0.3.8")?)?;
    Ok(())
}

/// Test for paragraph rendering.
#[tokio::test]
async fn paragraph_render_paragraph() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestRenderParagraph.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_paragraph(request).await?;
    Ok(())
}

/// Test for paragraph rendering.
#[tokio::test]
async fn paragraph_render_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = RenderParagraphOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().render_paragraph_online(request).await?;
    Ok(())
}

/// Test for paragraph rendering without node path.
#[tokio::test]
async fn paragraph_render_paragraph_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestRenderParagraphWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_paragraph(request).await?;
    Ok(())
}

/// Test for getting paragraph format settings.
#[tokio::test]
async fn paragraph_get_paragraph_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphs.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_paragraph_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ParagraphFormat")?;
    assert_string(&result_json, "ParagraphFormat.StyleName", test_string!("Normal")?)?;
    Ok(())
}

/// Test for getting paragraph format settings online.
#[tokio::test]
async fn paragraph_get_paragraph_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetParagraphFormatOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_paragraph_format_online(request).await?;
    Ok(())
}

/// Test for getting paragraph format settings without node path.
#[tokio::test]
async fn paragraph_get_paragraph_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphsWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_paragraph_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ParagraphFormat")?;
    assert_string(&result_json, "ParagraphFormat.StyleName", test_string!("Normal")?)?;
    Ok(())
}

/// Test for updating  paragraph format settings.
#[tokio::test]
async fn paragraph_update_paragraph_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentParagraphs.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestParagraphFormatDto = ParagraphFormatUpdate::default();
    requestParagraphFormatDto.r#alignment = Some((ParagraphFormatBase_AlignmentEnum::Right).into());

    let request = UpdateParagraphFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestParagraphFormatDto).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_paragraph_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ParagraphFormat")?;

    Ok(())
}

/// Test for updating  paragraph format settings online.
#[tokio::test]
async fn paragraph_update_paragraph_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestParagraphFormatDto = ParagraphFormatUpdate::default();
    requestParagraphFormatDto.r#alignment = Some((ParagraphFormatBase_AlignmentEnum::Right).into());

    let request = UpdateParagraphFormatOnlineRequest::new(
        (requestDocument).into(),
        (requestParagraphFormatDto).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().update_paragraph_format_online(request).await?;
    Ok(())
}

/// Test for deleting  a paragraph.
#[tokio::test]
async fn paragraph_delete_paragraph() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteParagraph.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_paragraph(request).await?;
    Ok(())
}

/// Test for deleting  a paragraph online.
#[tokio::test]
async fn paragraph_delete_paragraph_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteParagraphOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_paragraph_online(request).await?;
    Ok(())
}

/// Test for deleting  a paragraph without node path.
#[tokio::test]
async fn paragraph_delete_paragraph_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteParagraphWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteParagraphRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_paragraph(request).await?;
    Ok(())
}

/// Test for getting paragraph list format.
#[tokio::test]
async fn paragraph_get_paragraph_list_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;
    let remoteFileName = test_string!("TestParagraphGetListFormat.docx")?;

    context.upload_file(test_string!(listFolder + "/ParagraphGetListFormat.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphListFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;

    let requestDocument = context.load_binary_file(test_string!(listFolder + "/ParagraphGetListFormat.doc")?).await?;

    let request = GetParagraphListFormatOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_paragraph_list_format_online(request).await?;
    Ok(())
}

/// Test for getting paragraph list format without node path.
#[tokio::test]
async fn paragraph_get_paragraph_list_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;
    let remoteFileName = test_string!("TestParagraphGetListFormatWithoutNodePath.docx")?;

    context.upload_file(test_string!(listFolder + "/ParagraphGetListFormat.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphListFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;
    let remoteFileName = test_string!("TestUpdateParagraphListFormat.docx")?;

    context.upload_file(test_string!(listFolder + "/ParagraphUpdateListFormat.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestListFormatDto = ListFormatUpdate::default();
    requestListFormatDto.r#list_id = Some((2).into());

    let request = UpdateParagraphListFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestListFormatDto).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;

    let requestDocument = context.load_binary_file(test_string!(listFolder + "/ParagraphUpdateListFormat.doc")?).await?;
    let mut requestListFormatDto = ListFormatUpdate::default();
    requestListFormatDto.r#list_id = Some((2).into());

    let request = UpdateParagraphListFormatOnlineRequest::new(
        (requestDocument).into(),
        (requestListFormatDto).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().update_paragraph_list_format_online(request).await?;
    Ok(())
}

/// Test for updating paragraph list format without node path.
#[tokio::test]
async fn paragraph_update_paragraph_list_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;
    let remoteFileName = test_string!("TestUpdateParagraphListFormatWithoutNodePath.docx")?;

    context.upload_file(test_string!(listFolder + "/ParagraphUpdateListFormat.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestListFormatDto = ListFormatUpdate::default();
    requestListFormatDto.r#list_id = Some((2).into());

    let request = UpdateParagraphListFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestListFormatDto).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;
    let remoteFileName = test_string!("TestDeleteParagraphListFormat.docx")?;

    context.upload_file(test_string!(listFolder + "/ParagraphDeleteListFormat.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteParagraphListFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_paragraph_list_format(request).await?;
    Ok(())
}

/// Test for deleting paragraph list format online.
#[tokio::test]
async fn paragraph_delete_paragraph_list_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;

    let requestDocument = context.load_binary_file(test_string!(listFolder + "/ParagraphDeleteListFormat.doc")?).await?;

    let request = DeleteParagraphListFormatOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_paragraph_list_format_online(request).await?;
    Ok(())
}

/// Test for deleting paragraph list format without node path.
#[tokio::test]
async fn paragraph_delete_paragraph_list_format_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let listFolder = test_string!("DocumentElements/ParagraphListFormat")?;
    let remoteFileName = test_string!("TestDeleteParagraphListFormatWithoutNodePath.docx")?;

    context.upload_file(test_string!(listFolder + "/ParagraphDeleteListFormat.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteParagraphListFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_paragraph_list_format(request).await?;
    Ok(())
}

/// Test for getting paragraph tab stops.
#[tokio::test]
async fn paragraph_get_paragraph_tab_stops() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestGetParagraphTabStops.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphTabStopsRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;

    let requestDocument = context.load_binary_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?).await?;

    let request = GetParagraphTabStopsOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_paragraph_tab_stops_online(request).await?;
    Ok(())
}

/// Test for getting paragraph tab stops without node path.
#[tokio::test]
async fn paragraph_get_paragraph_tab_stops_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestGetParagraphTabStopsWithoutNodePath.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetParagraphTabStopsRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestInsertOrUpdateParagraphTabStop.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestTabStopInsertDto = TabStopInsert::default();
    requestTabStopInsertDto.r#alignment = Some((TabStopBase_AlignmentEnum::Left).into());
    requestTabStopInsertDto.r#leader = Some((TabStopBase_LeaderEnum::None).into());
    requestTabStopInsertDto.r#position = Some(((100.0) as f64).into());

    let request = InsertOrUpdateParagraphTabStopRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestTabStopInsertDto).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;

    let requestDocument = context.load_binary_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?).await?;
    let mut requestTabStopInsertDto = TabStopInsert::default();
    requestTabStopInsertDto.r#alignment = Some((TabStopBase_AlignmentEnum::Left).into());
    requestTabStopInsertDto.r#leader = Some((TabStopBase_LeaderEnum::None).into());
    requestTabStopInsertDto.r#position = Some(((72) as f64).into());

    let request = InsertOrUpdateParagraphTabStopOnlineRequest::new(
        (requestDocument).into(),
        (requestTabStopInsertDto).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().insert_or_update_paragraph_tab_stop_online(request).await?;
    Ok(())
}

/// Test for inserting paragraph tab stop without node path.
#[tokio::test]
async fn paragraph_insert_paragraph_tab_stops_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestInsertOrUpdateParagraphTabStopWithoutNodePath.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestTabStopInsertDto = TabStopInsert::default();
    requestTabStopInsertDto.r#alignment = Some((TabStopBase_AlignmentEnum::Left).into());
    requestTabStopInsertDto.r#leader = Some((TabStopBase_LeaderEnum::None).into());
    requestTabStopInsertDto.r#position = Some(((100.0) as f64).into());

    let request = InsertOrUpdateParagraphTabStopRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestTabStopInsertDto).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestDeleteAllParagraphTabStops.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteAllParagraphTabStopsRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;

    let requestDocument = context.load_binary_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?).await?;

    let request = DeleteAllParagraphTabStopsOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_all_paragraph_tab_stops_online(request).await?;
    Ok(())
}

/// Test for deleting all paragraph tab stops without node path.
#[tokio::test]
async fn paragraph_delete_all_paragraph_tab_stops_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestDeleteAllParagraphTabStopsWithoutNodePath.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteAllParagraphTabStopsRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestDeleteParagraphTabStop.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteParagraphTabStopRequest::new(
        (test_string!(remoteFileName)?).into(),
        ((72.0) as f64).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;

    let requestDocument = context.load_binary_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?).await?;

    let request = DeleteParagraphTabStopOnlineRequest::new(
        (requestDocument).into(),
        ((72.0) as f64).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_paragraph_tab_stop_online(request).await?;
    Ok(())
}

/// Test for deleting a tab stops without node path.
#[tokio::test]
async fn paragraph_delete_paragraph_tab_stop_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Paragraphs")?;
    let tabStopFolder = test_string!("DocumentElements/Paragraphs")?;
    let remoteFileName = test_string!("TestDeleteParagraphTabStopWithoutNodePath.docx")?;

    context.upload_file(test_string!(tabStopFolder + "/ParagraphTabStops.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteParagraphTabStopRequest::new(
        (test_string!(remoteFileName)?).into(),
        ((72.0) as f64).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().delete_paragraph_tab_stop(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "TabStops")?;
    assert_length(&result_json, "TabStops", 1)?;
    Ok(())
}