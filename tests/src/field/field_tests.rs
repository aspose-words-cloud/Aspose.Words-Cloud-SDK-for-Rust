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

/// Test for getting fields.
#[tokio::test]
async fn field_get_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestGetFields.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Fields")?;
    assert_not_null(&result_json, "Fields.List")?;
    assert_length(&result_json, "Fields.List", 1)?;
    assert_string(&result_json, "Fields.List[0].Result", test_string!("1")?)?;
    Ok(())
}

/// Test for getting fields online.
#[tokio::test]
async fn field_get_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/Fields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/GetField.docx")?).await?;

    let request = GetFieldsOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_fields_online(request).await?;
    Ok(())
}

/// Test for getting fields without node path.
#[tokio::test]
async fn field_get_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestGetFieldsWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Fields")?;
    assert_not_null(&result_json, "Fields.List")?;
    assert_length(&result_json, "Fields.List", 1)?;
    assert_string(&result_json, "Fields.List[0].Result", test_string!("1")?)?;
    Ok(())
}

/// Test for getting field by index.
#[tokio::test]
async fn field_get_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestGetField.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.Result", test_string!("1")?)?;
    Ok(())
}

/// Test for getting field by index online.
#[tokio::test]
async fn field_get_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/Fields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/GetField.docx")?).await?;

    let request = GetFieldOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into());

    context.api().get_field_online(request).await?;
    Ok(())
}

/// Test for getting field by index without node path.
#[tokio::test]
async fn field_get_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestGetFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.Result", test_string!("1")?)?;
    Ok(())
}

/// Test for putting field.
#[tokio::test]
async fn field_insert_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let textFolder = test_string!("DocumentElements/Text")?;
    let localFileName = test_string!("SampleWordDocument.docx")?;
    let remoteFileName = test_string!("TestInsertField.docx")?;

    context.upload_file(test_string!(textFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestField = FieldInsert::default();
    requestField.r#field_code = Some((test_string!("{ NUMPAGES }")?).into());

    let request = InsertFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestField).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.FieldCode", test_string!("{ NUMPAGES }")?)?;
    assert_string(&result_json, "Field.NodeId", test_string!("0.0.0.1")?)?;
    Ok(())
}

/// Test for putting field online.
#[tokio::test]
async fn field_insert_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/Fields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/GetField.docx")?).await?;
    let mut requestField = FieldInsert::default();
    requestField.r#field_code = Some((test_string!("{ NUMPAGES }")?).into());

    let request = InsertFieldOnlineRequest::new(
        (requestDocument).into(),
        (requestField).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into());

    context.api().insert_field_online(request).await?;
    Ok(())
}

/// Test for putting field without node path.
#[tokio::test]
async fn field_insert_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let textFolder = test_string!("DocumentElements/Text")?;
    let localFileName = test_string!("SampleWordDocument.docx")?;
    let remoteFileName = test_string!("TestInsertFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!(textFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestField = FieldInsert::default();
    requestField.r#field_code = Some((test_string!("{ NUMPAGES }")?).into());

    let request = InsertFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestField).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.FieldCode", test_string!("{ NUMPAGES }")?)?;
    assert_string(&result_json, "Field.NodeId", test_string!("5.0.22.0")?)?;
    Ok(())
}

/// Test for posting field.
#[tokio::test]
async fn field_update_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestUpdateField.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestField = FieldUpdate::default();
    requestField.r#field_code = Some((test_string!("{ NUMPAGES }")?).into());

    let request = UpdateFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestField).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.FieldCode", test_string!("{ NUMPAGES }")?)?;
    assert_string(&result_json, "Field.NodeId", test_string!("0.0.0.0")?)?;
    Ok(())
}

/// Test for posting field online.
#[tokio::test]
async fn field_update_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/Fields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/GetField.docx")?).await?;
    let mut requestField = FieldUpdate::default();
    requestField.r#field_code = Some((test_string!("{ NUMPAGES }")?).into());

    let request = UpdateFieldOnlineRequest::new(
        (requestDocument).into(),
        (requestField).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into());

    context.api().update_field_online(request).await?;
    Ok(())
}

/// Test for inserting page numbers field.
#[tokio::test]
async fn field_insert_page_numbers() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertPageNumbers.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestPageNumber = PageNumber::default();
    requestPageNumber.r#alignment = Some((test_string!("center")?).into());
    requestPageNumber.r#format = Some((test_string!("{PAGE} of {NUMPAGES}")?).into());
    requestPageNumber.r#is_top = Some((true).into());
    requestPageNumber.r#set_page_number_on_first_page = Some((true).into());

    let request = InsertPageNumbersRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestPageNumber).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().insert_page_numbers(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestInsertPageNumbers.docx")?)?;
    Ok(())
}

/// Test for inserting page numbers field online.
#[tokio::test]
async fn field_insert_page_numbers_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFileName = test_string!("test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!("Common/" + localFileName)?).await?;
    let mut requestPageNumber = PageNumber::default();
    requestPageNumber.r#alignment = Some((test_string!("center")?).into());
    requestPageNumber.r#format = Some((test_string!("{PAGE} of {NUMPAGES}")?).into());
    requestPageNumber.r#is_top = Some((true).into());
    requestPageNumber.r#set_page_number_on_first_page = Some((true).into());

    let request = InsertPageNumbersOnlineRequest::new(
        (requestDocument).into(),
        (requestPageNumber).into()
    );

    context.api().insert_page_numbers_online(request).await?;
    Ok(())
}

/// Test for deleting field.
#[tokio::test]
async fn field_delete_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestDeleteField.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_field(request).await?;
    Ok(())
}

/// Test for deleting field online.
#[tokio::test]
async fn field_delete_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/Fields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/GetField.docx")?).await?;

    let request = DeleteFieldOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into());

    context.api().delete_field_online(request).await?;
    Ok(())
}

/// Test for deleting field without node path.
#[tokio::test]
async fn field_delete_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let fieldFolder = test_string!("DocumentElements/Fields")?;
    let localFileName = test_string!("GetField.docx")?;
    let remoteFileName = test_string!("TestDeleteFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_field(request).await?;
    Ok(())
}

/// Test for deleting paragraph fields.
#[tokio::test]
async fn field_delete_paragraph_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteParagraphFields.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting paragraph fields without node path.
#[tokio::test]
async fn field_delete_paragraph_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteParagraphFieldsWithoutNodePath.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting section fields.
#[tokio::test]
async fn field_delete_section_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteSectionFields.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting section fields without node path.
#[tokio::test]
async fn field_delete_section_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteSectionFieldsWithoutNodePath.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting paragraph fields in section.
#[tokio::test]
async fn field_delete_section_paragraph_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteSectionParagraphFields.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting fields.
#[tokio::test]
async fn field_delete_document_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteSectionParagraphFields.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting fields online.
#[tokio::test]
async fn field_delete_document_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFileName = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFileName)?).await?;

    let request = DeleteFieldsOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_fields_online(request).await?;
    Ok(())
}

/// Test for posting updated fields.
#[tokio::test]
async fn field_update_document_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Fields")?;
    let localFileName = test_string!("test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateDocumentFields.docx")?;

    context.upload_file(test_string!("Common/" + localFileName)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = UpdateFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestUpdateDocumentFields.docx")?)?;
    Ok(())
}

/// Test for posting updated fields online.
#[tokio::test]
async fn field_update_document_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = UpdateFieldsOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().update_fields_online(request).await?;
    Ok(())
}