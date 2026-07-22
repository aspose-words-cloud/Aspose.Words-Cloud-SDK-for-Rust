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

/// Test for getting tables.
#[tokio::test]
async fn table_get_tables() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTables.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTablesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_tables(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Tables")?;
    assert_not_null(&result_json, "Tables.TableLinkList")?;
    assert_length(&result_json, "Tables.TableLinkList", 5)?;
    assert_string(&result_json, "Tables.TableLinkList[0].NodeId", test_string!("0.0.1")?)?;
    Ok(())
}

/// Test for getting tables online.
#[tokio::test]
async fn table_get_tables_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTablesOnlineRequest::new(
        (requestDocument).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_tables_online(request).await?;
    Ok(())
}

/// Test for getting tables without node path.
#[tokio::test]
async fn table_get_tables_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTablesWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTablesRequest::new(
        (test_string!(remoteFileName)?).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_tables(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Tables")?;
    assert_not_null(&result_json, "Tables.TableLinkList")?;
    assert_length(&result_json, "Tables.TableLinkList", 5)?;
    assert_string(&result_json, "Tables.TableLinkList[0].NodeId", test_string!("0.0.1")?)?;
    Ok(())
}

/// Test for getting table.
#[tokio::test]
async fn table_get_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTable.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Table")?;
    assert_not_null(&result_json, "Table.TableRowList")?;
    assert_length(&result_json, "Table.TableRowList", 1)?;
    assert_not_null(&result_json, "Table.TableRowList[0].TableCellList")?;
    assert_length(&result_json, "Table.TableRowList[0].TableCellList", 2)?;
    Ok(())
}

/// Test for getting table online.
#[tokio::test]
async fn table_get_table_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTableOnlineRequest::new(
        (requestDocument).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_table_online(request).await?;
    Ok(())
}

/// Test for getting table without node path.
#[tokio::test]
async fn table_get_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTableWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Table")?;
    assert_not_null(&result_json, "Table.TableRowList")?;
    assert_length(&result_json, "Table.TableRowList", 1)?;
    assert_not_null(&result_json, "Table.TableRowList[0].TableCellList")?;
    assert_length(&result_json, "Table.TableRowList[0].TableCellList", 2)?;
    Ok(())
}

/// Test for deleting table.
#[tokio::test]
async fn table_delete_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestDeleteTable.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_table(request).await?;
    Ok(())
}

/// Test for deleting table online.
#[tokio::test]
async fn table_delete_table_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteTableOnlineRequest::new(
        (requestDocument).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into());

    context.api().delete_table_online(request).await?;
    Ok(())
}

/// Test for deleting table without node path.
#[tokio::test]
async fn table_delete_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestDeleteTableWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_table(request).await?;
    Ok(())
}

/// Test for adding table.
#[tokio::test]
async fn table_insert_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestInsertTable.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestTable = TableInsert::default();
    requestTable.r#columns_count = Some((5).into());
    requestTable.r#rows_count = Some((4).into());

    let request = InsertTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestTable).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_table(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Table")?;
    assert_not_null(&result_json, "Table.TableRowList")?;
    assert_length(&result_json, "Table.TableRowList", 4)?;
    assert_not_null(&result_json, "Table.TableRowList[0].TableCellList")?;
    assert_length(&result_json, "Table.TableRowList[0].TableCellList", 5)?;
    Ok(())
}

/// Test for adding table online.
#[tokio::test]
async fn table_insert_table_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestTable = TableInsert::default();
    requestTable.r#columns_count = Some((5).into());
    requestTable.r#rows_count = Some((4).into());

    let request = InsertTableOnlineRequest::new(
        (requestDocument).into(),
        (requestTable).into()
    ).with_node_path((test_string!("")?).into());

    context.api().insert_table_online(request).await?;
    Ok(())
}

/// Test for adding table without node path.
#[tokio::test]
async fn table_insert_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestInsertTableWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestTable = TableInsert::default();
    requestTable.r#columns_count = Some((5).into());
    requestTable.r#rows_count = Some((4).into());

    let request = InsertTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestTable).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_table(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Table")?;
    assert_not_null(&result_json, "Table.TableRowList")?;
    assert_length(&result_json, "Table.TableRowList", 4)?;
    assert_not_null(&result_json, "Table.TableRowList[0].TableCellList")?;
    assert_length(&result_json, "Table.TableRowList[0].TableCellList", 5)?;
    Ok(())
}

