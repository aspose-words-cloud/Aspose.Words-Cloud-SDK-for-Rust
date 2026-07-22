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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/CompareDocument")?;
    let localFolder = test_string!("DocumentActions/CompareDocument")?;
    let localName1 = test_string!("compareTestDoc1.doc")?;
    let localName2 = test_string!("compareTestDoc2.doc")?;
    let remoteName1 = test_string!("TestCompareDocument1.doc")?;
    let remoteName2 = test_string!("TestCompareDocument2.doc")?;

    context.upload_file(test_string!(localFolder + "/" + localName1)?, test_string!(remoteFolder + "/" + remoteName1)?).await?;
    context.upload_file(test_string!(localFolder + "/" + localName2)?, test_string!(remoteFolder + "/" + remoteName2)?).await?;
    let requestCompareDataFileReference = FileReference::remote(
    test_string!(remoteFolder + "/" + remoteName2)?,
    None,
    );
    let requestCompareDataDateTime = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut requestCompareData = CompareData::default();
    requestCompareData.r#author = Some((test_string!("author")?).into());
    requestCompareData.r#date_time = Some((requestCompareDataDateTime).into());
    requestCompareData.r#file_reference = Some((requestCompareDataFileReference).into());

    let request = CompareDocumentRequest::new(
        (test_string!(remoteName1)?).into(),
        (requestCompareData).into()
    ).with_folder((test_string!(remoteFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/TestCompareDocumentOut.doc")?).into());

    let result = context.api().compare_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestCompareDocumentOut.doc")?)?;
    Ok(())
}

/// Test for document comparison online.
#[tokio::test]
async fn compare_document_compare_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/CompareDocument")?;
    let localFolder = test_string!("DocumentActions/CompareDocument")?;
    let localName1 = test_string!("compareTestDoc1.doc")?;
    let localName2 = test_string!("compareTestDoc2.doc")?;
    let remoteName2 = test_string!("TestCompareDocument2.doc")?;

    context.upload_file(test_string!(localFolder + "/" + localName2)?, test_string!(remoteFolder + "/" + remoteName2)?).await?;
    let requestDocument = context.load_binary_file(test_string!(localFolder + "/" + localName1)?).await?;
    let requestCompareDataFileReference = FileReference::remote(
    test_string!(remoteFolder + "/" + remoteName2)?,
    None,
    );
    let requestCompareDataDateTime = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut requestCompareData = CompareData::default();
    requestCompareData.r#author = Some((test_string!("author")?).into());
    requestCompareData.r#date_time = Some((requestCompareDataDateTime).into());
    requestCompareData.r#file_reference = Some((requestCompareDataFileReference).into());

    let request = CompareDocumentOnlineRequest::new(
        (requestDocument).into(),
        (requestCompareData).into()
    ).with_dest_file_name((test_string!(baseTestOutPath + "/TestCompareDocumentOut.doc")?).into());

    context.api().compare_document_online(request).await?;
    Ok(())
}

/// Test for document comparison online.
#[tokio::test]
async fn compare_document_compare_two_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/CompareDocument")?;
    let localFolder = test_string!("DocumentActions/CompareDocument")?;
    let localName1 = test_string!("compareTestDoc1.doc")?;
    let localName2 = test_string!("compareTestDoc2.doc")?;
    let remoteName2 = test_string!("TestCompareDocument2.doc")?;

    context.upload_file(test_string!(localFolder + "/" + localName2)?, test_string!(remoteFolder + "/" + remoteName2)?).await?;
    let requestDocument = context.load_binary_file(test_string!(localFolder + "/" + localName1)?).await?;
    let requestCompareDataFileReferenceContent = context.load_binary_file(test_string!(localFolder + "/" + localName2)?).await?;
    let requestCompareDataFileReference = FileReference::local(
    requestCompareDataFileReferenceContent,
    None,
    );
    let requestCompareDataDateTime = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut requestCompareData = CompareData::default();
    requestCompareData.r#author = Some((test_string!("author")?).into());
    requestCompareData.r#date_time = Some((requestCompareDataDateTime).into());
    requestCompareData.r#file_reference = Some((requestCompareDataFileReference).into());

    let request = CompareDocumentOnlineRequest::new(
        (requestDocument).into(),
        (requestCompareData).into()
    ).with_dest_file_name((test_string!(baseTestOutPath + "/TestCompareDocumentOut.doc")?).into());

    context.api().compare_document_online(request).await?;
    Ok(())
}

/// Test for document comparison with password protection.
#[tokio::test]
async fn compare_document_compare_document_with_password() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/CompareDocument")?;
    let localName = test_string!("DocWithPassword.docx")?;
    let remoteName1 = test_string!("TestCompareDocument1.docx")?;
    let remoteName2 = test_string!("TestCompareDocument2.docx")?;

    context.upload_file(test_string!("Common/" + localName)?, test_string!(remoteFolder + "/" + remoteName1)?).await?;
    context.upload_file(test_string!("Common/" + localName)?, test_string!(remoteFolder + "/" + remoteName2)?).await?;
    let requestCompareDataFileReference = FileReference::remote(
    test_string!(remoteFolder + "/" + remoteName2)?,
    Some("12345".to_owned()),
    );
    let requestCompareDataDateTime = Utc.with_ymd_and_hms(
    2015, 10, 26,
    0, 0, 0,
    ).single().ok_or_else(|| TestError::Assertion("invalid generated date".to_owned()))?;
    let mut requestCompareData = CompareData::default();
    requestCompareData.r#author = Some((test_string!("author")?).into());
    requestCompareData.r#date_time = Some((requestCompareDataDateTime).into());
    requestCompareData.r#file_reference = Some((requestCompareDataFileReference).into());

    let request = CompareDocumentRequest::new(
        (test_string!(remoteName1)?).into(),
        (requestCompareData).into()
    ).with_folder((test_string!(remoteFolder)?).into())
.with_password((test_string!("12345")?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/TestCompareDocumentOut.docx")?).into());

    let result = context.api().compare_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestCompareDocumentOut.docx")?)?;
    Ok(())
}