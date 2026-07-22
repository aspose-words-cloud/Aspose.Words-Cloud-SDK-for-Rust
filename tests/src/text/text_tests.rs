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

/// Test for replacing text.
#[tokio::test]
async fn text_replace_text() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Text")?;
    let remoteFileName = test_string!("TestReplaceText.docx")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestReplaceText = ReplaceTextParameters::default();
    requestReplaceText.r#old_value = Some((test_string!("Testing")?).into());
    requestReplaceText.r#new_value = Some((test_string!("Aspose testing")?).into());
    requestReplaceText.r#is_match_case = Some((true).into());
    requestReplaceText.r#is_match_whole_word = Some((false).into());
    requestReplaceText.r#is_old_value_regex = Some((false).into());

    let request = ReplaceTextRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestReplaceText).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().replace_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_integer(&result_json, "Matches", 3)?;
    Ok(())
}

/// Test for replacing text online.
#[tokio::test]
async fn text_replace_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestReplaceText = ReplaceTextParameters::default();
    requestReplaceText.r#old_value = Some((test_string!("aspose")?).into());
    requestReplaceText.r#new_value = Some((test_string!("aspose new")?).into());
    requestReplaceText.r#is_match_case = Some((true).into());
    requestReplaceText.r#is_match_whole_word = Some((false).into());
    requestReplaceText.r#is_old_value_regex = Some((false).into());

    let request = ReplaceTextOnlineRequest::new(
        (requestDocument).into(),
        (requestReplaceText).into()
    );

    context.api().replace_text_online(request).await?;
    Ok(())
}

/// Test for searching.
#[tokio::test]
async fn text_search() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Text")?;
    let remoteFileName = test_string!("TestSearch.docx")?;
    let localFile = test_string!("DocumentElements/Text/SampleWordDocument.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = SearchRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("aspose")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().search(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SearchResults")?;
    assert_not_null(&result_json, "SearchResults.ResultsList")?;
    assert_length(&result_json, "SearchResults.ResultsList", 23)?;
    assert_not_null(&result_json, "SearchResults.ResultsList[0].RangeStart")?;
    assert_integer(&result_json, "SearchResults.ResultsList[0].RangeStart.Offset", 65)?;
    Ok(())
}

/// Test for searching online.
#[tokio::test]
async fn text_search_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Text/SampleWordDocument.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = SearchOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("aspose")?).into()
    );

    context.api().search_online(request).await?;
    Ok(())
}