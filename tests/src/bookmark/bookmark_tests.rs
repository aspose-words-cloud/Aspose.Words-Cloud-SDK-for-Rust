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

/// Test for getting bookmarks from document.
#[tokio::test]
async fn bookmark_get_bookmarks() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Bookmarks")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetDocumentBookmarks.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetBookmarksRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_bookmarks(request).await?;
    Ok(())
}

/// Test for getting bookmarks from document online.
#[tokio::test]
async fn bookmark_get_bookmarks_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetBookmarksOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().get_bookmarks_online(request).await?;
    Ok(())
}

/// Test for getting bookmark by specified name.
#[tokio::test]
async fn bookmark_get_bookmark_by_name() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Bookmarks")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let bookmarkName = test_string!("aspose")?;
    let remoteFileName = test_string!("TestGetDocumentBookmarkByName.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetBookmarkByNameRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!(bookmarkName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().get_bookmark_by_name(request).await?;
    Ok(())
}

/// Test for getting bookmark by specified name online.
#[tokio::test]
async fn bookmark_get_bookmark_by_name_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let bookmarkName = test_string!("aspose")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetBookmarkByNameOnlineRequest::new(
        (requestDocument).into(),
        (test_string!(bookmarkName)?).into()
    );

    context.api().get_bookmark_by_name_online(request).await?;
    Ok(())
}

/// Test for updating existed bookmark.
#[tokio::test]
async fn bookmark_update_bookmark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Bookmarks")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let bookmarkName = test_string!("aspose")?;
    let remoteFileName = test_string!("TestUpdateDocumentBookmark.docx")?;
    let bookmarkText = test_string!("This will be the text for Aspose")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestBookmarkData = BookmarkData::default();
    requestBookmarkData.r#name = Some((test_string!(bookmarkName)?).into());
    requestBookmarkData.r#text = Some((test_string!(bookmarkText)?).into());

    let request = UpdateBookmarkRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!(bookmarkName)?).into(),
        (requestBookmarkData).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().update_bookmark(request).await?;
    Ok(())
}

/// Test for updating existed bookmark online.
#[tokio::test]
async fn bookmark_update_bookmark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let bookmarkName = test_string!("aspose")?;
    let remoteFileName = test_string!("TestUpdateDocumentBookmark.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestBookmarkData = BookmarkData::default();
    requestBookmarkData.r#name = Some((test_string!(bookmarkName)?).into());
    requestBookmarkData.r#text = Some((test_string!("This will be the text for Aspose")?).into());

    let request = UpdateBookmarkOnlineRequest::new(
        (requestDocument).into(),
        (test_string!(bookmarkName)?).into(),
        (requestBookmarkData).into()
    ).with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().update_bookmark_online(request).await?;
    Ok(())
}

/// Test for deleting bookmark by specified name.
#[tokio::test]
async fn bookmark_delete_bookmark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Bookmarks")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let bookmarkName = test_string!("aspose")?;
    let remoteFileName = test_string!("TestDeleteBookmark.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteBookmarkRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!(bookmarkName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_bookmark(request).await?;
    Ok(())
}

/// Test for deleting bookmark by specified name online.
#[tokio::test]
async fn bookmark_delete_bookmark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let bookmarkName = test_string!("aspose")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteBookmarkOnlineRequest::new(
        (requestDocument).into(),
        (test_string!(bookmarkName)?).into()
    );

    context.api().delete_bookmark_online(request).await?;
    Ok(())
}

/// Test for deleting all bookmarks from document.
#[tokio::test]
async fn bookmark_delete_bookmarks() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Bookmarks")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteBookmarks.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteBookmarksRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_bookmarks(request).await?;
    Ok(())
}

/// Test for deleting all bookmarks from document online.
#[tokio::test]
async fn bookmark_delete_bookmarks_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteBookmarksOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().delete_bookmarks_online(request).await?;
    Ok(())
}

/// Test for inserting new bookmark.
#[tokio::test]
async fn bookmark_insert_bookmark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Bookmarks")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertBookmark.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestBookmarkStartRange = PositionInsideNode::default();
    requestBookmarkStartRange.r#node_id = Some((test_string!("0.0.0.0")?).into());
    requestBookmarkStartRange.r#offset = Some((0).into());
    let mut requestBookmarkEndRange = PositionInsideNode::default();
    requestBookmarkEndRange.r#node_id = Some((test_string!("0.0.0.0")?).into());
    requestBookmarkEndRange.r#offset = Some((0).into());
    let mut requestBookmark = BookmarkInsert::default();
    requestBookmark.r#start_range = Some((requestBookmarkStartRange).into());
    requestBookmark.r#end_range = Some((requestBookmarkEndRange).into());
    requestBookmark.r#name = Some((test_string!("new_bookmark")?).into());
    requestBookmark.r#text = Some((test_string!("Some text")?).into());

    let request = InsertBookmarkRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestBookmark).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().insert_bookmark(request).await?;
    Ok(())
}

/// Test for inserting new bookmark online.
#[tokio::test]
async fn bookmark_insert_bookmark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestBookmarkStartRange = PositionInsideNode::default();
    requestBookmarkStartRange.r#node_id = Some((test_string!("0.0.0.0")?).into());
    requestBookmarkStartRange.r#offset = Some((0).into());
    let mut requestBookmarkEndRange = PositionInsideNode::default();
    requestBookmarkEndRange.r#node_id = Some((test_string!("0.0.0.0")?).into());
    requestBookmarkEndRange.r#offset = Some((0).into());
    let mut requestBookmark = BookmarkInsert::default();
    requestBookmark.r#start_range = Some((requestBookmarkStartRange).into());
    requestBookmark.r#end_range = Some((requestBookmarkEndRange).into());
    requestBookmark.r#name = Some((test_string!("new_bookmark")?).into());
    requestBookmark.r#text = Some((test_string!("Some text")?).into());

    let request = InsertBookmarkOnlineRequest::new(
        (requestDocument).into(),
        (requestBookmark).into()
    );

    context.api().insert_bookmark_online(request).await?;
    Ok(())
}