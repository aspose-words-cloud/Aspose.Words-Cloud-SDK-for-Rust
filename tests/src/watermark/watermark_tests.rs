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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Watermark";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertWatermarkText.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_watermark_data = WatermarkDataText::default();
    request_watermark_data.text = Some(("watermark text".to_owned()).into());

    let request = InsertWatermarkRequest::new(
        (remote_file_name.clone()).into(),
        (request_watermark_data).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().insert_watermark(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    Ok(())
}

/// Test for adding watermark text online.
#[tokio::test]
async fn watermark_insert_watermark_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_watermark_data = WatermarkDataText::default();
    request_watermark_data.text = Some(("watermark text".to_owned()).into());

    let request = InsertWatermarkOnlineRequest::new(
        (request_document).into(),
        (request_watermark_data).into()
    );

    context.api().insert_watermark_online(request).await?;
    Ok(())
}

/// Test for adding watermark text.
#[tokio::test]
async fn watermark_insert_watermark_image() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Watermark";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertWatermarkImage.docx".to_owned();
    let remote_image_path = remote_data_folder.clone() + "/TestInsertWatermarkImage.png";

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    context.upload_file("Common/aspose-cloud.png".to_owned(), remote_image_path.clone()).await?;
    let request_watermark_data_image = FileReference::remote(
    remote_image_path.clone(),
    None,
    );
    let mut request_watermark_data = WatermarkDataImage::default();
    request_watermark_data.image = Some((request_watermark_data_image).into());

    let request = InsertWatermarkRequest::new(
        (remote_file_name.clone()).into(),
        (request_watermark_data).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().insert_watermark(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    Ok(())
}

/// Test for adding watermark text online.
#[tokio::test]
async fn watermark_insert_watermark_image_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let request_watermark_data_image_content = context.load_binary_file("Common/aspose-cloud.png".to_owned()).await?;
    let request_watermark_data_image = FileReference::local(
    request_watermark_data_image_content,
    None,
    );
    let mut request_watermark_data = WatermarkDataImage::default();
    request_watermark_data.image = Some((request_watermark_data_image).into());

    let request = InsertWatermarkOnlineRequest::new(
        (request_document).into(),
        (request_watermark_data).into()
    );

    context.api().insert_watermark_online(request).await?;
    Ok(())
}

/// Test for adding watermark image.
#[tokio::test]
async fn watermark_insert_watermark_image_deprecated() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Watermark";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertWatermarkImage.docx".to_owned();
    let remote_image_path = remote_data_folder.clone() + "/TestInsertWatermarkImage.png";

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    context.upload_file("Common/aspose-cloud.png".to_owned(), remote_image_path.clone()).await?;

    let request = InsertWatermarkImageRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into())
.with_image((remote_image_path.clone()).into());

    let result = context.api().insert_watermark_image(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestInsertWatermarkImage.docx".to_owned())?;
    Ok(())
}

/// Test for adding watermark image online.
#[tokio::test]
async fn watermark_insert_watermark_image_deprecated_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let request_image_file = context.load_binary_file("Common/aspose-cloud.png".to_owned()).await?;

    let request = InsertWatermarkImageOnlineRequest::new(
        (request_document).into()
    ).with_image_file((request_image_file).into());

    context.api().insert_watermark_image_online(request).await?;
    Ok(())
}

/// Test for adding watermark text.
#[tokio::test]
async fn watermark_insert_watermark_text_deprecated() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Watermark";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertWatermarkText.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_watermark_text = WatermarkText::default();
    request_watermark_text.text = Some(("This is the text".to_owned()).into());
    request_watermark_text.rotation_angle = Some(((90.0) as f64).into());

    let request = InsertWatermarkTextRequest::new(
        (remote_file_name.clone()).into(),
        (request_watermark_text).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().insert_watermark_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestInsertWatermarkText.docx".to_owned())?;
    Ok(())
}

/// Test for adding watermark text online.
#[tokio::test]
async fn watermark_insert_watermark_text_deprecated_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_watermark_text = WatermarkText::default();
    request_watermark_text.text = Some(("This is the text".to_owned()).into());
    request_watermark_text.rotation_angle = Some(((90) as f64).into());

    let request = InsertWatermarkTextOnlineRequest::new(
        (request_document).into(),
        (request_watermark_text).into()
    );

    context.api().insert_watermark_text_online(request).await?;
    Ok(())
}

/// Test for deleting watermark.
#[tokio::test]
async fn watermark_delete_watermark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Watermark";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteWatermark.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteWatermarkRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().delete_watermark(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestDeleteWatermark.docx".to_owned())?;
    Ok(())
}

/// Test for deleting watermark online.
#[tokio::test]
async fn watermark_delete_watermark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteWatermarkOnlineRequest::new(
        (request_document).into()
    );

    context.api().delete_watermark_online(request).await?;
    Ok(())
}