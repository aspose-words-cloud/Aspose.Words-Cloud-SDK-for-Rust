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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjects.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentDrawingObjectsRequest::new((remote_file_name.clone()).into())
        .with_node_path(("sections/0".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().get_document_drawing_objects(request).await?;
    Ok(())
}

/// Test for getting drawing objects from document online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_objects_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetDocumentDrawingObjectsOnlineRequest::new((request_document).into())
        .with_node_path(("sections/0".to_owned()).into());

    context
        .api()
        .get_document_drawing_objects_online(request)
        .await?;
    Ok(())
}

/// Test for getting drawing objects from document without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_objects_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectsWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentDrawingObjectsRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().get_document_drawing_objects(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_by_index() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectByIndex.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        GetDocumentDrawingObjectByIndexRequest::new((remote_file_name.clone()).into(), (0).into())
            .with_node_path(("sections/0".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .get_document_drawing_object_by_index(request)
        .await?;
    Ok(())
}

/// Test for getting drawing object by specified index online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_by_index_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        GetDocumentDrawingObjectByIndexOnlineRequest::new((request_document).into(), (0).into())
            .with_node_path(("sections/0".to_owned()).into());

    context
        .api()
        .get_document_drawing_object_by_index_online(request)
        .await?;
    Ok(())
}

/// Test for getting drawing object by specified index without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_by_index_without_node_path() -> TestResult<()>
{
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectByIndexWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        GetDocumentDrawingObjectByIndexRequest::new((remote_file_name.clone()).into(), (0).into())
            .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .get_document_drawing_object_by_index(request)
        .await?;
    Ok(())
}

/// Test for getting drawing object by specified index and format.
#[tokio::test]
async fn drawing_objects_render_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectByIndexWithFormat.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = RenderDrawingObjectRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into(),
    )
    .with_node_path(("sections/0".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context.api().render_drawing_object(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index and format online.
#[tokio::test]
async fn drawing_objects_render_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = RenderDrawingObjectOnlineRequest::new(
        (request_document).into(),
        ("png".to_owned()).into(),
        (0).into(),
    )
    .with_node_path(("sections/0".to_owned()).into());

    context.api().render_drawing_object_online(request).await?;
    Ok(())
}

/// Test for getting drawing object by specified index and format without node path.
#[tokio::test]
async fn drawing_objects_render_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name =
        "TestGetDocumentDrawingObjectByIndexWithFormatWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = RenderDrawingObjectRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().render_drawing_object(request).await?;
    Ok(())
}

/// Test for reading drawing object's image data.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_image_data() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectImageData.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentDrawingObjectImageDataRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
    )
    .with_node_path(("sections/0".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .get_document_drawing_object_image_data(request)
        .await?;
    Ok(())
}

/// Test for reading drawing object's image data online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_image_data_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        GetDocumentDrawingObjectImageDataOnlineRequest::new((request_document).into(), (0).into())
            .with_node_path(("sections/0".to_owned()).into());

    context
        .api()
        .get_document_drawing_object_image_data_online(request)
        .await?;
    Ok(())
}

/// Test for reading drawing object's image data without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_image_data_without_node_path() -> TestResult<()>
{
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectImageDataWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentDrawingObjectImageDataRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .get_document_drawing_object_image_data(request)
        .await?;
    Ok(())
}

