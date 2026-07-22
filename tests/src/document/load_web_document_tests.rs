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

/// Test for loading web document.
#[tokio::test]
async fn load_web_document_load_web_document() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();

    let mut requestDataSaveOptions = DocSaveOptionsData::default();
    requestDataSaveOptions.r#file_name = Some((test_string!("google.doc")?).into());
    requestDataSaveOptions.r#dml_effects_rendering_mode = Some((SaveOptionsData_DmlEffectsRenderingModeEnum::None).into());
    requestDataSaveOptions.r#dml_rendering_mode = Some((SaveOptionsData_DmlRenderingModeEnum::DrawingML).into());
    requestDataSaveOptions.r#zip_output = Some((false).into());
    let mut requestData = LoadWebDocumentData::default();
    requestData.r#loading_document_url = Some((test_string!("http://google.com")?).into());
    requestData.r#save_options = Some((requestDataSaveOptions).into());

    let request = LoadWebDocumentRequest::new(
        (requestData).into()
    );

    let result = context.api().load_web_document(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "SaveResult")?;
    assert_not_null(&result_json, "SaveResult.DestDocument")?;
    assert_string(&result_json, "SaveResult.DestDocument.Href", test_string!("google.doc")?)?;
    Ok(())
}

/// Test for loading web document online.
#[tokio::test]
async fn load_web_document_load_web_document_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();

    let mut requestDataSaveOptions = DocSaveOptionsData::default();
    requestDataSaveOptions.r#file_name = Some((test_string!("google.doc")?).into());
    requestDataSaveOptions.r#dml_effects_rendering_mode = Some((SaveOptionsData_DmlEffectsRenderingModeEnum::None).into());
    requestDataSaveOptions.r#dml_rendering_mode = Some((SaveOptionsData_DmlRenderingModeEnum::DrawingML).into());
    requestDataSaveOptions.r#zip_output = Some((false).into());
    let mut requestData = LoadWebDocumentData::default();
    requestData.r#loading_document_url = Some((test_string!("http://google.com")?).into());
    requestData.r#save_options = Some((requestDataSaveOptions).into());

    let request = LoadWebDocumentOnlineRequest::new(
        (requestData).into()
    );

    context.api().load_web_document_online(request).await?;
    Ok(())
}