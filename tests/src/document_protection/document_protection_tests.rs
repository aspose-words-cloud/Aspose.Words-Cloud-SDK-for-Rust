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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProtection")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestProtectDocument.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestProtectionRequest = ProtectionRequestV2::default();
    requestProtectionRequest.r#protection_password = Some((test_string!("123")?).into());
    requestProtectionRequest.r#protection_type = Some((ProtectionRequestV2_ProtectionTypeEnum::ReadOnly).into());

    let request = ProtectDocumentRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestProtectionRequest).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().protect_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ProtectionData")?;

    Ok(())
}

/// Test for setting document protection.
#[tokio::test]
async fn document_protection_protect_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestProtectionRequest = ProtectionRequestV2::default();
    requestProtectionRequest.r#protection_password = Some((test_string!("123")?).into());
    requestProtectionRequest.r#protection_type = Some((ProtectionRequestV2_ProtectionTypeEnum::ReadOnly).into());

    let request = ProtectDocumentOnlineRequest::new(
        (requestDocument).into(),
        (requestProtectionRequest).into()
    );

    context.api().protect_document_online(request).await?;
    Ok(())
}

/// Test for getting document protection.
#[tokio::test]
async fn document_protection_get_document_protection() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProtection")?;
    let localFilePath = test_string!("DocumentActions/DocumentProtection/SampleProtectedBlankWordDocument.docx")?;
    let remoteFileName = test_string!("TestGetDocumentProtection.docx")?;

    context.upload_file(test_string!(localFilePath)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentProtectionRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_protection(request).await?;
    Ok(())
}

/// Test for getting document protection.
#[tokio::test]
async fn document_protection_get_document_protection_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetDocumentProtectionOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().get_document_protection_online(request).await?;
    Ok(())
}

/// Test for deleting unprotect document.
#[tokio::test]
async fn document_protection_delete_unprotect_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProtection")?;
    let localFilePath = test_string!("DocumentActions/DocumentProtection/SampleProtectedBlankWordDocument.docx")?;
    let remoteFileName = test_string!("TestDeleteUnprotectDocument.docx")?;

    context.upload_file(test_string!(localFilePath)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = UnprotectDocumentRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().unprotect_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "ProtectionData")?;

    Ok(())
}

/// Test for deleting unprotect document.
#[tokio::test]
async fn document_protection_delete_unprotect_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFilePath = test_string!("DocumentActions/DocumentProtection/SampleProtectedBlankWordDocument.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFilePath)?).await?;

    let request = UnprotectDocumentOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().unprotect_document_online(request).await?;
    Ok(())
}