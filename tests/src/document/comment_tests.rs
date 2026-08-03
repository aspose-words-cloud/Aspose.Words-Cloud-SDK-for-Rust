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

/// Test for getting comment by specified comment's index.
#[tokio::test]
async fn comment_get_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Comments";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetComment.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetCommentRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_comment(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comment")?;
    assert_string(&result_json, "Comment.Text", "Comment 1".to_owned() + "\r\n\r\n")?;
    Ok(())
}

/// Test for getting comment by specified comment's index online.
#[tokio::test]
async fn comment_get_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetCommentOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().get_comment_online(request).await?;
    Ok(())
}

/// Test for getting all comments from document.
#[tokio::test]
async fn comment_get_comments() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Comments";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestGetComments.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = GetCommentsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_comments(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comments")?;
    assert_not_null(&result_json, "Comments.CommentList")?;
    assert_length(&result_json, "Comments.CommentList", 1)?;
    assert_string(&result_json, "Comments.CommentList[0].Text", "Comment 1".to_owned() + "\r\n\r\n")?;
    Ok(())
}

/// Test for getting all comments from document online.
#[tokio::test]
async fn comment_get_comments_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetCommentsOnlineRequest::new(
        (request_document).into()
    );

    context.api().get_comments_online(request).await?;
    Ok(())
}

/// Test for adding comment.
#[tokio::test]
async fn comment_insert_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Comments";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestInsertComment.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_comment_range_start = PositionInsideNode::default();
    request_comment_range_start.node_id = Some(("0.3.0.3".to_owned()).into());
    request_comment_range_start.offset = Some((0).into());
    let mut request_comment_range_end = PositionInsideNode::default();
    request_comment_range_end.node_id = Some(("0.3.0.3".to_owned()).into());
    request_comment_range_end.offset = Some((0).into());
    let mut request_comment = CommentInsert::default();
    request_comment.range_start = Some((request_comment_range_start).into());
    request_comment.range_end = Some((request_comment_range_end).into());
    request_comment.initial = Some(("IA".to_owned()).into());
    request_comment.author = Some(("Imran Anwar".to_owned()).into());
    request_comment.text = Some(("A new Comment".to_owned()).into());

    let request = InsertCommentRequest::new(
        (remote_file_name.clone()).into(),
        (request_comment).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_comment(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comment")?;
    assert_string(&result_json, "Comment.Text", "A new Comment".to_owned() + "\r\n")?;
    assert_not_null(&result_json, "Comment.RangeStart")?;
    assert_not_null(&result_json, "Comment.RangeStart.Node")?;
    assert_string(&result_json, "Comment.RangeStart.Node.NodeId", "0.3.0.4".to_owned())?;
    Ok(())
}

/// Test for adding comment online.
#[tokio::test]
async fn comment_insert_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_comment_range_start = PositionInsideNode::default();
    request_comment_range_start.node_id = Some(("0.3.0.3".to_owned()).into());
    request_comment_range_start.offset = Some((0).into());
    let mut request_comment_range_end = PositionInsideNode::default();
    request_comment_range_end.node_id = Some(("0.3.0.3".to_owned()).into());
    request_comment_range_end.offset = Some((0).into());
    let mut request_comment = CommentInsert::default();
    request_comment.range_start = Some((request_comment_range_start).into());
    request_comment.range_end = Some((request_comment_range_end).into());
    request_comment.initial = Some(("IA".to_owned()).into());
    request_comment.author = Some(("Imran Anwar".to_owned()).into());
    request_comment.text = Some(("A new Comment".to_owned()).into());

    let request = InsertCommentOnlineRequest::new(
        (request_document).into(),
        (request_comment).into()
    );

    context.api().insert_comment_online(request).await?;
    Ok(())
}

/// Test for updating comment.
#[tokio::test]
async fn comment_update_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Comments";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestUpdateComment.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;
    let mut request_comment_range_start = PositionInsideNode::default();
    request_comment_range_start.node_id = Some(("0.3.0".to_owned()).into());
    request_comment_range_start.offset = Some((0).into());
    let mut request_comment_range_end = PositionInsideNode::default();
    request_comment_range_end.node_id = Some(("0.3.0".to_owned()).into());
    request_comment_range_end.offset = Some((0).into());
    let mut request_comment = CommentUpdate::default();
    request_comment.range_start = Some((request_comment_range_start).into());
    request_comment.range_end = Some((request_comment_range_end).into());
    request_comment.initial = Some(("IA".to_owned()).into());
    request_comment.author = Some(("Imran Anwar".to_owned()).into());
    request_comment.text = Some(("A new Comment".to_owned()).into());

    let request = UpdateCommentRequest::new(
        (remote_file_name.clone()).into(),
        (0).into(),
        (request_comment).into()
    ).with_folder((remote_data_folder.clone()).into());

    let result = context.api().update_comment(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comment")?;
    assert_string(&result_json, "Comment.Text", "A new Comment".to_owned() + "\r\n")?;
    assert_not_null(&result_json, "Comment.RangeStart")?;
    assert_not_null(&result_json, "Comment.RangeStart.Node")?;
    assert_string(&result_json, "Comment.RangeStart.Node.NodeId", "0.3.0.1".to_owned())?;
    Ok(())
}

/// Test for updating comment online.
#[tokio::test]
async fn comment_update_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_comment_range_start = PositionInsideNode::default();
    request_comment_range_start.node_id = Some(("0.3.0".to_owned()).into());
    request_comment_range_start.offset = Some((0).into());
    let mut request_comment_range_end = PositionInsideNode::default();
    request_comment_range_end.node_id = Some(("0.3.0".to_owned()).into());
    request_comment_range_end.offset = Some((0).into());
    let mut request_comment = CommentUpdate::default();
    request_comment.range_start = Some((request_comment_range_start).into());
    request_comment.range_end = Some((request_comment_range_end).into());
    request_comment.initial = Some(("IA".to_owned()).into());
    request_comment.author = Some(("Imran Anwar".to_owned()).into());
    request_comment.text = Some(("A new Comment".to_owned()).into());

    let request = UpdateCommentOnlineRequest::new(
        (request_document).into(),
        (0).into(),
        (request_comment).into()
    );

    context.api().update_comment_online(request).await?;
    Ok(())
}

/// A test for DeleteComment.
#[tokio::test]
async fn comment_delete_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Comments";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteComment.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteCommentRequest::new(
        (remote_file_name.clone()).into(),
        (0).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_comment(request).await?;
    Ok(())
}

/// A test for DeleteComment online.
#[tokio::test]
async fn comment_delete_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteCommentOnlineRequest::new(
        (request_document).into(),
        (0).into()
    );

    context.api().delete_comment_online(request).await?;
    Ok(())
}

/// A test for DeleteComments.
#[tokio::test]
async fn comment_delete_comments() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/Comments";
    let local_file = "Common/test_multi_pages.docx".to_owned();
    let remote_file_name = "TestDeleteComment.docx".to_owned();

    context.upload_file(local_file.clone(), remote_data_folder.clone() + "/" + &remote_file_name).await?;

    let request = DeleteCommentsRequest::new(
        (remote_file_name.clone()).into()
    ).with_folder((remote_data_folder.clone()).into())
.with_dest_file_name((base_test_out_path.clone() + "/" + &remote_file_name).into());

    context.api().delete_comments(request).await?;
    Ok(())
}

/// A test for DeleteComments online.
#[tokio::test]
async fn comment_delete_comments_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "Common/test_multi_pages.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteCommentsOnlineRequest::new(
        (request_document).into()
    );

    context.api().delete_comments_online(request).await?;
    Ok(())
}