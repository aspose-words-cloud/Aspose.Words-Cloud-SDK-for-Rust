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

use std::{any::Any, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::{SdkError, SdkResult};

use super::*;

/// Common behavior required by request models.
pub trait Model: erased_serde::Serialize + std::fmt::Debug + Send + Sync {
    fn validate(&self) -> SdkResult<()>;

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>);

    fn as_any(&self) -> &dyn Any;
}

erased_serde::serialize_trait_object!(Model);

/// Owns a polymorphic model while keeping dynamic dispatch restricted to abstract schemas.
pub struct ModelBox(Box<dyn Model>);

impl ModelBox {
    pub fn validate(&self) -> SdkResult<()> {
        self.0.validate()
    }

    pub fn collect_file_references<'a>(
        &'a self,
        output: &mut Vec<&'a FileReference>,
    ) {
        self.0.collect_file_references(output);
    }
}

impl fmt::Debug for ModelBox {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<T> From<T> for ModelBox
where
    T: Model + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl AsRef<dyn Model> for ModelBox {
    fn as_ref(&self) -> &(dyn Model + 'static) {
        self.0.as_ref()
    }
}

impl Serialize for ModelBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        erased_serde::serialize(self.0.as_ref(), serializer)
    }
}

fn deserialize_model_value(value: Value) -> SdkResult<ModelBox> {
    let model_type = value
        .get("$type")
        .and_then(Value::as_str)
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .ok_or_else(|| {
            SdkError::InvalidResponse(
                "a polymorphic model response does not contain a $type field".to_owned(),
            )
        })?;

    match model_type {
        "ApiError" => Ok(serde_json::from_value::<ApiError>(value)?.into()),
        "AvailableFontsResponse" => Ok(serde_json::from_value::<AvailableFontsResponse>(value)?.into()),
        "Azw3SaveOptionsData" => Ok(serde_json::from_value::<Azw3SaveOptionsData>(value)?.into()),
        "BmpSaveOptionsData" => Ok(serde_json::from_value::<BmpSaveOptionsData>(value)?.into()),
        "Bookmark" => Ok(serde_json::from_value::<Bookmark>(value)?.into()),
        "BookmarkData" => Ok(serde_json::from_value::<BookmarkData>(value)?.into()),
        "BookmarkInsert" => Ok(serde_json::from_value::<BookmarkInsert>(value)?.into()),
        "BookmarkResponse" => Ok(serde_json::from_value::<BookmarkResponse>(value)?.into()),
        "Bookmarks" => Ok(serde_json::from_value::<Bookmarks>(value)?.into()),
        "BookmarksOutlineLevelData" => Ok(serde_json::from_value::<BookmarksOutlineLevelData>(value)?.into()),
        "BookmarksResponse" => Ok(serde_json::from_value::<BookmarksResponse>(value)?.into()),
        "Border" => Ok(serde_json::from_value::<Border>(value)?.into()),
        "BorderResponse" => Ok(serde_json::from_value::<BorderResponse>(value)?.into()),
        "BordersCollection" => Ok(serde_json::from_value::<BordersCollection>(value)?.into()),
        "BordersResponse" => Ok(serde_json::from_value::<BordersResponse>(value)?.into()),
        "Comment" => Ok(serde_json::from_value::<Comment>(value)?.into()),
        "CommentInsert" => Ok(serde_json::from_value::<CommentInsert>(value)?.into()),
        "CommentLink" => Ok(serde_json::from_value::<CommentLink>(value)?.into()),
        "CommentRangeEnd" => Ok(serde_json::from_value::<CommentRangeEnd>(value)?.into()),
        "CommentRangeStart" => Ok(serde_json::from_value::<CommentRangeStart>(value)?.into()),
        "CommentResponse" => Ok(serde_json::from_value::<CommentResponse>(value)?.into()),
        "CommentsCollection" => Ok(serde_json::from_value::<CommentsCollection>(value)?.into()),
        "CommentsResponse" => Ok(serde_json::from_value::<CommentsResponse>(value)?.into()),
        "CommentUpdate" => Ok(serde_json::from_value::<CommentUpdate>(value)?.into()),
        "CompareData" => Ok(serde_json::from_value::<CompareData>(value)?.into()),
        "CompareOptions" => Ok(serde_json::from_value::<CompareOptions>(value)?.into()),
        "CompressOptions" => Ok(serde_json::from_value::<CompressOptions>(value)?.into()),
        "CompressResponse" => Ok(serde_json::from_value::<CompressResponse>(value)?.into()),
        "CsvDataLoadOptions" => Ok(serde_json::from_value::<CsvDataLoadOptions>(value)?.into()),
        "CustomXmlPart" => Ok(serde_json::from_value::<CustomXmlPart>(value)?.into()),
        "CustomXmlPartInsert" => Ok(serde_json::from_value::<CustomXmlPartInsert>(value)?.into()),
        "CustomXmlPartLink" => Ok(serde_json::from_value::<CustomXmlPartLink>(value)?.into()),
        "CustomXmlPartResponse" => Ok(serde_json::from_value::<CustomXmlPartResponse>(value)?.into()),
        "CustomXmlPartsCollection" => Ok(serde_json::from_value::<CustomXmlPartsCollection>(value)?.into()),
        "CustomXmlPartsResponse" => Ok(serde_json::from_value::<CustomXmlPartsResponse>(value)?.into()),
        "CustomXmlPartUpdate" => Ok(serde_json::from_value::<CustomXmlPartUpdate>(value)?.into()),
        "DigitalSignatureDetails" => Ok(serde_json::from_value::<DigitalSignatureDetails>(value)?.into()),
        "DoclingSaveOptionsData" => Ok(serde_json::from_value::<DoclingSaveOptionsData>(value)?.into()),
        "DocmSaveOptionsData" => Ok(serde_json::from_value::<DocmSaveOptionsData>(value)?.into()),
        "DocSaveOptionsData" => Ok(serde_json::from_value::<DocSaveOptionsData>(value)?.into()),
        "Document" => Ok(serde_json::from_value::<Document>(value)?.into()),
        "DocumentEntry" => Ok(serde_json::from_value::<DocumentEntry>(value)?.into()),
        "DocumentEntryList" => Ok(serde_json::from_value::<DocumentEntryList>(value)?.into()),
        "DocumentPosition" => Ok(serde_json::from_value::<DocumentPosition>(value)?.into()),
        "DocumentProperties" => Ok(serde_json::from_value::<DocumentProperties>(value)?.into()),
        "DocumentPropertiesResponse" => Ok(serde_json::from_value::<DocumentPropertiesResponse>(value)?.into()),
        "DocumentProperty" => Ok(serde_json::from_value::<DocumentProperty>(value)?.into()),
        "DocumentPropertyCreateOrUpdate" => Ok(serde_json::from_value::<DocumentPropertyCreateOrUpdate>(value)?.into()),
        "DocumentPropertyResponse" => Ok(serde_json::from_value::<DocumentPropertyResponse>(value)?.into()),
        "DocumentResponse" => Ok(serde_json::from_value::<DocumentResponse>(value)?.into()),
        "DocumentStatData" => Ok(serde_json::from_value::<DocumentStatData>(value)?.into()),
        "DocxSaveOptionsData" => Ok(serde_json::from_value::<DocxSaveOptionsData>(value)?.into()),
        "DotmSaveOptionsData" => Ok(serde_json::from_value::<DotmSaveOptionsData>(value)?.into()),
        "DotSaveOptionsData" => Ok(serde_json::from_value::<DotSaveOptionsData>(value)?.into()),
        "DotxSaveOptionsData" => Ok(serde_json::from_value::<DotxSaveOptionsData>(value)?.into()),
        "DownsampleOptionsData" => Ok(serde_json::from_value::<DownsampleOptionsData>(value)?.into()),
        "DrawingObject" => Ok(serde_json::from_value::<DrawingObject>(value)?.into()),
        "DrawingObjectCollection" => Ok(serde_json::from_value::<DrawingObjectCollection>(value)?.into()),
        "DrawingObjectInsert" => Ok(serde_json::from_value::<DrawingObjectInsert>(value)?.into()),
        "DrawingObjectLink" => Ok(serde_json::from_value::<DrawingObjectLink>(value)?.into()),
        "DrawingObjectResponse" => Ok(serde_json::from_value::<DrawingObjectResponse>(value)?.into()),
        "DrawingObjectsResponse" => Ok(serde_json::from_value::<DrawingObjectsResponse>(value)?.into()),
        "DrawingObjectUpdate" => Ok(serde_json::from_value::<DrawingObjectUpdate>(value)?.into()),
        "EmfSaveOptionsData" => Ok(serde_json::from_value::<EmfSaveOptionsData>(value)?.into()),
        "EpsSaveOptionsData" => Ok(serde_json::from_value::<EpsSaveOptionsData>(value)?.into()),
        "EpubSaveOptionsData" => Ok(serde_json::from_value::<EpubSaveOptionsData>(value)?.into()),
        "Error" => Ok(serde_json::from_value::<Error>(value)?.into()),
        "ErrorDetails" => Ok(serde_json::from_value::<ErrorDetails>(value)?.into()),
        "Field" => Ok(serde_json::from_value::<Field>(value)?.into()),
        "FieldCollection" => Ok(serde_json::from_value::<FieldCollection>(value)?.into()),
        "FieldInsert" => Ok(serde_json::from_value::<FieldInsert>(value)?.into()),
        "FieldLink" => Ok(serde_json::from_value::<FieldLink>(value)?.into()),
        "FieldNames" => Ok(serde_json::from_value::<FieldNames>(value)?.into()),
        "FieldNamesResponse" => Ok(serde_json::from_value::<FieldNamesResponse>(value)?.into()),
        "FieldOptions" => Ok(serde_json::from_value::<FieldOptions>(value)?.into()),
        "FieldResponse" => Ok(serde_json::from_value::<FieldResponse>(value)?.into()),
        "FieldsResponse" => Ok(serde_json::from_value::<FieldsResponse>(value)?.into()),
        "FieldUpdate" => Ok(serde_json::from_value::<FieldUpdate>(value)?.into()),
        "FileLink" => Ok(serde_json::from_value::<FileLink>(value)?.into()),
        "FilesList" => Ok(serde_json::from_value::<FilesList>(value)?.into()),
        "FilesUploadResult" => Ok(serde_json::from_value::<FilesUploadResult>(value)?.into()),
        "FlatOpcMacroSaveOptionsData" => Ok(serde_json::from_value::<FlatOpcMacroSaveOptionsData>(value)?.into()),
        "FlatOpcSaveOptionsData" => Ok(serde_json::from_value::<FlatOpcSaveOptionsData>(value)?.into()),
        "FlatOpcTemplateMacroSaveOptionsData" => Ok(serde_json::from_value::<FlatOpcTemplateMacroSaveOptionsData>(value)?.into()),
        "FlatOpcTemplateSaveOptionsData" => Ok(serde_json::from_value::<FlatOpcTemplateSaveOptionsData>(value)?.into()),
        "Font" => Ok(serde_json::from_value::<Font>(value)?.into()),
        "FontInfo" => Ok(serde_json::from_value::<FontInfo>(value)?.into()),
        "FontResponse" => Ok(serde_json::from_value::<FontResponse>(value)?.into()),
        "Footnote" => Ok(serde_json::from_value::<Footnote>(value)?.into()),
        "FootnoteCollection" => Ok(serde_json::from_value::<FootnoteCollection>(value)?.into()),
        "FootnoteInsert" => Ok(serde_json::from_value::<FootnoteInsert>(value)?.into()),
        "FootnoteLink" => Ok(serde_json::from_value::<FootnoteLink>(value)?.into()),
        "FootnoteResponse" => Ok(serde_json::from_value::<FootnoteResponse>(value)?.into()),
        "FootnotesResponse" => Ok(serde_json::from_value::<FootnotesResponse>(value)?.into()),
        "FootnotesStatData" => Ok(serde_json::from_value::<FootnotesStatData>(value)?.into()),
        "FootnoteUpdate" => Ok(serde_json::from_value::<FootnoteUpdate>(value)?.into()),
        "FormFieldCheckbox" => Ok(serde_json::from_value::<FormFieldCheckbox>(value)?.into()),
        "FormFieldCheckboxLink" => Ok(serde_json::from_value::<FormFieldCheckboxLink>(value)?.into()),
        "FormFieldCollection" => Ok(serde_json::from_value::<FormFieldCollection>(value)?.into()),
        "FormFieldDropDown" => Ok(serde_json::from_value::<FormFieldDropDown>(value)?.into()),
        "FormFieldDropDownLink" => Ok(serde_json::from_value::<FormFieldDropDownLink>(value)?.into()),
        "FormFieldResponse" => Ok(serde_json::from_value::<FormFieldResponse>(value)?.into()),
        "FormFieldsResponse" => Ok(serde_json::from_value::<FormFieldsResponse>(value)?.into()),
        "FormFieldTextInput" => Ok(serde_json::from_value::<FormFieldTextInput>(value)?.into()),
        "FormFieldTextInputLink" => Ok(serde_json::from_value::<FormFieldTextInputLink>(value)?.into()),
        "GifSaveOptionsData" => Ok(serde_json::from_value::<GifSaveOptionsData>(value)?.into()),
        "HeaderFooter" => Ok(serde_json::from_value::<HeaderFooter>(value)?.into()),
        "HeaderFooterLink" => Ok(serde_json::from_value::<HeaderFooterLink>(value)?.into()),
        "HeaderFooterLinkCollection" => Ok(serde_json::from_value::<HeaderFooterLinkCollection>(value)?.into()),
        "HeaderFooterResponse" => Ok(serde_json::from_value::<HeaderFooterResponse>(value)?.into()),
        "HeaderFootersResponse" => Ok(serde_json::from_value::<HeaderFootersResponse>(value)?.into()),
        "HtmlFixedSaveOptionsData" => Ok(serde_json::from_value::<HtmlFixedSaveOptionsData>(value)?.into()),
        "HtmlSaveOptionsData" => Ok(serde_json::from_value::<HtmlSaveOptionsData>(value)?.into()),
        "Hyperlink" => Ok(serde_json::from_value::<Hyperlink>(value)?.into()),
        "HyperlinkResponse" => Ok(serde_json::from_value::<HyperlinkResponse>(value)?.into()),
        "Hyperlinks" => Ok(serde_json::from_value::<Hyperlinks>(value)?.into()),
        "HyperlinksResponse" => Ok(serde_json::from_value::<HyperlinksResponse>(value)?.into()),
        "ImageEntry" => Ok(serde_json::from_value::<ImageEntry>(value)?.into()),
        "ImageEntryList" => Ok(serde_json::from_value::<ImageEntryList>(value)?.into()),
        "InfoAdditionalItem" => Ok(serde_json::from_value::<InfoAdditionalItem>(value)?.into()),
        "InfoResponse" => Ok(serde_json::from_value::<InfoResponse>(value)?.into()),
        "JobInfo" => Ok(serde_json::from_value::<JobInfo>(value)?.into()),
        "JpegSaveOptionsData" => Ok(serde_json::from_value::<JpegSaveOptionsData>(value)?.into()),
        "JsonDataLoadOptions" => Ok(serde_json::from_value::<JsonDataLoadOptions>(value)?.into()),
        "Link" => Ok(serde_json::from_value::<Link>(value)?.into()),
        "LinkElement" => Ok(serde_json::from_value::<LinkElement>(value)?.into()),
        "ListFormat" => Ok(serde_json::from_value::<ListFormat>(value)?.into()),
        "ListFormatUpdate" => Ok(serde_json::from_value::<ListFormatUpdate>(value)?.into()),
        "ListInfo" => Ok(serde_json::from_value::<ListInfo>(value)?.into()),
        "ListInsert" => Ok(serde_json::from_value::<ListInsert>(value)?.into()),
        "ListLevel" => Ok(serde_json::from_value::<ListLevel>(value)?.into()),
        "ListLevels" => Ok(serde_json::from_value::<ListLevels>(value)?.into()),
        "ListLevelUpdate" => Ok(serde_json::from_value::<ListLevelUpdate>(value)?.into()),
        "ListResponse" => Ok(serde_json::from_value::<ListResponse>(value)?.into()),
        "Lists" => Ok(serde_json::from_value::<Lists>(value)?.into()),
        "ListsResponse" => Ok(serde_json::from_value::<ListsResponse>(value)?.into()),
        "ListUpdate" => Ok(serde_json::from_value::<ListUpdate>(value)?.into()),
        "LoadWebDocumentData" => Ok(serde_json::from_value::<LoadWebDocumentData>(value)?.into()),
        "MarkdownSaveOptionsData" => Ok(serde_json::from_value::<MarkdownSaveOptionsData>(value)?.into()),
        "MetafileRenderingOptionsData" => Ok(serde_json::from_value::<MetafileRenderingOptionsData>(value)?.into()),
        "MhtmlSaveOptionsData" => Ok(serde_json::from_value::<MhtmlSaveOptionsData>(value)?.into()),
        "ModificationOperationResult" => Ok(serde_json::from_value::<ModificationOperationResult>(value)?.into()),
        "NodeLink" => Ok(serde_json::from_value::<NodeLink>(value)?.into()),
        "OdtSaveOptionsData" => Ok(serde_json::from_value::<OdtSaveOptionsData>(value)?.into()),
        "OfficeMathLink" => Ok(serde_json::from_value::<OfficeMathLink>(value)?.into()),
        "OfficeMathObject" => Ok(serde_json::from_value::<OfficeMathObject>(value)?.into()),
        "OfficeMathObjectResponse" => Ok(serde_json::from_value::<OfficeMathObjectResponse>(value)?.into()),
        "OfficeMathObjectsCollection" => Ok(serde_json::from_value::<OfficeMathObjectsCollection>(value)?.into()),
        "OfficeMathObjectsResponse" => Ok(serde_json::from_value::<OfficeMathObjectsResponse>(value)?.into()),
        "OpenXpsSaveOptionsData" => Ok(serde_json::from_value::<OpenXpsSaveOptionsData>(value)?.into()),
        "OptimizationOptions" => Ok(serde_json::from_value::<OptimizationOptions>(value)?.into()),
        "OttSaveOptionsData" => Ok(serde_json::from_value::<OttSaveOptionsData>(value)?.into()),
        "OutlineOptionsData" => Ok(serde_json::from_value::<OutlineOptionsData>(value)?.into()),
        "PageNumber" => Ok(serde_json::from_value::<PageNumber>(value)?.into()),
        "PageSetup" => Ok(serde_json::from_value::<PageSetup>(value)?.into()),
        "PageStatData" => Ok(serde_json::from_value::<PageStatData>(value)?.into()),
        "Paragraph" => Ok(serde_json::from_value::<Paragraph>(value)?.into()),
        "ParagraphFormat" => Ok(serde_json::from_value::<ParagraphFormat>(value)?.into()),
        "ParagraphFormatResponse" => Ok(serde_json::from_value::<ParagraphFormatResponse>(value)?.into()),
        "ParagraphFormatUpdate" => Ok(serde_json::from_value::<ParagraphFormatUpdate>(value)?.into()),
        "ParagraphInsert" => Ok(serde_json::from_value::<ParagraphInsert>(value)?.into()),
        "ParagraphLink" => Ok(serde_json::from_value::<ParagraphLink>(value)?.into()),
        "ParagraphLinkCollection" => Ok(serde_json::from_value::<ParagraphLinkCollection>(value)?.into()),
        "ParagraphLinkCollectionResponse" => Ok(serde_json::from_value::<ParagraphLinkCollectionResponse>(value)?.into()),
        "ParagraphListFormatResponse" => Ok(serde_json::from_value::<ParagraphListFormatResponse>(value)?.into()),
        "ParagraphResponse" => Ok(serde_json::from_value::<ParagraphResponse>(value)?.into()),
        "PclSaveOptionsData" => Ok(serde_json::from_value::<PclSaveOptionsData>(value)?.into()),
        "PdfDigitalSignatureDetailsData" => Ok(serde_json::from_value::<PdfDigitalSignatureDetailsData>(value)?.into()),
        "PdfEncryptionDetailsData" => Ok(serde_json::from_value::<PdfEncryptionDetailsData>(value)?.into()),

        "PdfSaveOptionsData" => Ok(serde_json::from_value::<PdfSaveOptionsData>(value)?.into()),
        "PngSaveOptionsData" => Ok(serde_json::from_value::<PngSaveOptionsData>(value)?.into()),
        "PositionAfterNode" => Ok(serde_json::from_value::<PositionAfterNode>(value)?.into()),
        "PositionBeforeNode" => Ok(serde_json::from_value::<PositionBeforeNode>(value)?.into()),
        "PositionInsideNode" => Ok(serde_json::from_value::<PositionInsideNode>(value)?.into()),
        "PreferredWidth" => Ok(serde_json::from_value::<PreferredWidth>(value)?.into()),
        "ProtectionData" => Ok(serde_json::from_value::<ProtectionData>(value)?.into()),
        "ProtectionDataResponse" => Ok(serde_json::from_value::<ProtectionDataResponse>(value)?.into()),
        "ProtectionRequest" => Ok(serde_json::from_value::<ProtectionRequest>(value)?.into()),
        "ProtectionRequestV2" => Ok(serde_json::from_value::<ProtectionRequestV2>(value)?.into()),
        "PsSaveOptionsData" => Ok(serde_json::from_value::<PsSaveOptionsData>(value)?.into()),
        "PublicKeyResponse" => Ok(serde_json::from_value::<PublicKeyResponse>(value)?.into()),
        "RangeDocument" => Ok(serde_json::from_value::<RangeDocument>(value)?.into()),
        "RangeTextResponse" => Ok(serde_json::from_value::<RangeTextResponse>(value)?.into()),
        "ReplaceRange" => Ok(serde_json::from_value::<ReplaceRange>(value)?.into()),
        "ReplaceTextParameters" => Ok(serde_json::from_value::<ReplaceTextParameters>(value)?.into()),
        "ReplaceTextResponse" => Ok(serde_json::from_value::<ReplaceTextResponse>(value)?.into()),

        "ReportEngineSettings" => Ok(serde_json::from_value::<ReportEngineSettings>(value)?.into()),
        "Revision" => Ok(serde_json::from_value::<Revision>(value)?.into()),
        "RevisionCollection" => Ok(serde_json::from_value::<RevisionCollection>(value)?.into()),
        "RevisionsModificationResponse" => Ok(serde_json::from_value::<RevisionsModificationResponse>(value)?.into()),
        "RevisionsResponse" => Ok(serde_json::from_value::<RevisionsResponse>(value)?.into()),
        "RtfSaveOptionsData" => Ok(serde_json::from_value::<RtfSaveOptionsData>(value)?.into()),
        "Run" => Ok(serde_json::from_value::<Run>(value)?.into()),
        "RunInsert" => Ok(serde_json::from_value::<RunInsert>(value)?.into()),
        "RunLink" => Ok(serde_json::from_value::<RunLink>(value)?.into()),
        "RunResponse" => Ok(serde_json::from_value::<RunResponse>(value)?.into()),
        "Runs" => Ok(serde_json::from_value::<Runs>(value)?.into()),
        "RunsResponse" => Ok(serde_json::from_value::<RunsResponse>(value)?.into()),
        "RunUpdate" => Ok(serde_json::from_value::<RunUpdate>(value)?.into()),
        "SaveResponse" => Ok(serde_json::from_value::<SaveResponse>(value)?.into()),
        "SaveResult" => Ok(serde_json::from_value::<SaveResult>(value)?.into()),
        "SearchResponse" => Ok(serde_json::from_value::<SearchResponse>(value)?.into()),
        "SearchResult" => Ok(serde_json::from_value::<SearchResult>(value)?.into()),
        "SearchResultsCollection" => Ok(serde_json::from_value::<SearchResultsCollection>(value)?.into()),
        "Section" => Ok(serde_json::from_value::<Section>(value)?.into()),
        "SectionLink" => Ok(serde_json::from_value::<SectionLink>(value)?.into()),
        "SectionLinkCollection" => Ok(serde_json::from_value::<SectionLinkCollection>(value)?.into()),
        "SectionLinkCollectionResponse" => Ok(serde_json::from_value::<SectionLinkCollectionResponse>(value)?.into()),
        "SectionPageSetupResponse" => Ok(serde_json::from_value::<SectionPageSetupResponse>(value)?.into()),
        "SectionResponse" => Ok(serde_json::from_value::<SectionResponse>(value)?.into()),
        "Shading" => Ok(serde_json::from_value::<Shading>(value)?.into()),
        "Signature" => Ok(serde_json::from_value::<Signature>(value)?.into()),
        "SignatureCollectionResponse" => Ok(serde_json::from_value::<SignatureCollectionResponse>(value)?.into()),
        "SignOptions" => Ok(serde_json::from_value::<SignOptions>(value)?.into()),
        "SplitDocumentResponse" => Ok(serde_json::from_value::<SplitDocumentResponse>(value)?.into()),
        "SplitDocumentResult" => Ok(serde_json::from_value::<SplitDocumentResult>(value)?.into()),
        "StatDataResponse" => Ok(serde_json::from_value::<StatDataResponse>(value)?.into()),
        "StorageFile" => Ok(serde_json::from_value::<StorageFile>(value)?.into()),
        "StoryChildNodes" => Ok(serde_json::from_value::<StoryChildNodes>(value)?.into()),
        "StructuredDocumentTag" => Ok(serde_json::from_value::<StructuredDocumentTag>(value)?.into()),
        "StructuredDocumentTagCollection" => Ok(serde_json::from_value::<StructuredDocumentTagCollection>(value)?.into()),
        "StructuredDocumentTagInsert" => Ok(serde_json::from_value::<StructuredDocumentTagInsert>(value)?.into()),
        "StructuredDocumentTagListItem" => Ok(serde_json::from_value::<StructuredDocumentTagListItem>(value)?.into()),
        "StructuredDocumentTagResponse" => Ok(serde_json::from_value::<StructuredDocumentTagResponse>(value)?.into()),
        "StructuredDocumentTagsResponse" => Ok(serde_json::from_value::<StructuredDocumentTagsResponse>(value)?.into()),
        "StructuredDocumentTagUpdate" => Ok(serde_json::from_value::<StructuredDocumentTagUpdate>(value)?.into()),
        "Style" => Ok(serde_json::from_value::<Style>(value)?.into()),
        "StyleApply" => Ok(serde_json::from_value::<StyleApply>(value)?.into()),
        "StyleCopy" => Ok(serde_json::from_value::<StyleCopy>(value)?.into()),
        "StyleInsert" => Ok(serde_json::from_value::<StyleInsert>(value)?.into()),
        "StyleResponse" => Ok(serde_json::from_value::<StyleResponse>(value)?.into()),
        "StylesResponse" => Ok(serde_json::from_value::<StylesResponse>(value)?.into()),
        "StyleUpdate" => Ok(serde_json::from_value::<StyleUpdate>(value)?.into()),
        "SvgSaveOptionsData" => Ok(serde_json::from_value::<SvgSaveOptionsData>(value)?.into()),
        "Table" => Ok(serde_json::from_value::<Table>(value)?.into()),
        "TableCell" => Ok(serde_json::from_value::<TableCell>(value)?.into()),
        "TableCellFormat" => Ok(serde_json::from_value::<TableCellFormat>(value)?.into()),
        "TableCellFormatResponse" => Ok(serde_json::from_value::<TableCellFormatResponse>(value)?.into()),
        "TableCellInsert" => Ok(serde_json::from_value::<TableCellInsert>(value)?.into()),
        "TableCellResponse" => Ok(serde_json::from_value::<TableCellResponse>(value)?.into()),
        "TableInsert" => Ok(serde_json::from_value::<TableInsert>(value)?.into()),
        "TableLink" => Ok(serde_json::from_value::<TableLink>(value)?.into()),
        "TableLinkCollection" => Ok(serde_json::from_value::<TableLinkCollection>(value)?.into()),
        "TableLinkCollectionResponse" => Ok(serde_json::from_value::<TableLinkCollectionResponse>(value)?.into()),
        "TableProperties" => Ok(serde_json::from_value::<TableProperties>(value)?.into()),
        "TablePropertiesResponse" => Ok(serde_json::from_value::<TablePropertiesResponse>(value)?.into()),
        "TableResponse" => Ok(serde_json::from_value::<TableResponse>(value)?.into()),
        "TableRow" => Ok(serde_json::from_value::<TableRow>(value)?.into()),
        "TableRowFormat" => Ok(serde_json::from_value::<TableRowFormat>(value)?.into()),
        "TableRowFormatResponse" => Ok(serde_json::from_value::<TableRowFormatResponse>(value)?.into()),
        "TableRowInsert" => Ok(serde_json::from_value::<TableRowInsert>(value)?.into()),
        "TableRowResponse" => Ok(serde_json::from_value::<TableRowResponse>(value)?.into()),
        "TabStop" => Ok(serde_json::from_value::<TabStop>(value)?.into()),
        "TabStopInsert" => Ok(serde_json::from_value::<TabStopInsert>(value)?.into()),
        "TabStopsResponse" => Ok(serde_json::from_value::<TabStopsResponse>(value)?.into()),
        "TextSaveOptionsData" => Ok(serde_json::from_value::<TextSaveOptionsData>(value)?.into()),
        "TiffSaveOptionsData" => Ok(serde_json::from_value::<TiffSaveOptionsData>(value)?.into()),
        "TimeZoneInfoData" => Ok(serde_json::from_value::<TimeZoneInfoData>(value)?.into()),
        "TranslateNodeIdResponse" => Ok(serde_json::from_value::<TranslateNodeIdResponse>(value)?.into()),
        "UserInformation" => Ok(serde_json::from_value::<UserInformation>(value)?.into()),
        "WatermarkDataImage" => Ok(serde_json::from_value::<WatermarkDataImage>(value)?.into()),
        "WatermarkDataText" => Ok(serde_json::from_value::<WatermarkDataText>(value)?.into()),
        "WatermarkText" => Ok(serde_json::from_value::<WatermarkText>(value)?.into()),
        "WordMLSaveOptionsData" => Ok(serde_json::from_value::<WordMLSaveOptionsData>(value)?.into()),
        "WordsApiErrorResponse" => Ok(serde_json::from_value::<WordsApiErrorResponse>(value)?.into()),
        "WordsApiLink" => Ok(serde_json::from_value::<WordsApiLink>(value)?.into()),
        "WordsResponse" => Ok(serde_json::from_value::<WordsResponse>(value)?.into()),
        "XamlFixedSaveOptionsData" => Ok(serde_json::from_value::<XamlFixedSaveOptionsData>(value)?.into()),
        "XamlFlowPackSaveOptionsData" => Ok(serde_json::from_value::<XamlFlowPackSaveOptionsData>(value)?.into()),
        "XamlFlowSaveOptionsData" => Ok(serde_json::from_value::<XamlFlowSaveOptionsData>(value)?.into()),
        "XmlColor" => Ok(serde_json::from_value::<XmlColor>(value)?.into()),
        "XmlDataLoadOptions" => Ok(serde_json::from_value::<XmlDataLoadOptions>(value)?.into()),
        "XpsSaveOptionsData" => Ok(serde_json::from_value::<XpsSaveOptionsData>(value)?.into()),
        value => Err(SdkError::InvalidResponse(format!(
            "unsupported polymorphic model type: {value}"
        ))),
    }
}

pub(crate) fn deserialize_optional_model<'de, D>(
    deserializer: D,
) -> Result<Option<ModelBox>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    value
        .map(deserialize_model_value)
        .transpose()
        .map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_optional_models<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<ModelBox>>, D::Error>
where
    D: Deserializer<'de>,
{
    let values = Option::<Vec<Value>>::deserialize(deserializer)?;
    values
        .map(|values| {
            values
                .into_iter()
                .map(deserialize_model_value)
                .collect::<SdkResult<Vec<_>>>()
        })
        .transpose()
        .map_err(serde::de::Error::custom)
}