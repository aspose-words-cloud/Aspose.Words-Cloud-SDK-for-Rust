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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestUpdateFormField.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFormField = FormFieldTextInput::default();
    requestFormField.r#name = Some((test_string!("FullName")?).into());
    requestFormField.r#enabled = Some((true).into());
    requestFormField.r#calculate_on_exit = Some((true).into());
    requestFormField.r#status_text = Some((test_string!("")?).into());
    requestFormField.r#text_input_type = Some((FormFieldTextInput_TextInputTypeEnum::Regular).into());
    requestFormField.r#text_input_default = Some((test_string!("No name")?).into());
    requestFormField.r#text_input_format = Some((test_string!("")?).into());

    let request = UpdateFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestFormField).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().update_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", test_string!("FullName")?)?;
    assert_string(&result_json, "FormField.StatusText", test_string!("")?)?;
    Ok(())
}

/// Test for posting form field online.
#[tokio::test]
async fn form_field_update_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/FormFields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/FormFilled.docx")?).await?;
    let mut requestFormField = FormFieldTextInput::default();
    requestFormField.r#name = Some((test_string!("FullName")?).into());
    requestFormField.r#enabled = Some((true).into());
    requestFormField.r#calculate_on_exit = Some((true).into());
    requestFormField.r#status_text = Some((test_string!("")?).into());
    requestFormField.r#text_input_type = Some((FormFieldTextInput_TextInputTypeEnum::Regular).into());
    requestFormField.r#text_input_default = Some((test_string!("No name")?).into());
    requestFormField.r#text_input_format = Some((test_string!("")?).into());

    let request = UpdateFormFieldOnlineRequest::new(
        (requestDocument).into(),
        (requestFormField).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().update_form_field_online(request).await?;
    Ok(())
}

/// Test for posting form field without node path.
#[tokio::test]
async fn form_field_update_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestUpdateFormFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFormField = FormFieldTextInput::default();
    requestFormField.r#name = Some((test_string!("FullName")?).into());
    requestFormField.r#enabled = Some((true).into());
    requestFormField.r#calculate_on_exit = Some((true).into());
    requestFormField.r#status_text = Some((test_string!("")?).into());
    requestFormField.r#text_input_type = Some((FormFieldTextInput_TextInputTypeEnum::Regular).into());
    requestFormField.r#text_input_default = Some((test_string!("No name")?).into());
    requestFormField.r#text_input_format = Some((test_string!("")?).into());

    let request = UpdateFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestFormField).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().update_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", test_string!("FullName")?)?;
    assert_string(&result_json, "FormField.StatusText", test_string!("")?)?;
    Ok(())
}

/// Test for getting form field.
#[tokio::test]
async fn form_field_get_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestGetFormField.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", test_string!("FullName")?)?;
    Ok(())
}

/// Test for getting form field online.
#[tokio::test]
async fn form_field_get_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/FormFields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/FormFilled.docx")?).await?;

    let request = GetFormFieldOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_form_field_online(request).await?;
    Ok(())
}

/// Test for getting form field without node path.
#[tokio::test]
async fn form_field_get_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestGetFormFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", test_string!("FullName")?)?;
    Ok(())
}

/// Test for getting form fields.
#[tokio::test]
async fn form_field_get_form_fields() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestGetFormFields.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFormFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_form_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormFields")?;
    assert_not_null(&result_json, "FormFields.List")?;
    assert_length(&result_json, "FormFields.List", 5)?;
    assert_string(&result_json, "FormFields.List[0].Name", test_string!("FullName")?)?;
    Ok(())
}

/// Test for getting form fields online.
#[tokio::test]
async fn form_field_get_form_fields_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/FormFields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/FormFilled.docx")?).await?;

    let request = GetFormFieldsOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().get_form_fields_online(request).await?;
    Ok(())
}

/// Test for getting form fields without node path.
#[tokio::test]
async fn form_field_get_form_fields_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestGetFormFieldsWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFormFieldsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_form_fields(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormFields")?;
    assert_not_null(&result_json, "FormFields.List")?;
    assert_length(&result_json, "FormFields.List", 5)?;
    assert_string(&result_json, "FormFields.List[0].Name", test_string!("FullName")?)?;
    Ok(())
}

