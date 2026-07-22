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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/PageSetup")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetSectionPageSetup.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetSectionPageSetupRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetSectionPageSetupOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    );

    context.api().get_section_page_setup_online(request).await?;
    Ok(())
}

/// Test for updating page settings.
#[tokio::test]
async fn page_setup_update_section_page_setup() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/PageSetup")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateSectionPageSetup.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestPageSetup = PageSetup::default();
    requestPageSetup.r#rtl_gutter = Some((true).into());
    requestPageSetup.r#left_margin = Some(((10.0) as f64).into());
    requestPageSetup.r#orientation = Some((PageSetup_OrientationEnum::Landscape).into());
    requestPageSetup.r#paper_size = Some((PageSetup_PaperSizeEnum::A5).into());

    let request = UpdateSectionPageSetupRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestPageSetup).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestPageSetup = PageSetup::default();
    requestPageSetup.r#rtl_gutter = Some((true).into());
    requestPageSetup.r#left_margin = Some(((10) as f64).into());
    requestPageSetup.r#orientation = Some((PageSetup_OrientationEnum::Landscape).into());
    requestPageSetup.r#paper_size = Some((PageSetup_PaperSizeEnum::A5).into());

    let request = UpdateSectionPageSetupOnlineRequest::new(
        (requestDocument).into(),
        (0).into(),
        (requestPageSetup).into()
    );

    context.api().update_section_page_setup_online(request).await?;
    Ok(())
}

/// Test for page rendering.
#[tokio::test]
async fn page_setup_get_render_page() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/PageSetup")?;
    let localTextFile = test_string!("DocumentElements/Text/SampleWordDocument.docx")?;
    let remoteFileName = test_string!("TestGetRenderPage.docx")?;

    context.upload_file(test_string!(localTextFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderPageRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into(),
        (test_string!("jpg")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_page(request).await?;
    Ok(())
}

/// Test for page rendering.
#[tokio::test]
async fn page_setup_get_render_page_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localTextFile = test_string!("DocumentElements/Text/SampleWordDocument.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localTextFile)?).await?;

    let request = RenderPageOnlineRequest::new(
        (requestDocument).into(),
        (1).into(),
        (test_string!("jpg")?).into()
    );

    context.api().render_page_online(request).await?;
    Ok(())
}