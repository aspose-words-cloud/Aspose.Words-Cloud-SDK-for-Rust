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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTables.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTablesRequest::new((remote_file_name.clone()).into())
        .with_node_path(("".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_tables(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Tables")?;
    assert_not_null(&result_json, "Tables.TableLinkList")?;
    assert_length(&result_json, "Tables.TableLinkList", 5)?;
    assert_string(
        &result_json,
        "Tables.TableLinkList[0].NodeId",
        "0.0.1".to_owned(),
    )?;
    Ok(())
}

/// Test for getting tables online.
#[tokio::test]
async fn table_get_tables_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTablesOnlineRequest::new((request_document).into())
        .with_node_path(("".to_owned()).into());

    context.api().get_tables_online(request).await?;
    Ok(())
}

/// Test for getting tables without node path.
#[tokio::test]
async fn table_get_tables_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTablesWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTablesRequest::new((remote_file_name.clone()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_tables(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Tables")?;
    assert_not_null(&result_json, "Tables.TableLinkList")?;
    assert_length(&result_json, "Tables.TableLinkList", 5)?;
    assert_string(
        &result_json,
        "Tables.TableLinkList[0].NodeId",
        "0.0.1".to_owned(),
    )?;
    Ok(())
}

/// Test for getting table.
#[tokio::test]
async fn table_get_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTable.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTableRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_node_path(("".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTableOnlineRequest::new((request_document).into(), (1).into())
        .with_node_path(("".to_owned()).into());

    context.api().get_table_online(request).await?;
    Ok(())
}

/// Test for getting table without node path.
#[tokio::test]
async fn table_get_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTableWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTableRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestDeleteTable.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteTableRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_node_path(("".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().delete_table(request).await?;
    Ok(())
}

/// Test for deleting table online.
#[tokio::test]
async fn table_delete_table_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteTableOnlineRequest::new((request_document).into(), (1).into())
        .with_node_path(("".to_owned()).into());

    context.api().delete_table_online(request).await?;
    Ok(())
}

/// Test for deleting table without node path.
#[tokio::test]
async fn table_delete_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestDeleteTableWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteTableRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_folder((remote_data_folder.clone()).into());

    context.api().delete_table(request).await?;
    Ok(())
}

/// Test for adding table.
#[tokio::test]
async fn table_insert_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestInsertTable.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_table = TableInsert::default();
    request_table.columns_count = Some((5).into());
    request_table.rows_count = Some((4).into());

    let request =
        InsertTableRequest::new((remote_file_name.clone()).into(), (request_table).into())
            .with_node_path(("".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_table = TableInsert::default();
    request_table.columns_count = Some((5).into());
    request_table.rows_count = Some((4).into());

    let request = InsertTableOnlineRequest::new((request_document).into(), (request_table).into())
        .with_node_path(("".to_owned()).into());

    context.api().insert_table_online(request).await?;
    Ok(())
}

/// Test for adding table without node path.
#[tokio::test]
async fn table_insert_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestInsertTableWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_table = TableInsert::default();
    request_table.columns_count = Some((5).into());
    request_table.rows_count = Some((4).into());

    let request =
        InsertTableRequest::new((remote_file_name.clone()).into(), (request_table).into())
            .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTableProperties.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTablePropertiesRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_node_path(("".to_owned()).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_table_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Properties")?;
    assert_string(
        &result_json,
        "Properties.StyleName",
        "Table Grid".to_owned(),
    )?;
    Ok(())
}

/// Test for getting document properties online.
#[tokio::test]
async fn table_get_table_properties_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTablePropertiesOnlineRequest::new((request_document).into(), (1).into())
        .with_node_path(("".to_owned()).into());

    context.api().get_table_properties_online(request).await?;
    Ok(())
}

/// Test for getting document properties without node path.
#[tokio::test]
async fn table_get_table_properties_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTablePropertiesWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTablePropertiesRequest::new((remote_file_name.clone()).into(), (1).into())
        .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_table_properties(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Properties")?;
    assert_string(
        &result_json,
        "Properties.StyleName",
        "Table Grid".to_owned(),
    )?;
    Ok(())
}

/// Test for updating table properties.
#[tokio::test]
async fn table_update_table_properties() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestUpdateTableProperties.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_properties = TableProperties::default();
    request_properties.alignment = Some((TablePropertiesAlignmentEnum::Right).into());
    request_properties.allow_auto_fit = Some((false).into());
    request_properties.bidi = Some((true).into());
    request_properties.bottom_padding = Some(((1) as f64).into());
    request_properties.cell_spacing = Some(((2.0) as f64).into());
    request_properties.style_options = Some((TablePropertiesStyleOptionsEnum::ColumnBands).into());

    let request = UpdateTablePropertiesRequest::new(
        (remote_file_name.clone()).into(),
        (1).into(),
        (request_properties).into(),
    )
    .with_node_path(("".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_properties = TableProperties::default();
    request_properties.alignment = Some((TablePropertiesAlignmentEnum::Right).into());
    request_properties.allow_auto_fit = Some((false).into());
    request_properties.bidi = Some((true).into());
    request_properties.bottom_padding = Some(((1) as f64).into());
    request_properties.cell_spacing = Some(((2) as f64).into());
    request_properties.style_options = Some((TablePropertiesStyleOptionsEnum::ColumnBands).into());

    let request = UpdateTablePropertiesOnlineRequest::new(
        (request_document).into(),
        (request_properties).into(),
        (1).into(),
    )
    .with_node_path(("".to_owned()).into());

    context
        .api()
        .update_table_properties_online(request)
        .await?;
    Ok(())
}

/// Test for updating table properties without node path.
#[tokio::test]
async fn table_update_table_properties_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestUpdateTablePropertiesWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_properties = TableProperties::default();
    request_properties.alignment = Some((TablePropertiesAlignmentEnum::Right).into());
    request_properties.allow_auto_fit = Some((false).into());
    request_properties.bidi = Some((true).into());
    request_properties.bottom_padding = Some(((1.0) as f64).into());
    request_properties.cell_spacing = Some(((2.0) as f64).into());
    request_properties.style_options = Some((TablePropertiesStyleOptionsEnum::ColumnBands).into());

    let request = UpdateTablePropertiesRequest::new(
        (remote_file_name.clone()).into(),
        (1).into(),
        (request_properties).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTableRow.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTableRowRequest::new(
        (remote_file_name.clone()).into(),
        ("tables/1".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTableRowOnlineRequest::new(
        (request_document).into(),
        ("tables/1".to_owned()).into(),
        (0).into(),
    );

    context.api().get_table_row_online(request).await?;
    Ok(())
}

/// Test for deleting table row.
#[tokio::test]
async fn table_delete_table_row() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestDeleteTableRow.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteTableRowRequest::new(
        (remote_file_name.clone()).into(),
        ("tables/1".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().delete_table_row(request).await?;
    Ok(())
}

/// Test for deleting table row online.
#[tokio::test]
async fn table_delete_table_row_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteTableRowOnlineRequest::new(
        (request_document).into(),
        ("tables/1".to_owned()).into(),
        (0).into(),
    );

    context.api().delete_table_row_online(request).await?;
    Ok(())
}

/// Test for adding row.
#[tokio::test]
async fn table_insert_table_row() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestInsertTableRow.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_row = TableRowInsert::default();
    request_row.columns_count = Some((5).into());

    let request =
        InsertTableRowRequest::new((remote_file_name.clone()).into(), (request_row).into())
            .with_node_path(("sections/0/tables/2".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_row = TableRowInsert::default();
    request_row.columns_count = Some((5).into());

    let request = InsertTableRowOnlineRequest::new((request_document).into(), (request_row).into())
        .with_node_path(("sections/0/tables/2".to_owned()).into());

    context.api().insert_table_row_online(request).await?;
    Ok(())
}

/// Test for getting row format.
#[tokio::test]
async fn table_get_table_row_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTableRowFormat.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTableRowFormatRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/tables/2".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTableRowFormatOnlineRequest::new(
        (request_document).into(),
        ("sections/0/tables/2".to_owned()).into(),
        (0).into(),
    );

    context.api().get_table_row_format_online(request).await?;
    Ok(())
}

/// Test updating row format.
#[tokio::test]
async fn table_update_table_row_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestUpdateTableRowFormat.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_format = TableRowFormat::default();
    request_format.allow_break_across_pages = Some((true).into());
    request_format.heading_format = Some((true).into());
    request_format.height = Some(((10.0) as f64).into());
    request_format.height_rule = Some((TableRowFormatHeightRuleEnum::Exactly).into());

    let request = UpdateTableRowFormatRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/tables/2".to_owned()).into(),
        (0).into(),
        (request_format).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_format = TableRowFormat::default();
    request_format.allow_break_across_pages = Some((true).into());
    request_format.heading_format = Some((true).into());
    request_format.height = Some(((10) as f64).into());
    request_format.height_rule = Some((TableRowFormatHeightRuleEnum::Auto).into());

    let request = UpdateTableRowFormatOnlineRequest::new(
        (request_document).into(),
        ("sections/0/tables/2".to_owned()).into(),
        (request_format).into(),
        (0).into(),
    );

    context
        .api()
        .update_table_row_format_online(request)
        .await?;
    Ok(())
}

/// Test for getting table cell.
#[tokio::test]
async fn table_get_table_cell() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTableCell.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTableCellRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    let result = context.api().get_table_cell(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Cell")?;
    assert_string(&result_json, "Cell.NodeId", "0.0.5.0.0".to_owned())?;
    Ok(())
}

/// Test for getting table cell online.
#[tokio::test]
async fn table_get_table_cell_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTableCellOnlineRequest::new(
        (request_document).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
    );

    context.api().get_table_cell_online(request).await?;
    Ok(())
}

/// Test for deleting cell.
#[tokio::test]
async fn table_delete_table_cell() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestDeleteTableCell.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = DeleteTableCellRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().delete_table_cell(request).await?;
    Ok(())
}

/// Test for deleting cell online.
#[tokio::test]
async fn table_delete_table_cell_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = DeleteTableCellOnlineRequest::new(
        (request_document).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
    );

    context.api().delete_table_cell_online(request).await?;
    Ok(())
}

/// Test for adding cell.
#[tokio::test]
async fn table_insert_table_cell() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestInsertTableCell.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_cell = TableCellInsert::default();

    let request =
        InsertTableCellRequest::new((remote_file_name.clone()).into(), (request_cell).into())
            .with_table_row_path(("sections/0/tables/2/rows/0".to_owned()).into())
            .with_folder((remote_data_folder.clone()).into());

    let result = context.api().insert_table_cell(request).await?;
    let result_json = serialize_result(&result)?;
    assert_not_null(&result_json, "Cell")?;
    assert_string(&result_json, "Cell.NodeId", "0.0.5.0.3".to_owned())?;
    Ok(())
}