/// Test for insert form field without node path.
#[tokio::test]
async fn form_field_insert_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestInsertFormField.docx")?;

    context.upload_file(test_string!("Common/test_multi_pages.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFormField = FormFieldTextInput::default();
    requestFormField.r#name = Some((test_string!("FullName")?).into());
    requestFormField.r#enabled = Some((true).into());
    requestFormField.r#calculate_on_exit = Some((true).into());
    requestFormField.r#status_text = Some((test_string!("")?).into());
    requestFormField.r#text_input_type = Some((FormFieldTextInput_TextInputTypeEnum::Regular).into());
    requestFormField.r#text_input_default = Some((test_string!("123")?).into());
    requestFormField.r#text_input_format = Some((test_string!("UPPERCASE")?).into());

    let request = InsertFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestFormField).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().insert_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", test_string!("FullName")?)?;
    assert_string(&result_json, "FormField.StatusText", test_string!("")?)?;
    Ok(())
}

/// Test for insert form field without node path online.
#[tokio::test]
async fn form_field_insert_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/FormFields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/FormFilled.docx")?).await?;
    let mut requestFormField = FormFieldTextInput::default();
    requestFormField.r#name = Some((test_string!("FullName")?).into());
    requestFormField.r#enabled = Some((true).into());
    requestFormField.r#calculate_on_exit = Some((true).into());
    requestFormField.r#status_text = Some((test_string!("")?).into());
    requestFormField.r#text_input_type = Some((FormFieldTextInput_TextInputTypeEnum::Regular).into());
    requestFormField.r#text_input_default = Some((test_string!("123")?).into());
    requestFormField.r#text_input_format = Some((test_string!("UPPERCASE")?).into());

    let request = InsertFormFieldOnlineRequest::new(
        (requestDocument).into(),
        (requestFormField).into()
    ).with_node_path((test_string!("sections/0/paragraphs/0")?).into());

    context.api().insert_form_field_online(request).await?;
    Ok(())
}

/// Test for insert form field without node path.
#[tokio::test]
async fn form_field_insert_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestInsertFormFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!("Common/test_multi_pages.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFormField = FormFieldTextInput::default();
    requestFormField.r#name = Some((test_string!("FullName")?).into());
    requestFormField.r#enabled = Some((true).into());
    requestFormField.r#calculate_on_exit = Some((true).into());
    requestFormField.r#status_text = Some((test_string!("")?).into());
    requestFormField.r#text_input_type = Some((FormFieldTextInput_TextInputTypeEnum::Regular).into());
    requestFormField.r#text_input_default = Some((test_string!("123")?).into());
    requestFormField.r#text_input_format = Some((test_string!("UPPERCASE")?).into());

    let request = InsertFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestFormField).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().insert_form_field(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FormField")?;
    assert_string(&result_json, "FormField.Name", test_string!("FullName")?)?;
    assert_string(&result_json, "FormField.StatusText", test_string!("")?)?;
    Ok(())
}

/// Test for deleting form field.
#[tokio::test]
async fn form_field_delete_form_field() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestDeleteFormField.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_form_field(request).await?;
    Ok(())
}

/// Test for deleting form field online.
#[tokio::test]
async fn form_field_delete_form_field_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let fieldFolder = test_string!("DocumentElements/FormFields")?;

    let requestDocument = context.load_binary_file(test_string!(fieldFolder + "/FormFilled.docx")?).await?;

    let request = DeleteFormFieldOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("sections/0")?).into());

    context.api().delete_form_field_online(request).await?;
    Ok(())
}

/// Test for deleting form field without node path.
#[tokio::test]
async fn form_field_delete_form_field_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/FormFields")?;
    let fieldFolder = test_string!("DocumentElements/FormFields")?;
    let remoteFileName = test_string!("TestDeleteFormFieldWithoutNodePath.docx")?;

    context.upload_file(test_string!(fieldFolder + "/FormFilled.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFormFieldRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_form_field(request).await?;
    Ok(())
}