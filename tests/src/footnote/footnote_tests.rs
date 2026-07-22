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

/// Test for adding footnote.
#[tokio::test]
async fn footnote_insert_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestInsertFootnote.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFootnoteDto = FootnoteInsert::default();
    requestFootnoteDto.r#footnote_type = Some((FootnoteBase_FootnoteTypeEnum::Endnote).into());
    requestFootnoteDto.r#text = Some((test_string!("test endnote")?).into());

    let request = InsertFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestFootnoteDto).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.NodeId", test_string!("0.1.7.1")?)?;
    assert_string(&result_json, "Footnote.Text", test_string!(" test endnote" + "\r\n")?)?;
    Ok(())
}

/// Test for adding footnote online.
#[tokio::test]
async fn footnote_insert_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;

    let requestDocument = context.load_binary_file(test_string!(footnoteFolder + "/Footnote.doc")?).await?;
    let mut requestFootnoteDto = FootnoteInsert::default();
    requestFootnoteDto.r#footnote_type = Some((FootnoteBase_FootnoteTypeEnum::Endnote).into());
    requestFootnoteDto.r#text = Some((test_string!("test endnote")?).into());

    let request = InsertFootnoteOnlineRequest::new(
        (requestDocument).into(),
        (requestFootnoteDto).into()
    ).with_node_path((test_string!("")?).into());

    context.api().insert_footnote_online(request).await?;
    Ok(())
}

/// Test for adding footnote without node path.
#[tokio::test]
async fn footnote_insert_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestInsertFootnoteWithoutNodePath.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFootnoteDto = FootnoteInsert::default();
    requestFootnoteDto.r#footnote_type = Some((FootnoteBase_FootnoteTypeEnum::Endnote).into());
    requestFootnoteDto.r#text = Some((test_string!("test endnote")?).into());

    let request = InsertFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestFootnoteDto).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.NodeId", test_string!("0.1.7.1")?)?;
    assert_string(&result_json, "Footnote.Text", test_string!(" test endnote" + "\r\n")?)?;
    Ok(())
}

/// Test for deleting footnote.
#[tokio::test]
async fn footnote_delete_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestDeleteFootnote.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_footnote(request).await?;
    Ok(())
}

/// Test for deleting footnote online.
#[tokio::test]
async fn footnote_delete_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;

    let requestDocument = context.load_binary_file(test_string!(footnoteFolder + "/Footnote.doc")?).await?;

    let request = DeleteFootnoteOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_footnote_online(request).await?;
    Ok(())
}

/// Test for deleting footnote without node path.
#[tokio::test]
async fn footnote_delete_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestDeleteFootnoteWithoutNodePath.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_footnote(request).await?;
    Ok(())
}

/// Test for getting footnotes.
#[tokio::test]
async fn footnote_get_footnotes() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestGetFootnotes.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFootnotesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_footnotes(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnotes")?;
    assert_not_null(&result_json, "Footnotes.List")?;
    assert_length(&result_json, "Footnotes.List", 6)?;
    assert_string(&result_json, "Footnotes.List[0].Text", test_string!(" Footnote 1." + "\r\n")?)?;
    Ok(())
}

/// Test for getting footnotes online.
#[tokio::test]
async fn footnote_get_footnotes_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;

    let requestDocument = context.load_binary_file(test_string!(footnoteFolder + "/Footnote.doc")?).await?;

    let request = GetFootnotesOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_footnotes_online(request).await?;
    Ok(())
}

/// Test for getting footnotes without node path.
#[tokio::test]
async fn footnote_get_footnotes_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestGetFootnotesWithoutNodePath.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFootnotesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_footnotes(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnotes")?;
    assert_not_null(&result_json, "Footnotes.List")?;
    assert_length(&result_json, "Footnotes.List", 6)?;
    assert_string(&result_json, "Footnotes.List[0].Text", test_string!(" Footnote 1." + "\r\n")?)?;
    Ok(())
}

/// Test for getting footnote.
#[tokio::test]
async fn footnote_get_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestGetFootnote.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", test_string!(" Footnote 1." + "\r\n")?)?;
    Ok(())
}

/// Test for getting footnote online.
#[tokio::test]
async fn footnote_get_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;

    let requestDocument = context.load_binary_file(test_string!(footnoteFolder + "/Footnote.doc")?).await?;

    let request = GetFootnoteOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_footnote_online(request).await?;
    Ok(())
}

/// Test for getting footnote without node path.
#[tokio::test]
async fn footnote_get_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestGetFootnoteWithoutNodePath.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", test_string!(" Footnote 1." + "\r\n")?)?;
    Ok(())
}

/// Test for updating footnote.
#[tokio::test]
async fn footnote_update_footnote() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestUpdateFootnote.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFootnoteDto = FootnoteUpdate::default();
    requestFootnoteDto.r#text = Some((test_string!("new text is here")?).into());

    let request = UpdateFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestFootnoteDto).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", test_string!(" new text is here" + "\r\n")?)?;
    Ok(())
}

/// Test for updating footnote online.
#[tokio::test]
async fn footnote_update_footnote_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;

    let requestDocument = context.load_binary_file(test_string!(footnoteFolder + "/Footnote.doc")?).await?;
    let mut requestFootnoteDto = FootnoteUpdate::default();
    requestFootnoteDto.r#text = Some((test_string!("new text is here")?).into());

    let request = UpdateFootnoteOnlineRequest::new(
        (requestDocument).into(),
        (requestFootnoteDto).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().update_footnote_online(request).await?;
    Ok(())
}

/// Test for updating footnote without node path.
#[tokio::test]
async fn footnote_update_footnote_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Footnotes")?;
    let footnoteFolder = test_string!("DocumentElements/Footnotes")?;
    let remoteFileName = test_string!("TestUpdateFootnoteWithoutNodePath.docx")?;

    context.upload_file(test_string!(footnoteFolder + "/Footnote.doc")?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFootnoteDto = FootnoteUpdate::default();
    requestFootnoteDto.r#text = Some((test_string!("new text is here")?).into());

    let request = UpdateFootnoteRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestFootnoteDto).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_footnote(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Footnote")?;
    assert_string(&result_json, "Footnote.Text", test_string!(" new text is here" + "\r\n")?)?;
    Ok(())
}