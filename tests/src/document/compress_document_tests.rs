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

/// Test for compression document.
#[tokio::test]
async fn compress_document_compress_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteFolder = test_string!(remoteBaseTestDataFolder + "/DocumentActions/CompressDocument")?;
    let localFolder = test_string!("DocumentActions/CompressDocument")?;
    let localName = test_string!("TestCompress.docx")?;
    let remoteName = test_string!("TestCompress.docx")?;

    context.upload_file(test_string!(localFolder + "/" + localName)?, test_string!(remoteFolder + "/" + remoteName)?).await?;
    let mut requestCompressOptions = CompressOptions::default();


    let request = CompressDocumentRequest::new(
        (test_string!(remoteName)?).into(),
        (requestCompressOptions).into()
    ).with_folder((test_string!(remoteFolder)?).into());

    let result = context.api().compress_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    Ok(())
}

/// Test for compression document online.
#[tokio::test]
async fn compress_document_compress_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFolder = test_string!("DocumentActions/CompressDocument")?;
    let localName = test_string!("TestCompress.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFolder + "/" + localName)?).await?;
    let mut requestCompressOptions = CompressOptions::default();


    let request = CompressDocumentOnlineRequest::new(
        (requestDocument).into(),
        (requestCompressOptions).into()
    );

    context.api().compress_document_online(request).await?;
    Ok(())
}