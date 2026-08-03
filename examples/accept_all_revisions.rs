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

#![allow(dead_code, unused_imports)]
// Example expressions use the same schema-independent conversion rules as tests.
#![allow(clippy::unnecessary_to_owned, clippy::useless_conversion)]

use std::env;
use std::path::PathBuf;

use aspose_words_cloud::*;
use chrono::{TimeZone, Utc};
use uuid::Uuid;

fn create_random_guid() -> String {
    Uuid::new_v4().to_string()
}

async fn read_text_file(path: String) -> SdkResult<String> {
    let examples_data = env::var_os("ASPOSE_EXAMPLES_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("examples_data"));
    Ok(tokio::fs::read_to_string(examples_data.join(path)).await?)
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let client_id = env::var("ASPOSE_CLIENT_ID").map_err(|error| {
        SdkError::InvalidRequest(format!("ASPOSE_CLIENT_ID is not set: {error}"))
    })?;
    let client_secret = env::var("ASPOSE_CLIENT_SECRET").map_err(|error| {
        SdkError::InvalidRequest(format!("ASPOSE_CLIENT_SECRET is not set: {error}"))
    })?;
    let mut configuration = Configuration::new(client_id, client_secret);
    if let Ok(base_url) = env::var("ASPOSE_BASE_URL") {
        configuration = configuration.with_base_url(base_url);
    }
    let words_api = WordsApi::new(configuration)?;
    let examples_data = env::var_os("ASPOSE_EXAMPLES_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("examples_data"));

    let file_name = "test_doc.docx".to_owned();

    // Upload original document to cloud storage.
    let my_var1 = tokio::fs::read(
    examples_data.join(file_name.clone()),
    ).await?;
    let my_var2 = file_name.clone();
    let upload_file_request = UploadFileRequest::new(
        (my_var1).into(),
        (my_var2).into()
    );
    let _result = words_api.upload_file(upload_file_request).await?;


    // Calls AcceptAllRevisions method for document in cloud.
    let my_var3 = file_name.clone();
    let request = AcceptAllRevisionsRequest::new(
        (my_var3).into()
    );
    let _result = words_api.accept_all_revisions(request).await?;

    Ok(())
}