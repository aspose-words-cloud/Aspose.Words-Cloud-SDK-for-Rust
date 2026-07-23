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

/// Test for getting page settings.
#[tokio::test]
async fn page_setup_get_section_page_setup() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/PageSetup";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetSectionPageSetup.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetSectionPageSetupRequest::new((remote_file_name.clone()).into(), (0).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_section_page_setup(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "PageSetup")?;
    assert_integer(&result_json, "PageSetup.LineStartingNumber", 1)?;
    Ok(())
}

/// Test for getting page settings online.
#[tokio::test]
async fn page_setup_get_section_page_setup_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetSectionPageSetupOnlineRequest::new((request_document).into(), (0).into());

    context.api().get_section_page_setup_online(request).await?;
    Ok(())
}

/// Test for updating page settings.
#[tokio::test]
async fn page_setup_update_section_page_setup() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/PageSetup";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateSectionPageSetup.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_page_setup = PageSetup::default();
    request_page_setup.rtl_gutter = Some((true).into());
    request_page_setup.left_margin = Some(((10.0) as f64).into());
    request_page_setup.orientation = Some((PageSetupOrientationEnum::Landscape).into());
    request_page_setup.paper_size = Some((PageSetupPaperSizeEnum::A5).into());

    let request = UpdateSectionPageSetupRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_page_setup).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_section_page_setup(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "PageSetup")?;
    assert_bool(&result_json, "PageSetup.RtlGutter", true)?;

    Ok(())
}

/// Test for updating page settings online.
#[tokio::test]
async fn page_setup_update_section_page_setup_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_page_setup = PageSetup::default();
    request_page_setup.rtl_gutter = Some((true).into());
    request_page_setup.left_margin = Some(((10) as f64).into());
    request_page_setup.orientation = Some((PageSetupOrientationEnum::Landscape).into());
    request_page_setup.paper_size = Some((PageSetupPaperSizeEnum::A5).into());

    let request = UpdateSectionPageSetupOnlineRequest::new(
        (request_document).into(),
        (0).into(),
        (request_page_setup).into(),
    );

    context
        .api()
        .update_section_page_setup_online(request)
        .await?;
    Ok(())
}

/// Test for page rendering.
#[tokio::test]
async fn page_setup_get_render_page() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/PageSetup";
    let local_text_file = "DocumentElements/Text/SampleWordDocument.docx".to_owned();
    let remote_file_name = "TestGetRenderPage.docx".to_owned();

    context
        .upload_file(
            local_text_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = RenderPageRequest::new(
        (remote_file_name.clone()).into(),
        (1).into(),
        ("jpg".to_owned()).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().render_page(request).await?;
    Ok(())
}

/// Test for page rendering.
#[tokio::test]
async fn page_setup_get_render_page_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_text_file = "DocumentElements/Text/SampleWordDocument.docx".to_owned();

    let request_document = context.load_binary_file(local_text_file.clone()).await?;

    let request = RenderPageOnlineRequest::new(
        (request_document).into(),
        (1).into(),
        ("jpg".to_owned()).into(),
    );

    context.api().render_page_online(request).await?;
    Ok(())
}
