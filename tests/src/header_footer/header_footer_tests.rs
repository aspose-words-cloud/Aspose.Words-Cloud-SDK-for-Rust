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

/// Test for getting headers and footers.
#[tokio::test]
async fn header_footer_get_header_footers() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/HeaderFooters")?;
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;
    let remoteFileName = test_string!("TestGetHeadersFooters.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetHeaderFootersRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_header_footers(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "HeaderFooters")?;
    assert_not_null(&result_json, "HeaderFooters.List")?;
    assert_length(&result_json, "HeaderFooters.List", 6)?;
    Ok(())
}

/// Test for getting headers and footers online.
#[tokio::test]
async fn header_footer_get_header_footers_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetHeaderFootersOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("")?).into()
    );

    context.api().get_header_footers_online(request).await?;
    Ok(())
}

/// Test for getting headerfooter.
#[tokio::test]
async fn header_footer_get_header_footer() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/HeaderFooters")?;
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;
    let remoteFileName = test_string!("TestGetHeaderFooter.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetHeaderFooterRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_header_footer(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "HeaderFooter")?;
    assert_not_null(&result_json, "HeaderFooter.ChildNodes")?;
    assert_length(&result_json, "HeaderFooter.ChildNodes", 1)?;
    assert_string(&result_json, "HeaderFooter.ChildNodes[0].NodeId", test_string!("0.0.0")?)?;
    Ok(())
}

/// Test for getting headerfooter online.
#[tokio::test]
async fn header_footer_get_header_footer_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetHeaderFooterOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    );

    context.api().get_header_footer_online(request).await?;
    Ok(())
}

/// Test for getting headerfooter of section.
#[tokio::test]
async fn header_footer_get_header_footer_of_section() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/HeaderFooters")?;
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;
    let remoteFileName = test_string!("TestGetHeaderFooterOfSection.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetHeaderFooterOfSectionRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_header_footer_of_section(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "HeaderFooter")?;
    assert_not_null(&result_json, "HeaderFooter.ChildNodes")?;
    assert_length(&result_json, "HeaderFooter.ChildNodes", 1)?;
    assert_string(&result_json, "HeaderFooter.ChildNodes[0].NodeId", test_string!("0.0.0")?)?;
    Ok(())
}

/// Test for getting headerfooter of section online.
#[tokio::test]
async fn header_footer_get_header_footer_of_section_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetHeaderFooterOfSectionOnlineRequest::new(
        (requestDocument).into(),
        (0).into(),
        (0).into()
    );

    context.api().get_header_footer_of_section_online(request).await?;
    Ok(())
}

/// Test for deleting headerfooter.
#[tokio::test]
async fn header_footer_delete_header_footer() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/HeaderFooters")?;
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;
    let remoteFileName = test_string!("TestDeleteHeaderFooter.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteHeaderFooterRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_header_footer(request).await?;
    Ok(())
}

/// Test for deleting headerfooter online.
#[tokio::test]
async fn header_footer_delete_header_footer_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteHeaderFooterOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("")?).into(),
        (0).into()
    );

    context.api().delete_header_footer_online(request).await?;
    Ok(())
}

/// Test for deleting headerfooters.
#[tokio::test]
async fn header_footer_delete_headers_footers() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/HeaderFooters")?;
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;
    let remoteFileName = test_string!("TestDeleteHeadersFooters.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteHeadersFootersRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_headers_footers(request).await?;
    Ok(())
}

/// Test for deleting headerfooters online.
#[tokio::test]
async fn header_footer_delete_headers_footers_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteHeadersFootersOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("")?).into()
    );

    context.api().delete_headers_footers_online(request).await?;
    Ok(())
}

/// Test for adding headerfooters.
#[tokio::test]
async fn header_footer_insert_header_footer() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/HeaderFooters")?;
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;
    let remoteFileName = test_string!("TestInsertHeaderFooter.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = InsertHeaderFooterRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("")?).into(),
        (test_string!("FooterEven")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().insert_header_footer(request).await?;
    Ok(())
}

/// Test for adding headerfooters online.
#[tokio::test]
async fn header_footer_insert_header_footer_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/HeaderFooters/HeadersFooters.doc")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = InsertHeaderFooterOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("")?).into(),
        (test_string!("FooterEven")?).into()
    );

    let result = context.api().insert_header_footer_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.HeaderFooter")?;
    assert_not_null(&result_json, "Model.HeaderFooter.ChildNodes")?;
    assert_length(&result_json, "Model.HeaderFooter.ChildNodes", 1)?;
    assert_string(&result_json, "Model.HeaderFooter.ChildNodes[0].NodeId", test_string!("0.2.0")?)?;
    Ok(())
}