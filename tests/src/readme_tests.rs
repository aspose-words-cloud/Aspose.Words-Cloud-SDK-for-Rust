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

use aspose_words_cloud::*;

use crate::test_context::*;

#[tokio::test]
async fn readme_example() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let configuration = Configuration::new(
        context.api().configuration().client_id.clone(),
        context.api().configuration().client_secret.clone(),
    )
    .with_base_url(context.api().configuration().base_url.clone());
    let words_api = WordsApi::new(configuration)?;

    // Upload a document to cloud storage.
    let file_content = context.load_binary_file("Common/test_doc.docx").await?;
    let upload_request =
        UploadFileRequest::new(file_content, "fileStoredInCloud.docx".to_owned());
    words_api.upload_file(upload_request).await?;

    // Save the document as PDF in cloud storage.
    let mut save_options = PdfSaveOptionsData::default();
    save_options.r#file_name = Some("destStoredInCloud.pdf".to_owned());
    let save_request = SaveAsRequest::new(
        "fileStoredInCloud.docx".to_owned(),
        save_options.into(),
    );
    words_api.save_as(save_request).await?;
    Ok(())
}