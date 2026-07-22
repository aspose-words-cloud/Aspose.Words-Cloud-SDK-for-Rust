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

/// Test for getting document with specified format.
#[tokio::test]
async fn document_with_format_get_document_with_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/DocumentWithFormat")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentWithFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentWithFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("text")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_document_with_format(request).await?;
    Ok(())
}

/// Test for getting document with specified format.
#[tokio::test]
async fn document_with_format_get_document_with_format_and_out_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/DocumentWithFormat")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentWithFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentWithFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("text")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_out_path((test_string!(baseTestOutPath + "/TestGetDocumentWithFormatAndOutPath.text")?).into());

    context.api().get_document_with_format(request).await?;
    Ok(())
}