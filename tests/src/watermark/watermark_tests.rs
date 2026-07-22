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

/// Test for adding watermark text.
#[tokio::test]
async fn watermark_insert_watermark_text() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Watermark")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertWatermarkText.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestWatermarkData = WatermarkDataText::default();
    requestWatermarkData.r#text = Some((test_string!("watermark text")?).into());

    let request = InsertWatermarkRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestWatermarkData).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().insert_watermark(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    Ok(())
}

/// Test for adding watermark text online.
#[tokio::test]
async fn watermark_insert_watermark_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestWatermarkData = WatermarkDataText::default();
    requestWatermarkData.r#text = Some((test_string!("watermark text")?).into());

    let request = InsertWatermarkOnlineRequest::new(
        (requestDocument).into(),
        (requestWatermarkData).into()
    );

    context.api().insert_watermark_online(request).await?;
    Ok(())
}

/// Test for adding watermark text.
#[tokio::test]
async fn watermark_insert_watermark_image() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Watermark")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertWatermarkImage.docx")?;
    let remoteImagePath = test_string!(remoteDataFolder + "/TestInsertWatermarkImage.png")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    context.upload_file(test_string!("Common/aspose-cloud.png")?, test_string!(remoteImagePath)?).await?;
    let requestWatermarkDataImage = FileReference::remote(
    test_string!(remoteImagePath)?,
    None,
    );
    let mut requestWatermarkData = WatermarkDataImage::default();
    requestWatermarkData.r#image = Some((requestWatermarkDataImage).into());

    let request = InsertWatermarkRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestWatermarkData).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().insert_watermark(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    Ok(())
}

/// Test for adding watermark text online.
#[tokio::test]
async fn watermark_insert_watermark_image_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let requestWatermarkDataImageContent = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;
    let requestWatermarkDataImage = FileReference::local(
    requestWatermarkDataImageContent,
    None,
    );
    let mut requestWatermarkData = WatermarkDataImage::default();
    requestWatermarkData.r#image = Some((requestWatermarkDataImage).into());

    let request = InsertWatermarkOnlineRequest::new(
        (requestDocument).into(),
        (requestWatermarkData).into()
    );

    context.api().insert_watermark_online(request).await?;
    Ok(())
}

/// Test for adding watermark image.
#[tokio::test]
async fn watermark_insert_watermark_image_deprecated() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Watermark")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertWatermarkImage.docx")?;
    let remoteImagePath = test_string!(remoteDataFolder + "/TestInsertWatermarkImage.png")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    context.upload_file(test_string!("Common/aspose-cloud.png")?, test_string!(remoteImagePath)?).await?;

    let request = InsertWatermarkImageRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into())
.with_image((test_string!(remoteImagePath)?).into());

    let result = context.api().insert_watermark_image(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestInsertWatermarkImage.docx")?)?;
    Ok(())
}

/// Test for adding watermark image online.
#[tokio::test]
async fn watermark_insert_watermark_image_deprecated_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = InsertWatermarkImageOnlineRequest::new(
        (requestDocument).into()
    ).with_image_file((requestImageFile).into());

    context.api().insert_watermark_image_online(request).await?;
    Ok(())
}

/// Test for adding watermark text.
#[tokio::test]
async fn watermark_insert_watermark_text_deprecated() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Watermark")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertWatermarkText.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestWatermarkText = WatermarkText::default();
    requestWatermarkText.r#text = Some((test_string!("This is the text")?).into());
    requestWatermarkText.r#rotation_angle = Some(((90.0) as f64).into());

    let request = InsertWatermarkTextRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestWatermarkText).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().insert_watermark_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestInsertWatermarkText.docx")?)?;
    Ok(())
}

/// Test for adding watermark text online.
#[tokio::test]
async fn watermark_insert_watermark_text_deprecated_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestWatermarkText = WatermarkText::default();
    requestWatermarkText.r#text = Some((test_string!("This is the text")?).into());
    requestWatermarkText.r#rotation_angle = Some(((90) as f64).into());

    let request = InsertWatermarkTextOnlineRequest::new(
        (requestDocument).into(),
        (requestWatermarkText).into()
    );

    context.api().insert_watermark_text_online(request).await?;
    Ok(())
}

/// Test for deleting watermark.
#[tokio::test]
async fn watermark_delete_watermark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Watermark")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteWatermark.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteWatermarkRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().delete_watermark(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestDeleteWatermark.docx")?)?;
    Ok(())
}

/// Test for deleting watermark online.
#[tokio::test]
async fn watermark_delete_watermark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteWatermarkOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().delete_watermark_online(request).await?;
    Ok(())
}