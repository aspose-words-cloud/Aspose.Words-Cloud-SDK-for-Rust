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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/ConvertDocument")?;
    let localName = test_string!("test_multi_pages.docx")?;
    let remoteName = test_string!("TestSaveAs.docx")?;

    context.upload_file(test_string!("Common/" + localName)?, test_string!(remoteFolder + "/" + remoteName)?).await?;
    let mut requestSaveOptionsData = PdfSaveOptionsData::default();
    requestSaveOptionsData.r#file_name = Some((test_string!(baseTestOutPath + "/TestSaveAs.pdf")?).into());

    let request = SaveAsRequest::new(
        (test_string!(remoteName)?).into(),
        (requestSaveOptionsData).into()
    ).with_folder((test_string!(remoteFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localName = test_string!("test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!("Common/" + localName)?).await?;
    let mut requestSaveOptionsData = PdfSaveOptionsData::default();
    requestSaveOptionsData.r#file_name = Some((test_string!(baseTestOutPath + "/TestSaveAs.pdf")?).into());

    let request = SaveAsOnlineRequest::new(
        (requestDocument).into(),
        (requestSaveOptionsData).into()
    );

    context.api().save_as_online(request).await?;
    Ok(())
}

/// Test for converting document online to html with additional files like css and images.
#[tokio::test]
async fn convert_document_save_as_online_html_multifile() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localName = test_string!("test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!("Common/" + localName)?).await?;
    let mut requestSaveOptionsData = HtmlSaveOptionsData::default();
    requestSaveOptionsData.r#file_name = Some((test_string!(baseTestOutPath + "/TestSaveAsHtml.html")?).into());
    requestSaveOptionsData.r#css_style_sheet_type = Some((HtmlSaveOptionsData_CssStyleSheetTypeEnum::External).into());
    requestSaveOptionsData.r#css_style_sheet_file_name = Some((test_string!(baseTestOutPath + "/TestSaveAsHtml.css")?).into());

    let request = SaveAsOnlineRequest::new(
        (requestDocument).into(),
        (requestSaveOptionsData).into()
    );

    context.api().save_as_online(request).await?;
    Ok(())
}

/// Test for converting document to one of the available formats.
#[tokio::test]
async fn convert_document_save_as_docx() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/ConvertDocument")?;
    let localFolder = test_string!("DocumentActions/ConvertDocument")?;
    let localName = test_string!("45.pdf")?;
    let remoteName = test_string!("TestSaveAsFromPdfToDoc.pdf")?;

    context.upload_file(test_string!(localFolder + "/" + localName)?, test_string!(remoteFolder + "/" + remoteName)?).await?;
    let mut requestSaveOptionsData = DocxSaveOptionsData::default();
    requestSaveOptionsData.r#file_name = Some((test_string!(baseTestOutPath + "/TestSaveAsFromPdfToDoc.docx")?).into());

    let request = SaveAsRequest::new(
        (test_string!(remoteName)?).into(),
        (requestSaveOptionsData).into()
    ).with_folder((test_string!(remoteFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/ConvertDocument")?;
    let localName = test_string!("test_multi_pages.docx")?;
    let remoteName = test_string!("TestSaveAsTiff.pdf")?;

    context.upload_file(test_string!("Common/" + localName)?, test_string!(remoteFolder + "/" + remoteName)?).await?;
    let mut requestSaveOptions = TiffSaveOptionsData::default();
    requestSaveOptions.r#file_name = Some((test_string!(baseTestOutPath + "/abc.tiff")?).into());

    let request = SaveAsTiffRequest::new(
        (test_string!(remoteName)?).into(),
        (requestSaveOptions).into()
    ).with_folder((test_string!(remoteFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localName = test_string!("test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!("Common/" + localName)?).await?;
    let mut requestSaveOptions = TiffSaveOptionsData::default();
    requestSaveOptions.r#file_name = Some((test_string!(baseTestOutPath + "/abc.tiff")?).into());

    let request = SaveAsTiffOnlineRequest::new(
        (requestDocument).into(),
        (requestSaveOptions).into()
    );

    context.api().save_as_tiff_online(request).await?;
    Ok(())
}

/// A test for ConvertDocument.
#[tokio::test]
async fn convert_document_convert_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFolder = test_string!("DocumentActions/ConvertDocument")?;

    let requestDocument = context.load_binary_file(test_string!(localFolder + "/test_uploadfile.docx")?).await?;

    let request = ConvertDocumentRequest::new(
        (requestDocument).into(),
        (test_string!("pdf")?).into()
    );

    context.api().convert_document(request).await?;
    Ok(())
}

/// A test for ConvertDocument as a job.
#[tokio::test]
async fn convert_document_convert_document_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFolder = test_string!("DocumentActions/ConvertDocument")?;

    let requestDocument = context.load_binary_file(test_string!(localFolder + "/test_uploadfile.docx")?).await?;

    let request = ConvertDocumentJobRequest::new(
        (requestDocument).into(),
        (test_string!("pdf")?).into()
    );

    let job_handler = context.api().convert_document_job(request).await?;
    job_handler.wait_result(Duration::from_secs(3)).await?;
    Ok(())
}