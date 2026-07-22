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

/// Test for posting form field.
#[tokio::test]
async fn form_field_update_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestUpdateFormField.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_form_field = FormFieldTextInput::default();
    request_form_field.name = Some(("FullName".to_owned()).into());
    request_form_field.enabled = Some((true).into());
    request_form_field.calculate_on_exit = Some((true).into());
    request_form_field.status_text = Some(("".to_owned()).into());
    request_form_field.text_input_type = Some((FormFieldTextInputTextInputTypeEnum::Regular).into());
    request_form_field.text_input_default = Some(("No name".to_owned()).into());
    request_form_field.text_input_format = Some(("".to_owned()).into());

    let request = UpdateFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_form_field).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().update_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", "FullName".to_owned())?;
    assert_string(&result_json, "FormField.StatusText", "".to_owned())?;
    Ok(())
}

/// Test for posting form field online.
#[tokio::test]
async fn form_field_update_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/FormFields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/FormFilled.docx").await?;
    let mut request_form_field = FormFieldTextInput::default();
    request_form_field.name = Some(("FullName".to_owned()).into());
    request_form_field.enabled = Some((true).into());
    request_form_field.calculate_on_exit = Some((true).into());
    request_form_field.status_text = Some(("".to_owned()).into());
    request_form_field.text_input_type = Some((FormFieldTextInputTextInputTypeEnum::Regular).into());
    request_form_field.text_input_default = Some(("No name".to_owned()).into());
    request_form_field.text_input_format = Some(("".to_owned()).into());

    let request = UpdateFormFieldOnlineRequest::new(
        (request_document).into(),
        (request_form_field).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().update_form_field_online(request).await?;
    Ok(())
}

/// Test for posting form field without node path.
#[tokio::test]
async fn form_field_update_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestUpdateFormFieldWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_form_field = FormFieldTextInput::default();
    request_form_field.name = Some(("FullName".to_owned()).into());
    request_form_field.enabled = Some((true).into());
    request_form_field.calculate_on_exit = Some((true).into());
    request_form_field.status_text = Some(("".to_owned()).into());
    request_form_field.text_input_type = Some((FormFieldTextInputTextInputTypeEnum::Regular).into());
    request_form_field.text_input_default = Some(("No name".to_owned()).into());
    request_form_field.text_input_format = Some(("".to_owned()).into());

    let request = UpdateFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_form_field).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().update_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", "FullName".to_owned())?;
    assert_string(&result_json, "FormField.StatusText", "".to_owned())?;
    Ok(())
}

/// Test for getting form field.
#[tokio::test]
async fn form_field_get_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestGetFormField.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", "FullName".to_owned())?;
    Ok(())
}

/// Test for getting form field online.
#[tokio::test]
async fn form_field_get_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/FormFields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/FormFilled.docx").await?;

    let request = GetFormFieldOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().get_form_field_online(request).await?;
    Ok(())
}

/// Test for getting form field without node path.
#[tokio::test]
async fn form_field_get_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestGetFormFieldWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", "FullName".to_owned())?;
    Ok(())
}

/// Test for getting form fields.
#[tokio::test]
async fn form_field_get_form_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestGetFormFields.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFormFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_form_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormFields")?;
    assert_not_null(&result_json, "FormFields.List")?;
    assert_length(&result_json, "FormFields.List", 5)?;
    assert_string(&result_json, "FormFields.List[0].Name", "FullName".to_owned())?;
    Ok(())
}

/// Test for getting form fields online.
#[tokio::test]
async fn form_field_get_form_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/FormFields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/FormFilled.docx").await?;

    let request = GetFormFieldsOnlineRequest::new(
        (request_document).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().get_form_fields_online(request).await?;
    Ok(())
}

/// Test for getting form fields without node path.
#[tokio::test]
async fn form_field_get_form_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestGetFormFieldsWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetFormFieldsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_form_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormFields")?;
    assert_not_null(&result_json, "FormFields.List")?;
    assert_length(&result_json, "FormFields.List", 5)?;
    assert_string(&result_json, "FormFields.List[0].Name", "FullName".to_owned())?;
    Ok(())
}

