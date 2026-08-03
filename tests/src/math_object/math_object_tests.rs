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

/// Test for getting mathObjects.
#[tokio::test]
async fn math_object_get_office_math_objects() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestGetOfficeMathObjects.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetOfficeMathObjectsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_office_math_objects(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "OfficeMathObjects")?;
    assert_not_null(&result_json, "OfficeMathObjects.List")?;
    assert_length(&result_json, "OfficeMathObjects.List", 16)?;
    assert_string(&result_json, "OfficeMathObjects.List[0].NodeId", "0.0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting mathObjects online.
#[tokio::test]
async fn math_object_get_office_math_objects_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetOfficeMathObjectsOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_office_math_objects_online(request).await?;
    Ok(())
}

/// Test for getting mathObjects without node path.
#[tokio::test]
async fn math_object_get_office_math_objects_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestGetOfficeMathObjectsWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetOfficeMathObjectsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_office_math_objects(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "OfficeMathObjects")?;
    assert_not_null(&result_json, "OfficeMathObjects.List")?;
    assert_length(&result_json, "OfficeMathObjects.List", 16)?;
    assert_string(&result_json, "OfficeMathObjects.List[0].NodeId", "0.0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting mathObject.
#[tokio::test]
async fn math_object_get_office_math_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestGetOfficeMathObject.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetOfficeMathObjectRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_office_math_object(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "OfficeMathObject")?;
    assert_string(&result_json, "OfficeMathObject.NodeId", "0.0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting mathObject online.
#[tokio::test]
async fn math_object_get_office_math_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetOfficeMathObjectOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().get_office_math_object_online(request).await?;
    Ok(())
}

/// Test for getting mathObject without node path.
#[tokio::test]
async fn math_object_get_office_math_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestGetOfficeMathObjectWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetOfficeMathObjectRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_office_math_object(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "OfficeMathObject")?;
    assert_string(&result_json, "OfficeMathObject.NodeId", "0.0.0.0".to_owned())?;
    Ok(())
}

/// Test for rendering mathObject.
#[tokio::test]
async fn math_object_render_math_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestRenderMathObject.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = RenderMathObjectRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().render_math_object(request).await?;
    Ok(())
}

/// Test for rendering mathObject.
#[tokio::test]
async fn math_object_render_math_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = RenderMathObjectOnlineRequest::new(
        (request_document).into(),
        ("png".to_owned()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().render_math_object_online(request).await?;
    Ok(())
}

/// Test for rendering mathObject without node path.
#[tokio::test]
async fn math_object_render_math_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestRenderMathObjectWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = RenderMathObjectRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().render_math_object(request).await?;
    Ok(())
}

/// Test for deleting mathObject.
#[tokio::test]
async fn math_object_delete_office_math_object() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestDeleteOfficeMathObject.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteOfficeMathObjectRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_office_math_object(request).await?;
    Ok(())
}

/// Test for deleting mathObject online.
#[tokio::test]
async fn math_object_delete_office_math_object_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteOfficeMathObjectOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_office_math_object_online(request).await?;
    Ok(())
}

/// Test for deleting mathObject without node path.
#[tokio::test]
async fn math_object_delete_office_math_object_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestDeleteOfficeMathObjectWithoutNodePath.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteOfficeMathObjectRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_office_math_object(request).await?;
    Ok(())
}

/// Test for deleting math objects.
#[tokio::test]
async fn math_object_delete_office_math_objects() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/MathObjects";
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();
    let remote_file_name = "TestDeleteOfficeMathObject.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteOfficeMathObjectsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_office_math_objects(request).await?;
    Ok(())
}

/// Test for deleting math objects online.
#[tokio::test]
async fn math_object_delete_office_math_objects_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/MathObjects/MathObjects.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteOfficeMathObjectsOnlineRequest::new(
        (request_document).into()
    );

    context.api().delete_office_math_objects_online(request).await?;
    Ok(())
}