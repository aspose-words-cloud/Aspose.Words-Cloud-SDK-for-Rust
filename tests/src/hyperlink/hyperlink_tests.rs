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

/// Test for getting hyperlink by specified index.
#[tokio::test]
async fn hyperlink_get_document_hyperlink_by_index() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Hyperlink";
    let local_file = "Common/test_doc.docx".to_owned();
    let remote_file_name = "TestGetDocumentHyperlinkByIndex.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetDocumentHyperlinkByIndexRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_document_hyperlink_by_index(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Hyperlink")?;
    assert_string(&result_json, "Hyperlink.DisplayText", "Aspose".to_owned())?;
    Ok(())
}

/// Test for getting hyperlink by specified index online.
#[tokio::test]
async fn hyperlink_get_document_hyperlink_by_index_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_doc.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetDocumentHyperlinkByIndexOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().get_document_hyperlink_by_index_online(request).await?;
    Ok(())
}

/// Test for getting hyperlinks.
#[tokio::test]
async fn hyperlink_get_document_hyperlinks() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Hyperlink";
    let local_file = "Common/test_doc.docx".to_owned();
    let remote_file_name = "TestGetDocumentHyperlinks.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetDocumentHyperlinksRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_document_hyperlinks(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Hyperlinks")?;
    assert_not_null(&result_json, "Hyperlinks.HyperlinkList")?;
    assert_length(&result_json, "Hyperlinks.HyperlinkList", 2)?;
    assert_string(&result_json, "Hyperlinks.HyperlinkList[0].DisplayText", "Aspose".to_owned())?;
    Ok(())
}

/// Test for getting hyperlinks online.
#[tokio::test]
async fn hyperlink_get_document_hyperlinks_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_doc.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetDocumentHyperlinksOnlineRequest::new(
        (request_document).into()
    );

    context.api().get_document_hyperlinks_online(request).await?;
    Ok(())
}