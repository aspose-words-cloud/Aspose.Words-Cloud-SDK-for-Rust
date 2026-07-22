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

/// Test for getting styles from document.
#[tokio::test]
async fn styles_get_styles() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestGetStyles.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetStylesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_styles(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Styles")?;
    assert_length(&result_json, "Styles", 22)?;
    assert_string(&result_json, "Styles[0].Name", test_string!("Default Paragraph Font")?)?;
    Ok(())
}

/// Test for getting styles from document online.
#[tokio::test]
async fn styles_get_styles_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetStylesOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().get_styles_online(request).await?;
    Ok(())
}

/// Test for getting style from document.
#[tokio::test]
async fn styles_get_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestGetStyle.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetStyleRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("Heading 1")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", test_string!("Heading 1")?)?;
    Ok(())
}

/// Test for getting style from document online.
#[tokio::test]
async fn styles_get_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetStyleOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("Heading 1")?).into()
    );

    context.api().get_style_online(request).await?;
    Ok(())
}

/// Test for updating style from document.
#[tokio::test]
async fn styles_update_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestUpdateStyle.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestStyleUpdate = StyleUpdate::default();
    requestStyleUpdate.r#name = Some((test_string!("My Style")?).into());

    let request = UpdateStyleRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("Heading 1")?).into(),
        (requestStyleUpdate).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", test_string!("My Style")?)?;
    Ok(())
}

/// Test for updating style from document online.
#[tokio::test]
async fn styles_update_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestStyleUpdate = StyleUpdate::default();
    requestStyleUpdate.r#name = Some((test_string!("My Style")?).into());

    let request = UpdateStyleOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("Heading 1")?).into(),
        (requestStyleUpdate).into()
    );

    context.api().update_style_online(request).await?;
    Ok(())
}

/// Test for inserting style from document.
#[tokio::test]
async fn styles_insert_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestInsertStyle.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestStyleInsert = StyleInsert::default();
    requestStyleInsert.r#style_name = Some((test_string!("My Style")?).into());
    requestStyleInsert.r#style_type = Some((StyleInsert_StyleTypeEnum::Paragraph).into());

    let request = InsertStyleRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestStyleInsert).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", test_string!("My Style")?)?;
    Ok(())
}

/// Test for inserting style from document online.
#[tokio::test]
async fn styles_insert_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestStyleInsert = StyleInsert::default();
    requestStyleInsert.r#style_name = Some((test_string!("My Style")?).into());
    requestStyleInsert.r#style_type = Some((StyleInsert_StyleTypeEnum::Paragraph).into());

    let request = InsertStyleOnlineRequest::new(
        (requestDocument).into(),
        (requestStyleInsert).into()
    );

    context.api().insert_style_online(request).await?;
    Ok(())
}

/// Test for coping style from document.
#[tokio::test]
async fn styles_copy_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestCopyStyle.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestStyleCopy = StyleCopy::default();
    requestStyleCopy.r#style_name = Some((test_string!("Heading 1")?).into());

    let request = CopyStyleRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestStyleCopy).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().copy_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", test_string!("Heading 1_0")?)?;
    Ok(())
}

/// Test for coping style from document online.
#[tokio::test]
async fn styles_copy_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestStyleCopy = StyleCopy::default();
    requestStyleCopy.r#style_name = Some((test_string!("Heading 1")?).into());

    let request = CopyStyleOnlineRequest::new(
        (requestDocument).into(),
        (requestStyleCopy).into()
    );

    context.api().copy_style_online(request).await?;
    Ok(())
}

/// Test for getting style from document element.
#[tokio::test]
async fn styles_get_style_from_document_element() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestGetStyleFromDocumentElement.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetStyleFromDocumentElementRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/1/paragraphFormat")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_style_from_document_element(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", test_string!("TOC 1")?)?;
    Ok(())
}

/// Test for getting style from document element online.
#[tokio::test]
async fn styles_get_style_from_document_element_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetStyleFromDocumentElementOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/1/paragraphFormat")?).into()
    );

    context.api().get_style_from_document_element_online(request).await?;
    Ok(())
}

/// Test for applying style to document element.
#[tokio::test]
async fn styles_apply_style_to_document_element() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestApplyStyleToDocumentElement.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestStyleApply = StyleApply::default();
    requestStyleApply.r#style_name = Some((test_string!("Heading 1")?).into());

    let request = ApplyStyleToDocumentElementRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("paragraphs/1/paragraphFormat")?).into(),
        (requestStyleApply).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().apply_style_to_document_element(request).await?;
    Ok(())
}

/// Test for applying style to document element online.
#[tokio::test]
async fn styles_apply_style_to_document_element_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestStyleApply = StyleApply::default();
    requestStyleApply.r#style_name = Some((test_string!("Heading 1")?).into());

    let request = ApplyStyleToDocumentElementOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("paragraphs/1/paragraphFormat")?).into(),
        (requestStyleApply).into()
    );

    context.api().apply_style_to_document_element_online(request).await?;
    Ok(())
}

/// Test for copying styles from a template.
#[tokio::test]
async fn styles_copy_styles_from_template() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Styles")?;
    let localFile = test_string!("DocumentElements/Styles/GetStyles.docx")?;
    let remoteFileName = test_string!("TestCopyStylesFromTemplate.docx")?;
    let templateFolder = test_string!("DocumentElements/Styles")?;
    let templateName = test_string!("StyleTemplate.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    context.upload_file(test_string!(templateFolder + "/" + templateName)?, test_string!(remoteDataFolder + "/" + templateName)?).await?;

    let request = CopyStylesFromTemplateRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!(templateName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().copy_styles_from_template(request).await?;
    Ok(())
}