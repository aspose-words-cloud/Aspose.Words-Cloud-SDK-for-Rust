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
async fn url_encoding() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let data_folder = format!(
        "{}/DocumentElements/Section",
        context.remote_base_test_data_folder()
    );
    let local_name = "test_multi_pages.docx";
    let remote_name = "[“Test_Two,_Inc.”]-_83(b)Election([“Bill_Gates”]).docx";
    let full_name = format!("{data_folder}/{remote_name}");
    context
        .upload_file(format!("Common/{local_name}"), full_name)
        .await?;
    let request = GetSectionRequest::new(remote_name.to_owned(), 0)
        .with_folder(data_folder);
    let _result = context.api().get_section(request).await?;
    Ok(())
}