/// Test for getting drawing object OLE data.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_ole_data() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_drawing_file = "DocumentElements/DrawingObjects/sample_EmbeddedOLE.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectOleData.docx".to_owned();

    context
        .upload_file(
            local_drawing_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        GetDocumentDrawingObjectOleDataRequest::new((remote_file_name.clone()).into(), (0).into())
            .with_node_path(("sections/0".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .get_document_drawing_object_ole_data(request)
        .await?;
    Ok(())
}

/// Test for getting drawing object OLE data online.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_ole_data_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_drawing_file = "DocumentElements/DrawingObjects/sample_EmbeddedOLE.docx".to_owned();

    let request_document = context.load_binary_file(local_drawing_file.clone()).await?;

    let request =
        GetDocumentDrawingObjectOleDataOnlineRequest::new((request_document).into(), (0).into())
            .with_node_path(("sections/0".to_owned()).into());

    context
        .api()
        .get_document_drawing_object_ole_data_online(request)
        .await?;
    Ok(())
}

/// Test for getting drawing object OLE data without node path.
#[tokio::test]
async fn drawing_objects_get_document_drawing_object_ole_data_without_node_path() -> TestResult<()>
{
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_drawing_file = "DocumentElements/DrawingObjects/sample_EmbeddedOLE.docx".to_owned();
    let remote_file_name = "TestGetDocumentDrawingObjectOleDataWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_drawing_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request =
        GetDocumentDrawingObjectOleDataRequest::new((remote_file_name.clone()).into(), (0).into())
            .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .get_document_drawing_object_ole_data(request)
        .await?;
    Ok(())
}

/// Test for adding drawing object.
#[tokio::test]
async fn drawing_objects_insert_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsetDrawingObject.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_drawing_object = DrawingObjectInsert::default();
    request_drawing_object.height = Some(((0) as f64).into());
    request_drawing_object.left = Some(((0) as f64).into());
    request_drawing_object.top = Some(((0) as f64).into());
    request_drawing_object.width = Some(((0) as f64).into());
    request_drawing_object.relative_horizontal_position =
        Some((DrawingObjectInsertRelativeHorizontalPositionEnum::Margin).into());
    request_drawing_object.relative_vertical_position =
        Some((DrawingObjectInsertRelativeVerticalPositionEnum::Margin).into());
    request_drawing_object.wrap_type = Some((DrawingObjectInsertWrapTypeEnum::Inline).into());
    let request_image_file = context
        .load_binary_file("Common/aspose-cloud.png".to_owned())
        .await?;

    let request = InsertDrawingObjectRequest::new(
        (remote_file_name.clone()).into(),
        (request_drawing_object).into(),
        (request_image_file).into(),
    )
    .with_node_path(("".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context.api().insert_drawing_object(request).await?;
    Ok(())
}

/// Test for adding drawing object online.
#[tokio::test]
async fn drawing_objects_insert_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_drawing_object = DrawingObjectInsert::default();
    request_drawing_object.height = Some(((0) as f64).into());
    request_drawing_object.left = Some(((0) as f64).into());
    request_drawing_object.top = Some(((0) as f64).into());
    request_drawing_object.width = Some(((0) as f64).into());
    request_drawing_object.relative_horizontal_position =
        Some((DrawingObjectInsertRelativeHorizontalPositionEnum::Margin).into());
    request_drawing_object.relative_vertical_position =
        Some((DrawingObjectInsertRelativeVerticalPositionEnum::Margin).into());
    request_drawing_object.wrap_type = Some((DrawingObjectInsertWrapTypeEnum::Inline).into());
    let request_image_file = context
        .load_binary_file("Common/aspose-cloud.png".to_owned())
        .await?;

    let request = InsertDrawingObjectOnlineRequest::new(
        (request_document).into(),
        (request_drawing_object).into(),
        (request_image_file).into(),
    )
    .with_node_path(("".to_owned()).into());

    context.api().insert_drawing_object_online(request).await?;
    Ok(())
}

/// Test for adding drawing object without node path.
#[tokio::test]
async fn drawing_objects_insert_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsetDrawingObjectWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_drawing_object = DrawingObjectInsert::default();
    request_drawing_object.height = Some(((0) as f64).into());
    request_drawing_object.left = Some(((0) as f64).into());
    request_drawing_object.top = Some(((0) as f64).into());
    request_drawing_object.width = Some(((0) as f64).into());
    request_drawing_object.relative_horizontal_position =
        Some((DrawingObjectInsertRelativeHorizontalPositionEnum::Margin).into());
    request_drawing_object.relative_vertical_position =
        Some((DrawingObjectInsertRelativeVerticalPositionEnum::Margin).into());
    request_drawing_object.wrap_type = Some((DrawingObjectInsertWrapTypeEnum::Inline).into());
    let request_image_file = context
        .load_binary_file("Common/aspose-cloud.png".to_owned())
        .await?;

    let request = InsertDrawingObjectRequest::new(
        (remote_file_name.clone()).into(),
        (request_drawing_object).into(),
        (request_image_file).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().insert_drawing_object(request).await?;
    Ok(())
}

/// Test for deleting drawing object.
#[tokio::test]
async fn drawing_objects_delete_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteDrawingObject.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteDrawingObjectRequest::new((remote_file_name.clone()).into(), (0).into())
        .with_node_path(("".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().delete_drawing_object(request).await?;
    Ok(())
}

/// Test for deleting drawing object online.
#[tokio::test]
async fn drawing_objects_delete_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteDrawingObjectOnlineRequest::new((request_document).into(), (0).into())
        .with_node_path(("".to_owned()).into());

    context.api().delete_drawing_object_online(request).await?;
    Ok(())
}

/// Test for deleting drawing object without node path.
#[tokio::test]
async fn drawing_objects_delete_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteDrawingObjectWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteDrawingObjectRequest::new((remote_file_name.clone()).into(), (0).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().delete_drawing_object(request).await?;
    Ok(())
}

/// Test for updating drawing object.
#[tokio::test]
async fn drawing_objects_update_drawing_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateDrawingObject.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_drawing_object = DrawingObjectUpdate::default();
    request_drawing_object.left = Some(((0) as f64).into());
    let request_image_file = context
        .load_binary_file("Common/aspose-cloud.png".to_owned())
        .await?;

    let request = UpdateDrawingObjectRequest::new(
        (remote_file_name.clone()).into(),
        (request_drawing_object).into(),
        (request_image_file).into(),
        (0).into(),
    )
    .with_node_path(("".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context.api().update_drawing_object(request).await?;
    Ok(())
}

/// Test for updating drawing object online.
#[tokio::test]
async fn drawing_objects_update_drawing_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_drawing_object = DrawingObjectUpdate::default();
    request_drawing_object.left = Some(((0) as f64).into());
    let request_image_file = context
        .load_binary_file("Common/aspose-cloud.png".to_owned())
        .await?;

    let request = UpdateDrawingObjectOnlineRequest::new(
        (request_document).into(),
        (request_drawing_object).into(),
        (request_image_file).into(),
        (0).into(),
    )
    .with_node_path(("".to_owned()).into());

    context.api().update_drawing_object_online(request).await?;
    Ok(())
}

/// Test for updating drawing object without node path.
#[tokio::test]
async fn drawing_objects_update_drawing_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DrawingObjectss";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateDrawingObjectWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_drawing_object = DrawingObjectUpdate::default();
    request_drawing_object.left = Some(((0) as f64).into());
    let request_image_file = context
        .load_binary_file("Common/aspose-cloud.png".to_owned())
        .await?;

    let request = UpdateDrawingObjectRequest::new(
        (remote_file_name.clone()).into(),
        (request_drawing_object).into(),
        (request_image_file).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().update_drawing_object(request).await?;
    Ok(())
}
