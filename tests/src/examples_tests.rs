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

#![allow(non_snake_case, unused_imports, unused_variables)]

use aspose_words_cloud::*;
use chrono::{TimeZone, Utc};

use crate::test_context::{create_random_guid, TestContext, TestResult};

async fn read_text_file(path: String) -> TestResult<String> {
    let sdk_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    Ok(tokio::fs::read_to_string(sdk_root.join("examples_data").join(path)).await?)
}

#[tokio::test]
async fn accept_all_revisions() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let words_api = context.api();
    let examples_data = context.examples_data_dir();

    let file_name = "test_doc.docx".to_owned();

    // Upload original document to cloud storage.
    let my_var1 = tokio::fs::read(examples_data.join(file_name.clone())).await?;
    let my_var2 = file_name.clone();
    let upload_file_request = UploadFileRequest::new((my_var1).into(), (my_var2).into());
    let _result = words_api.upload_file(upload_file_request).await?;

    // Calls AcceptAllRevisions method for document in cloud.
    let my_var3 = file_name.clone();
    let request = AcceptAllRevisionsRequest::new((my_var3).into());
    let _result = words_api.accept_all_revisions(request).await?;

    Ok(())
}

#[tokio::test]
async fn accept_all_revisions_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let words_api = context.api();
    let examples_data = context.examples_data_dir();

    let file_name = "test_doc.docx".to_owned();

    // Calls AcceptAllRevisionsOnline method for document in cloud.
    let request_document = tokio::fs::read(examples_data.join(file_name.clone())).await?;
    let request = AcceptAllRevisionsOnlineRequest::new((request_document).into());
    let result = words_api.accept_all_revisions_online(request).await?;
    let files = result.r#document.ok_or_else(|| {
        SdkError::InvalidResponse("the example response does not contain a document".to_owned())
    })?;
    let content = files.into_values().next().ok_or_else(|| {
        SdkError::InvalidResponse("the example response contains an empty document".to_owned())
    })?;
    tokio::fs::write(examples_data.join("test_result.docx"), content).await?;
    Ok(())
}