/// Test for getting document properties.
#[tokio::test]
async fn table_get_table_properties() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTableProperties.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTablePropertiesRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Properties")?;
    assert_string(&result_json, "Properties.StyleName", test_string!("Table Grid")?)?;
    Ok(())
}

/// Test for getting document properties online.
#[tokio::test]
async fn table_get_table_properties_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTablePropertiesOnlineRequest::new(
        (requestDocument).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into());

    context.api().get_table_properties_online(request).await?;
    Ok(())
}

/// Test for getting document properties without node path.
#[tokio::test]
async fn table_get_table_properties_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTablePropertiesWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTablePropertiesRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Properties")?;
    assert_string(&result_json, "Properties.StyleName", test_string!("Table Grid")?)?;
    Ok(())
}

/// Test for updating table properties.
#[tokio::test]
async fn table_update_table_properties() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestUpdateTableProperties.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestProperties = TableProperties::default();
    requestProperties.r#alignment = Some((TableProperties_AlignmentEnum::Right).into());
    requestProperties.r#allow_auto_fit = Some((false).into());
    requestProperties.r#bidi = Some((true).into());
    requestProperties.r#bottom_padding = Some(((1) as f64).into());
    requestProperties.r#cell_spacing = Some(((2.0) as f64).into());
    requestProperties.r#style_options = Some((TableProperties_StyleOptionsEnum::ColumnBands).into());

    let request = UpdateTablePropertiesRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into(),
        (requestProperties).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_table_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Properties")?;
    assert_bool(&result_json, "Properties.AllowAutoFit", false)?;
    assert_bool(&result_json, "Properties.Bidi", true)?;
    assert_float(&result_json, "Properties.BottomPadding", (1.0) as f64)?;
    assert_float(&result_json, "Properties.CellSpacing", (2.0) as f64)?;
    Ok(())
}

/// Test for updating table properties online.
#[tokio::test]
async fn table_update_table_properties_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestProperties = TableProperties::default();
    requestProperties.r#alignment = Some((TableProperties_AlignmentEnum::Right).into());
    requestProperties.r#allow_auto_fit = Some((false).into());
    requestProperties.r#bidi = Some((true).into());
    requestProperties.r#bottom_padding = Some(((1) as f64).into());
    requestProperties.r#cell_spacing = Some(((2) as f64).into());
    requestProperties.r#style_options = Some((TableProperties_StyleOptionsEnum::ColumnBands).into());

    let request = UpdateTablePropertiesOnlineRequest::new(
        (requestDocument).into(),
        (requestProperties).into(),
        (1).into()
    ).with_node_path((test_string!("")?).into());

    context.api().update_table_properties_online(request).await?;
    Ok(())
}

/// Test for updating table properties without node path.
#[tokio::test]
async fn table_update_table_properties_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestUpdateTablePropertiesWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestProperties = TableProperties::default();
    requestProperties.r#alignment = Some((TableProperties_AlignmentEnum::Right).into());
    requestProperties.r#allow_auto_fit = Some((false).into());
    requestProperties.r#bidi = Some((true).into());
    requestProperties.r#bottom_padding = Some(((1.0) as f64).into());
    requestProperties.r#cell_spacing = Some(((2.0) as f64).into());
    requestProperties.r#style_options = Some((TableProperties_StyleOptionsEnum::ColumnBands).into());

    let request = UpdateTablePropertiesRequest::new(
        (test_string!(remoteFileName)?).into(),
        (1).into(),
        (requestProperties).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_table_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Properties")?;
    assert_bool(&result_json, "Properties.AllowAutoFit", false)?;
    assert_bool(&result_json, "Properties.Bidi", true)?;
    assert_float(&result_json, "Properties.BottomPadding", (1.0) as f64)?;
    assert_float(&result_json, "Properties.CellSpacing", (2.0) as f64)?;
    Ok(())
}

/// Test for getting table row.
#[tokio::test]
async fn table_get_table_row() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTableRow.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTableRowRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("tables/1")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table_row(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Row")?;
    assert_not_null(&result_json, "Row.TableCellList")?;
    assert_length(&result_json, "Row.TableCellList", 2)?;
    Ok(())
}

/// Test for getting table row online.
#[tokio::test]
async fn table_get_table_row_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTableRowOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("tables/1")?).into(),
        (0).into()
    );

    context.api().get_table_row_online(request).await?;
    Ok(())
}

