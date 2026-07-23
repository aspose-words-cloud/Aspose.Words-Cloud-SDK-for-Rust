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

/// Test for setting document protection.
#[tokio::test]
async fn document_protection_protect_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProtection";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestProtectDocument.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_protection_request = ProtectionRequestV2::default();
    request_protection_request.protection_password = Some(("123".to_owned()).into());
    request_protection_request.protection_type =
        Some((ProtectionRequestV2ProtectionTypeEnum::ReadOnly).into());

    let request = ProtectDocumentRequest::new(
        (remote_file_name.clone()).into(),
        (request_protection_request).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().protect_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ProtectionData")?;

    Ok(())
}

/// Test for setting document protection.
#[tokio::test]
async fn document_protection_protect_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_protection_request = ProtectionRequestV2::default();
    request_protection_request.protection_password = Some(("123".to_owned()).into());
    request_protection_request.protection_type =
        Some((ProtectionRequestV2ProtectionTypeEnum::ReadOnly).into());

    let request = ProtectDocumentOnlineRequest::new(
        (request_document).into(),
        (request_protection_request).into(),
    );

    context.api().protect_document_online(request).await?;
    Ok(())
}

/// Test for getting document protection.
#[tokio::test]
async fn document_protection_get_document_protection() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProtection";
    let local_file_path =
        "DocumentActions/DocumentProtection/SampleProtectedBlankWordDocument.docx".to_owned();
    let remote_file_name = "TestGetDocumentProtection.docx".to_owned();

    context
        .upload_file(
            local_file_path.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetDocumentProtectionRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().get_document_protection(request).await?;
    Ok(())
}

/// Test for getting document protection.
#[tokio::test]
async fn document_protection_get_document_protection_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetDocumentProtectionOnlineRequest::new((request_document).into());

    context
        .api()
        .get_document_protection_online(request)
        .await?;
    Ok(())
}

/// Test for deleting unprotect document.
#[tokio::test]
async fn document_protection_delete_unprotect_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentElements/DocumentProtection";
    let local_file_path =
        "DocumentActions/DocumentProtection/SampleProtectedBlankWordDocument.docx".to_owned();
    let remote_file_name = "TestDeleteUnprotectDocument.docx".to_owned();

    context
        .upload_file(
            local_file_path.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = UnprotectDocumentRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().unprotect_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ProtectionData")?;

    Ok(())
}

/// Test for deleting unprotect document.
#[tokio::test]
async fn document_protection_delete_unprotect_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file_path =
        "DocumentActions/DocumentProtection/SampleProtectedBlankWordDocument.docx".to_owned();

    let request_document = context.load_binary_file(local_file_path.clone()).await?;

    let request = UnprotectDocumentOnlineRequest::new((request_document).into());

    context.api().unprotect_document_online(request).await?;
    Ok(())
}
