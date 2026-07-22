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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Bookmarks";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetDocumentBookmarks.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetBookmarksRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().get_bookmarks(request).await?;
    Ok(())
}

/// Test for getting bookmarks from document online.
#[tokio::test]
async fn bookmark_get_bookmarks_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetBookmarksOnlineRequest::new(
        (request_document).into()
    );

    context.api().get_bookmarks_online(request).await?;
    Ok(())
}

/// Test for getting bookmark by specified name.
#[tokio::test]
async fn bookmark_get_bookmark_by_name() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Bookmarks";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let bookmark_name = "aspose".to_owned();
    let remote_file_name = "TestGetDocumentBookmarkByName.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetBookmarkByNameRequest::new(
        (remote_file_name.clone()).into(),
        (bookmark_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().get_bookmark_by_name(request).await?;
    Ok(())
}

/// Test for getting bookmark by specified name online.
#[tokio::test]
async fn bookmark_get_bookmark_by_name_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let bookmark_name = "aspose".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetBookmarkByNameOnlineRequest::new(
        (request_document).into(),
        (bookmark_name.clone()).into()
    );

    context.api().get_bookmark_by_name_online(request).await?;
    Ok(())
}

/// Test for updating existed bookmark.
#[tokio::test]
async fn bookmark_update_bookmark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Bookmarks";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let bookmark_name = "aspose".to_owned();
    let remote_file_name = "TestUpdateDocumentBookmark.docx".to_owned();
    let bookmark_text = "This will be the text for Aspose".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_bookmark_data = BookmarkData::default();
    request_bookmark_data.name = Some((bookmark_name.clone()).into());
    request_bookmark_data.text = Some((bookmark_text.clone()).into());

    let request = UpdateBookmarkRequest::new(
        (remote_file_name.clone()).into(),
        (bookmark_name.clone()).into(),
        (request_bookmark_data).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().update_bookmark(request).await?;
    Ok(())
}

/// Test for updating existed bookmark online.
#[tokio::test]
async fn bookmark_update_bookmark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let bookmark_name = "aspose".to_owned();
    let remote_file_name = "TestUpdateDocumentBookmark.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_bookmark_data = BookmarkData::default();
    request_bookmark_data.name = Some((bookmark_name.clone()).into());
    request_bookmark_data.text = Some(("This will be the text for Aspose".to_owned()).into());

    let request = UpdateBookmarkOnlineRequest::new(
        (request_document).into(),
        (bookmark_name.clone()).into(),
        (request_bookmark_data).into()
    ).with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().update_bookmark_online(request).await?;
    Ok(())
}

/// Test for deleting bookmark by specified name.
#[tokio::test]
async fn bookmark_delete_bookmark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Bookmarks";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let bookmark_name = "aspose".to_owned();
    let remote_file_name = "TestDeleteBookmark.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteBookmarkRequest::new(
        (remote_file_name.clone()).into(),
        (bookmark_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_bookmark(request).await?;
    Ok(())
}

/// Test for deleting bookmark by specified name online.
#[tokio::test]
async fn bookmark_delete_bookmark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let bookmark_name = "aspose".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteBookmarkOnlineRequest::new(
        (request_document).into(),
        (bookmark_name.clone()).into()
    );

    context.api().delete_bookmark_online(request).await?;
    Ok(())
}

/// Test for deleting all bookmarks from document.
#[tokio::test]
async fn bookmark_delete_bookmarks() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Bookmarks";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteBookmarks.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteBookmarksRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().delete_bookmarks(request).await?;
    Ok(())
}

/// Test for deleting all bookmarks from document online.
#[tokio::test]
async fn bookmark_delete_bookmarks_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteBookmarksOnlineRequest::new(
        (request_document).into()
    );

    context.api().delete_bookmarks_online(request).await?;
    Ok(())
}

/// Test for inserting new bookmark.
#[tokio::test]
async fn bookmark_insert_bookmark() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Bookmarks";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertBookmark.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_bookmark_start_range = PositionInsideNode::default();
    request_bookmark_start_range.node_id = Some(("0.0.0.0".to_owned()).into());
    request_bookmark_start_range.offset = Some((0).into());
    let mut request_bookmark_end_range = PositionInsideNode::default();
    request_bookmark_end_range.node_id = Some(("0.0.0.0".to_owned()).into());
    request_bookmark_end_range.offset = Some((0).into());
    let mut request_bookmark = BookmarkInsert::default();
    request_bookmark.start_range = Some((request_bookmark_start_range).into());
    request_bookmark.end_range = Some((request_bookmark_end_range).into());
    request_bookmark.name = Some(("new_bookmark".to_owned()).into());
    request_bookmark.text = Some(("Some text".to_owned()).into());

    let request = InsertBookmarkRequest::new(
        (remote_file_name.clone()).into(),
        (request_bookmark).into()
    ).with_folder((remote_data_folder.clone()).into());

    context.api().insert_bookmark(request).await?;
    Ok(())
}

/// Test for inserting new bookmark online.
#[tokio::test]
async fn bookmark_insert_bookmark_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_bookmark_start_range = PositionInsideNode::default();
    request_bookmark_start_range.node_id = Some(("0.0.0.0".to_owned()).into());
    request_bookmark_start_range.offset = Some((0).into());
    let mut request_bookmark_end_range = PositionInsideNode::default();
    request_bookmark_end_range.node_id = Some(("0.0.0.0".to_owned()).into());
    request_bookmark_end_range.offset = Some((0).into());
    let mut request_bookmark = BookmarkInsert::default();
    request_bookmark.start_range = Some((request_bookmark_start_range).into());
    request_bookmark.end_range = Some((request_bookmark_end_range).into());
    request_bookmark.name = Some(("new_bookmark".to_owned()).into());
    request_bookmark.text = Some(("Some text".to_owned()).into());

    let request = InsertBookmarkOnlineRequest::new(
        (request_document).into(),
        (request_bookmark).into()
    );

    context.api().insert_bookmark_online(request).await?;
    Ok(())
}