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

/// Test for document comparison.
#[tokio::test]
async fn compare_document_compare_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/CompareDocument";
    let local_folder = "DocumentActions/CompareDocument".to_owned();
    let local_name1 = "compareTestDoc1.doc".to_owned();
    let local_name2 = "compareTestDoc2.doc".to_owned();
    let remote_name1 = "TestCompareDocument1.doc".to_owned();
    let remote_name2 = "TestCompareDocument2.doc".to_owned();

    context.upload_file(local_folder.clone() + "/" + &local_name1, remote_folder.clone() + "/" + &remote_name1).await?;
    context.upload_file(local_folder.clone() + "/" + &local_name2, remote_folder.clone() + "/" + &remote_name2).await?;
    let request_compare_data_file_reference = FileReference::remote(
    remote_folder.clone() + "/" + &remote_name2,
    None,
    );
    let request_compare_data_date_time = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut request_compare_data = CompareData::default();
    request_compare_data.author = Some(("author".to_owned()).into());
    request_compare_data.date_time = Some((request_compare_data_date_time).into());
    request_compare_data.file_reference = Some((request_compare_data_file_reference).into());

    let request = CompareDocumentRequest::new(
        (remote_name1.clone()).into(),
        (request_compare_data).into()
    ).with_folder((remote_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/TestCompareDocumentOut.doc").into());

    let result = context.api().compare_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestCompareDocumentOut.doc".to_owned())?;
    Ok(())
}

/// Test for document comparison online.
#[tokio::test]
async fn compare_document_compare_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/CompareDocument";
    let local_folder = "DocumentActions/CompareDocument".to_owned();
    let local_name1 = "compareTestDoc1.doc".to_owned();
    let local_name2 = "compareTestDoc2.doc".to_owned();
    let remote_name2 = "TestCompareDocument2.doc".to_owned();

    context.upload_file(local_folder.clone() + "/" + &local_name2, remote_folder.clone() + "/" + &remote_name2).await?;
    let request_document = context.load_binary_file(local_folder.clone() + "/" + &local_name1).await?;
    let request_compare_data_file_reference = FileReference::remote(
    remote_folder.clone() + "/" + &remote_name2,
    None,
    );
    let request_compare_data_date_time = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut request_compare_data = CompareData::default();
    request_compare_data.author = Some(("author".to_owned()).into());
    request_compare_data.date_time = Some((request_compare_data_date_time).into());
    request_compare_data.file_reference = Some((request_compare_data_file_reference).into());

    let request = CompareDocumentOnlineRequest::new(
        (request_document).into(),
        (request_compare_data).into()
    ).with_dest_file_name((base_test_out_path.clone() + "/TestCompareDocumentOut.doc").into());

    context.api().compare_document_online(request).await?;
    Ok(())
}

/// Test for document comparison online.
#[tokio::test]
async fn compare_document_compare_two_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/CompareDocument";
    let local_folder = "DocumentActions/CompareDocument".to_owned();
    let local_name1 = "compareTestDoc1.doc".to_owned();
    let local_name2 = "compareTestDoc2.doc".to_owned();
    let remote_name2 = "TestCompareDocument2.doc".to_owned();

    context.upload_file(local_folder.clone() + "/" + &local_name2, remote_folder.clone() + "/" + &remote_name2).await?;
    let request_document = context.load_binary_file(local_folder.clone() + "/" + &local_name1).await?;
    let request_compare_data_file_reference_content = context.load_binary_file(local_folder.clone() + "/" + &local_name2).await?;
    let request_compare_data_file_reference = FileReference::local(
    request_compare_data_file_reference_content,
    None,
    );
    let request_compare_data_date_time = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut request_compare_data = CompareData::default();
    request_compare_data.author = Some(("author".to_owned()).into());
    request_compare_data.date_time = Some((request_compare_data_date_time).into());
    request_compare_data.file_reference = Some((request_compare_data_file_reference).into());

    let request = CompareDocumentOnlineRequest::new(
        (request_document).into(),
        (request_compare_data).into()
    ).with_dest_file_name((base_test_out_path.clone() + "/TestCompareDocumentOut.doc").into());

    context.api().compare_document_online(request).await?;
    Ok(())
}

/// Test for document comparison with password protection.
#[tokio::test]
async fn compare_document_compare_document_with_password() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/CompareDocument";
    let local_name = "DocWithPassword.docx".to_owned();
    let remote_name1 = "TestCompareDocument1.docx".to_owned();
    let remote_name2 = "TestCompareDocument2.docx".to_owned();

    context.upload_file("Common/".to_owned() + &local_name, remote_folder.clone() + "/" + &remote_name1).await?;
    context.upload_file("Common/".to_owned() + &local_name, remote_folder.clone() + "/" + &remote_name2).await?;
    let request_compare_data_file_reference = FileReference::remote(
    remote_folder.clone() + "/" + &remote_name2,
    Some("12345".to_owned()),
    );
    let request_compare_data_date_time = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut request_compare_data = CompareData::default();
    request_compare_data.author = Some(("author".to_owned()).into());
    request_compare_data.date_time = Some((request_compare_data_date_time).into());
    request_compare_data.file_reference = Some((request_compare_data_file_reference).into());

    let request = CompareDocumentRequest::new(
        (remote_name1.clone()).into(),
        (request_compare_data).into()
    ).with_folder((remote_folder.clone()).into())
.with_password(("12345".to_owned()).into())
.with_dest_file_name((base_test_out_path.clone() + "/TestCompareDocumentOut.docx").into());

    let result = context.api().compare_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestCompareDocumentOut.docx".to_owned())?;
    Ok(())
}