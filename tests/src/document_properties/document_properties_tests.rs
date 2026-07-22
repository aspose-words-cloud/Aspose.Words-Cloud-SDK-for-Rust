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

/// Test for getting document properties.
#[tokio::test]
async fn document_properties_get_document_properties() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProperties")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentProperties.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentPropertiesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_document_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "DocumentProperties")?;
    assert_not_null(&result_json, "DocumentProperties.List")?;
    assert_length(&result_json, "DocumentProperties.List", 27)?;
    assert_not_null(&result_json, "DocumentProperties.List[0]")?;
    assert_string(&result_json, "DocumentProperties.List[0].Name", test_string!("Author")?)?;
    assert_string(&result_json, "DocumentProperties.List[0].Value", test_string!("")?)?;
    Ok(())
}

/// Test for getting document properties online.
#[tokio::test]
async fn document_properties_get_document_properties_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetDocumentPropertiesOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().get_document_properties_online(request).await?;
    Ok(())
}

/// A test for GetDocumentProperty.
#[tokio::test]
async fn document_properties_get_document_property() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProperties")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentProperty.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetDocumentPropertyRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("Author")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_document_property(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "DocumentProperty")?;
    assert_string(&result_json, "DocumentProperty.Name", test_string!("Author")?)?;
    assert_string(&result_json, "DocumentProperty.Value", test_string!("")?)?;
    Ok(())
}

/// A test for GetDocumentProperty online.
#[tokio::test]
async fn document_properties_get_document_property_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetDocumentPropertyOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("Author")?).into()
    );

    context.api().get_document_property_online(request).await?;
    Ok(())
}

/// Test for deleting document property.
#[tokio::test]
async fn document_properties_delete_document_property() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProperties")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteDocumentProperty.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteDocumentPropertyRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("testProp")?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_document_property(request).await?;
    Ok(())
}

/// Test for deleting document property online.
#[tokio::test]
async fn document_properties_delete_document_property_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteDocumentPropertyOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("testProp")?).into()
    );

    context.api().delete_document_property_online(request).await?;
    Ok(())
}

/// Test for updating document property.
#[tokio::test]
async fn document_properties_update_document_property() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/DocumentProperties")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateDocumentProperty.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestProperty = DocumentPropertyCreateOrUpdate::default();
    requestProperty.r#value = Some((test_string!("Imran Anwar")?).into());

    let request = CreateOrUpdateDocumentPropertyRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("AsposeAuthor")?).into(),
        (requestProperty).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    let result = context.api().create_or_update_document_property(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "DocumentProperty")?;
    assert_string(&result_json, "DocumentProperty.Name", test_string!("AsposeAuthor")?)?;
    assert_string(&result_json, "DocumentProperty.Value", test_string!("Imran Anwar")?)?;
    Ok(())
}

/// Test for updating document property online.
#[tokio::test]
async fn document_properties_update_document_property_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestProperty = DocumentPropertyCreateOrUpdate::default();
    requestProperty.r#value = Some((test_string!("Imran Anwar")?).into());

    let request = CreateOrUpdateDocumentPropertyOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("AsposeAuthor")?).into(),
        (requestProperty).into()
    );

    context.api().create_or_update_document_property_online(request).await?;
    Ok(())
}