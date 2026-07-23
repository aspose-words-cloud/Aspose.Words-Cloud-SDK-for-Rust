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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Signature";
    let local_folder = "DocumentActions/Signature".to_owned();
    let signed_document = "signedDocument.docx".to_owned();
    let remote_name = "TestGetSignatures.docx".to_owned();

    context
        .upload_file(
            local_folder.clone() + "/" + &signed_document,
            remote_folder.clone() + "/" + &remote_name,
        )
        .await?;

    let request = GetSignaturesRequest::new((remote_name.clone()).into())
        .with_folder((remote_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_folder = "DocumentActions/Signature".to_owned();
    let signed_document = "signedDocument.docx".to_owned();

    let request_document = context
        .load_binary_file(local_folder.clone() + "/" + &signed_document)
        .await?;

    let request = GetSignaturesOnlineRequest::new((request_document).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Signature";
    let local_folder = "DocumentActions/Signature".to_owned();
    let signed_document = "signedDocument.docx".to_owned();
    let remote_name = "TestRemoveAllSignatures.docx".to_owned();

    context
        .upload_file(
            local_folder.clone() + "/" + &signed_document,
            remote_folder.clone() + "/" + &remote_name,
        )
        .await?;

    let request = RemoveAllSignaturesRequest::new((remote_name.clone()).into())
        .with_folder((remote_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_folder = "DocumentActions/Signature".to_owned();
    let signed_document = "signedDocument.docx".to_owned();

    let request_document = context
        .load_binary_file(local_folder.clone() + "/" + &signed_document)
        .await?;

    let request = RemoveAllSignaturesOnlineRequest::new((request_document).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Signature";
    let local_folder = "DocumentActions/Signature".to_owned();
    let unsigned_document = "unsignedDocument.docx".to_owned();
    let certificate_name = "morzal.pfx".to_owned();
    let certificate_password = "aw".to_owned();
    let remote_name = "TestSignDocument.docx".to_owned();
    let remote_certificate_name = "TestCertificate.pfx".to_owned();

    context
        .upload_file(
            local_folder.clone() + "/" + &unsigned_document,
            remote_folder.clone() + "/" + &remote_name,
        )
        .await?;
    context
        .upload_file(
            local_folder.clone() + "/" + &certificate_name,
            remote_folder.clone() + "/" + &remote_certificate_name,
        )
        .await?;

    let request = SignDocumentRequest::new(
        (remote_name.clone()).into(),
        (remote_folder.clone() + "/" + &remote_certificate_name).into(),
        (certificate_password.clone()).into(),
    )
    .with_folder((remote_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/Signature";
    let local_folder = "DocumentActions/Signature".to_owned();
    let unsigned_document = "unsignedDocument.docx".to_owned();
    let certificate_name = "morzal.pfx".to_owned();
    let certificate_password = "aw".to_owned();
    let remote_certificate_name = "TestCertificateOnline.pfx".to_owned();

    context
        .upload_file(
            local_folder.clone() + "/" + &certificate_name,
            remote_folder.clone() + "/" + &remote_certificate_name,
        )
        .await?;
    let request_document = context
        .load_binary_file(local_folder.clone() + "/" + &unsigned_document)
        .await?;

    let request = SignDocumentOnlineRequest::new(
        (request_document).into(),
        (remote_folder.clone() + "/" + &remote_certificate_name).into(),
        (certificate_password.clone()).into(),
    );

    let result = context.api().sign_document_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.Signatures")?;
    assert_length(&result_json, "Model.Signatures", 1)?;
    Ok(())
}
