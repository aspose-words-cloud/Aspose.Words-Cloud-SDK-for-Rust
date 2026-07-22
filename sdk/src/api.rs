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

use std::collections::HashMap;

use crate::batch::execute_batch;
use crate::request::TypedRequest;
use crate::*;

/// Strongly typed asynchronous client for Aspose.Words for Cloud.
#[derive(Clone)]
pub struct WordsApi {
    client: ApiClient,
}

impl WordsApi {
    pub fn new(configuration: Configuration) -> SdkResult<Self> {
        Ok(Self {
            client: ApiClient::new(configuration)?,
        })
    }

    pub fn configuration(&self) -> &Configuration {
        self.client.configuration()
    }

    /// Accepts all the revisions in the document.
    pub async fn accept_all_revisions(
        &self,
        request: AcceptAllRevisionsRequest,
    ) -> SdkResult<RevisionsModificationResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Accepts all the revisions in the document.
    pub async fn accept_all_revisions_online(
        &self,
        request: AcceptAllRevisionsOnlineRequest,
    ) -> SdkResult<AcceptAllRevisionsOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Appends documents to the original document.
    pub async fn append_document(
        &self,
        request: AppendDocumentRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Appends documents to the original document.
    pub async fn append_document_job(
        &self,
        request: AppendDocumentJobRequest,
    ) -> SdkResult<JobHandler<AppendDocumentRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Appends documents to the original document.
    pub async fn append_document_online(
        &self,
        request: AppendDocumentOnlineRequest,
    ) -> SdkResult<AppendDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Appends documents to the original document.
    pub async fn append_document_online_job(
        &self,
        request: AppendDocumentOnlineJobRequest,
    ) -> SdkResult<JobHandler<AppendDocumentOnlineRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Applies a style to the document node.
    pub async fn apply_style_to_document_element(
        &self,
        request: ApplyStyleToDocumentElementRequest,
    ) -> SdkResult<WordsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Applies a style to the document node.
    pub async fn apply_style_to_document_element_online(
        &self,
        request: ApplyStyleToDocumentElementOnlineRequest,
    ) -> SdkResult<ApplyStyleToDocumentElementOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Executes the report generation process using the specified document template and the external data source in XML, JSON or CSV format.
    pub async fn build_report(
        &self,
        request: BuildReportRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Executes the report generation process online using the specified document template and the external data source in XML, JSON or CSV format.
    pub async fn build_report_online(
        &self,
        request: BuildReportOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Compares two documents.
    pub async fn compare_document(
        &self,
        request: CompareDocumentRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Compares two documents.
    pub async fn compare_document_online(
        &self,
        request: CompareDocumentOnlineRequest,
    ) -> SdkResult<CompareDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Compress and resize images inside the document.
    /// The default settings allows to reduce the size of the document without any visible degradation of images quality.
    pub async fn compress_document(
        &self,
        request: CompressDocumentRequest,
    ) -> SdkResult<CompressResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Compress and resize images inside the document.
    /// The default settings allows to reduce the size of the document without any visible degradation of images quality.
    pub async fn compress_document_online(
        &self,
        request: CompressDocumentOnlineRequest,
    ) -> SdkResult<CompressDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document on a local drive to the specified format.
    pub async fn convert_document(
        &self,
        request: ConvertDocumentRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document on a local drive to the specified format.
    pub async fn convert_document_job(
        &self,
        request: ConvertDocumentJobRequest,
    ) -> SdkResult<JobHandler<ConvertDocumentRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Copy file.
    pub async fn copy_file(
        &self,
        request: CopyFileRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Copy folder.
    pub async fn copy_folder(
        &self,
        request: CopyFolderRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Makes a copy of the style in the document.
    pub async fn copy_style(
        &self,
        request: CopyStyleRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Makes a copy of the style in the document.
    pub async fn copy_style_online(
        &self,
        request: CopyStyleOnlineRequest,
    ) -> SdkResult<CopyStyleOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Copies styles from the origin document to the target document.
    pub async fn copy_styles_from_template(
        &self,
        request: CopyStylesFromTemplateRequest,
    ) -> SdkResult<WordsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Creates a new document in cloud storage in the format, determined by the file extension.
    /// Supported all save format extensions.
    pub async fn create_document(
        &self,
        request: CreateDocumentRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Create the folder.
    pub async fn create_folder(
        &self,
        request: CreateFolderRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Adds a new or updates an existing document property.
    pub async fn create_or_update_document_property(
        &self,
        request: CreateOrUpdateDocumentPropertyRequest,
    ) -> SdkResult<DocumentPropertyResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Adds a new or updates an existing document property.
    pub async fn create_or_update_document_property_online(
        &self,
        request: CreateOrUpdateDocumentPropertyOnlineRequest,
    ) -> SdkResult<CreateOrUpdateDocumentPropertyOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes paragraph tab stops from the document node.
    pub async fn delete_all_paragraph_tab_stops(
        &self,
        request: DeleteAllParagraphTabStopsRequest,
    ) -> SdkResult<TabStopsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes paragraph tab stops from the document node.
    pub async fn delete_all_paragraph_tab_stops_online(
        &self,
        request: DeleteAllParagraphTabStopsOnlineRequest,
    ) -> SdkResult<DeleteAllParagraphTabStopsOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a bookmark from the document.
    pub async fn delete_bookmark(
        &self,
        request: DeleteBookmarkRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a bookmark from the document.
    pub async fn delete_bookmark_online(
        &self,
        request: DeleteBookmarkOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all bookmarks from the document.
    pub async fn delete_bookmarks(
        &self,
        request: DeleteBookmarksRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all bookmarks from the document.
    pub async fn delete_bookmarks_online(
        &self,
        request: DeleteBookmarksOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a border from the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn delete_border(
        &self,
        request: DeleteBorderRequest,
    ) -> SdkResult<BorderResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a border from the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn delete_border_online(
        &self,
        request: DeleteBorderOnlineRequest,
    ) -> SdkResult<DeleteBorderOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes borders from the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn delete_borders(
        &self,
        request: DeleteBordersRequest,
    ) -> SdkResult<BordersResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes borders from the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn delete_borders_online(
        &self,
        request: DeleteBordersOnlineRequest,
    ) -> SdkResult<DeleteBordersOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a comment from the document.
    pub async fn delete_comment(
        &self,
        request: DeleteCommentRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a comment from the document.
    pub async fn delete_comment_online(
        &self,
        request: DeleteCommentOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all comments from the document.
    pub async fn delete_comments(
        &self,
        request: DeleteCommentsRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all comments from the document.
    pub async fn delete_comments_online(
        &self,
        request: DeleteCommentsOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes the custom xml part from the document.
    pub async fn delete_custom_xml_part(
        &self,
        request: DeleteCustomXmlPartRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes the custom xml part from the document.
    pub async fn delete_custom_xml_part_online(
        &self,
        request: DeleteCustomXmlPartOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all custom xml parts from the document.
    pub async fn delete_custom_xml_parts(
        &self,
        request: DeleteCustomXmlPartsRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all custom xml parts from the document.
    pub async fn delete_custom_xml_parts_online(
        &self,
        request: DeleteCustomXmlPartsOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a document property.
    pub async fn delete_document_property(
        &self,
        request: DeleteDocumentPropertyRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a document property.
    pub async fn delete_document_property_online(
        &self,
        request: DeleteDocumentPropertyOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a DrawingObject from the document node.
    pub async fn delete_drawing_object(
        &self,
        request: DeleteDrawingObjectRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a DrawingObject from the document node.
    pub async fn delete_drawing_object_online(
        &self,
        request: DeleteDrawingObjectOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a field from the document node.
    pub async fn delete_field(
        &self,
        request: DeleteFieldRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a field from the document node.
    pub async fn delete_field_online(
        &self,
        request: DeleteFieldOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes fields from the document node.
    pub async fn delete_fields(
        &self,
        request: DeleteFieldsRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes fields from the document node.
    pub async fn delete_fields_online(
        &self,
        request: DeleteFieldsOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Delete file.
    pub async fn delete_file(
        &self,
        request: DeleteFileRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Delete folder.
    pub async fn delete_folder(
        &self,
        request: DeleteFolderRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a footnote from the document node.
    pub async fn delete_footnote(
        &self,
        request: DeleteFootnoteRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a footnote from the document node.
    pub async fn delete_footnote_online(
        &self,
        request: DeleteFootnoteOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a form field from the document node.
    pub async fn delete_form_field(
        &self,
        request: DeleteFormFieldRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a form field from the document node.
    pub async fn delete_form_field_online(
        &self,
        request: DeleteFormFieldOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a HeaderFooter object from the document section.
    pub async fn delete_header_footer(
        &self,
        request: DeleteHeaderFooterRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a HeaderFooter object from the document section.
    pub async fn delete_header_footer_online(
        &self,
        request: DeleteHeaderFooterOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes HeaderFooter objects from the document section.
    pub async fn delete_headers_footers(
        &self,
        request: DeleteHeadersFootersRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes HeaderFooter objects from the document section.
    pub async fn delete_headers_footers_online(
        &self,
        request: DeleteHeadersFootersOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes macros from the document.
    pub async fn delete_macros(
        &self,
        request: DeleteMacrosRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes macros from the document.
    pub async fn delete_macros_online(
        &self,
        request: DeleteMacrosOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes an OfficeMath object from the document node.
    pub async fn delete_office_math_object(
        &self,
        request: DeleteOfficeMathObjectRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes an OfficeMath object from the document node.
    pub async fn delete_office_math_object_online(
        &self,
        request: DeleteOfficeMathObjectOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all office math objects from the document.
    pub async fn delete_office_math_objects(
        &self,
        request: DeleteOfficeMathObjectsRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all office math objects from the document.
    pub async fn delete_office_math_objects_online(
        &self,
        request: DeleteOfficeMathObjectsOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a paragraph from the document node.
    pub async fn delete_paragraph(
        &self,
        request: DeleteParagraphRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes the formatting properties of a paragraph list from the document node.
    pub async fn delete_paragraph_list_format(
        &self,
        request: DeleteParagraphListFormatRequest,
    ) -> SdkResult<ParagraphListFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes the formatting properties of a paragraph list from the document node.
    pub async fn delete_paragraph_list_format_online(
        &self,
        request: DeleteParagraphListFormatOnlineRequest,
    ) -> SdkResult<DeleteParagraphListFormatOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a paragraph from the document node.
    pub async fn delete_paragraph_online(
        &self,
        request: DeleteParagraphOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a paragraph tab stop from the document node.
    pub async fn delete_paragraph_tab_stop(
        &self,
        request: DeleteParagraphTabStopRequest,
    ) -> SdkResult<TabStopsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a paragraph tab stop from the document node.
    pub async fn delete_paragraph_tab_stop_online(
        &self,
        request: DeleteParagraphTabStopOnlineRequest,
    ) -> SdkResult<DeleteParagraphTabStopOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a Run object from the paragraph.
    pub async fn delete_run(
        &self,
        request: DeleteRunRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a Run object from the paragraph.
    pub async fn delete_run_online(
        &self,
        request: DeleteRunOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a section from the document.
    pub async fn delete_section(
        &self,
        request: DeleteSectionRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a section from the document.
    pub async fn delete_section_online(
        &self,
        request: DeleteSectionOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a StructuredDocumentTag (SDT) from the document node.
    pub async fn delete_structured_document_tag(
        &self,
        request: DeleteStructuredDocumentTagRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a StructuredDocumentTag (SDT) from the document node.
    pub async fn delete_structured_document_tag_online(
        &self,
        request: DeleteStructuredDocumentTagOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a table from the document node.
    pub async fn delete_table(
        &self,
        request: DeleteTableRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a cell from the table row.
    pub async fn delete_table_cell(
        &self,
        request: DeleteTableCellRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a cell from the table row.
    pub async fn delete_table_cell_online(
        &self,
        request: DeleteTableCellOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a table from the document node.
    pub async fn delete_table_online(
        &self,
        request: DeleteTableOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a row from the table.
    pub async fn delete_table_row(
        &self,
        request: DeleteTableRowRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a row from the table.
    pub async fn delete_table_row_online(
        &self,
        request: DeleteTableRowOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a watermark from the document.
    pub async fn delete_watermark(
        &self,
        request: DeleteWatermarkRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a watermark from the document.
    pub async fn delete_watermark_online(
        &self,
        request: DeleteWatermarkOnlineRequest,
    ) -> SdkResult<DeleteWatermarkOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Download file.
    pub async fn download_file(
        &self,
        request: DownloadFileRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Executes a Mail Merge operation.
    pub async fn execute_mail_merge(
        &self,
        request: ExecuteMailMergeRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Executes a Mail Merge operation.
    pub async fn execute_mail_merge_job(
        &self,
        request: ExecuteMailMergeJobRequest,
    ) -> SdkResult<JobHandler<ExecuteMailMergeRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Executes a Mail Merge operation online.
    pub async fn execute_mail_merge_online(
        &self,
        request: ExecuteMailMergeOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Executes a Mail Merge operation online.
    pub async fn execute_mail_merge_online_job(
        &self,
        request: ExecuteMailMergeOnlineJobRequest,
    ) -> SdkResult<JobHandler<ExecuteMailMergeOnlineRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Get all information about revisions.
    pub async fn get_all_revisions(
        &self,
        request: GetAllRevisionsRequest,
    ) -> SdkResult<RevisionsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Get all information about revisions.
    pub async fn get_all_revisions_online(
        &self,
        request: GetAllRevisionsOnlineRequest,
    ) -> SdkResult<RevisionsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads available fonts from the document.
    pub async fn get_available_fonts(
        &self,
        request: GetAvailableFontsRequest,
    ) -> SdkResult<AvailableFontsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a bookmark, specified by name, from the document.
    pub async fn get_bookmark_by_name(
        &self,
        request: GetBookmarkByNameRequest,
    ) -> SdkResult<BookmarkResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a bookmark, specified by name, from the document.
    pub async fn get_bookmark_by_name_online(
        &self,
        request: GetBookmarkByNameOnlineRequest,
    ) -> SdkResult<BookmarkResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads bookmarks from the document.
    pub async fn get_bookmarks(
        &self,
        request: GetBookmarksRequest,
    ) -> SdkResult<BookmarksResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads bookmarks from the document.
    pub async fn get_bookmarks_online(
        &self,
        request: GetBookmarksOnlineRequest,
    ) -> SdkResult<BookmarksResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a border from the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn get_border(
        &self,
        request: GetBorderRequest,
    ) -> SdkResult<BorderResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a border from the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn get_border_online(
        &self,
        request: GetBorderOnlineRequest,
    ) -> SdkResult<BorderResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads borders from the document node.
    pub async fn get_borders(
        &self,
        request: GetBordersRequest,
    ) -> SdkResult<BordersResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads borders from the document node.
    pub async fn get_borders_online(
        &self,
        request: GetBordersOnlineRequest,
    ) -> SdkResult<BordersResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a comment from the document.
    pub async fn get_comment(
        &self,
        request: GetCommentRequest,
    ) -> SdkResult<CommentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a comment from the document.
    pub async fn get_comment_online(
        &self,
        request: GetCommentOnlineRequest,
    ) -> SdkResult<CommentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads comments from the document.
    pub async fn get_comments(
        &self,
        request: GetCommentsRequest,
    ) -> SdkResult<CommentsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads comments from the document.
    pub async fn get_comments_online(
        &self,
        request: GetCommentsOnlineRequest,
    ) -> SdkResult<CommentsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the custom xml part from the document.
    pub async fn get_custom_xml_part(
        &self,
        request: GetCustomXmlPartRequest,
    ) -> SdkResult<CustomXmlPartResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the custom xml part from the document.
    pub async fn get_custom_xml_part_online(
        &self,
        request: GetCustomXmlPartOnlineRequest,
    ) -> SdkResult<CustomXmlPartResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads custom xml parts from the document.
    pub async fn get_custom_xml_parts(
        &self,
        request: GetCustomXmlPartsRequest,
    ) -> SdkResult<CustomXmlPartsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads custom xml parts from the document.
    pub async fn get_custom_xml_parts_online(
        &self,
        request: GetCustomXmlPartsOnlineRequest,
    ) -> SdkResult<CustomXmlPartsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads common information from the document.
    pub async fn get_document(
        &self,
        request: GetDocumentRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a DrawingObject from the document node.
    pub async fn get_document_drawing_object_by_index(
        &self,
        request: GetDocumentDrawingObjectByIndexRequest,
    ) -> SdkResult<DrawingObjectResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a DrawingObject from the document node.
    pub async fn get_document_drawing_object_by_index_online(
        &self,
        request: GetDocumentDrawingObjectByIndexOnlineRequest,
    ) -> SdkResult<DrawingObjectResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads image data of a DrawingObject from the document node.
    pub async fn get_document_drawing_object_image_data(
        &self,
        request: GetDocumentDrawingObjectImageDataRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads image data of a DrawingObject from the document node.
    pub async fn get_document_drawing_object_image_data_online(
        &self,
        request: GetDocumentDrawingObjectImageDataOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads OLE data of a DrawingObject from the document node.
    pub async fn get_document_drawing_object_ole_data(
        &self,
        request: GetDocumentDrawingObjectOleDataRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads OLE data of a DrawingObject from the document node.
    pub async fn get_document_drawing_object_ole_data_online(
        &self,
        request: GetDocumentDrawingObjectOleDataOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads DrawingObjects from the document node.
    pub async fn get_document_drawing_objects(
        &self,
        request: GetDocumentDrawingObjectsRequest,
    ) -> SdkResult<DrawingObjectsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads DrawingObjects from the document node.
    pub async fn get_document_drawing_objects_online(
        &self,
        request: GetDocumentDrawingObjectsOnlineRequest,
    ) -> SdkResult<DrawingObjectsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads merge field names from the document.
    pub async fn get_document_field_names(
        &self,
        request: GetDocumentFieldNamesRequest,
    ) -> SdkResult<FieldNamesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads merge field names from the document.
    pub async fn get_document_field_names_online(
        &self,
        request: GetDocumentFieldNamesOnlineRequest,
    ) -> SdkResult<FieldNamesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a hyperlink from the document.
    pub async fn get_document_hyperlink_by_index(
        &self,
        request: GetDocumentHyperlinkByIndexRequest,
    ) -> SdkResult<HyperlinkResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a hyperlink from the document.
    pub async fn get_document_hyperlink_by_index_online(
        &self,
        request: GetDocumentHyperlinkByIndexOnlineRequest,
    ) -> SdkResult<HyperlinkResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads hyperlinks from the document.
    pub async fn get_document_hyperlinks(
        &self,
        request: GetDocumentHyperlinksRequest,
    ) -> SdkResult<HyperlinksResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads hyperlinks from the document.
    pub async fn get_document_hyperlinks_online(
        &self,
        request: GetDocumentHyperlinksOnlineRequest,
    ) -> SdkResult<HyperlinksResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads document properties.
    pub async fn get_document_properties(
        &self,
        request: GetDocumentPropertiesRequest,
    ) -> SdkResult<DocumentPropertiesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads document properties.
    pub async fn get_document_properties_online(
        &self,
        request: GetDocumentPropertiesOnlineRequest,
    ) -> SdkResult<DocumentPropertiesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a document property.
    pub async fn get_document_property(
        &self,
        request: GetDocumentPropertyRequest,
    ) -> SdkResult<DocumentPropertyResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a document property.
    pub async fn get_document_property_online(
        &self,
        request: GetDocumentPropertyOnlineRequest,
    ) -> SdkResult<DocumentPropertyResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads protection properties from the document.
    pub async fn get_document_protection(
        &self,
        request: GetDocumentProtectionRequest,
    ) -> SdkResult<ProtectionDataResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads protection properties from the document.
    pub async fn get_document_protection_online(
        &self,
        request: GetDocumentProtectionOnlineRequest,
    ) -> SdkResult<ProtectionDataResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads document statistics.
    pub async fn get_document_statistics(
        &self,
        request: GetDocumentStatisticsRequest,
    ) -> SdkResult<StatDataResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads document statistics.
    pub async fn get_document_statistics_online(
        &self,
        request: GetDocumentStatisticsOnlineRequest,
    ) -> SdkResult<StatDataResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document in cloud storage to the specified format.
    pub async fn get_document_with_format(
        &self,
        request: GetDocumentWithFormatRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a field from the document node.
    pub async fn get_field(
        &self,
        request: GetFieldRequest,
    ) -> SdkResult<FieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a field from the document node.
    pub async fn get_field_online(
        &self,
        request: GetFieldOnlineRequest,
    ) -> SdkResult<FieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads fields from the document node.
    pub async fn get_fields(
        &self,
        request: GetFieldsRequest,
    ) -> SdkResult<FieldsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads fields from the document node.
    pub async fn get_fields_online(
        &self,
        request: GetFieldsOnlineRequest,
    ) -> SdkResult<FieldsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Get all files and folders within a folder.
    pub async fn get_files_list(
        &self,
        request: GetFilesListRequest,
    ) -> SdkResult<FilesList> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a footnote from the document node.
    pub async fn get_footnote(
        &self,
        request: GetFootnoteRequest,
    ) -> SdkResult<FootnoteResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a footnote from the document node.
    pub async fn get_footnote_online(
        &self,
        request: GetFootnoteOnlineRequest,
    ) -> SdkResult<FootnoteResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads footnotes from the document node.
    pub async fn get_footnotes(
        &self,
        request: GetFootnotesRequest,
    ) -> SdkResult<FootnotesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads footnotes from the document node.
    pub async fn get_footnotes_online(
        &self,
        request: GetFootnotesOnlineRequest,
    ) -> SdkResult<FootnotesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a form field from the document node.
    pub async fn get_form_field(
        &self,
        request: GetFormFieldRequest,
    ) -> SdkResult<FormFieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a form field from the document node.
    pub async fn get_form_field_online(
        &self,
        request: GetFormFieldOnlineRequest,
    ) -> SdkResult<FormFieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads form fields from the document node.
    pub async fn get_form_fields(
        &self,
        request: GetFormFieldsRequest,
    ) -> SdkResult<FormFieldsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads form fields from the document node.
    pub async fn get_form_fields_online(
        &self,
        request: GetFormFieldsOnlineRequest,
    ) -> SdkResult<FormFieldsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a HeaderFooter object from the document.
    pub async fn get_header_footer(
        &self,
        request: GetHeaderFooterRequest,
    ) -> SdkResult<HeaderFooterResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a HeaderFooter object from the document section.
    pub async fn get_header_footer_of_section(
        &self,
        request: GetHeaderFooterOfSectionRequest,
    ) -> SdkResult<HeaderFooterResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a HeaderFooter object from the document section.
    pub async fn get_header_footer_of_section_online(
        &self,
        request: GetHeaderFooterOfSectionOnlineRequest,
    ) -> SdkResult<HeaderFooterResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a HeaderFooter object from the document.
    pub async fn get_header_footer_online(
        &self,
        request: GetHeaderFooterOnlineRequest,
    ) -> SdkResult<HeaderFooterResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads HeaderFooter objects from the document section.
    pub async fn get_header_footers(
        &self,
        request: GetHeaderFootersRequest,
    ) -> SdkResult<HeaderFootersResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads HeaderFooter objects from the document section.
    pub async fn get_header_footers_online(
        &self,
        request: GetHeaderFootersOnlineRequest,
    ) -> SdkResult<HeaderFootersResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Returns application info.
    pub async fn get_info(
        &self,
        request: GetInfoRequest,
    ) -> SdkResult<InfoResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a list from the document.
    pub async fn get_list(
        &self,
        request: GetListRequest,
    ) -> SdkResult<ListResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a list from the document.
    pub async fn get_list_online(
        &self,
        request: GetListOnlineRequest,
    ) -> SdkResult<ListResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads lists from the document.
    pub async fn get_lists(
        &self,
        request: GetListsRequest,
    ) -> SdkResult<ListsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads lists from the document.
    pub async fn get_lists_online(
        &self,
        request: GetListsOnlineRequest,
    ) -> SdkResult<ListsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads an OfficeMath object from the document node.
    pub async fn get_office_math_object(
        &self,
        request: GetOfficeMathObjectRequest,
    ) -> SdkResult<OfficeMathObjectResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads an OfficeMath object from the document node.
    pub async fn get_office_math_object_online(
        &self,
        request: GetOfficeMathObjectOnlineRequest,
    ) -> SdkResult<OfficeMathObjectResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads OfficeMath objects from the document node.
    pub async fn get_office_math_objects(
        &self,
        request: GetOfficeMathObjectsRequest,
    ) -> SdkResult<OfficeMathObjectsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads OfficeMath objects from the document node.
    pub async fn get_office_math_objects_online(
        &self,
        request: GetOfficeMathObjectsOnlineRequest,
    ) -> SdkResult<OfficeMathObjectsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a paragraph from the document node.
    pub async fn get_paragraph(
        &self,
        request: GetParagraphRequest,
    ) -> SdkResult<ParagraphResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a paragraph from the document node.
    pub async fn get_paragraph_format(
        &self,
        request: GetParagraphFormatRequest,
    ) -> SdkResult<ParagraphFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a paragraph from the document node.
    pub async fn get_paragraph_format_online(
        &self,
        request: GetParagraphFormatOnlineRequest,
    ) -> SdkResult<ParagraphFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a paragraph list from the document node.
    pub async fn get_paragraph_list_format(
        &self,
        request: GetParagraphListFormatRequest,
    ) -> SdkResult<ParagraphListFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a paragraph list from the document node.
    pub async fn get_paragraph_list_format_online(
        &self,
        request: GetParagraphListFormatOnlineRequest,
    ) -> SdkResult<ParagraphListFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a paragraph from the document node.
    pub async fn get_paragraph_online(
        &self,
        request: GetParagraphOnlineRequest,
    ) -> SdkResult<ParagraphResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads paragraphs from the document node.
    pub async fn get_paragraphs(
        &self,
        request: GetParagraphsRequest,
    ) -> SdkResult<ParagraphLinkCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads paragraphs from the document node.
    pub async fn get_paragraphs_online(
        &self,
        request: GetParagraphsOnlineRequest,
    ) -> SdkResult<ParagraphLinkCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads paragraph tab stops from the document node.
    pub async fn get_paragraph_tab_stops(
        &self,
        request: GetParagraphTabStopsRequest,
    ) -> SdkResult<TabStopsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads paragraph tab stops from the document node.
    pub async fn get_paragraph_tab_stops_online(
        &self,
        request: GetParagraphTabStopsOnlineRequest,
    ) -> SdkResult<TabStopsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Get asymmetric public key.
    pub async fn get_public_key(
        &self,
        request: GetPublicKeyRequest,
    ) -> SdkResult<PublicKeyResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads range text from the document.
    pub async fn get_range_text(
        &self,
        request: GetRangeTextRequest,
    ) -> SdkResult<RangeTextResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads range text from the document.
    pub async fn get_range_text_online(
        &self,
        request: GetRangeTextOnlineRequest,
    ) -> SdkResult<RangeTextResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a Run object from the paragraph.
    pub async fn get_run(
        &self,
        request: GetRunRequest,
    ) -> SdkResult<RunResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the font properties of a Run object from the paragraph.
    pub async fn get_run_font(
        &self,
        request: GetRunFontRequest,
    ) -> SdkResult<FontResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the font properties of a Run object from the paragraph.
    pub async fn get_run_font_online(
        &self,
        request: GetRunFontOnlineRequest,
    ) -> SdkResult<FontResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a Run object from the paragraph.
    pub async fn get_run_online(
        &self,
        request: GetRunOnlineRequest,
    ) -> SdkResult<RunResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads Run objects from the paragraph.
    pub async fn get_runs(
        &self,
        request: GetRunsRequest,
    ) -> SdkResult<RunsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads Run objects from the paragraph.
    pub async fn get_runs_online(
        &self,
        request: GetRunsOnlineRequest,
    ) -> SdkResult<RunsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a section from the document.
    pub async fn get_section(
        &self,
        request: GetSectionRequest,
    ) -> SdkResult<SectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a section from the document.
    pub async fn get_section_online(
        &self,
        request: GetSectionOnlineRequest,
    ) -> SdkResult<SectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the page setup of a section from the document.
    pub async fn get_section_page_setup(
        &self,
        request: GetSectionPageSetupRequest,
    ) -> SdkResult<SectionPageSetupResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the page setup of a section from the document.
    pub async fn get_section_page_setup_online(
        &self,
        request: GetSectionPageSetupOnlineRequest,
    ) -> SdkResult<SectionPageSetupResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads sections from the document.
    pub async fn get_sections(
        &self,
        request: GetSectionsRequest,
    ) -> SdkResult<SectionLinkCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads sections from the document.
    pub async fn get_sections_online(
        &self,
        request: GetSectionsOnlineRequest,
    ) -> SdkResult<SectionLinkCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Gets signatures from the document.
    pub async fn get_signatures(
        &self,
        request: GetSignaturesRequest,
    ) -> SdkResult<SignatureCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Gets signatures from the document.
    pub async fn get_signatures_online(
        &self,
        request: GetSignaturesOnlineRequest,
    ) -> SdkResult<SignatureCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a StructuredDocumentTag (SDT) from the document node.
    pub async fn get_structured_document_tag(
        &self,
        request: GetStructuredDocumentTagRequest,
    ) -> SdkResult<StructuredDocumentTagResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a StructuredDocumentTag (SDT) from the document node.
    pub async fn get_structured_document_tag_online(
        &self,
        request: GetStructuredDocumentTagOnlineRequest,
    ) -> SdkResult<StructuredDocumentTagResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads StructuredDocumentTags (SDT) from the document node.
    pub async fn get_structured_document_tags(
        &self,
        request: GetStructuredDocumentTagsRequest,
    ) -> SdkResult<StructuredDocumentTagsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads StructuredDocumentTags (SDT) from the document node.
    pub async fn get_structured_document_tags_online(
        &self,
        request: GetStructuredDocumentTagsOnlineRequest,
    ) -> SdkResult<StructuredDocumentTagsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a style from the document.
    pub async fn get_style(
        &self,
        request: GetStyleRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a style from the document node.
    pub async fn get_style_from_document_element(
        &self,
        request: GetStyleFromDocumentElementRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a style from the document node.
    pub async fn get_style_from_document_element_online(
        &self,
        request: GetStyleFromDocumentElementOnlineRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a style from the document.
    pub async fn get_style_online(
        &self,
        request: GetStyleOnlineRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads styles from the document.
    pub async fn get_styles(
        &self,
        request: GetStylesRequest,
    ) -> SdkResult<StylesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads styles from the document.
    pub async fn get_styles_online(
        &self,
        request: GetStylesOnlineRequest,
    ) -> SdkResult<StylesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a table from the document node.
    pub async fn get_table(
        &self,
        request: GetTableRequest,
    ) -> SdkResult<TableResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a cell from the table row.
    pub async fn get_table_cell(
        &self,
        request: GetTableCellRequest,
    ) -> SdkResult<TableCellResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a table cell.
    pub async fn get_table_cell_format(
        &self,
        request: GetTableCellFormatRequest,
    ) -> SdkResult<TableCellFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a table cell.
    pub async fn get_table_cell_format_online(
        &self,
        request: GetTableCellFormatOnlineRequest,
    ) -> SdkResult<TableCellFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a cell from the table row.
    pub async fn get_table_cell_online(
        &self,
        request: GetTableCellOnlineRequest,
    ) -> SdkResult<TableCellResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a table from the document node.
    pub async fn get_table_online(
        &self,
        request: GetTableOnlineRequest,
    ) -> SdkResult<TableResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads properties of a table from the document node.
    pub async fn get_table_properties(
        &self,
        request: GetTablePropertiesRequest,
    ) -> SdkResult<TablePropertiesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads properties of a table from the document node.
    pub async fn get_table_properties_online(
        &self,
        request: GetTablePropertiesOnlineRequest,
    ) -> SdkResult<TablePropertiesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a row from the table.
    pub async fn get_table_row(
        &self,
        request: GetTableRowRequest,
    ) -> SdkResult<TableRowResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a table row.
    pub async fn get_table_row_format(
        &self,
        request: GetTableRowFormatRequest,
    ) -> SdkResult<TableRowFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads the formatting properties of a table row.
    pub async fn get_table_row_format_online(
        &self,
        request: GetTableRowFormatOnlineRequest,
    ) -> SdkResult<TableRowFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads a row from the table.
    pub async fn get_table_row_online(
        &self,
        request: GetTableRowOnlineRequest,
    ) -> SdkResult<TableRowResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads tables from the document node.
    pub async fn get_tables(
        &self,
        request: GetTablesRequest,
    ) -> SdkResult<TableLinkCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reads tables from the document node.
    pub async fn get_tables_online(
        &self,
        request: GetTablesOnlineRequest,
    ) -> SdkResult<TableLinkCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new bookmark to the document.
    pub async fn insert_bookmark(
        &self,
        request: InsertBookmarkRequest,
    ) -> SdkResult<BookmarkResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new bookmark to the document.
    pub async fn insert_bookmark_online(
        &self,
        request: InsertBookmarkOnlineRequest,
    ) -> SdkResult<InsertBookmarkOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new comment to the document.
    pub async fn insert_comment(
        &self,
        request: InsertCommentRequest,
    ) -> SdkResult<CommentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new comment to the document.
    pub async fn insert_comment_online(
        &self,
        request: InsertCommentOnlineRequest,
    ) -> SdkResult<InsertCommentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new custom xml part to the document.
    pub async fn insert_custom_xml_part(
        &self,
        request: InsertCustomXmlPartRequest,
    ) -> SdkResult<CustomXmlPartResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new custom xml part to the document.
    pub async fn insert_custom_xml_part_online(
        &self,
        request: InsertCustomXmlPartOnlineRequest,
    ) -> SdkResult<InsertCustomXmlPartOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new DrawingObject to the document node.
    pub async fn insert_drawing_object(
        &self,
        request: InsertDrawingObjectRequest,
    ) -> SdkResult<DrawingObjectResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new DrawingObject to the document node.
    pub async fn insert_drawing_object_online(
        &self,
        request: InsertDrawingObjectOnlineRequest,
    ) -> SdkResult<InsertDrawingObjectOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new field to the document node.
    pub async fn insert_field(
        &self,
        request: InsertFieldRequest,
    ) -> SdkResult<FieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new field to the document node.
    pub async fn insert_field_online(
        &self,
        request: InsertFieldOnlineRequest,
    ) -> SdkResult<InsertFieldOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new footnote to the document node.
    pub async fn insert_footnote(
        &self,
        request: InsertFootnoteRequest,
    ) -> SdkResult<FootnoteResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new footnote to the document node.
    pub async fn insert_footnote_online(
        &self,
        request: InsertFootnoteOnlineRequest,
    ) -> SdkResult<InsertFootnoteOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new form field to the document node.
    pub async fn insert_form_field(
        &self,
        request: InsertFormFieldRequest,
    ) -> SdkResult<FormFieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new form field to the document node.
    pub async fn insert_form_field_online(
        &self,
        request: InsertFormFieldOnlineRequest,
    ) -> SdkResult<InsertFormFieldOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new HeaderFooter object to the document section.
    pub async fn insert_header_footer(
        &self,
        request: InsertHeaderFooterRequest,
    ) -> SdkResult<HeaderFooterResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new HeaderFooter object to the document section.
    pub async fn insert_header_footer_online(
        &self,
        request: InsertHeaderFooterOnlineRequest,
    ) -> SdkResult<InsertHeaderFooterOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new list to the document.
    pub async fn insert_list(
        &self,
        request: InsertListRequest,
    ) -> SdkResult<ListResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new list to the document.
    pub async fn insert_list_online(
        &self,
        request: InsertListOnlineRequest,
    ) -> SdkResult<InsertListOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new or updates an existing paragraph tab stop in the document node.
    pub async fn insert_or_update_paragraph_tab_stop(
        &self,
        request: InsertOrUpdateParagraphTabStopRequest,
    ) -> SdkResult<TabStopsResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new or updates an existing paragraph tab stop in the document node.
    pub async fn insert_or_update_paragraph_tab_stop_online(
        &self,
        request: InsertOrUpdateParagraphTabStopOnlineRequest,
    ) -> SdkResult<InsertOrUpdateParagraphTabStopOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts page numbers to the document.
    pub async fn insert_page_numbers(
        &self,
        request: InsertPageNumbersRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts page numbers to the document.
    pub async fn insert_page_numbers_online(
        &self,
        request: InsertPageNumbersOnlineRequest,
    ) -> SdkResult<InsertPageNumbersOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new paragraph to the document node.
    pub async fn insert_paragraph(
        &self,
        request: InsertParagraphRequest,
    ) -> SdkResult<ParagraphResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new paragraph to the document node.
    pub async fn insert_paragraph_online(
        &self,
        request: InsertParagraphOnlineRequest,
    ) -> SdkResult<InsertParagraphOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new Run object to the paragraph.
    pub async fn insert_run(
        &self,
        request: InsertRunRequest,
    ) -> SdkResult<RunResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new Run object to the paragraph.
    pub async fn insert_run_online(
        &self,
        request: InsertRunOnlineRequest,
    ) -> SdkResult<InsertRunOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a section to the document.
    pub async fn insert_section(
        &self,
        request: InsertSectionRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a section to the document.
    pub async fn insert_section_online(
        &self,
        request: InsertSectionOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new StructuredDocumentTag (SDT) to the document node.
    pub async fn insert_structured_document_tag(
        &self,
        request: InsertStructuredDocumentTagRequest,
    ) -> SdkResult<StructuredDocumentTagResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new StructuredDocumentTag (SDT) to the document node.
    pub async fn insert_structured_document_tag_online(
        &self,
        request: InsertStructuredDocumentTagOnlineRequest,
    ) -> SdkResult<InsertStructuredDocumentTagOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new style to the document.
    pub async fn insert_style(
        &self,
        request: InsertStyleRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new style to the document.
    pub async fn insert_style_online(
        &self,
        request: InsertStyleOnlineRequest,
    ) -> SdkResult<InsertStyleOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new table to the document node.
    pub async fn insert_table(
        &self,
        request: InsertTableRequest,
    ) -> SdkResult<TableResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new cell to the table row.
    pub async fn insert_table_cell(
        &self,
        request: InsertTableCellRequest,
    ) -> SdkResult<TableCellResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new cell to the table row.
    pub async fn insert_table_cell_online(
        &self,
        request: InsertTableCellOnlineRequest,
    ) -> SdkResult<InsertTableCellOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new table to the document node.
    pub async fn insert_table_online(
        &self,
        request: InsertTableOnlineRequest,
    ) -> SdkResult<InsertTableOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new row to the table.
    pub async fn insert_table_row(
        &self,
        request: InsertTableRowRequest,
    ) -> SdkResult<TableRowResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new row to the table.
    pub async fn insert_table_row_online(
        &self,
        request: InsertTableRowOnlineRequest,
    ) -> SdkResult<InsertTableRowOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Insert a watermark to the document.
    pub async fn insert_watermark(
        &self,
        request: InsertWatermarkRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new watermark image to the document.
    pub async fn insert_watermark_image(
        &self,
        request: InsertWatermarkImageRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new watermark image to the document.
    pub async fn insert_watermark_image_online(
        &self,
        request: InsertWatermarkImageOnlineRequest,
    ) -> SdkResult<InsertWatermarkImageOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Insert a watermark to the document.
    pub async fn insert_watermark_online(
        &self,
        request: InsertWatermarkOnlineRequest,
    ) -> SdkResult<InsertWatermarkOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new watermark text to the document.
    pub async fn insert_watermark_text(
        &self,
        request: InsertWatermarkTextRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Inserts a new watermark text to the document.
    pub async fn insert_watermark_text_online(
        &self,
        request: InsertWatermarkTextOnlineRequest,
    ) -> SdkResult<InsertWatermarkTextOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Links headers / footers of the section to the previous one.
    pub async fn link_header_footers_to_previous(
        &self,
        request: LinkHeaderFootersToPreviousRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Downloads a document from the Web using URL and saves it to cloud storage in the specified format.
    pub async fn load_web_document(
        &self,
        request: LoadWebDocumentRequest,
    ) -> SdkResult<SaveResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Downloads a document from the Web using URL and saves it to cloud storage in the specified format.
    pub async fn load_web_document_online(
        &self,
        request: LoadWebDocumentOnlineRequest,
    ) -> SdkResult<LoadWebDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Merge the section with the next one.
    pub async fn merge_with_next(
        &self,
        request: MergeWithNextRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Merge the section with the next one.
    pub async fn merge_with_next_online(
        &self,
        request: MergeWithNextOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Move file.
    pub async fn move_file(
        &self,
        request: MoveFileRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Move folder.
    pub async fn move_folder(
        &self,
        request: MoveFolderRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Applies document content optimization options, specific to a particular versions of Microsoft Word.
    pub async fn optimize_document(
        &self,
        request: OptimizeDocumentRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Applies document content optimization options, specific to a particular versions of Microsoft Word.
    pub async fn optimize_document_online(
        &self,
        request: OptimizeDocumentOnlineRequest,
    ) -> SdkResult<HashMap<String, Vec<u8>>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Changes the document protection. The previous protection will be overwritten if it exist.
    pub async fn protect_document(
        &self,
        request: ProtectDocumentRequest,
    ) -> SdkResult<ProtectionDataResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Changes the document protection. The previous protection will be overwritten if it exist.
    pub async fn protect_document_online(
        &self,
        request: ProtectDocumentOnlineRequest,
    ) -> SdkResult<ProtectDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Rejects all revisions in the document.
    pub async fn reject_all_revisions(
        &self,
        request: RejectAllRevisionsRequest,
    ) -> SdkResult<RevisionsModificationResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Rejects all revisions in the document.
    pub async fn reject_all_revisions_online(
        &self,
        request: RejectAllRevisionsOnlineRequest,
    ) -> SdkResult<RejectAllRevisionsOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all signatures of the document.
    pub async fn remove_all_signatures(
        &self,
        request: RemoveAllSignaturesRequest,
    ) -> SdkResult<SignatureCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes all signatures of the document.
    pub async fn remove_all_signatures_online(
        &self,
        request: RemoveAllSignaturesOnlineRequest,
    ) -> SdkResult<RemoveAllSignaturesOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a range from the document.
    pub async fn remove_range(
        &self,
        request: RemoveRangeRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes a range from the document.
    pub async fn remove_range_online(
        &self,
        request: RemoveRangeOnlineRequest,
    ) -> SdkResult<RemoveRangeOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a DrawingObject to the specified format.
    pub async fn render_drawing_object(
        &self,
        request: RenderDrawingObjectRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a DrawingObject to the specified format.
    pub async fn render_drawing_object_online(
        &self,
        request: RenderDrawingObjectOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders an OfficeMath object to the specified format.
    pub async fn render_math_object(
        &self,
        request: RenderMathObjectRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders an OfficeMath object to the specified format.
    pub async fn render_math_object_online(
        &self,
        request: RenderMathObjectOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a page to the specified format.
    pub async fn render_page(
        &self,
        request: RenderPageRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a page to the specified format.
    pub async fn render_page_online(
        &self,
        request: RenderPageOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a paragraph to the specified format.
    pub async fn render_paragraph(
        &self,
        request: RenderParagraphRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a paragraph to the specified format.
    pub async fn render_paragraph_online(
        &self,
        request: RenderParagraphOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a table to the specified format.
    pub async fn render_table(
        &self,
        request: RenderTableRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Renders a table to the specified format.
    pub async fn render_table_online(
        &self,
        request: RenderTableOnlineRequest,
    ) -> SdkResult<Vec<u8>> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Replaces text in the document.
    pub async fn replace_text(
        &self,
        request: ReplaceTextRequest,
    ) -> SdkResult<ReplaceTextResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Replaces text in the document.
    pub async fn replace_text_online(
        &self,
        request: ReplaceTextOnlineRequest,
    ) -> SdkResult<ReplaceTextOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Replaces a range with text in the document.
    pub async fn replace_with_text(
        &self,
        request: ReplaceWithTextRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Replaces a range with text in the document.
    pub async fn replace_with_text_online(
        &self,
        request: ReplaceWithTextOnlineRequest,
    ) -> SdkResult<ReplaceWithTextOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Clears the font cache.
    pub async fn reset_cache(
        &self,
        request: ResetCacheRequest,
    ) -> SdkResult<()> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document in cloud storage to the specified format.
    pub async fn save_as(
        &self,
        request: SaveAsRequest,
    ) -> SdkResult<SaveResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document in cloud storage to the specified format.
    pub async fn save_as_online(
        &self,
        request: SaveAsOnlineRequest,
    ) -> SdkResult<SaveAsOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Saves a range as a new document.
    pub async fn save_as_range(
        &self,
        request: SaveAsRangeRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Saves a range as a new document.
    pub async fn save_as_range_online(
        &self,
        request: SaveAsRangeOnlineRequest,
    ) -> SdkResult<SaveAsRangeOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document in cloud storage to TIFF format using detailed conversion settings.
    pub async fn save_as_tiff(
        &self,
        request: SaveAsTiffRequest,
    ) -> SdkResult<SaveResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Converts a document in cloud storage to TIFF format using detailed conversion settings.
    pub async fn save_as_tiff_online(
        &self,
        request: SaveAsTiffOnlineRequest,
    ) -> SdkResult<SaveAsTiffOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Searches text, specified by the regular expression, in the document.
    pub async fn search(
        &self,
        request: SearchRequest,
    ) -> SdkResult<SearchResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Searches text, specified by the regular expression, in the document.
    pub async fn search_online(
        &self,
        request: SearchOnlineRequest,
    ) -> SdkResult<SearchResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Signs the document with given certificate.
    pub async fn sign_document(
        &self,
        request: SignDocumentRequest,
    ) -> SdkResult<SignatureCollectionResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Signs the document with given certificate.
    pub async fn sign_document_online(
        &self,
        request: SignDocumentOnlineRequest,
    ) -> SdkResult<SignDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Splits a document into parts and saves them in the specified format.
    pub async fn split_document(
        &self,
        request: SplitDocumentRequest,
    ) -> SdkResult<SplitDocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Splits a document into parts and saves them in the specified format.
    pub async fn split_document_job(
        &self,
        request: SplitDocumentJobRequest,
    ) -> SdkResult<JobHandler<SplitDocumentRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Splits a document into parts and saves them in the specified format.
    pub async fn split_document_online(
        &self,
        request: SplitDocumentOnlineRequest,
    ) -> SdkResult<SplitDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Splits a document into parts and saves them in the specified format.
    pub async fn split_document_online_job(
        &self,
        request: SplitDocumentOnlineJobRequest,
    ) -> SdkResult<JobHandler<SplitDocumentOnlineRequest>> {
        let response = self.client.execute(&request).await?;
        let info = request.parse_response(response).await?;
        Ok(JobHandler::new(
            self.client.clone(),
            request.into_original(),
            info,
        ))
    }

    /// Translate a node id to a node path.
    pub async fn translate_node_id(
        &self,
        request: TranslateNodeIdRequest,
    ) -> SdkResult<TranslateNodeIdResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Translate a node id to a node path.
    pub async fn translate_node_id_online(
        &self,
        request: TranslateNodeIdOnlineRequest,
    ) -> SdkResult<TranslateNodeIdResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes protection from the document.
    pub async fn unprotect_document(
        &self,
        request: UnprotectDocumentRequest,
    ) -> SdkResult<ProtectionDataResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Removes protection from the document.
    pub async fn unprotect_document_online(
        &self,
        request: UnprotectDocumentOnlineRequest,
    ) -> SdkResult<UnprotectDocumentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a bookmark in the document.
    pub async fn update_bookmark(
        &self,
        request: UpdateBookmarkRequest,
    ) -> SdkResult<BookmarkResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a bookmark in the document.
    pub async fn update_bookmark_online(
        &self,
        request: UpdateBookmarkOnlineRequest,
    ) -> SdkResult<UpdateBookmarkOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a border in the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn update_border(
        &self,
        request: UpdateBorderRequest,
    ) -> SdkResult<BorderResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a border in the document node.
    /// The 'nodePath' parameter should refer to a paragraph, a cell or a row.
    pub async fn update_border_online(
        &self,
        request: UpdateBorderOnlineRequest,
    ) -> SdkResult<UpdateBorderOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a comment in the document.
    pub async fn update_comment(
        &self,
        request: UpdateCommentRequest,
    ) -> SdkResult<CommentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a comment in the document.
    pub async fn update_comment_online(
        &self,
        request: UpdateCommentOnlineRequest,
    ) -> SdkResult<UpdateCommentOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the custom xml part in the document.
    pub async fn update_custom_xml_part(
        &self,
        request: UpdateCustomXmlPartRequest,
    ) -> SdkResult<CustomXmlPartResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the custom xml part in the document.
    pub async fn update_custom_xml_part_online(
        &self,
        request: UpdateCustomXmlPartOnlineRequest,
    ) -> SdkResult<UpdateCustomXmlPartOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a DrawingObject in the document node.
    pub async fn update_drawing_object(
        &self,
        request: UpdateDrawingObjectRequest,
    ) -> SdkResult<DrawingObjectResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a DrawingObject in the document node.
    pub async fn update_drawing_object_online(
        &self,
        request: UpdateDrawingObjectOnlineRequest,
    ) -> SdkResult<UpdateDrawingObjectOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a field in the document node.
    pub async fn update_field(
        &self,
        request: UpdateFieldRequest,
    ) -> SdkResult<FieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a field in the document node.
    pub async fn update_field_online(
        &self,
        request: UpdateFieldOnlineRequest,
    ) -> SdkResult<UpdateFieldOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reevaluates field values in the document.
    pub async fn update_fields(
        &self,
        request: UpdateFieldsRequest,
    ) -> SdkResult<DocumentResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Reevaluates field values in the document.
    pub async fn update_fields_online(
        &self,
        request: UpdateFieldsOnlineRequest,
    ) -> SdkResult<UpdateFieldsOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a footnote in the document node.
    pub async fn update_footnote(
        &self,
        request: UpdateFootnoteRequest,
    ) -> SdkResult<FootnoteResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a footnote in the document node.
    pub async fn update_footnote_online(
        &self,
        request: UpdateFootnoteOnlineRequest,
    ) -> SdkResult<UpdateFootnoteOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a form field in the document node.
    pub async fn update_form_field(
        &self,
        request: UpdateFormFieldRequest,
    ) -> SdkResult<FormFieldResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a form field in the document node.
    pub async fn update_form_field_online(
        &self,
        request: UpdateFormFieldOnlineRequest,
    ) -> SdkResult<UpdateFormFieldOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a list in the document.
    pub async fn update_list(
        &self,
        request: UpdateListRequest,
    ) -> SdkResult<ListResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the level of a List element in the document.
    pub async fn update_list_level(
        &self,
        request: UpdateListLevelRequest,
    ) -> SdkResult<ListResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the level of a List element in the document.
    pub async fn update_list_level_online(
        &self,
        request: UpdateListLevelOnlineRequest,
    ) -> SdkResult<UpdateListLevelOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a list in the document.
    pub async fn update_list_online(
        &self,
        request: UpdateListOnlineRequest,
    ) -> SdkResult<UpdateListOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a paragraph in the document node.
    pub async fn update_paragraph_format(
        &self,
        request: UpdateParagraphFormatRequest,
    ) -> SdkResult<ParagraphFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a paragraph in the document node.
    pub async fn update_paragraph_format_online(
        &self,
        request: UpdateParagraphFormatOnlineRequest,
    ) -> SdkResult<UpdateParagraphFormatOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a paragraph list in the document node.
    pub async fn update_paragraph_list_format(
        &self,
        request: UpdateParagraphListFormatRequest,
    ) -> SdkResult<ParagraphListFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a paragraph list in the document node.
    pub async fn update_paragraph_list_format_online(
        &self,
        request: UpdateParagraphListFormatOnlineRequest,
    ) -> SdkResult<UpdateParagraphListFormatOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a Run object in the paragraph.
    pub async fn update_run(
        &self,
        request: UpdateRunRequest,
    ) -> SdkResult<RunResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the font properties of a Run object in the paragraph.
    pub async fn update_run_font(
        &self,
        request: UpdateRunFontRequest,
    ) -> SdkResult<FontResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the font properties of a Run object in the paragraph.
    pub async fn update_run_font_online(
        &self,
        request: UpdateRunFontOnlineRequest,
    ) -> SdkResult<UpdateRunFontOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a Run object in the paragraph.
    pub async fn update_run_online(
        &self,
        request: UpdateRunOnlineRequest,
    ) -> SdkResult<UpdateRunOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the page setup of a section in the document.
    pub async fn update_section_page_setup(
        &self,
        request: UpdateSectionPageSetupRequest,
    ) -> SdkResult<SectionPageSetupResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the page setup of a section in the document.
    pub async fn update_section_page_setup_online(
        &self,
        request: UpdateSectionPageSetupOnlineRequest,
    ) -> SdkResult<UpdateSectionPageSetupOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a StructuredDocumentTag (SDT) in the document node.
    pub async fn update_structured_document_tag(
        &self,
        request: UpdateStructuredDocumentTagRequest,
    ) -> SdkResult<StructuredDocumentTagResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a StructuredDocumentTag (SDT) in the document node.
    pub async fn update_structured_document_tag_online(
        &self,
        request: UpdateStructuredDocumentTagOnlineRequest,
    ) -> SdkResult<UpdateStructuredDocumentTagOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a style in the document.
    pub async fn update_style(
        &self,
        request: UpdateStyleRequest,
    ) -> SdkResult<StyleResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates a style in the document.
    pub async fn update_style_online(
        &self,
        request: UpdateStyleOnlineRequest,
    ) -> SdkResult<UpdateStyleOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a cell in the table row.
    pub async fn update_table_cell_format(
        &self,
        request: UpdateTableCellFormatRequest,
    ) -> SdkResult<TableCellFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a cell in the table row.
    pub async fn update_table_cell_format_online(
        &self,
        request: UpdateTableCellFormatOnlineRequest,
    ) -> SdkResult<UpdateTableCellFormatOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates properties of a table in the document node.
    pub async fn update_table_properties(
        &self,
        request: UpdateTablePropertiesRequest,
    ) -> SdkResult<TablePropertiesResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates properties of a table in the document node.
    pub async fn update_table_properties_online(
        &self,
        request: UpdateTablePropertiesOnlineRequest,
    ) -> SdkResult<UpdateTablePropertiesOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a table row.
    pub async fn update_table_row_format(
        &self,
        request: UpdateTableRowFormatRequest,
    ) -> SdkResult<TableRowFormatResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Updates the formatting properties of a table row.
    pub async fn update_table_row_format_online(
        &self,
        request: UpdateTableRowFormatOnlineRequest,
    ) -> SdkResult<UpdateTableRowFormatOnlineResponse> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    /// Upload file.
    pub async fn upload_file(
        &self,
        request: UploadFileRequest,
    ) -> SdkResult<FilesUploadResult> {
        let response = self.client.execute(&request).await?;
        request.parse_response(response).await
    }

    pub async fn batch(
        &self,
        requests: Vec<BatchRequest>,
        display_intermediate_results: bool,
    ) -> SdkResult<Vec<BatchResult>> {
        execute_batch(&self.client, &requests, display_intermediate_results).await
    }

    pub async fn encrypt_password(&self, value: &str) -> SdkResult<String> {
        self.client.encrypt_password(value).await
    }
}