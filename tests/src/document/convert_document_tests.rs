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

/// Test for converting document to one of the available formats.
#[tokio::test]
async fn convert_document_save_as() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/ConvertDocument";
    let local_name = "test_multi_pages.docx".to_owned();
    let remote_name = "TestSaveAs.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_name, remote_folder.clone() + "/" + &remote_name).await?;
    let mut request_save_options_data = PdfSaveOptionsData::default();
    request_save_options_data.file_name = Some((base_test_out_path.clone() + "/TestSaveAs.pdf").into());

    let request = SaveAsRequest::new(
        (remote_name.clone()).into(),
        (request_save_options_data).into()
    ).with_folder((remote_folder.clone()).into());

    let result = context.api().save_as(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SaveResult")?;
    assert_not_null(&result_json, "SaveResult.DestDocument")?;
    Ok(())
}

/// Test for converting document online to one of the available formats.
#[tokio::test]
async fn convert_document_save_as_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_name = "test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file("Common/".to_owned() + &local_name).await?;
    let mut request_save_options_data = PdfSaveOptionsData::default();
    request_save_options_data.file_name = Some((base_test_out_path.clone() + "/TestSaveAs.pdf").into());

    let request = SaveAsOnlineRequest::new(
        (request_document).into(),
        (request_save_options_data).into()
    );

    context.api().save_as_online(request).await?;
    Ok(())
}

/// Test for converting document online to html with additional files like css and images.
#[tokio::test]
async fn convert_document_save_as_online_html_multifile() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_name = "test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file("Common/".to_owned() + &local_name).await?;
    let mut request_save_options_data = HtmlSaveOptionsData::default();
    request_save_options_data.file_name = Some((base_test_out_path.clone() + "/TestSaveAsHtml.html").into());
    request_save_options_data.css_style_sheet_type = Some((HtmlSaveOptionsDataCssStyleSheetTypeEnum::External).into());
    request_save_options_data.css_style_sheet_file_name = Some((base_test_out_path.clone() + "/TestSaveAsHtml.css").into());

    let request = SaveAsOnlineRequest::new(
        (request_document).into(),
        (request_save_options_data).into()
    );

    context.api().save_as_online(request).await?;
    Ok(())
}

/// Test for converting document to one of the available formats.
#[tokio::test]
async fn convert_document_save_as_docx() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/ConvertDocument";
    let local_folder = "DocumentActions/ConvertDocument".to_owned();
    let local_name = "45.pdf".to_owned();
    let remote_name = "TestSaveAsFromPdfToDoc.pdf".to_owned();

    context.upload_file(local_folder.clone() + "/" + &local_name, remote_folder.clone() + "/" + &remote_name).await?;
    let mut request_save_options_data = DocxSaveOptionsData::default();
    request_save_options_data.file_name = Some((base_test_out_path.clone() + "/TestSaveAsFromPdfToDoc.docx").into());

    let request = SaveAsRequest::new(
        (remote_name.clone()).into(),
        (request_save_options_data).into()
    ).with_folder((remote_folder.clone()).into());

    let result = context.api().save_as(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SaveResult")?;
    assert_not_null(&result_json, "SaveResult.DestDocument")?;
    Ok(())
}

/// Test for converting document to one of the available formats.
#[tokio::test]
async fn convert_document_save_as_tiff() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/ConvertDocument";
    let local_name = "test_multi_pages.docx".to_owned();
    let remote_name = "TestSaveAsTiff.pdf".to_owned();

    context.upload_file("Common/".to_owned() + &local_name, remote_folder.clone() + "/" + &remote_name).await?;
    let mut request_save_options = TiffSaveOptionsData::default();
    request_save_options.file_name = Some((base_test_out_path.clone() + "/abc.tiff").into());

    let request = SaveAsTiffRequest::new(
        (remote_name.clone()).into(),
        (request_save_options).into()
    ).with_folder((remote_folder.clone()).into());

    let result = context.api().save_as_tiff(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SaveResult")?;
    assert_not_null(&result_json, "SaveResult.DestDocument")?;
    Ok(())
}

/// Test for converting document to one of the available formats.
#[tokio::test]
async fn convert_document_save_as_tiff_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_name = "test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file("Common/".to_owned() + &local_name).await?;
    let mut request_save_options = TiffSaveOptionsData::default();
    request_save_options.file_name = Some((base_test_out_path.clone() + "/abc.tiff").into());

    let request = SaveAsTiffOnlineRequest::new(
        (request_document).into(),
        (request_save_options).into()
    );

    context.api().save_as_tiff_online(request).await?;
    Ok(())
}

/// A test for ConvertDocument.
#[tokio::test]
async fn convert_document_convert_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_folder = "DocumentActions/ConvertDocument".to_owned();

    let request_document = context.load_binary_file(local_folder.clone() + "/test_uploadfile.docx").await?;

    let request = ConvertDocumentRequest::new(
        (request_document).into(),
        ("pdf".to_owned()).into()
    );

    context.api().convert_document(request).await?;
    Ok(())
}

/// A test for ConvertDocument as a job.
#[tokio::test]
async fn convert_document_convert_document_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_folder = "DocumentActions/ConvertDocument".to_owned();

    let request_document = context.load_binary_file(local_folder.clone() + "/test_uploadfile.docx").await?;

    let request = ConvertDocumentJobRequest::new(
        (request_document).into(),
        ("pdf".to_owned()).into()
    );

    let job_handler = context.api().convert_document_job(request).await?;
    job_handler.wait_result(Duration::from_secs(3)).await?;
    Ok(())
}