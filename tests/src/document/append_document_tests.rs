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

/// Test for appending document.
#[tokio::test]
async fn append_document_append_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentActions/AppendDocument";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestAppendDocument.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let request_document_list_document_entries0_file_reference =
        FileReference::remote(remote_data_folder.clone() + "/" + &remote_file_name, None);
    let mut request_document_list_document_entries0 = DocumentEntry::default();
    request_document_list_document_entries0.file_reference =
        Some((request_document_list_document_entries0_file_reference).into());
    request_document_list_document_entries0.import_format_mode =
        Some((DocumentEntryImportFormatModeEnum::KeepSourceFormatting).into());
    let request_document_list_document_entries = vec![request_document_list_document_entries0];
    let mut request_document_list = DocumentEntryList::default();
    request_document_list.document_entries = Some((request_document_list_document_entries).into());

    let request = AppendDocumentRequest::new(
        (remote_file_name.clone()).into(),
        (request_document_list).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let result = context.api().append_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(
        &result_json,
        "Document.FileName",
        "TestAppendDocument.docx".to_owned(),
    )?;
    Ok(())
}

/// Test for appending document job.
#[tokio::test]
async fn append_document_append_document_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder =
        remote_base_test_data_folder.clone() + "/DocumentActions/AppendDocument";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestAppendDocument.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let request_document_list_document_entries0_file_reference =
        FileReference::remote(remote_data_folder.clone() + "/" + &remote_file_name, None);
    let mut request_document_list_document_entries0 = DocumentEntry::default();
    request_document_list_document_entries0.file_reference =
        Some((request_document_list_document_entries0_file_reference).into());
    request_document_list_document_entries0.import_format_mode =
        Some((DocumentEntryImportFormatModeEnum::KeepSourceFormatting).into());
    let request_document_list_document_entries = vec![request_document_list_document_entries0];
    let mut request_document_list = DocumentEntryList::default();
    request_document_list.document_entries = Some((request_document_list_document_entries).into());

    let request = AppendDocumentJobRequest::new(
        (remote_file_name.clone()).into(),
        (request_document_list).into(),
    )
    .with_folder((remote_data_folder.clone()).into())
    .with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    let job_handler = context.api().append_document_job(request).await?;
    let result = job_handler.wait_result(Duration::from_secs(3)).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Document")?;
    assert_string(
        &result_json,
        "Document.FileName",
        "TestAppendDocument.docx".to_owned(),
    )?;
    Ok(())
}

/// Test for appending document online.
#[tokio::test]
async fn append_document_append_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let request_document_list_document_entries0_file_reference_content =
        context.load_binary_file(local_file.clone()).await?;
    let request_document_list_document_entries0_file_reference = FileReference::local(
        request_document_list_document_entries0_file_reference_content,
        None,
    );
    let mut request_document_list_document_entries0 = DocumentEntry::default();
    request_document_list_document_entries0.file_reference =
        Some((request_document_list_document_entries0_file_reference).into());
    request_document_list_document_entries0.import_format_mode =
        Some((DocumentEntryImportFormatModeEnum::KeepSourceFormatting).into());
    let request_document_list_document_entries = vec![request_document_list_document_entries0];
    let mut request_document_list = DocumentEntryList::default();
    request_document_list.document_entries = Some((request_document_list_document_entries).into());

    let request =
        AppendDocumentOnlineRequest::new((request_document).into(), (request_document_list).into());

    context.api().append_document_online(request).await?;
    Ok(())
}

/// Test for appending document online job.
#[tokio::test]
async fn append_document_append_document_online_job() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let request_document_list_document_entries0_file_reference_content =
        context.load_binary_file(local_file.clone()).await?;
    let request_document_list_document_entries0_file_reference = FileReference::local(
        request_document_list_document_entries0_file_reference_content,
        None,
    );
    let mut request_document_list_document_entries0 = DocumentEntry::default();
    request_document_list_document_entries0.file_reference =
        Some((request_document_list_document_entries0_file_reference).into());
    request_document_list_document_entries0.import_format_mode =
        Some((DocumentEntryImportFormatModeEnum::KeepSourceFormatting).into());
    let request_document_list_document_entries = vec![request_document_list_document_entries0];
    let mut request_document_list = DocumentEntryList::default();
    request_document_list.document_entries = Some((request_document_list_document_entries).into());

    let request = AppendDocumentOnlineJobRequest::new(
        (request_document).into(),
        (request_document_list).into(),
    );

    let job_handler = context.api().append_document_online_job(request).await?;
    job_handler.wait_result(Duration::from_secs(3)).await?;
    Ok(())
}
