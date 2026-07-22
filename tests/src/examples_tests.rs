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

use crate::test_context::*;

#[tokio::test]
async fn accept_all_revisions() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let words_api = context.api();
    let examples_data = context.examples_data_dir();

    let fileName = ("test_doc.docx").to_string();

    // Upload original document to cloud storage.
    let myVar1 = tokio::fs::read(
    examples_data.join((fileName).to_string()),
    ).await?;
    let myVar2 = (fileName).to_string();
    let uploadFileRequest = UploadFileRequest::new(
        (myVar1).into(),
        (myVar2).into()
    );
    let _result = words_api.upload_file(uploadFileRequest).await?;


    // Calls AcceptAllRevisions method for document in cloud.
    let myVar3 = (fileName).to_string();
    let request = AcceptAllRevisionsRequest::new(
        (myVar3).into()
    );
    let _result = words_api.accept_all_revisions(request).await?;

    Ok(())
}

#[tokio::test]
async fn accept_all_revisions_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let words_api = context.api();
    let examples_data = context.examples_data_dir();

    let fileName = ("test_doc.docx").to_string();

    // Calls AcceptAllRevisionsOnline method for document in cloud.
    let requestDocument = tokio::fs::read(
    examples_data.join((fileName).to_string()),
    ).await?;
    let request = AcceptAllRevisionsOnlineRequest::new(
        (requestDocument).into()
    );
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