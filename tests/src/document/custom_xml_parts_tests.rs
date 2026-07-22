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

/// Test for getting custom xml part by specified index.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/CustomXmlParts")?;
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;
    let remoteFileName = test_string!("TestGetCustomXmlPart.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetCustomXmlPartRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_custom_xml_part(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", test_string!("aspose")?)?;
    assert_string(&result_json, "CustomXmlPart.Data", test_string!("<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>")?)?;
    Ok(())
}

/// Test for getting custom xml part by specified index online.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetCustomXmlPartOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    );

    let result = context.api().get_custom_xml_part_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", test_string!("aspose")?)?;
    assert_string(&result_json, "CustomXmlPart.Data", test_string!("<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>")?)?;
    Ok(())
}

/// Test for getting all custom xml parts from document.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_parts() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/CustomXmlParts")?;
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;
    let remoteFileName = test_string!("TestGetCustomXmlParts.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetCustomXmlPartsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_custom_xml_parts(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlParts")?;
    assert_not_null(&result_json, "CustomXmlParts.CustomXmlPartsList")?;
    assert_length(&result_json, "CustomXmlParts.CustomXmlPartsList", 2)?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Id", test_string!("aspose")?)?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Data", test_string!("<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>")?)?;
    Ok(())
}

/// Test for getting all custom xml parts from document online.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_parts_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetCustomXmlPartsOnlineRequest::new(
        (requestDocument).into()
    );

    let result = context.api().get_custom_xml_parts_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlParts")?;
    assert_not_null(&result_json, "CustomXmlParts.CustomXmlPartsList")?;
    assert_length(&result_json, "CustomXmlParts.CustomXmlPartsList", 2)?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Id", test_string!("aspose")?)?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Data", test_string!("<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>")?)?;
    Ok(())
}

/// Test for adding custom xml part.
#[tokio::test]
async fn custom_xml_parts_insert_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/CustomXmlParts")?;
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;
    let remoteFileName = test_string!("TestInsertCustomXmlPart.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestCustomXmlPart = CustomXmlPartInsert::default();
    requestCustomXmlPart.r#id = Some((test_string!("hello")?).into());
    requestCustomXmlPart.r#data = Some((test_string!("<data>Hello world</data>")?).into());

    let request = InsertCustomXmlPartRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestCustomXmlPart).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_custom_xml_part(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", test_string!("hello")?)?;
    assert_string(&result_json, "CustomXmlPart.Data", test_string!("<data>Hello world</data>")?)?;
    Ok(())
}

/// Test for adding custom xml part online.
#[tokio::test]
async fn custom_xml_parts_insert_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestCustomXmlPart = CustomXmlPartInsert::default();
    requestCustomXmlPart.r#id = Some((test_string!("hello")?).into());
    requestCustomXmlPart.r#data = Some((test_string!("<data>Hello world</data>")?).into());

    let request = InsertCustomXmlPartOnlineRequest::new(
        (requestDocument).into(),
        (requestCustomXmlPart).into()
    );

    let result = context.api().insert_custom_xml_part_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.CustomXmlPart")?;
    assert_string(&result_json, "Model.CustomXmlPart.Id", test_string!("hello")?)?;
    assert_string(&result_json, "Model.CustomXmlPart.Data", test_string!("<data>Hello world</data>")?)?;
    Ok(())
}

/// Test for updating custom xml part.
#[tokio::test]
async fn custom_xml_parts_update_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/CustomXmlParts")?;
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;
    let remoteFileName = test_string!("TestUpdateCustomXmlPart.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestCustomXmlPart = CustomXmlPartUpdate::default();
    requestCustomXmlPart.r#data = Some((test_string!("<data>Hello world</data>")?).into());

    let request = UpdateCustomXmlPartRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestCustomXmlPart).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_custom_xml_part(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", test_string!("aspose")?)?;
    assert_string(&result_json, "CustomXmlPart.Data", test_string!("<data>Hello world</data>")?)?;
    Ok(())
}

/// Test for updating custom xml part online.
#[tokio::test]
async fn custom_xml_parts_update_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestCustomXmlPart = CustomXmlPartUpdate::default();
    requestCustomXmlPart.r#data = Some((test_string!("<data>Hello world</data>")?).into());

    let request = UpdateCustomXmlPartOnlineRequest::new(
        (requestDocument).into(),
        (0).into(),
        (requestCustomXmlPart).into()
    );

    let result = context.api().update_custom_xml_part_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.CustomXmlPart")?;
    assert_string(&result_json, "Model.CustomXmlPart.Id", test_string!("aspose")?)?;
    assert_string(&result_json, "Model.CustomXmlPart.Data", test_string!("<data>Hello world</data>")?)?;
    Ok(())
}

/// A test for DeleteCustomXmlPart.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/CustomXmlParts")?;
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;
    let remoteFileName = test_string!("TestDeleteCustomXmlPart.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteCustomXmlPartRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_custom_xml_part(request).await?;
    Ok(())
}

/// A test for DeleteCustomXmlPart online.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteCustomXmlPartOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    );

    context.api().delete_custom_xml_part_online(request).await?;
    Ok(())
}

/// A test for DeleteCustomXmlParts.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_parts() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/CustomXmlParts")?;
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;
    let remoteFileName = test_string!("TestDeleteCustomXmlPart.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteCustomXmlPartsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_custom_xml_parts(request).await?;
    Ok(())
}

/// A test for DeleteCustomXmlParts online.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_parts_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteCustomXmlPartsOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().delete_custom_xml_parts_online(request).await?;
    Ok(())
}