/// Test for deleting table row.
#[tokio::test]
async fn table_delete_table_row() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestDeleteTableRow.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteTableRowRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("tables/1")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_table_row(request).await?;
    Ok(())
}

/// Test for deleting table row online.
#[tokio::test]
async fn table_delete_table_row_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteTableRowOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("tables/1")?).into(),
        (0).into()
    );

    context.api().delete_table_row_online(request).await?;
    Ok(())
}

/// Test for adding row.
#[tokio::test]
async fn table_insert_table_row() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestInsertTableRow.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestRow = TableRowInsert::default();
    requestRow.r#columns_count = Some((5).into());

    let request = InsertTableRowRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestRow).into()
    ).with_node_path((test_string!("sections/0/tables/2")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_table_row(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Row")?;
    assert_not_null(&result_json, "Row.TableCellList")?;
    assert_length(&result_json, "Row.TableCellList", 5)?;
    Ok(())
}

/// Test for adding row online.
#[tokio::test]
async fn table_insert_table_row_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestRow = TableRowInsert::default();
    requestRow.r#columns_count = Some((5).into());

    let request = InsertTableRowOnlineRequest::new(
        (requestDocument).into(),
        (requestRow).into()
    ).with_node_path((test_string!("sections/0/tables/2")?).into());

    context.api().insert_table_row_online(request).await?;
    Ok(())
}

/// Test for getting row format.
#[tokio::test]
async fn table_get_table_row_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTableRowFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTableRowFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/tables/2")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table_row_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "RowFormat")?;
    assert_bool(&result_json, "RowFormat.AllowBreakAcrossPages", true)?;
    Ok(())
}

/// Test for getting row format online.
#[tokio::test]
async fn table_get_table_row_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTableRowFormatOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/tables/2")?).into(),
        (0).into()
    );

    context.api().get_table_row_format_online(request).await?;
    Ok(())
}

/// Test updating row format.
#[tokio::test]
async fn table_update_table_row_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestUpdateTableRowFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFormat = TableRowFormat::default();
    requestFormat.r#allow_break_across_pages = Some((true).into());
    requestFormat.r#heading_format = Some((true).into());
    requestFormat.r#height = Some(((10.0) as f64).into());
    requestFormat.r#height_rule = Some((TableRowFormat_HeightRuleEnum::Exactly).into());

    let request = UpdateTableRowFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/tables/2")?).into(),
        (0).into(),
        (requestFormat).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_table_row_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "RowFormat")?;
    assert_bool(&result_json, "RowFormat.AllowBreakAcrossPages", true)?;
    assert_bool(&result_json, "RowFormat.HeadingFormat", true)?;
    assert_float(&result_json, "RowFormat.Height", (10.0) as f64)?;
    Ok(())
}

/// Test updating row format online.
#[tokio::test]
async fn table_update_table_row_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestFormat = TableRowFormat::default();
    requestFormat.r#allow_break_across_pages = Some((true).into());
    requestFormat.r#heading_format = Some((true).into());
    requestFormat.r#height = Some(((10) as f64).into());
    requestFormat.r#height_rule = Some((TableRowFormat_HeightRuleEnum::Auto).into());

    let request = UpdateTableRowFormatOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/tables/2")?).into(),
        (requestFormat).into(),
        (0).into()
    );

    context.api().update_table_row_format_online(request).await?;
    Ok(())
}

/// Test for getting table cell.
#[tokio::test]
async fn table_get_table_cell() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTableCell.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTableCellRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table_cell(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Cell")?;
    assert_string(&result_json, "Cell.NodeId", test_string!("0.0.5.0.0")?)?;
    Ok(())
}

/// Test for getting table cell online.
#[tokio::test]
async fn table_get_table_cell_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTableCellOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into()
    );

    context.api().get_table_cell_online(request).await?;
    Ok(())
}

/// Test for deleting cell.
#[tokio::test]
async fn table_delete_table_cell() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestDeleteTableCell.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = DeleteTableCellRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().delete_table_cell(request).await?;
    Ok(())
}

/// Test for deleting cell online.
#[tokio::test]
async fn table_delete_table_cell_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = DeleteTableCellOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into()
    );

    context.api().delete_table_cell_online(request).await?;
    Ok(())
}

