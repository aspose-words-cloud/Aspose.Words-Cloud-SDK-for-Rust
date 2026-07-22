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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use aspose_words_cloud::*;

use crate::test_context::*;

#[tokio::test]
async fn batch_with_intermediate_results() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_folder = format!(
        "{}/DocumentElements/Paragraphs",
        context.remote_base_test_data_folder()
    );
    let remote_name = "TestBatchDocument.docx".to_owned();
    context
        .upload_file(
            "Common/test_multi_pages.docx".to_owned(),
            format!("{remote_folder}/{remote_name}"),
        )
        .await?;

    let request1 = BatchRequest::new(
        GetParagraphsRequest::new(remote_name.clone())
            .with_node_path("sections/0".to_owned())
            .with_folder(remote_folder.clone()),
    );
    let mut request2 = BatchRequest::new(
        GetParagraphRequest::new(remote_name.clone(), 0)
            .with_node_path("sections/0".to_owned())
            .with_folder(remote_folder.clone()),
    );
    request2.set_depends_on(&request1);

    let mut paragraph = ParagraphInsert::default();
    paragraph.r#text = Some("This is a new paragraph for your document".to_owned());
    let mut request3 = BatchRequest::new(
        InsertParagraphRequest::new(remote_name.clone(), paragraph)
            .with_node_path("sections/0".to_owned())
            .with_folder(remote_folder.clone()),
    );
    request3.set_depends_on(&request2);

    let mut request4 = BatchRequest::new(
        DeleteParagraphRequest::new(remote_name, 0)
            .with_node_path(String::new())
            .with_folder(remote_folder),
    );
    request4.set_depends_on(&request3);

    let report_data = context
        .load_text_file("DocumentActions/Reporting/ReportData.json")
        .await?;
    let mut settings = ReportEngineSettings::default();
    settings.r#data_source_type = Some(ReportEngineSettings_DataSourceTypeEnum::Json);
    settings.r#data_source_name = Some("persons".to_owned());
    let mut request5 = BatchRequest::new(BuildReportOnlineRequest::new(
        request4.result_of(),
        report_data,
        settings,
    ));
    request5.set_depends_on(&request4);

    let result = context
        .api()
        .batch(vec![request1, request2, request3, request4, request5], true)
        .await?;
    ensure(result.len() == 5, "batch response must contain five parts")?;
    ensure(
        result.first().and_then(BatchResult::downcast_ref::<ParagraphLinkCollectionResponse>).is_some(),
        "GetParagraphs batch result has an unexpected type",
    )?;
    ensure(
        result.get(1).and_then(BatchResult::downcast_ref::<ParagraphResponse>).is_some(),
        "GetParagraph batch result has an unexpected type",
    )?;
    ensure(
        result.get(2).and_then(BatchResult::downcast_ref::<ParagraphResponse>).is_some(),
        "InsertParagraph batch result has an unexpected type",
    )?;
    ensure(
        result.get(3).and_then(BatchResult::downcast_ref::<()>).is_some(),
        "DeleteParagraph batch result has an unexpected type",
    )?;
    ensure(
        result.get(4).and_then(BatchResult::downcast_ref::<Vec<u8>>).is_some(),
        "BuildReportOnline batch result has an unexpected type",
    )?;
    Ok(())
}

#[tokio::test]
async fn batch_without_intermediate_results() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_folder = format!(
        "{}/DocumentElements/Paragraphs",
        context.remote_base_test_data_folder()
    );
    let remote_name = "TestBatchDocument.docx".to_owned();
    context
        .upload_file(
            "Common/test_multi_pages.docx".to_owned(),
            format!("{remote_folder}/{remote_name}"),
        )
        .await?;

    let request1 = BatchRequest::new(
        GetParagraphsRequest::new(remote_name.clone())
            .with_node_path("sections/0".to_owned())
            .with_folder(remote_folder.clone()),
    );
    let mut request2 = BatchRequest::new(
        GetParagraphRequest::new(remote_name, 0)
            .with_node_path("sections/0".to_owned())
            .with_folder(remote_folder),
    );
    request2.set_depends_on(&request1);

    let report_data = context
        .load_text_file("DocumentActions/Reporting/ReportData.json")
        .await?;
    let mut settings = ReportEngineSettings::default();
    settings.r#data_source_type = Some(ReportEngineSettings_DataSourceTypeEnum::Json);
    settings.r#data_source_name = Some("persons".to_owned());
    let mut request3 = BatchRequest::new(BuildReportOnlineRequest::new(
        request2.result_of(),
        report_data,
        settings,
    ));
    request3.set_depends_on(&request2);

    let result = context
        .api()
        .batch(vec![request1, request2, request3], false)
        .await?;
    ensure(result.len() == 1, "batch response must contain one final part")?;
    ensure(
        result
            .first()
            .and_then(BatchResult::downcast_ref::<Vec<u8>>)
            .is_some(),
        "BuildReportOnline batch result has an unexpected type",
    )
}

#[tokio::test]
async fn progress_callbacks() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let document = context
        .load_binary_file("DocumentActions/ConvertDocument/test_uploadfile.docx")
        .await?;
    let send_called = Arc::new(AtomicBool::new(false));
    let receive_called = Arc::new(AtomicBool::new(false));
    let send_state = send_called.clone();
    let receive_state = receive_called.clone();
    let request = ConvertDocumentRequest::new(document, "pdf".to_owned())
        .with_send_progress(Arc::new(move |_current, _total| {
            send_state.store(true, Ordering::Relaxed);
        }))
        .with_receive_progress(Arc::new(move |_current, _total| {
            receive_state.store(true, Ordering::Relaxed);
        }));
    let _result = context.api().convert_document(request).await?;
    ensure(
        send_called.load(Ordering::Relaxed),
        "send progress callback was not called",
    )?;
    ensure(
        receive_called.load(Ordering::Relaxed),
        "receive progress callback was not called",
    )
}