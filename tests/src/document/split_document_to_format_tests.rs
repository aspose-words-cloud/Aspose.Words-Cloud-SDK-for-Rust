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

/// Test for document splitting.
#[tokio::test]
async fn split_document_to_format_split_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentActions/SplitDocument";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestSplitDocument.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = SplitDocumentRequest::new(
        (remote_file_name.clone()).into(),
        ("text".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/TestSplitDocument.text").into())
    .with_from((1).into())
    .with_to((2).into());

    let result = context.api().split_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SplitResult")?;
    assert_not_null(&result_json, "SplitResult.Pages")?;
    assert_length(&result_json, "SplitResult.Pages", 2)?;
    Ok(())
}

/// Test for document splitting job.
#[tokio::test]
async fn split_document_to_format_split_document_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentActions/SplitDocument";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestSplitDocument.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = SplitDocumentJobRequest::new(
        (remote_file_name.clone()).into(),
        ("text".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/TestSplitDocument.text").into())
    .with_from((1).into())
    .with_to((2).into());

    let job_handler = context.api().split_document_job(request).await?;
    let result = job_handler.wait_result(Duration::from_secs(3)).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SplitResult")?;
    assert_not_null(&result_json, "SplitResult.Pages")?;
    assert_length(&result_json, "SplitResult.Pages", 2)?;
    Ok(())
}

/// Test for document splitting online.
#[tokio::test]
async fn split_document_to_format_split_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        SplitDocumentOnlineRequest::new((request_document).into(), ("text".to_owned()).into())
            .with_dest_file_name((base_test_out_path.clone() + "/TestSplitDocument.text").into())
            .with_from((1).into())
            .with_to((2).into());

    context.api().split_document_online(request).await?;
    Ok(())
}

/// Test for document splitting online job.
#[tokio::test]
async fn split_document_to_format_split_document_online_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request =
        SplitDocumentOnlineJobRequest::new((request_document).into(), ("text".to_owned()).into())
            .with_dest_file_name((base_test_out_path.clone() + "/TestSplitDocument.text").into())
            .with_from((1).into())
            .with_to((2).into());

    let job_handler = context.api().split_document_online_job(request).await?;
    job_handler.wait_result(Duration::from_secs(3)).await?;
    Ok(())
}
