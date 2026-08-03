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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/HeaderFooters";
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();
    let remote_file_name = "TestGetHeadersFooters.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetHeaderFootersRequest::new(
        (remote_file_name.clone()).into(),
        ("".to_owned()).into()
    ).with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetHeaderFootersOnlineRequest::new(
        (request_document).into(),
        ("".to_owned()).into()
    );

    context.api().get_header_footers_online(request).await?;
    Ok(())
}

/// Test for getting headerfooter.
#[tokio::test]
async fn header_footer_get_header_footer() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/HeaderFooters";
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();
    let remote_file_name = "TestGetHeaderFooter.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetHeaderFooterRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_header_footer(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "HeaderFooter")?;
    assert_not_null(&result_json, "HeaderFooter.ChildNodes")?;
    assert_length(&result_json, "HeaderFooter.ChildNodes", 1)?;
    assert_string(&result_json, "HeaderFooter.ChildNodes[0].NodeId", "0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting headerfooter online.
#[tokio::test]
async fn header_footer_get_header_footer_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetHeaderFooterOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().get_header_footer_online(request).await?;
    Ok(())
}

/// Test for getting headerfooter of section.
#[tokio::test]
async fn header_footer_get_header_footer_of_section() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/HeaderFooters";
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();
    let remote_file_name = "TestGetHeaderFooterOfSection.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetHeaderFooterOfSectionRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_header_footer_of_section(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "HeaderFooter")?;
    assert_not_null(&result_json, "HeaderFooter.ChildNodes")?;
    assert_length(&result_json, "HeaderFooter.ChildNodes", 1)?;
    assert_string(&result_json, "HeaderFooter.ChildNodes[0].NodeId", "0.0.0".to_owned())?;
    Ok(())
}

/// Test for getting headerfooter of section online.
#[tokio::test]
async fn header_footer_get_header_footer_of_section_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetHeaderFooterOfSectionOnlineRequest::new(
        (request_document).into(),
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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/HeaderFooters";
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();
    let remote_file_name = "TestDeleteHeaderFooter.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteHeaderFooterRequest::new(
        (remote_file_name.clone()).into(),
        ("".to_owned()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_header_footer(request).await?;
    Ok(())
}

/// Test for deleting headerfooter online.
#[tokio::test]
async fn header_footer_delete_header_footer_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteHeaderFooterOnlineRequest::new(
        (request_document).into(),
        ("".to_owned()).into(),
        (0).into()
    );

    context.api().delete_header_footer_online(request).await?;
    Ok(())
}

/// Test for deleting headerfooters.
#[tokio::test]
async fn header_footer_delete_headers_footers() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/HeaderFooters";
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();
    let remote_file_name = "TestDeleteHeadersFooters.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteHeadersFootersRequest::new(
        (remote_file_name.clone()).into(),
        ("".to_owned()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_headers_footers(request).await?;
    Ok(())
}

/// Test for deleting headerfooters online.
#[tokio::test]
async fn header_footer_delete_headers_footers_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteHeadersFootersOnlineRequest::new(
        (request_document).into(),
        ("".to_owned()).into()
    );

    context.api().delete_headers_footers_online(request).await?;
    Ok(())
}

/// Test for adding headerfooters.
#[tokio::test]
async fn header_footer_insert_header_footer() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/HeaderFooters";
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();
    let remote_file_name = "TestInsertHeaderFooter.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = InsertHeaderFooterRequest::new(
        (remote_file_name.clone()).into(),
        ("".to_owned()).into(),
        ("FooterEven".to_owned()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().insert_header_footer(request).await?;
    Ok(())
}

/// Test for adding headerfooters online.
#[tokio::test]
async fn header_footer_insert_header_footer_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/HeaderFooters/HeadersFooters.doc".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = InsertHeaderFooterOnlineRequest::new(
        (request_document).into(),
        ("".to_owned()).into(),
        ("FooterEven".to_owned()).into()
    );

    let result = context.api().insert_header_footer_online(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Model.HeaderFooter")?;
    assert_not_null(&result_json, "Model.HeaderFooter.ChildNodes")?;
    assert_length(&result_json, "Model.HeaderFooter.ChildNodes", 1)?;
    assert_string(&result_json, "Model.HeaderFooter.ChildNodes[0].NodeId", "0.2.0".to_owned())?;
    Ok(())
}