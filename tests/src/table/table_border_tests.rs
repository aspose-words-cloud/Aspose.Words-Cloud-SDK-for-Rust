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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetBorders.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetBordersRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_borders(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Borders")?;
    assert_not_null(&result_json, "Borders.List")?;
    assert_length(&result_json, "Borders.List", 6)?;
    assert_not_null(&result_json, "Borders.List[0].Color")?;
    assert_string(&result_json, "Borders.List[0].Color.Web", test_string!("#000000")?)?;
    Ok(())
}

/// Test for getting borders online.
#[tokio::test]
async fn table_border_get_borders_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetBordersOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into());

    context.api().get_borders_online(request).await?;
    Ok(())
}

/// Test for getting border.
#[tokio::test]
async fn table_border_get_border() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetBorder.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetBorderRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("left")?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_border(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Border")?;
    assert_not_null(&result_json, "Border.Color")?;
    assert_string(&result_json, "Border.Color.Web", test_string!("#000000")?)?;
    Ok(())
}

/// Test for getting border online.
#[tokio::test]
async fn table_border_get_border_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetBorderOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("left")?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into());

    context.api().get_border_online(request).await?;
    Ok(())
}

/// Test for deleting borders.
#[tokio::test]
async fn table_border_delete_borders() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestDeleteBorders.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteBordersRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_borders(request).await?;
    Ok(())
}

/// Test for deleting borders online.
#[tokio::test]
async fn table_border_delete_borders_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteBordersOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into());

    context.api().delete_borders_online(request).await?;
    Ok(())
}

/// Test for deleting border.
#[tokio::test]
async fn table_border_delete_border() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestDeleteBorder.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteBorderRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("left")?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_border(request).await?;
    Ok(())
}

/// Test for deleting border online.
#[tokio::test]
async fn table_border_delete_border_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteBorderOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("left")?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into());

    context.api().delete_border_online(request).await?;
    Ok(())
}

/// Test for updating border.
#[tokio::test]
async fn table_border_update_border() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestUpdateBorder.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestBorderPropertiesColor = XmlColor::default();
    requestBorderPropertiesColor.r#web = Some((test_string!("#AABBCC")?).into());
    let mut requestBorderProperties = Border::default();
    requestBorderProperties.r#border_type = Some((Border_BorderTypeEnum::Left).into());
    requestBorderProperties.r#color = Some((requestBorderPropertiesColor).into());
    requestBorderProperties.r#distance_from_text = Some(((6.0) as f64).into());
    requestBorderProperties.r#line_style = Some((Border_LineStyleEnum::DashDotStroker).into());
    requestBorderProperties.r#line_width = Some(((2.0) as f64).into());
    requestBorderProperties.r#shadow = Some((true).into());

    let request = UpdateBorderRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("left")?).into(),
        (requestBorderProperties).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_border(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Border")?;
    assert_not_null(&result_json, "Border.Color")?;
    assert_string(&result_json, "Border.Color.Web", test_string!("#AABBCC")?)?;
    assert_float(&result_json, "Border.DistanceFromText", (6.0) as f64)?;
    assert_float(&result_json, "Border.LineWidth", (2.0) as f64)?;
    assert_bool(&result_json, "Border.Shadow", true)?;
    Ok(())
}

/// Test for updating border online.
#[tokio::test]
async fn table_border_update_border_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestBorderPropertiesColor = XmlColor::default();
    requestBorderPropertiesColor.r#web = Some((test_string!("#AABBCC")?).into());
    let mut requestBorderProperties = Border::default();
    requestBorderProperties.r#border_type = Some((Border_BorderTypeEnum::Left).into());
    requestBorderProperties.r#color = Some((requestBorderPropertiesColor).into());
    requestBorderProperties.r#distance_from_text = Some(((6) as f64).into());
    requestBorderProperties.r#line_style = Some((Border_LineStyleEnum::DashDotStroker).into());
    requestBorderProperties.r#line_width = Some(((2) as f64).into());
    requestBorderProperties.r#shadow = Some((true).into());

    let request = UpdateBorderOnlineRequest::new(
        (requestDocument).into(),
        (requestBorderProperties).into(),
        (test_string!("left")?).into()
    ).with_node_path((test_string!("tables/1/rows/0/cells/0")?).into());

    context.api().update_border_online(request).await?;
    Ok(())
}