/// Test for adding cell online.
#[tokio::test]
async fn table_insert_table_cell_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_cell = TableCellInsert::default();

    let request =
        InsertTableCellOnlineRequest::new((request_document).into(), (request_cell).into())
            .with_table_row_path(("sections/0/tables/2/rows/0".to_owned()).into());

    context.api().insert_table_cell_online(request).await?;
    Ok(())
}

/// Test for getting cell format.
#[tokio::test]
async fn table_get_table_cell_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestGetTableCellFormat.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = GetTableCellFormatRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = GetTableCellFormatOnlineRequest::new(
        (request_document).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
    );

    context.api().get_table_cell_format_online(request).await?;
    Ok(())
}

/// Test for updating cell format.
#[tokio::test]
async fn table_update_table_cell_format() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestUpdateTableCellFormat.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;
    let mut request_format = TableCellFormat::default();
    request_format.bottom_padding = Some(((5.0) as f64).into());
    request_format.fit_text = Some((true).into());
    request_format.horizontal_merge = Some((TableCellFormatHorizontalMergeEnum::First).into());
    request_format.wrap_text = Some((true).into());

    let request = UpdateTableCellFormatRequest::new(
        (remote_file_name.clone()).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (0).into(),
        (request_format).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

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
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;
    let mut request_format = TableCellFormat::default();
    request_format.bottom_padding = Some(((5) as f64).into());
    request_format.fit_text = Some((true).into());
    request_format.horizontal_merge = Some((TableCellFormatHorizontalMergeEnum::First).into());
    request_format.wrap_text = Some((true).into());

    let request = UpdateTableCellFormatOnlineRequest::new(
        (request_document).into(),
        ("sections/0/tables/2/rows/0".to_owned()).into(),
        (request_format).into(),
        (0).into(),
    );

    context
        .api()
        .update_table_cell_format_online(request)
        .await?;
    Ok(())
}

