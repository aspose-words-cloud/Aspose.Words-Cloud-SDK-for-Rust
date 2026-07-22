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
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/Comments")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetComment.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetCommentRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_comment(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comment")?;
    assert_string(&result_json, "Comment.Text", test_string!("Comment 1" + "\r\n\r\n")?)?;
    Ok(())
}

/// Test for getting comment by specified comment's index online.
#[tokio::test]
async fn comment_get_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetCommentOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    );

    context.api().get_comment_online(request).await?;
    Ok(())
}

/// Test for getting all comments from document.
#[tokio::test]
async fn comment_get_comments() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/Comments")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestGetComments.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetCommentsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_comments(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comments")?;
    assert_not_null(&result_json, "Comments.CommentList")?;
    assert_length(&result_json, "Comments.CommentList", 1)?;
    assert_string(&result_json, "Comments.CommentList[0].Text", test_string!("Comment 1" + "\r\n\r\n")?)?;
    Ok(())
}

/// Test for getting all comments from document online.
#[tokio::test]
async fn comment_get_comments_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetCommentsOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().get_comments_online(request).await?;
    Ok(())
}

/// Test for adding comment.
#[tokio::test]
async fn comment_insert_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/Comments")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestInsertComment.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestCommentRangeStart = PositionInsideNode::default();
    requestCommentRangeStart.r#node_id = Some((test_string!("0.3.0.3")?).into());
    requestCommentRangeStart.r#offset = Some((0).into());
    let mut requestCommentRangeEnd = PositionInsideNode::default();
    requestCommentRangeEnd.r#node_id = Some((test_string!("0.3.0.3")?).into());
    requestCommentRangeEnd.r#offset = Some((0).into());
    let mut requestComment = CommentInsert::default();
    requestComment.r#range_start = Some((requestCommentRangeStart).into());
    requestComment.r#range_end = Some((requestCommentRangeEnd).into());
    requestComment.r#initial = Some((test_string!("IA")?).into());
    requestComment.r#author = Some((test_string!("Imran Anwar")?).into());
    requestComment.r#text = Some((test_string!("A new Comment")?).into());

    let request = InsertCommentRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestComment).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_comment(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comment")?;
    assert_string(&result_json, "Comment.Text", test_string!("A new Comment" + "\r\n")?)?;
    assert_not_null(&result_json, "Comment.RangeStart")?;
    assert_not_null(&result_json, "Comment.RangeStart.Node")?;
    assert_string(&result_json, "Comment.RangeStart.Node.NodeId", test_string!("0.3.0.4")?)?;
    Ok(())
}

/// Test for adding comment online.
#[tokio::test]
async fn comment_insert_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestCommentRangeStart = PositionInsideNode::default();
    requestCommentRangeStart.r#node_id = Some((test_string!("0.3.0.3")?).into());
    requestCommentRangeStart.r#offset = Some((0).into());
    let mut requestCommentRangeEnd = PositionInsideNode::default();
    requestCommentRangeEnd.r#node_id = Some((test_string!("0.3.0.3")?).into());
    requestCommentRangeEnd.r#offset = Some((0).into());
    let mut requestComment = CommentInsert::default();
    requestComment.r#range_start = Some((requestCommentRangeStart).into());
    requestComment.r#range_end = Some((requestCommentRangeEnd).into());
    requestComment.r#initial = Some((test_string!("IA")?).into());
    requestComment.r#author = Some((test_string!("Imran Anwar")?).into());
    requestComment.r#text = Some((test_string!("A new Comment")?).into());

    let request = InsertCommentOnlineRequest::new(
        (requestDocument).into(),
        (requestComment).into()
    );

    context.api().insert_comment_online(request).await?;
    Ok(())
}

/// Test for updating comment.
#[tokio::test]
async fn comment_update_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/Comments")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestUpdateComment.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestCommentRangeStart = PositionInsideNode::default();
    requestCommentRangeStart.r#node_id = Some((test_string!("0.3.0")?).into());
    requestCommentRangeStart.r#offset = Some((0).into());
    let mut requestCommentRangeEnd = PositionInsideNode::default();
    requestCommentRangeEnd.r#node_id = Some((test_string!("0.3.0")?).into());
    requestCommentRangeEnd.r#offset = Some((0).into());
    let mut requestComment = CommentUpdate::default();
    requestComment.r#range_start = Some((requestCommentRangeStart).into());
    requestComment.r#range_end = Some((requestCommentRangeEnd).into());
    requestComment.r#initial = Some((test_string!("IA")?).into());
    requestComment.r#author = Some((test_string!("Imran Anwar")?).into());
    requestComment.r#text = Some((test_string!("A new Comment")?).into());

    let request = UpdateCommentRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into(),
        (requestComment).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_comment(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Comment")?;
    assert_string(&result_json, "Comment.Text", test_string!("A new Comment" + "\r\n")?)?;
    assert_not_null(&result_json, "Comment.RangeStart")?;
    assert_not_null(&result_json, "Comment.RangeStart.Node")?;
    assert_string(&result_json, "Comment.RangeStart.Node.NodeId", test_string!("0.3.0.1")?)?;
    Ok(())
}

/// Test for updating comment online.
#[tokio::test]
async fn comment_update_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestCommentRangeStart = PositionInsideNode::default();
    requestCommentRangeStart.r#node_id = Some((test_string!("0.3.0")?).into());
    requestCommentRangeStart.r#offset = Some((0).into());
    let mut requestCommentRangeEnd = PositionInsideNode::default();
    requestCommentRangeEnd.r#node_id = Some((test_string!("0.3.0")?).into());
    requestCommentRangeEnd.r#offset = Some((0).into());
    let mut requestComment = CommentUpdate::default();
    requestComment.r#range_start = Some((requestCommentRangeStart).into());
    requestComment.r#range_end = Some((requestCommentRangeEnd).into());
    requestComment.r#initial = Some((test_string!("IA")?).into());
    requestComment.r#author = Some((test_string!("Imran Anwar")?).into());
    requestComment.r#text = Some((test_string!("A new Comment")?).into());

    let request = UpdateCommentOnlineRequest::new(
        (requestDocument).into(),
        (0).into(),
        (requestComment).into()
    );

    context.api().update_comment_online(request).await?;
    Ok(())
}

/// A test for DeleteComment.
#[tokio::test]
async fn comment_delete_comment() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/Comments")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteComment.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteCommentRequest::new(
        (test_string!(remoteFileName)?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_comment(request).await?;
    Ok(())
}

/// A test for DeleteComment online.
#[tokio::test]
async fn comment_delete_comment_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteCommentOnlineRequest::new(
        (requestDocument).into(),
        (0).into()
    );

    context.api().delete_comment_online(request).await?;
    Ok(())
}

/// A test for DeleteComments.
#[tokio::test]
async fn comment_delete_comments() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/Comments")?;
    let localFile = test_string!("Common/test_multi_pages.docx")?;
    let remoteFileName = test_string!("TestDeleteComment.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteCommentsRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into())
.with_dest_file_name((test_string!(baseTestOutPath + "/" + remoteFileName)?).into());

    context.api().delete_comments(request).await?;
    Ok(())
}

/// A test for DeleteComments online.
#[tokio::test]
async fn comment_delete_comments_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("Common/test_multi_pages.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteCommentsOnlineRequest::new(
        (requestDocument).into()
    );

    context.api().delete_comments_online(request).await?;
    Ok(())
}