/// Test for adding cell.
#[tokio::test]
async fn table_insert_table_cell() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestInsertTableCell.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestCell = TableCellInsert::default();


    let request = InsertTableCellRequest::new(
        (test_string!(remoteFileName)?).into(),
        (requestCell).into()
    ).with_table_row_path((test_string!("sections/0/tables/2/rows/0")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().insert_table_cell(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Cell")?;
    assert_string(&result_json, "Cell.NodeId", test_string!("0.0.5.0.3")?)?;
    Ok(())
}

/// Test for adding cell online.
#[tokio::test]
async fn table_insert_table_cell_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestCell = TableCellInsert::default();


    let request = InsertTableCellOnlineRequest::new(
        (requestDocument).into(),
        (requestCell).into()
    ).with_table_row_path((test_string!("sections/0/tables/2/rows/0")?).into());

    context.api().insert_table_cell_online(request).await?;
    Ok(())
}

/// Test for getting cell format.
#[tokio::test]
async fn table_get_table_cell_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestGetTableCellFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = GetTableCellFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().get_table_cell_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CellFormat")?;
    assert_bool(&result_json, "CellFormat.WrapText", true)?;
    Ok(())
}

/// Test for getting cell format online.
#[tokio::test]
async fn table_get_table_cell_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = GetTableCellFormatOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into()
    );

    context.api().get_table_cell_format_online(request).await?;
    Ok(())
}

/// Test for updating cell format.
#[tokio::test]
async fn table_update_table_cell_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestUpdateTableCellFormat.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;
    let mut requestFormat = TableCellFormat::default();
    requestFormat.r#bottom_padding = Some(((5.0) as f64).into());
    requestFormat.r#fit_text = Some((true).into());
    requestFormat.r#horizontal_merge = Some((TableCellFormat_HorizontalMergeEnum::First).into());
    requestFormat.r#wrap_text = Some((true).into());

    let request = UpdateTableCellFormatRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (0).into(),
        (requestFormat).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    let result = context.api().update_table_cell_format(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "CellFormat")?;
    assert_float(&result_json, "CellFormat.BottomPadding", (5.0) as f64)?;
    assert_bool(&result_json, "CellFormat.FitText", true)?;
    assert_bool(&result_json, "CellFormat.WrapText", true)?;
    Ok(())
}

/// Test for updating cell format online.
#[tokio::test]
async fn table_update_table_cell_format_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;
    let mut requestFormat = TableCellFormat::default();
    requestFormat.r#bottom_padding = Some(((5) as f64).into());
    requestFormat.r#fit_text = Some((true).into());
    requestFormat.r#horizontal_merge = Some((TableCellFormat_HorizontalMergeEnum::First).into());
    requestFormat.r#wrap_text = Some((true).into());

    let request = UpdateTableCellFormatOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("sections/0/tables/2/rows/0")?).into(),
        (requestFormat).into(),
        (0).into()
    );

    context.api().update_table_cell_format_online(request).await?;
    Ok(())
}

/// Test for table rendering.
#[tokio::test]
async fn table_render_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestRenderTable.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into())
.with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_table(request).await?;
    Ok(())
}

/// Test for table rendering.
#[tokio::test]
async fn table_render_table_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;

    let requestDocument = context.load_binary_file(test_string!(localFile)?).await?;

    let request = RenderTableOnlineRequest::new(
        (requestDocument).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_node_path((test_string!("")?).into());

    context.api().render_table_online(request).await?;
    Ok(())
}

/// Test for table rendering without node path.
#[tokio::test]
async fn table_render_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let t = &context;
    let remoteBaseTestDataFolder = context.remote_base_test_data_folder().to_owned();
    let baseTestOutPath = context.base_test_out_path().to_owned();
    let randomGuid = context.create_random_guid();
    let remoteDataFolder = test_string!(remoteBaseTestDataFolder + "/DocumentElements/Tables")?;
    let localFile = test_string!("DocumentElements/Tables/TablesGet.docx")?;
    let remoteFileName = test_string!("TestRenderTableWithoutNodePath.docx")?;

    context.upload_file(test_string!(localFile)?, test_string!(remoteDataFolder + "/" + remoteFileName)?).await?;

    let request = RenderTableRequest::new(
        (test_string!(remoteFileName)?).into(),
        (test_string!("png")?).into(),
        (0).into()
    ).with_folder((test_string!(remoteDataFolder)?).into());

    context.api().render_table(request).await?;
    Ok(())
}