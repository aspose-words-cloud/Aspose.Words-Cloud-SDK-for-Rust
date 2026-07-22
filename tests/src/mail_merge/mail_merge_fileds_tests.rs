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

/// Test for putting new fields.
#[tokio::test]
async fn mail_merge_fileds_get_document_field_names_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let mailMergeFolder = test_string!("DocumentActions/MailMerge")?;
    let localDocumentFile = test_string!("SampleExecuteTemplate.docx")?;

    let requestTemplate = context.load_binary_file(test_string!(mailMergeFolder + "/" + localDocumentFile)?).await?;

    let request = GetDocumentFieldNamesOnlineRequest::new(
        (requestTemplate).into()
    ).with_use_non_merge_fields((true).into());

    let result = context.api().get_document_field_names_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FieldNames")?;
    assert_not_null(&result_json, "FieldNames.Names")?;
    assert_length(&result_json, "FieldNames.Names", 15)?;
    assert_string(&result_json, "FieldNames.Names[0]", test_string!("TableStart:Order")?)?;
    Ok(())
}

/// Test for getting mailmerge fields.
#[tokio::test]
async fn mail_merge_fileds_get_document_field_names() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/MailMerge")?;
    let remoteFileName = test_string!("TestGetDocumentFieldNames.docx")?;

    context.upload_file(test_string!("Common/test_multi_pages.docx")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentFieldNamesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_document_field_names(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "FieldNames")?;
    assert_not_null(&result_json, "FieldNames.Names")?;
    assert_length(&result_json, "FieldNames.Names", 0)?;
    Ok(())
}