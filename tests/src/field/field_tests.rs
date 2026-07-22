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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestGetFields.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Fields")?;
    assert_not_null(&result_json, "Fields.List")?;
    assert_length(&result_json, "Fields.List", 1)?;
    assert_string(&result_json, "Fields.List[0].Result", "1".to_owned())?;
    Ok(())
}

/// Test for getting fields online.
#[tokio::test]
async fn field_get_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/Fields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/GetField.docx").await?;

    let request = GetFieldsOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().get_fields_online(request).await?;
    Ok(())
}

/// Test for getting fields without node path.
#[tokio::test]
async fn field_get_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestGetFieldsWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Fields")?;
    assert_not_null(&result_json, "Fields.List")?;
    assert_length(&result_json, "Fields.List", 1)?;
    assert_string(&result_json, "Fields.List[0].Result", "1".to_owned())?;
    Ok(())
}

/// Test for getting field by index.
#[tokio::test]
async fn field_get_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestGetField.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.Result", "1".to_owned())?;
    Ok(())
}

/// Test for getting field by index online.
#[tokio::test]
async fn field_get_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/Fields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/GetField.docx").await?;

    let request = GetFieldOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into());

    context.api().get_field_online(request).await?;
    Ok(())
}

/// Test for getting field by index without node path.
#[tokio::test]
async fn field_get_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestGetFieldWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.Result", "1".to_owned())?;
    Ok(())
}

/// Test for putting field.
#[tokio::test]
async fn field_insert_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let text_folder = "DocumentElements/Text".to_owned();
    let local_file_name = "SampleWordDocument.docx".to_owned();
    let remote_file_name = "TestInsertField.docx".to_owned();

    context.upload_file(text_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_field = FieldInsert::default();
    request_field.field_code = Some(("{ NUMPAGES }".to_owned()).into());

    let request = InsertFieldRequest::new(
        (remote_file_name.clone()).into(),
        (request_field).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.FieldCode", "{ NUMPAGES }".to_owned())?;
    assert_string(&result_json, "Field.NodeId", "0.0.0.1".to_owned())?;
    Ok(())
}

/// Test for putting field online.
#[tokio::test]
async fn field_insert_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/Fields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/GetField.docx").await?;
    let mut request_field = FieldInsert::default();
    request_field.field_code = Some(("{ NUMPAGES }".to_owned()).into());

    let request = InsertFieldOnlineRequest::new(
        (request_document).into(),
        (request_field).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into());

    context.api().insert_field_online(request).await?;
    Ok(())
}

/// Test for putting field without node path.
#[tokio::test]
async fn field_insert_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let text_folder = "DocumentElements/Text".to_owned();
    let local_file_name = "SampleWordDocument.docx".to_owned();
    let remote_file_name = "TestInsertFieldWithoutNodePath.docx".to_owned();

    context.upload_file(text_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_field = FieldInsert::default();
    request_field.field_code = Some(("{ NUMPAGES }".to_owned()).into());

    let request = InsertFieldRequest::new(
        (remote_file_name.clone()).into(),
        (request_field).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.FieldCode", "{ NUMPAGES }".to_owned())?;
    assert_string(&result_json, "Field.NodeId", "5.0.22.0".to_owned())?;
    Ok(())
}

/// Test for posting field.
#[tokio::test]
async fn field_update_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestUpdateField.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_field = FieldUpdate::default();
    request_field.field_code = Some(("{ NUMPAGES }".to_owned()).into());

    let request = UpdateFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_field).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Field")?;
    assert_string(&result_json, "Field.FieldCode", "{ NUMPAGES }".to_owned())?;
    assert_string(&result_json, "Field.NodeId", "0.0.0.0".to_owned())?;
    Ok(())
}

/// Test for posting field online.
#[tokio::test]
async fn field_update_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/Fields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/GetField.docx").await?;
    let mut request_field = FieldUpdate::default();
    request_field.field_code = Some(("{ NUMPAGES }".to_owned()).into());

    let request = UpdateFieldOnlineRequest::new(
        (request_document).into(),
        (request_field).into(),
        (0).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into());

    context.api().update_field_online(request).await?;
    Ok(())
}

/// Test for inserting page numbers field.
#[tokio::test]
async fn field_insert_page_numbers() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertPageNumbers.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_page_number = PageNumber::default();
    request_page_number.alignment = Some(("center".to_owned()).into());
    request_page_number.format = Some(("{PAGE} of {NUMPAGES}".to_owned()).into());
    request_page_number.is_top = Some((true).into());
    request_page_number.set_page_number_on_first_page = Some((true).into());

    let request = InsertPageNumbersRequest::new(
        (remote_file_name.clone()).into(),
        (request_page_number).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().insert_page_numbers(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestInsertPageNumbers.docx".to_owned())?;
    Ok(())
}

/// Test for inserting page numbers field online.
#[tokio::test]
async fn field_insert_page_numbers_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file_name = "test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file("Common/".to_owned() + &local_file_name).await?;
    let mut request_page_number = PageNumber::default();
    request_page_number.alignment = Some(("center".to_owned()).into());
    request_page_number.format = Some(("{PAGE} of {NUMPAGES}".to_owned()).into());
    request_page_number.is_top = Some((true).into());
    request_page_number.set_page_number_on_first_page = Some((true).into());

    let request = InsertPageNumbersOnlineRequest::new(
        (request_document).into(),
        (request_page_number).into()
    );

    context.api().insert_page_numbers_online(request).await?;
    Ok(())
}

/// Test for deleting field.
#[tokio::test]
async fn field_delete_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestDeleteField.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_field(request).await?;
    Ok(())
}

/// Test for deleting field online.
#[tokio::test]
async fn field_delete_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/Fields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/GetField.docx").await?;

    let request = DeleteFieldOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into());

    context.api().delete_field_online(request).await?;
    Ok(())
}

/// Test for deleting field without node path.
#[tokio::test]
async fn field_delete_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let field_folder = "DocumentElements/Fields".to_owned();
    let local_file_name = "GetField.docx".to_owned();
    let remote_file_name = "TestDeleteFieldWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/" + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_field(request).await?;
    Ok(())
}

/// Test for deleting paragraph fields.
#[tokio::test]
async fn field_delete_paragraph_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteParagraphFields.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting paragraph fields without node path.
#[tokio::test]
async fn field_delete_paragraph_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteParagraphFieldsWithoutNodePath.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting section fields.
#[tokio::test]
async fn field_delete_section_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteSectionFields.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting section fields without node path.
#[tokio::test]
async fn field_delete_section_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteSectionFieldsWithoutNodePath.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting paragraph fields in section.
#[tokio::test]
async fn field_delete_section_paragraph_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteSectionParagraphFields.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting fields.
#[tokio::test]
async fn field_delete_document_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteSectionParagraphFields.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    context.api().delete_fields(request).await?;
    Ok(())
}

/// Test for deleting fields online.
#[tokio::test]
async fn field_delete_document_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file_name = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file_name.clone()).await?;

    let request = DeleteFieldsOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("".to_owned()).into());

    context.api().delete_fields_online(request).await?;
    Ok(())
}

/// Test for posting updated fields.
#[tokio::test]
async fn field_update_document_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Fields";
    let local_file_name = "test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateDocumentFields.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_file_name, remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = UpdateFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestUpdateDocumentFields.docx".to_owned())?;
    Ok(())
}

/// Test for posting updated fields online.
#[tokio::test]
async fn field_update_document_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = UpdateFieldsOnlineRequest::new(
        (request_document).into()
    );

    context.api().update_fields_online(request).await?;
    Ok(())
}