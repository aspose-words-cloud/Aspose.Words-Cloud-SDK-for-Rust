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

/// Test for posting execute template.
#[tokio::test]
async fn execute_template_with_field_options_execute_template_with_field_options() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentActions/MailMerge";
    let mail_merge_folder = "DocumentActions/MailMerge".to_owned();
    let local_document_file = "TestMailMergeWithOptions.docx".to_owned();
    let remote_file_name = "TestMailMergeWithOptions.docx".to_owned();
    let local_data_file = read_text_file(mail_merge_folder.clone() + "/TestMailMergeData.xml").await?;

    context.upload_file(mail_merge_folder.clone() + "/" + &local_document_file, remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_options_current_user = UserInformation::default();
    request_options_current_user.name = Some(("SdkTestUser".to_owned()).into());
    let mut request_options = FieldOptions::default();
    request_options.current_user = Some((request_options_current_user).into());

    let request = ExecuteMailMergeRequest::new(
        (remote_file_name.clone()).into()
    ).with_data((local_data_file.clone()).into())
.with_options((request_options).into())
.with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().execute_mail_merge(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(&result_json, "Document.FileName", "TestMailMergeWithOptions.docx".to_owned())?;
    Ok(())
}

/// Test for execute template online.
#[tokio::test]
async fn execute_template_with_field_options_execute_template_online_with_field_options() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let mail_merge_folder = "DocumentActions/MailMerge".to_owned();
    let local_document_file = "TestMailMergeWithOptions.docx".to_owned();
    let local_data_file = "TestMailMergeData.xml".to_owned();

    let request_template = context.load_binary_file(mail_merge_folder.clone() + "/" + &local_document_file).await?;
    let request_data = context.load_binary_file(mail_merge_folder.clone() + "/" + &local_data_file).await?;
    let mut request_options_current_user = UserInformation::default();
    request_options_current_user.name = Some(("SdkTestUser".to_owned()).into());
    let mut request_options = FieldOptions::default();
    request_options.current_user = Some((request_options_current_user).into());

    let request = ExecuteMailMergeOnlineRequest::new(
        (request_template).into(),
        (request_data).into()
    ).with_options((request_options).into());

    context.api().execute_mail_merge_online(request).await?;
    Ok(())
}