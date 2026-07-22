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

/// Test for getting the text from range.
#[tokio::test]
async fn range_get_range_text() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Range")?;
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;
    let remoteFileName = test_string!("TestGetRangeText.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetRangeTextRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("id0.0.0")?).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_range_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_string(&result_json, "Text", test_string!("This is HEADER ")?)?;
    Ok(())
}

/// Test for getting the text from range online.
#[tokio::test]
async fn range_get_range_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetRangeTextOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("id0.0.0")?).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into());

    context.api().get_range_text_online(request).await?;
    Ok(())
}

/// Test for removing the text for range.
#[tokio::test]
async fn range_remove_range() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Range")?;
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;
    let remoteFileName = test_string!("TestRemoveRange.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RemoveRangeRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("id0.0.0")?).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().remove_range(request).await?;
    Ok(())
}

/// Test for removing the text for range online.
#[tokio::test]
async fn range_remove_range_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = RemoveRangeOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("id0.0.0")?).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into());

    context.api().remove_range_online(request).await?;
    Ok(())
}

/// Test for saving a range as a new document.
#[tokio::test]
async fn range_save_as_range() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Range")?;
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;
    let remoteFileName = test_string!("TestSaveAsRange.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestDocumentParameters = RangeDocument::default();
    requestDocumentParameters.r#document_name = Some((test_string!(remoteDataFolder + "/NewDoc.docx")?).into());

    let request = SaveAsRangeRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("id0.0.0")?).into(),
        (requestDocumentParameters).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().save_as_range(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("NewDoc.docx")?)?;
    Ok(())
}

/// Test for saving a range as a new document online.
#[tokio::test]
async fn range_save_as_range_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Range")?;
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestDocumentParameters = RangeDocument::default();
    requestDocumentParameters.r#document_name = Some((test_string!(remoteDataFolder + "/NewDoc.docx")?).into());

    let request = SaveAsRangeOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("id0.0.0")?).into(),
        (requestDocumentParameters).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into());

    context.api().save_as_range_online(request).await?;
    Ok(())
}

/// Test for replacing text in range.
#[tokio::test]
async fn range_replace_with_text() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Range")?;
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;
    let remoteFileName = test_string!("TestReplaceWithText.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestRangeText = ReplaceRange::default();
    requestRangeText.r#text = Some((test_string!("Replaced header")?).into());

    let request = ReplaceWithTextRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("id0.0.0")?).into(),
        (requestRangeText).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().replace_with_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", test_string!("TestReplaceWithText.docx")?)?;
    Ok(())
}

/// Test for replacing text in range online.
#[tokio::test]
async fn range_replace_with_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestRangeText = ReplaceRange::default();
    requestRangeText.r#text = Some((test_string!("Replaced header")?).into());

    let request = ReplaceWithTextOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("id0.0.0")?).into(),
        (requestRangeText).into()
    ).with_range_end_identifier((test_string!("id0.0.1")?).into());

    context.api().replace_with_text_online(request).await?;
    Ok(())
}

/// Test to translate node id to node path.
#[tokio::test]
async fn range_translate_node_id() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Range")?;
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;
    let remoteFileName = test_string!("TestTranslateNodeId.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = TranslateNodeIdRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("id0.0.0")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().translate_node_id(request).await?;
    let result_json = serialize_result(&result)?;
    assert_string(&result_json, "Path", test_string!("sections/0/body/paragraphs/0")?)?;
    Ok(())
}

/// Test to translate node id to node path online.
#[tokio::test]
async fn range_translate_node_id_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Range/RangeGet.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = TranslateNodeIdOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("id0.0.0")?).into()
    );

    context.api().translate_node_id_online(request).await?;
    Ok(())
}