/// Test for table rendering.
#[tokio::test]
async fn table_render_table() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestRenderTable.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = RenderTableRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into(),
    )
    .with_node_path(("".to_owned()).into())
    .with_folder((remote_data_folder.clone()).into());

    context.api().render_table(request).await?;
    Ok(())
}

/// Test for table rendering.
#[tokio::test]
async fn table_render_table_online() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();

    let request_document = context.load_binary_file(local_file.clone()).await?;

    let request = RenderTableOnlineRequest::new(
        (request_document).into(),
        ("png".to_owned()).into(),
        (0).into(),
    )
    .with_node_path(("".to_owned()).into());

    context.api().render_table_online(request).await?;
    Ok(())
}

/// Test for table rendering without node path.
#[tokio::test]
async fn table_render_table_without_node_path() -> TestResult<()> {
    let context = TestContext::from_settings().await?;
    let remote_base_test_data_folder = context.remote_base_test_data_folder().to_owned();
    let base_test_out_path = context.base_test_out_path().to_owned();
    let remote_data_folder = remote_base_test_data_folder.clone() + "/DocumentElements/Tables";
    let local_file = "DocumentElements/Tables/TablesGet.docx".to_owned();
    let remote_file_name = "TestRenderTableWithoutNodePath.docx".to_owned();

    context
        .upload_file(
            local_file.clone(),
            remote_data_folder.clone() + "/" + &remote_file_name,
        )
        .await?;

    let request = RenderTableRequest::new(
        (remote_file_name.clone()).into(),
        ("png".to_owned()).into(),
        (0).into(),
    )
    .with_folder((remote_data_folder.clone()).into());

    context.api().render_table(request).await?;
    Ok(())
}
