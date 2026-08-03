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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_folder = remote_base_test_data_folder.clone() + "/DocumentActions/CompressDocument";
    let local_folder = "DocumentActions/CompressDocument".to_owned();
    let local_name = "TestCompress.docx".to_owned();
    let remote_name = "TestCompress.docx".to_owned();

    context.upload_file(local_folder.clone() + "/" + &local_name, remote_folder.clone() + "/" + &remote_name).await?;
    let mut request_compress_options = CompressOptions::default();


    let request = CompressDocumentRequest::new(
        (remote_name.clone()).into(),
        (request_compress_options).into()
    ).with_folder((remote_folder.clone()).into());

    let result = context.api().compress_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    Ok(())
}

/// Test for compression document online.
#[tokio::test]
async fn compress_document_compress_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_folder = "DocumentActions/CompressDocument".to_owned();
    let local_name = "TestCompress.docx".to_owned();

    let request_document = context.load_binary_file(local_folder.clone() + "/" + &local_name).await?;
    let mut request_compress_options = CompressOptions::default();


    let request = CompressDocumentOnlineRequest::new(
        (request_document).into(),
        (request_compress_options).into()
    );

    context.api().compress_document_online(request).await?;
    Ok(())
}