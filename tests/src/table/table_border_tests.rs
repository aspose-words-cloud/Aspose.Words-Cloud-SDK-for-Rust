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

/// Test for getting borders.
#[tokio::test]
async fn table_border_get_borders() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetBorders.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetBordersRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_borders(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Borders")?;
    assert_not_null(&result_json, "Borders.List")?;
    assert_length(&result_json, "Borders.List", 6)?;
    assert_not_null(&result_json, "Borders.List[0].Color")?;
    assert_string(&result_json, "Borders.List[0].Color.Web", "#000000".to_owned())?;
    Ok(())
}

/// Test for getting borders online.
#[tokio::test]
async fn table_border_get_borders_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetBordersOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into());

    context.api().get_borders_online(request).await?;
    Ok(())
}

/// Test for getting border.
#[tokio::test]
async fn table_border_get_border() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetBorder.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetBorderRequest::new(
        (remote_file_name.clone()).into(),
        ("left".to_owned()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_border(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Border")?;
    assert_not_null(&result_json, "Border.Color")?;
    assert_string(&result_json, "Border.Color.Web", "#000000".to_owned())?;
    Ok(())
}

/// Test for getting border online.
#[tokio::test]
async fn table_border_get_border_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetBorderOnlineRequest::new(
        (request_document).into(),
        ("left".to_owned()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into());

    context.api().get_border_online(request).await?;
    Ok(())
}

/// Test for deleting borders.
#[tokio::test]
async fn table_border_delete_borders() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestDeleteBorders.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteBordersRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_borders(request).await?;
    Ok(())
}

/// Test for deleting borders online.
#[tokio::test]
async fn table_border_delete_borders_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteBordersOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into());

    context.api().delete_borders_online(request).await?;
    Ok(())
}

/// Test for deleting border.
#[tokio::test]
async fn table_border_delete_border() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestDeleteBorder.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteBorderRequest::new(
        (remote_file_name.clone()).into(),
        ("left".to_owned()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_border(request).await?;
    Ok(())
}

/// Test for deleting border online.
#[tokio::test]
async fn table_border_delete_border_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteBorderOnlineRequest::new(
        (request_document).into(),
        ("left".to_owned()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into());

    context.api().delete_border_online(request).await?;
    Ok(())
}

/// Test for updating border.
#[tokio::test]
async fn table_border_update_border() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestUpdateBorder.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_border_properties_color = XmlColor::default();
    request_border_properties_color.web = Some(("#AABBCC".to_owned()).into());
    let mut request_border_properties = Border::default();
    request_border_properties.border_type = Some((BorderBorderTypeEnum::Left).into());
    request_border_properties.color = Some((request_border_properties_color).into());
    request_border_properties.distance_from_text = Some(((6.0) as f64).into());
    request_border_properties.line_style = Some((BorderLineStyleEnum::DashDotStroker).into());
    request_border_properties.line_width = Some(((2.0) as f64).into());
    request_border_properties.shadow = Some((true).into());

    let request = UpdateBorderRequest::new(
        (remote_file_name.clone()).into(),
        ("left".to_owned()).into(),
        (request_border_properties).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_border(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Border")?;
    assert_not_null(&result_json, "Border.Color")?;
    assert_string(&result_json, "Border.Color.Web", "#AABBCC".to_owned())?;
    assert_float(&result_json, "Border.DistanceFromText", (6.0) as f64)?;
    assert_float(&result_json, "Border.LineWidth", (2.0) as f64)?;
    assert_bool(&result_json, "Border.Shadow", true)?;
    Ok(())
}

/// Test for updating border online.
#[tokio::test]
async fn table_border_update_border_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_border_properties_color = XmlColor::default();
    request_border_properties_color.web = Some(("#AABBCC".to_owned()).into());
    let mut request_border_properties = Border::default();
    request_border_properties.border_type = Some((BorderBorderTypeEnum::Left).into());
    request_border_properties.color = Some((request_border_properties_color).into());
    request_border_properties.distance_from_text = Some(((6) as f64).into());
    request_border_properties.line_style = Some((BorderLineStyleEnum::DashDotStroker).into());
    request_border_properties.line_width = Some(((2) as f64).into());
    request_border_properties.shadow = Some((true).into());

    let request = UpdateBorderOnlineRequest::new(
        (request_document).into(),
        (request_border_properties).into(),
        ("left".to_owned()).into()
    ).with_node_path(("tables/1/rows/0/cells/0".to_owned()).into());

    context.api().update_border_online(request).await?;
    Ok(())
}