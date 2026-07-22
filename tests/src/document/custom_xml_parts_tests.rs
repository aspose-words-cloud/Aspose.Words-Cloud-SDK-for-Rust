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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/CustomXmlParts";
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();
    let remote_file_name = "TestGetCustomXmlPart.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetCustomXmlPartRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_custom_xml_part(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", "aspose".to_owned())?;
    assert_string(&result_json, "CustomXmlPart.Data", "<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>".to_owned())?;
    Ok(())
}

/// Test for getting custom xml part by specified index online.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetCustomXmlPartOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    let result = context.api().get_custom_xml_part_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", "aspose".to_owned())?;
    assert_string(&result_json, "CustomXmlPart.Data", "<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>".to_owned())?;
    Ok(())
}

/// Test for getting all custom xml parts from document.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_parts() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/CustomXmlParts";
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();
    let remote_file_name = "TestGetCustomXmlParts.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetCustomXmlPartsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_custom_xml_parts(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlParts")?;
    assert_not_null(&result_json, "CustomXmlParts.CustomXmlPartsList")?;
    assert_length(&result_json, "CustomXmlParts.CustomXmlPartsList", 2)?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Id", "aspose".to_owned())?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Data", "<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>".to_owned())?;
    Ok(())
}

/// Test for getting all custom xml parts from document online.
#[tokio::test]
async fn custom_xml_parts_get_custom_xml_parts_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetCustomXmlPartsOnlineRequest::new(
        (request_document).into()
    );

    let result = context.api().get_custom_xml_parts_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlParts")?;
    assert_not_null(&result_json, "CustomXmlParts.CustomXmlPartsList")?;
    assert_length(&result_json, "CustomXmlParts.CustomXmlPartsList", 2)?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Id", "aspose".to_owned())?;
    assert_string(&result_json, "CustomXmlParts.CustomXmlPartsList[0].Data", "<Metadata><Author>author1</Author><Initial>initial</Initial><DateTime>2015-01-22T00:00:00</DateTime><Text>text</Text></Metadata>".to_owned())?;
    Ok(())
}

/// Test for adding custom xml part.
#[tokio::test]
async fn custom_xml_parts_insert_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/CustomXmlParts";
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();
    let remote_file_name = "TestInsertCustomXmlPart.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_custom_xml_part = CustomXmlPartInsert::default();
    request_custom_xml_part.id = Some(("hello".to_owned()).into());
    request_custom_xml_part.data = Some(("<data>Hello world</data>".to_owned()).into());

    let request = InsertCustomXmlPartRequest::new(
        (remote_file_name.clone()).into(),
        (request_custom_xml_part).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_custom_xml_part(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", "hello".to_owned())?;
    assert_string(&result_json, "CustomXmlPart.Data", "<data>Hello world</data>".to_owned())?;
    Ok(())
}

/// Test for adding custom xml part online.
#[tokio::test]
async fn custom_xml_parts_insert_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_custom_xml_part = CustomXmlPartInsert::default();
    request_custom_xml_part.id = Some(("hello".to_owned()).into());
    request_custom_xml_part.data = Some(("<data>Hello world</data>".to_owned()).into());

    let request = InsertCustomXmlPartOnlineRequest::new(
        (request_document).into(),
        (request_custom_xml_part).into()
    );

    let result = context.api().insert_custom_xml_part_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.CustomXmlPart")?;
    assert_string(&result_json, "Model.CustomXmlPart.Id", "hello".to_owned())?;
    assert_string(&result_json, "Model.CustomXmlPart.Data", "<data>Hello world</data>".to_owned())?;
    Ok(())
}

/// Test for updating custom xml part.
#[tokio::test]
async fn custom_xml_parts_update_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/CustomXmlParts";
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();
    let remote_file_name = "TestUpdateCustomXmlPart.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_custom_xml_part = CustomXmlPartUpdate::default();
    request_custom_xml_part.data = Some(("<data>Hello world</data>".to_owned()).into());

    let request = UpdateCustomXmlPartRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_custom_xml_part).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_custom_xml_part(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CustomXmlPart")?;
    assert_string(&result_json, "CustomXmlPart.Id", "aspose".to_owned())?;
    assert_string(&result_json, "CustomXmlPart.Data", "<data>Hello world</data>".to_owned())?;
    Ok(())
}

/// Test for updating custom xml part online.
#[tokio::test]
async fn custom_xml_parts_update_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_custom_xml_part = CustomXmlPartUpdate::default();
    request_custom_xml_part.data = Some(("<data>Hello world</data>".to_owned()).into());

    let request = UpdateCustomXmlPartOnlineRequest::new(
        (request_document).into(),
        (0).into(),
        (request_custom_xml_part).into()
    );

    let result = context.api().update_custom_xml_part_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.CustomXmlPart")?;
    assert_string(&result_json, "Model.CustomXmlPart.Id", "aspose".to_owned())?;
    assert_string(&result_json, "Model.CustomXmlPart.Data", "<data>Hello world</data>".to_owned())?;
    Ok(())
}

/// A test for DeleteCustomXmlPart.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_part() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/CustomXmlParts";
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();
    let remote_file_name = "TestDeleteCustomXmlPart.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteCustomXmlPartRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_custom_xml_part(request).await?;
    Ok(())
}

/// A test for DeleteCustomXmlPart online.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_part_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteCustomXmlPartOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().delete_custom_xml_part_online(request).await?;
    Ok(())
}

/// A test for DeleteCustomXmlParts.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_parts() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/CustomXmlParts";
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();
    let remote_file_name = "TestDeleteCustomXmlPart.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteCustomXmlPartsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_custom_xml_parts(request).await?;
    Ok(())
}

/// A test for DeleteCustomXmlParts online.
#[tokio::test]
async fn custom_xml_parts_delete_custom_xml_parts_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/CustomXmlParts/MultipleCustomXmlParts.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteCustomXmlPartsOnlineRequest::new(
        (request_document).into()
    );

    context.api().delete_custom_xml_parts_online(request).await?;
    Ok(())
}