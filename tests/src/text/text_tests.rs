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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Text";
    let remote_file_name = "TestReplaceText.docx".to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_replace_text = ReplaceTextParameters::default();
    request_replace_text.old_value = Some(("Testing".to_owned()).into());
    request_replace_text.new_value = Some(("Aspose testing".to_owned()).into());
    request_replace_text.is_match_case = Some((true).into());
    request_replace_text.is_match_whole_word = Some((false).into());
    request_replace_text.is_old_value_regex = Some((false).into());

    let request = ReplaceTextRequest::new(
        (remote_file_name.clone()).into(),
        (request_replace_text).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().replace_text(request).await?;
    let result_json = serialize_result(&result)?;
    assert_integer(&result_json, "Matches", 3)?;
    Ok(())
}

/// Test for replacing text online.
#[tokio::test]
async fn text_replace_text_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_replace_text = ReplaceTextParameters::default();
    request_replace_text.old_value = Some(("aspose".to_owned()).into());
    request_replace_text.new_value = Some(("aspose new".to_owned()).into());
    request_replace_text.is_match_case = Some((true).into());
    request_replace_text.is_match_whole_word = Some((false).into());
    request_replace_text.is_old_value_regex = Some((false).into());

    let request = ReplaceTextOnlineRequest::new(
        (request_document).into(),
        (request_replace_text).into()
    );

    context.api().replace_text_online(request).await?;
    Ok(())
}

/// Test for searching.
#[tokio::test]
async fn text_search() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Text";
    let remote_file_name = "TestSearch.docx".to_owned();
    let local_file = "DocumentElements/Text/SampleWordDocument.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = SearchRequest::new(
        (remote_file_name.clone()).into(),
        ("aspose".to_owned()).into()
    ).with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Text/SampleWordDocument.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = SearchOnlineRequest::new(
        (request_document).into(),
        ("aspose".to_owned()).into()
    );

    context.api().search_online(request).await?;
    Ok(())
}