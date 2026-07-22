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

/// Test for getting drawing objects from document.
#[tokio::test]
async fn drawing_objects_get_document_drawing_objects() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjects.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_objects(request).await?;
    Ok(())
}

/// Test for getting drawing objects from document online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_objects_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetDocumentDrawingObjectsOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_document_drawing_objects_online(request).await?;
    Ok(())
}

/// Test for getting drawing objects from document without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_objects_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectsWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_objects(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_by_index() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectByIndex.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectByIndexRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_object_by_index(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_by_index_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetDocumentDrawingObjectByIndexOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_document_drawing_object_by_index_online(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_by_index_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectByIndexWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectByIndexRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_object_by_index(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index and format.
#[tokio::test]
async fn drawing_objects_render_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectByIndexWithFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_drawing_object(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index and format online.
#[tokio::test]
async fn drawing_objects_render_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = RenderDrawingObjectOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().render_drawing_object_online(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index and format without node path.
#[tokio::test]
async fn drawing_objects_render_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectByIndexWithFormatWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_drawing_object(request).await?;
    Ok(())
}

/// Test for reading drawing object's image data.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_image_data() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectImageData.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectImageDataRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_object_image_data(request).await?;
    Ok(())
}

/// Test for reading drawing object's image data online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_image_data_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetDocumentDrawingObjectImageDataOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_document_drawing_object_image_data_online(request).await?;
    Ok(())
}

/// Test for reading drawing object's image data without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_image_data_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectImageDataWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectImageDataRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_object_image_data(request).await?;
    Ok(())
}

/// Test for getting drawing object OLE data.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_ole_data() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localDrawingFile = test_string!("DocumentElements/DrawingObjects/sample_EmbeddedOLE.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectOleData.docx")?;

    context.upload_file(test_string!(localDrawingFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectOleDataRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_object_ole_data(request).await?;
    Ok(())
}

/// Test for getting drawing object OLE data online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_ole_data_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localDrawingFile = test_string!("DocumentElements/DrawingObjects/sample_EmbeddedOLE.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localDrawingFile)?).await?;

    let request = GetDocumentDrawingObjectOleDataOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_document_drawing_object_ole_data_online(request).await?;
    Ok(())
}

/// Test for getting drawing object OLE data without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_ole_data_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localDrawingFile = test_string!("DocumentElements/DrawingObjects/sample_EmbeddedOLE.docx")?;
    let remoteFileName = test_string!("TestGetDocumentDrawingObjectOleDataWithoutNodePath.docx")?;

    context.upload_file(test_string!(localDrawingFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentDrawingObjectOleDataRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_drawing_object_ole_data(request).await?;
    Ok(())
}

/// Test for adding drawing object.
#[tokio::test]
async fn drawing_objects_insert_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsetDrawingObject.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestDrawingObject = DrawingObjectInsert::default();
    requestDrawingObject.r#height = Some(((0) as f64).into());
    requestDrawingObject.r#left = Some(((0) as f64).into());
    requestDrawingObject.r#top = Some(((0) as f64).into());
    requestDrawingObject.r#width = Some(((0) as f64).into());
    requestDrawingObject.r#relative_horizontal_position = Some((DrawingObjectInsert_RelativeHorizontalPositionEnum::Margin).into());
    requestDrawingObject.r#relative_vertical_position = Some((DrawingObjectInsert_RelativeVerticalPositionEnum::Margin).into());
    requestDrawingObject.r#wrap_type = Some((DrawingObjectInsert_WrapTypeEnum::Inline).into());
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = InsertDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestDrawingObject).into(),
        (requestImageFile).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().insert_drawing_object(request).await?;
    Ok(())
}

/// Test for adding drawing object online.
#[tokio::test]
async fn drawing_objects_insert_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestDrawingObject = DrawingObjectInsert::default();
    requestDrawingObject.r#height = Some(((0) as f64).into());
    requestDrawingObject.r#left = Some(((0) as f64).into());
    requestDrawingObject.r#top = Some(((0) as f64).into());
    requestDrawingObject.r#width = Some(((0) as f64).into());
    requestDrawingObject.r#relative_horizontal_position = Some((DrawingObjectInsert_RelativeHorizontalPositionEnum::Margin).into());
    requestDrawingObject.r#relative_vertical_position = Some((DrawingObjectInsert_RelativeVerticalPositionEnum::Margin).into());
    requestDrawingObject.r#wrap_type = Some((DrawingObjectInsert_WrapTypeEnum::Inline).into());
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = InsertDrawingObjectOnlineRequest::new(
        (requestDocument).into(),
        (requestDrawingObject).into(),
        (requestImageFile).into()
    ).with_node_path((test_string!("")?).into());

    context.api().insert_drawing_object_online(request).await?;
    Ok(())
}

/// Test for adding drawing object without node path.
#[tokio::test]
async fn drawing_objects_insert_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsetDrawingObjectWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestDrawingObject = DrawingObjectInsert::default();
    requestDrawingObject.r#height = Some(((0) as f64).into());
    requestDrawingObject.r#left = Some(((0) as f64).into());
    requestDrawingObject.r#top = Some(((0) as f64).into());
    requestDrawingObject.r#width = Some(((0) as f64).into());
    requestDrawingObject.r#relative_horizontal_position = Some((DrawingObjectInsert_RelativeHorizontalPositionEnum::Margin).into());
    requestDrawingObject.r#relative_vertical_position = Some((DrawingObjectInsert_RelativeVerticalPositionEnum::Margin).into());
    requestDrawingObject.r#wrap_type = Some((DrawingObjectInsert_WrapTypeEnum::Inline).into());
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = InsertDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestDrawingObject).into(),
        (requestImageFile).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().insert_drawing_object(request).await?;
    Ok(())
}

/// Test for deleting drawing object.
#[tokio::test]
async fn drawing_objects_delete_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteDrawingObject.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_drawing_object(request).await?;
    Ok(())
}

/// Test for deleting drawing object online.
#[tokio::test]
async fn drawing_objects_delete_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteDrawingObjectOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_drawing_object_online(request).await?;
    Ok(())
}

/// Test for deleting drawing object without node path.
#[tokio::test]
async fn drawing_objects_delete_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteDrawingObjectWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_drawing_object(request).await?;
    Ok(())
}

/// Test for updating drawing object.
#[tokio::test]
async fn drawing_objects_update_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateDrawingObject.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestDrawingObject = DrawingObjectUpdate::default();
    requestDrawingObject.r#left = Some(((0) as f64).into());
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = UpdateDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestDrawingObject).into(),
        (requestImageFile).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().update_drawing_object(request).await?;
    Ok(())
}

/// Test for updating drawing object online.
#[tokio::test]
async fn drawing_objects_update_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestDrawingObject = DrawingObjectUpdate::default();
    requestDrawingObject.r#left = Some(((0) as f64).into());
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = UpdateDrawingObjectOnlineRequest::new(
        (requestDocument).into(),
        (requestDrawingObject).into(),
        (requestImageFile).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().update_drawing_object_online(request).await?;
    Ok(())
}

/// Test for updating drawing object without node path.
#[tokio::test]
async fn drawing_objects_update_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DrawingObjectss")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateDrawingObjectWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestDrawingObject = DrawingObjectUpdate::default();
    requestDrawingObject.r#left = Some(((0) as f64).into());
    let requestImageFile = context.load_binary_file(test_string!("Common/aspose-cloud.png")?).await?;

    let request = UpdateDrawingObjectRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestDrawingObject).into(),
        (requestImageFile).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().update_drawing_object(request).await?;
    Ok(())
}