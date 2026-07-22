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

/// Test for getting all document signatures.
#[tokio::test]
async fn signature_get_signatures() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Signature")?;
    let localFolder = test_string!("DocumentActions/Signature")?;
    let signedDocument = test_string!("signedDocument.docx")?;
    let remoteName = test_string!("TestGetSignatures.docx")?;

    context.upload_file(test_string!(localFolder + "/" + signedDocument)?, test_string!(remoteFolder + "/" + remoteName)?).await?;

    let request = GetSignaturesRequest::new(
        (test_string!(remoteName)?).into()
    ).with_folder((test_string!(remoteFolder)?).into());

    let result = context.api().get_signatures(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Signatures")?;
    assert_length(&result_json, "Signatures", 1)?;
    Ok(())
}

/// Test for getting all document signatures online.
#[tokio::test]
async fn signature_get_signatures_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFolder = test_string!("DocumentActions/Signature")?;
    let signedDocument = test_string!("signedDocument.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFolder + "/" + signedDocument)?).await?;

    let request = GetSignaturesOnlineRequest::new(
        (requestDocument).into()
    );

    let result = context.api().get_signatures_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Signatures")?;
    assert_length(&result_json, "Signatures", 1)?;
    Ok(())
}

/// Test for removing all document signatures.
#[tokio::test]
async fn signature_remove_all_signatures() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Signature")?;
    let localFolder = test_string!("DocumentActions/Signature")?;
    let signedDocument = test_string!("signedDocument.docx")?;
    let remoteName = test_string!("TestRemoveAllSignatures.docx")?;

    context.upload_file(test_string!(localFolder + "/" + signedDocument)?, test_string!(remoteFolder + "/" + remoteName)?).await?;

    let request = RemoveAllSignaturesRequest::new(
        (test_string!(remoteName)?).into()
    ).with_folder((test_string!(remoteFolder)?).into());

    let result = context.api().remove_all_signatures(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Signatures")?;
    assert_length(&result_json, "Signatures", 0)?;
    Ok(())
}

/// Test for removing all document signatures online.
#[tokio::test]
async fn signature_remove_all_signatures_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFolder = test_string!("DocumentActions/Signature")?;
    let signedDocument = test_string!("signedDocument.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFolder + "/" + signedDocument)?).await?;

    let request = RemoveAllSignaturesOnlineRequest::new(
        (requestDocument).into()
    );

    let result = context.api().remove_all_signatures_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.Signatures")?;
    assert_length(&result_json, "Model.Signatures", 0)?;
    Ok(())
}

/// Test for signing document.
#[tokio::test]
async fn signature_sign_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Signature")?;
    let localFolder = test_string!("DocumentActions/Signature")?;
    let unsignedDocument = test_string!("unsignedDocument.docx")?;
    let certificateName = test_string!("morzal.pfx")?;
    let certificatePassword = test_string!("aw")?;
    let remoteName = test_string!("TestSignDocument.docx")?;
    let remoteCertificateName = test_string!("TestCertificate.pfx")?;

    context.upload_file(test_string!(localFolder + "/" + unsignedDocument)?, test_string!(remoteFolder + "/" + remoteName)?).await?;
    context.upload_file(test_string!(localFolder + "/" + certificateName)?, test_string!(remoteFolder + "/" + remoteCertificateName)?).await?;

    let request = SignDocumentRequest::new(
        (test_string!(remoteName)?).into(),
        (test_string!(remoteFolder + "/" + remoteCertificateName)?).into(),
        (test_string!(certificatePassword)?).into()
    ).with_folder((test_string!(remoteFolder)?).into());

    let result = context.api().sign_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Signatures")?;
    assert_length(&result_json, "Signatures", 1)?;
    Ok(())
}

/// Test for signing document online.
#[tokio::test]
async fn signature_sign_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/Signature")?;
    let localFolder = test_string!("DocumentActions/Signature")?;
    let unsignedDocument = test_string!("unsignedDocument.docx")?;
    let certificateName = test_string!("morzal.pfx")?;
    let certificatePassword = test_string!("aw")?;
    let remoteCertificateName = test_string!("TestCertificateOnline.pfx")?;

    context.upload_file(test_string!(localFolder + "/" + certificateName)?, test_string!(remoteFolder + "/" + remoteCertificateName)?).await?;
    let requestDocument = context.load_binary_file(test_string!(localFolder + "/" + unsignedDocument)?).await?;

    let request = SignDocumentOnlineRequest::new(
        (requestDocument).into(),
        (test_string!(remoteFolder + "/" + remoteCertificateName)?).into(),
        (test_string!(certificatePassword)?).into()
    );

    let result = context.api().sign_document_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.Signatures")?;
    assert_length(&result_json, "Model.Signatures", 1)?;
    Ok(())
}