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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestGetStyles.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetStylesRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_styles(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Styles")?;
    assert_length(&result_json, "Styles", 22)?;
    assert_string(
        &result_json,
        "Styles[0].Name",
        "Default Paragraph Font".to_owned(),
    )?;
    Ok(())
}

/// Test for getting styles from document online.
#[tokio::test]
async fn styles_get_styles_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetStylesOnlineRequest::new((request_document).into());

    context.api().get_styles_online(request).await?;
    Ok(())
}

/// Test for getting style from document.
#[tokio::test]
async fn styles_get_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestGetStyle.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetStyleRequest::new(
        (remote_file_name.clone()).into(),
        ("Heading 1".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", "Heading 1".to_owned())?;
    Ok(())
}

/// Test for getting style from document online.
#[tokio::test]
async fn styles_get_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        GetStyleOnlineRequest::new((request_document).into(), ("Heading 1".to_owned()).into());

    context.api().get_style_online(request).await?;
    Ok(())
}

/// Test for updating style from document.
#[tokio::test]
async fn styles_update_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestUpdateStyle.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_style_update = StyleUpdate::default();
    request_style_update.name = Some(("My Style".to_owned()).into());

    let request = UpdateStyleRequest::new(
        (remote_file_name.clone()).into(),
        ("Heading 1".to_owned()).into(),
        (request_style_update).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", "My Style".to_owned())?;
    Ok(())
}

/// Test for updating style from document online.
#[tokio::test]
async fn styles_update_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_style_update = StyleUpdate::default();
    request_style_update.name = Some(("My Style".to_owned()).into());

    let request = UpdateStyleOnlineRequest::new(
        (request_document).into(),
        ("Heading 1".to_owned()).into(),
        (request_style_update).into(),
    );

    context.api().update_style_online(request).await?;
    Ok(())
}

/// Test for inserting style from document.
#[tokio::test]
async fn styles_insert_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestInsertStyle.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_style_insert = StyleInsert::default();
    request_style_insert.style_name = Some(("My Style".to_owned()).into());
    request_style_insert.style_type = Some((StyleInsertStyleTypeEnum::Paragraph).into());

    let request = InsertStyleRequest::new(
        (remote_file_name.clone()).into(),
        (request_style_insert).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", "My Style".to_owned())?;
    Ok(())
}

/// Test for inserting style from document online.
#[tokio::test]
async fn styles_insert_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_style_insert = StyleInsert::default();
    request_style_insert.style_name = Some(("My Style".to_owned()).into());
    request_style_insert.style_type = Some((StyleInsertStyleTypeEnum::Paragraph).into());

    let request =
        InsertStyleOnlineRequest::new((request_document).into(), (request_style_insert).into());

    context.api().insert_style_online(request).await?;
    Ok(())
}

/// Test for coping style from document.
#[tokio::test]
async fn styles_copy_style() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestCopyStyle.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_style_copy = StyleCopy::default();
    request_style_copy.style_name = Some(("Heading 1".to_owned()).into());

    let request = CopyStyleRequest::new(
        (remote_file_name.clone()).into(),
        (request_style_copy).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().copy_style(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", "Heading 1_0".to_owned())?;
    Ok(())
}

/// Test for coping style from document online.
#[tokio::test]
async fn styles_copy_style_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_style_copy = StyleCopy::default();
    request_style_copy.style_name = Some(("Heading 1".to_owned()).into());

    let request =
        CopyStyleOnlineRequest::new((request_document).into(), (request_style_copy).into());

    context.api().copy_style_online(request).await?;
    Ok(())
}

/// Test for getting style from document element.
#[tokio::test]
async fn styles_get_style_from_document_element() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestGetStyleFromDocumentElement.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetStyleFromDocumentElementRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/1/paragraphFormat".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context
        .api()
        .get_style_from_document_element(request)
        .await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Style")?;
    assert_string(&result_json, "Style.Name", "TOC 1".to_owned())?;
    Ok(())
}

/// Test for getting style from document element online.
#[tokio::test]
async fn styles_get_style_from_document_element_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetStyleFromDocumentElementOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/1/paragraphFormat".to_owned()).into(),
    );

    context
        .api()
        .get_style_from_document_element_online(request)
        .await?;
    Ok(())
}

/// Test for applying style to document element.
#[tokio::test]
async fn styles_apply_style_to_document_element() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestApplyStyleToDocumentElement.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_style_apply = StyleApply::default();
    request_style_apply.style_name = Some(("Heading 1".to_owned()).into());

    let request = ApplyStyleToDocumentElementRequest::new(
        (remote_file_name.clone()).into(),
        ("paragraphs/1/paragraphFormat".to_owned()).into(),
        (request_style_apply).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context
        .api()
        .apply_style_to_document_element(request)
        .await?;
    Ok(())
}

/// Test for applying style to document element online.
#[tokio::test]
async fn styles_apply_style_to_document_element_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_style_apply = StyleApply::default();
    request_style_apply.style_name = Some(("Heading 1".to_owned()).into());

    let request = ApplyStyleToDocumentElementOnlineRequest::new(
        (request_document).into(),
        ("paragraphs/1/paragraphFormat".to_owned()).into(),
        (request_style_apply).into(),
    );

    context
        .api()
        .apply_style_to_document_element_online(request)
        .await?;
    Ok(())
}

/// Test for copying styles from a template.
#[tokio::test]
async fn styles_copy_styles_from_template() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Styles";
    let local_file = "DocumentElements/Styles/GetStyles.docx".to_owned();
    let remote_file_name = "TestCopyStylesFromTemplate.docx".to_owned();
    let template_folder = "DocumentElements/Styles".to_owned();
    let template_name = "StyleTemplate.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    context
        .upload_file(
            template_folder.clone() + "/" + &template_name,
            remote_data_folder.clone() + "/" + &template_name,
        )
        .await?;

    let request = CopyStylesFromTemplateRequest::new(
        (remote_file_name.clone()).into(),
        (template_name.clone()).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().copy_styles_from_template(request).await?;
    Ok(())
}
