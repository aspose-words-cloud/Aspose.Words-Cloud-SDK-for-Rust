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

/// Test for getting document properties.
#[tokio::test]
async fn document_properties_get_document_properties() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProperties";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentProperties.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentPropertiesRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_document_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "DocumentProperties")?;
    assert_not_null(&result_json, "DocumentProperties.List")?;
    assert_length(&result_json, "DocumentProperties.List", 27)?;
    assert_not_null(&result_json, "DocumentProperties.List[0]")?;
    assert_string(
        &result_json,
        "DocumentProperties.List[0].Name",
        "Author".to_owned(),
    )?;
    assert_string(
        &result_json,
        "DocumentProperties.List[0].Value",
        "".to_owned(),
    )?;
    Ok(())
}

/// Test for getting document properties online.
#[tokio::test]
async fn document_properties_get_document_properties_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetDocumentPropertiesOnlineRequest::new((request_document).into());

    context
        .api()
        .get_document_properties_online(request)
        .await?;
    Ok(())
}

/// A test for GetDocumentProperty.
#[tokio::test]
async fn document_properties_get_document_property() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProperties";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentProperty.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentPropertyRequest::new(
        (remote_file_name.clone()).into(),
        ("Author".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_document_property(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "DocumentProperty")?;
    assert_string(&result_json, "DocumentProperty.Name", "Author".to_owned())?;
    assert_string(&result_json, "DocumentProperty.Value", "".to_owned())?;
    Ok(())
}

/// A test for GetDocumentProperty online.
#[tokio::test]
async fn document_properties_get_document_property_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetDocumentPropertyOnlineRequest::new(
        (request_document).into(),
        ("Author".to_owned()).into(),
    );

    context.api().get_document_property_online(request).await?;
    Ok(())
}

/// Test for deleting document property.
#[tokio::test]
async fn document_properties_delete_document_property() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProperties";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteDocumentProperty.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteDocumentPropertyRequest::new(
        (remote_file_name.clone()).into(),
        ("testProp".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_document_property(request).await?;
    Ok(())
}

/// Test for deleting document property online.
#[tokio::test]
async fn document_properties_delete_document_property_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteDocumentPropertyOnlineRequest::new(
        (request_document).into(),
        ("testProp".to_owned()).into(),
    );

    context
        .api()
        .delete_document_property_online(request)
        .await?;
    Ok(())
}

/// Test for updating document property.
#[tokio::test]
async fn document_properties_update_document_property() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProperties";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateDocumentProperty.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_property = DocumentPropertyCreateOrUpdate::default();
    request_property.value = Some(("Imran Anwar".to_owned()).into());

    let request = CreateOrUpdateDocumentPropertyRequest::new(
        (remote_file_name.clone()).into(),
        ("AsposeAuthor".to_owned()).into(),
        (request_property).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context
        .api()
        .create_or_update_document_property(request)
        .await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "DocumentProperty")?;
    assert_string(
        &result_json,
        "DocumentProperty.Name",
        "AsposeAuthor".to_owned(),
    )?;
    assert_string(
        &result_json,
        "DocumentProperty.Value",
        "Imran Anwar".to_owned(),
    )?;
    Ok(())
}

/// Test for updating document property online.
#[tokio::test]
async fn document_properties_update_document_property_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_property = DocumentPropertyCreateOrUpdate::default();
    request_property.value = Some(("Imran Anwar".to_owned()).into());

    let request = CreateOrUpdateDocumentPropertyOnlineRequest::new(
        (request_document).into(),
        ("AsposeAuthor".to_owned()).into(),
        (request_property).into(),
    );

    context
        .api()
        .create_or_update_document_property_online(request)
        .await?;
    Ok(())
}
