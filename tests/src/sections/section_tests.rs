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

/// Test for getting section by index.
#[tokio::test]
async fn section_get_section() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Section";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetSection.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetSectionRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_section(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Section")?;
    assert_not_null(&result_json, "Section.ChildNodes")?;
    assert_length(&result_json, "Section.ChildNodes", 13)?;
    assert_string(&result_json, "Section.ChildNodes[0].NodeId", "0.3.0".to_owned())?;
    Ok(())
}

/// Test for getting section by index online.
#[tokio::test]
async fn section_get_section_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetSectionOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().get_section_online(request).await?;
    Ok(())
}

/// Test for getting sections.
#[tokio::test]
async fn section_get_sections() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Section";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetSections.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetSectionsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_sections(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Sections")?;
    assert_not_null(&result_json, "Sections.SectionLinkList")?;
    assert_length(&result_json, "Sections.SectionLinkList", 1)?;
    assert_string(&result_json, "Sections.SectionLinkList[0].NodeId", "0".to_owned())?;
    Ok(())
}

/// Test for getting sections online.
#[tokio::test]
async fn section_get_sections_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetSectionsOnlineRequest::new(
        (request_document).into()
    );

    context.api().get_sections_online(request).await?;
    Ok(())
}

/// Test for delete a section.
#[tokio::test]
async fn section_delete_section() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Section";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteSection.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteSectionRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_section(request).await?;
    Ok(())
}

/// Test for delete a section online.
#[tokio::test]
async fn section_delete_section_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteSectionOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().delete_section_online(request).await?;
    Ok(())
}

/// Test for merge a section with the next one.
#[tokio::test]
async fn section_merge_with_next() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Section";
    let remote_file_name = "TestMergeWithNext.docx".to_owned();

    context.upload_file("DocumentElements/Sections/Source.docx".to_owned(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = MergeWithNextRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().merge_with_next(request).await?;
    Ok(())
}

/// Test for merge a section with the next one online.
#[tokio::test]
async fn section_merge_with_next_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();

    let request_document = context.load_binary_file("DocumentElements/Sections/Source.docx".to_owned()).await?;

    let request = MergeWithNextOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().merge_with_next_online(request).await?;
    Ok(())
}

/// Test for insertion a section.
#[tokio::test]
async fn section_insert_section() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Section";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertSection.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = InsertSectionRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().insert_section(request).await?;
    Ok(())
}

/// Test for insertion a section online.
#[tokio::test]
async fn section_insert_section_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = InsertSectionOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().insert_section_online(request).await?;
    Ok(())
}

/// Test for linking headers and footers to previous section.
#[tokio::test]
async fn section_link_header_footers_to_previous() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Section";
    let remote_file_name = "TestLinkHeaderFootersToPrevious.docx".to_owned();

    context.upload_file("DocumentElements/Sections/Source.docx".to_owned(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = LinkHeaderFootersToPreviousRequest::new(
        (remote_file_name.clone()).into(),
        (1).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().link_header_footers_to_previous(request).await?;
    Ok(())
}