/// Test for insert form field without node path.
#[tokio::test]
async fn form_field_insert_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let remote_file_name = "TestInsertFormField.docx".to_owned();

    context.upload_file("Common/test_multi_pages.docx".to_owned(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_form_field = FormFieldTextInput::default();
    request_form_field.name = Some(("FullName".to_owned()).into());
    request_form_field.enabled = Some((true).into());
    request_form_field.calculate_on_exit = Some((true).into());
    request_form_field.status_text = Some(("".to_owned()).into());
    request_form_field.text_input_type = Some((FormFieldTextInputTextInputTypeEnum::Regular).into());
    request_form_field.text_input_default = Some(("123".to_owned()).into());
    request_form_field.text_input_format = Some(("UPPERCASE".to_owned()).into());

    let request = InsertFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (request_form_field).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().insert_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", "FullName".to_owned())?;
    assert_string(&result_json, "FormField.StatusText", "".to_owned())?;
    Ok(())
}

/// Test for insert form field without node path online.
#[tokio::test]
async fn form_field_insert_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/FormFields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/FormFilled.docx").await?;
    let mut request_form_field = FormFieldTextInput::default();
    request_form_field.name = Some(("FullName".to_owned()).into());
    request_form_field.enabled = Some((true).into());
    request_form_field.calculate_on_exit = Some((true).into());
    request_form_field.status_text = Some(("".to_owned()).into());
    request_form_field.text_input_type = Some((FormFieldTextInputTextInputTypeEnum::Regular).into());
    request_form_field.text_input_default = Some(("123".to_owned()).into());
    request_form_field.text_input_format = Some(("UPPERCASE".to_owned()).into());

    let request = InsertFormFieldOnlineRequest::new(
        (request_document).into(),
        (request_form_field).into()
    ).with_node_path(("sections/0/paragraphs/0".to_owned()).into());

    context.api().insert_form_field_online(request).await?;
    Ok(())
}

/// Test for insert form field without node path.
#[tokio::test]
async fn form_field_insert_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let remote_file_name = "TestInsertFormFieldWithoutNodePath.docx".to_owned();

    context.upload_file("Common/test_multi_pages.docx".to_owned(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_form_field = FormFieldTextInput::default();
    request_form_field.name = Some(("FullName".to_owned()).into());
    request_form_field.enabled = Some((true).into());
    request_form_field.calculate_on_exit = Some((true).into());
    request_form_field.status_text = Some(("".to_owned()).into());
    request_form_field.text_input_type = Some((FormFieldTextInputTextInputTypeEnum::Regular).into());
    request_form_field.text_input_default = Some(("123".to_owned()).into());
    request_form_field.text_input_format = Some(("UPPERCASE".to_owned()).into());

    let request = InsertFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (request_form_field).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().insert_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", "FullName".to_owned())?;
    assert_string(&result_json, "FormField.StatusText", "".to_owned())?;
    Ok(())
}

/// Test for deleting form field.
#[tokio::test]
async fn form_field_delete_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestDeleteFormField.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into())
.with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_form_field(request).await?;
    Ok(())
}

/// Test for deleting form field online.
#[tokio::test]
async fn form_field_delete_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let field_folder = "DocumentElements/FormFields".to_owned();

    let request_document = context.load_binary_file(field_folder.clone() + "/FormFilled.docx").await?;

    let request = DeleteFormFieldOnlineRequest::new(
        (request_document).into(),
        (0).into()
    ).with_node_path(("sections/0".to_owned()).into());

    context.api().delete_form_field_online(request).await?;
    Ok(())
}

/// Test for deleting form field without node path.
#[tokio::test]
async fn form_field_delete_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/FormFields";
    let field_folder = "DocumentElements/FormFields".to_owned();
    let remote_file_name = "TestDeleteFormFieldWithoutNodePath.docx".to_owned();

    context.upload_file(field_folder.clone() + "/FormFilled.docx", remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteFormFieldRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_form_field(request).await?;
    Ok(())
}