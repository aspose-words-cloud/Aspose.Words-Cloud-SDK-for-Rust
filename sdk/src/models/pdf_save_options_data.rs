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

use std::any::Any;
use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{SdkError, SdkResult};

use super::*;

/// Container class for pdf save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct PdfSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
        /// Gets or sets a value determining how attachments are embedded to the PDF document.
            /// The default value is None and attachments are not embedded.
            /// PDF/A-1, PDF/A-2 and regular PDF/A-4 (not PDF/A-4f) standards do not allow embedded files.
            /// None value will be used automatically.
        #[serde(rename = "AttachmentsEmbeddingMode", skip_serializing_if = "Option::is_none")]
        pub r#attachments_embedding_mode: Option<PdfSaveOptionsData_AttachmentsEmbeddingModeEnum>,


        /// Gets or sets a value determining whether or not to cache graphics placed in document's background.
            /// The default value is true and background graphics are written to the PDF document as an xObject. When the value is false background graphics are not cached. Some shapes are not supported for caching(shapes with fields, bookmarks, HRefs). Document background graphic is various shapes, charts, images placed in the footer or header,
            /// well as background and border of a page.
        #[serde(rename = "CacheBackgroundGraphics", skip_serializing_if = "Option::is_none")]
        pub r#cache_background_graphics: Option<bool>,


        /// Gets or sets the PDF standards compliance level for output documents.
        #[serde(rename = "Compliance", skip_serializing_if = "Option::is_none")]
        pub r#compliance: Option<PdfSaveOptionsData_ComplianceEnum>,


        /// Gets or sets a value indicating whether to convert footnote/endnote references in main text story into active hyperlinks.
            /// When clicked the hyperlink will lead to the corresponding footnote/endnote.
            /// The default value is false.
        #[serde(rename = "CreateNoteHyperlinks", skip_serializing_if = "Option::is_none")]
        pub r#create_note_hyperlinks: Option<bool>,


        /// Gets or sets the option that controls the way CustomDocumentProperties are exported to PDF file.
            /// The default value is None.
        #[serde(rename = "CustomPropertiesExport", skip_serializing_if = "Option::is_none")]
        pub r#custom_properties_export: Option<PdfSaveOptionsData_CustomPropertiesExportEnum>,


        /// Gets or sets the details for signing the output PDF document.
        #[serde(rename = "DigitalSignatureDetails", skip_serializing_if = "Option::is_none")]
        pub r#digital_signature_details: Option<PdfDigitalSignatureDetailsData>,


        /// Gets or sets a value indicating whether the window’s title bar should display the document title taken from the Title entry of the document information dictionary.
            /// If false, the title bar should instead display the name of the PDF file containing the document.
            /// The default value is false.
        #[serde(rename = "DisplayDocTitle", skip_serializing_if = "Option::is_none")]
        pub r#display_doc_title: Option<bool>,


        /// Gets or sets the downsample options.
        #[serde(rename = "DownsampleOptions", skip_serializing_if = "Option::is_none")]
        pub r#downsample_options: Option<DownsampleOptionsData>,


        /// Gets or sets a value determining whether or not to embed attachments to the PDF document.
            /// The default value is false and attachments are not embedded.
            /// When the value is true attachments are embedded to the PDF document.
            /// Embedding attachments is not supported when saving to PDF/A and PDF/UA compliance.
            /// false value will be used automatically.
            /// Embedding attachments is not supported when encryption is enabled. false value will be used automatically.
        #[serde(rename = "EmbedAttachments", skip_serializing_if = "Option::is_none")]
        pub r#embed_attachments: Option<bool>,


        /// Gets or sets a value indicating whether fonts are embedded into the resulting PDF documents.
        #[serde(rename = "EmbedFullFonts", skip_serializing_if = "Option::is_none")]
        pub r#embed_full_fonts: Option<bool>,


        /// Gets or sets the details for encrypting the output PDF document.
        #[serde(rename = "EncryptionDetails", skip_serializing_if = "Option::is_none")]
        pub r#encryption_details: Option<PdfEncryptionDetailsData>,


        /// Gets or sets a value indicating whether to export document structure.
        #[serde(rename = "ExportDocumentStructure", skip_serializing_if = "Option::is_none")]
        pub r#export_document_structure: Option<bool>,


        /// Gets or sets a value determining whether or not to create a "Span" tag in the document structure to export the text language.
            /// The default value is false and "Lang" attribute is attached to a marked-content sequence in a page content stream.
            /// When the value is true "Span" tag is created for the text with non-default language and "Lang" attribute is attached to this tag.
            /// This value is ignored when Aspose.Words.Saving.PdfSaveOptions.ExportDocumentStructure is false.
        #[serde(rename = "ExportLanguageToSpanTag", skip_serializing_if = "Option::is_none")]
        pub r#export_language_to_span_tag: Option<bool>,


        /// Gets or sets the font embedding mode.
        #[serde(rename = "FontEmbeddingMode", skip_serializing_if = "Option::is_none")]
        pub r#font_embedding_mode: Option<PdfSaveOptionsData_FontEmbeddingModeEnum>,


        /// Gets or sets the option that controls how bookmarks in headers/footers are exported.
            /// The default value is Aspose.Words.Saving.HeaderFooterBookmarksExportMode.All.
        #[serde(rename = "HeaderFooterBookmarksExportMode", skip_serializing_if = "Option::is_none")]
        pub r#header_footer_bookmarks_export_mode: Option<PdfSaveOptionsData_HeaderFooterBookmarksExportModeEnum>,


        /// Gets or sets the option that controls how the color space will be selected for the images in PDF document.
            /// The default value is "Auto". If "SimpleCmyk" value is specified, ImageCompression option is ignored and Flate compression is used for all images in the document.
        #[serde(rename = "ImageColorSpaceExportMode", skip_serializing_if = "Option::is_none")]
        pub r#image_color_space_export_mode: Option<PdfSaveOptionsData_ImageColorSpaceExportModeEnum>,


        /// Gets or sets the compression type to be used for all images in the document.
        #[serde(rename = "ImageCompression", skip_serializing_if = "Option::is_none")]
        pub r#image_compression: Option<String>,


        /// Gets or sets a value indicating whether image interpolation shall be performed by a conforming reader. When false is specified, the flag is not written to the output document and the default behavior of reader is used instead.
            /// When the resolution of a source image is significantly lower than that of the output device, each source sample covers many device pixels. As a result, images can appear jaggy or blocky. These visual artifacts can be reduced by applying an image interpolation algorithm during rendering. Instead of painting all pixels covered by a source sample with the same color, image interpolation attempts to produce a smooth transition between adjacent sample values. A conforming Reader may choose to not implement this feature of PDF, or may use any specific implementation of interpolation that it wishes. The default value is false.
        #[serde(rename = "InterpolateImages", skip_serializing_if = "Option::is_none")]
        pub r#interpolate_images: Option<bool>,


        /// Gets or sets a value indicating whether hyperlinks in the output Pdf document are forced to be opened in a new window (or tab) of a browser.
        #[serde(rename = "OpenHyperlinksInNewWindow", skip_serializing_if = "Option::is_none")]
        pub r#open_hyperlinks_in_new_window: Option<bool>,


        /// Gets or sets the outline options.
        #[serde(rename = "OutlineOptions", skip_serializing_if = "Option::is_none")]
        pub r#outline_options: Option<OutlineOptionsData>,


        /// Gets or sets the option that controls how the PDF document should be displayed when opened in the PDF reader.
        #[serde(rename = "PageMode", skip_serializing_if = "Option::is_none")]
        pub r#page_mode: Option<PdfSaveOptionsData_PageModeEnum>,


        /// Gets or sets a value indicating whether to preblend transparent images with black background color.
            /// Preblending images may improve PDF document visual appearance in Adobe Reader and remove anti-aliasing artifacts.In order to properly display preblended images, PDF viewer application must support /Matte entry in soft-mask image dictionary.
            /// Also preblending images may decrease PDF rendering performance.The default value is false.
        #[serde(rename = "PreblendImages", skip_serializing_if = "Option::is_none")]
        pub r#preblend_images: Option<bool>,


        /// Gets or sets a value indicating whether to preserve Microsoft Word form fields as form fields in PDF or convert them to text.
        #[serde(rename = "PreserveFormFields", skip_serializing_if = "Option::is_none")]
        pub r#preserve_form_fields: Option<bool>,


        /// Gets or sets a value indicating whether to render PDF choice form field border.
            /// PDF choice form fields are used for export of SDT Combo Box Content Control, SDT Drop-Down List Content
            /// Control and legacy Drop-Down Form Field when PreserveFormFields option is enabled.The default value is true.
        #[serde(rename = "RenderChoiceFormFieldBorder", skip_serializing_if = "Option::is_none")]
        pub r#render_choice_form_field_border: Option<bool>,


        /// Gets or sets the compression type to be used for all textual content in the document.
        #[serde(rename = "TextCompression", skip_serializing_if = "Option::is_none")]
        pub r#text_compression: Option<PdfSaveOptionsData_TextCompressionEnum>,


        /// Gets or sets a value indicating whether the document should be saved using a booklet printing layout.
        #[serde(rename = "UseBookFoldPrintingSettings", skip_serializing_if = "Option::is_none")]
        pub r#use_book_fold_printing_settings: Option<bool>,


        /// Gets or sets a value indicating whether to substitute TrueType fonts Arial, Times New Roman, Courier New and Symbol with core PDF Type 1 fonts.
        #[serde(rename = "UseCoreFonts", skip_serializing_if = "Option::is_none")]
        pub r#use_core_fonts: Option<bool>,


        /// Gets or sets a value indicating whether to use SDT control Tag or Id property as a name of form field in PDF.
            /// The default value is false.When set to false, SDT control Id property is used as a name of form field in PDF.When set to true, SDT control Tag property is used as a name of form field in PDF.If set to true and Tag is empty, Id property will be used as a form field name.If set to true and Tag values are not unique, duplicate Tag values will be altered to build
            /// unique PDF form field names.
        #[serde(rename = "UseSdtTagAsFormFieldName", skip_serializing_if = "Option::is_none")]
        pub r#use_sdt_tag_as_form_field_name: Option<bool>,


        /// Gets or sets the option that controls what type of zoom should be applied when a document is opened with a PDF viewer.
        #[serde(rename = "ZoomBehavior", skip_serializing_if = "Option::is_none")]
        pub r#zoom_behavior: Option<PdfSaveOptionsData_ZoomBehaviorEnum>,


        /// Gets or sets the zoom factor (in percentages) for a document.
        #[serde(rename = "ZoomFactor", skip_serializing_if = "Option::is_none")]
        pub r#zoom_factor: Option<i32>,


        /// Gets or sets a value determining whether floating shapes are exported as inline tags in the document structure.
            /// The default value is false and floating shapes will be exported as block-level tags,
            /// placed after the paragraph in which they are anchored. When the value is true floating shapes will be exported as inline tags,
            /// placed within the paragraph where they are anchored. This value is ignored when ExportDocumentStructure is false.
        #[serde(rename = "ExportFloatingShapesAsInlineTag", skip_serializing_if = "Option::is_none")]
        pub r#export_floating_shapes_as_inline_tag: Option<bool>,


}

impl Default for PdfSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        parent.r#save_format = Some("pdf".to_owned());
        Self {
            parent,
            r#attachments_embedding_mode: None,
            r#cache_background_graphics: None,
            r#compliance: None,
            r#create_note_hyperlinks: None,
            r#custom_properties_export: None,
            r#digital_signature_details: None,
            r#display_doc_title: None,
            r#downsample_options: None,
            r#embed_attachments: None,
            r#embed_full_fonts: None,
            r#encryption_details: None,
            r#export_document_structure: None,
            r#export_language_to_span_tag: None,
            r#font_embedding_mode: None,
            r#header_footer_bookmarks_export_mode: None,
            r#image_color_space_export_mode: None,
            r#image_compression: None,
            r#interpolate_images: None,
            r#open_hyperlinks_in_new_window: None,
            r#outline_options: None,
            r#page_mode: None,
            r#preblend_images: None,
            r#preserve_form_fields: None,
            r#render_choice_form_field_border: None,
            r#text_compression: None,
            r#use_book_fold_printing_settings: None,
            r#use_core_fonts: None,
            r#use_sdt_tag_as_form_field_name: None,
            r#zoom_behavior: None,
            r#zoom_factor: None,
            r#export_floating_shapes_as_inline_tag: None,

        }
    }
}

impl Deref for PdfSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for PdfSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for PdfSaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.r#digital_signature_details {
        value.validate()?;
        }

        if let Some(value) = &self.r#downsample_options {
        value.validate()?;
        }


        if let Some(value) = &self.r#encryption_details {
        value.validate()?;
        }








        if let Some(value) = &self.r#outline_options {
        value.validate()?;
        }












        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets a value determining how attachments are embedded to the PDF document.
/// The default value is None and attachments are not embedded.
/// PDF/A-1, PDF/A-2 and regular PDF/A-4 (not PDF/A-4f) standards do not allow embedded files.
/// None value will be used automatically.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_AttachmentsEmbeddingModeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Annotations")]
        Annotations,
    #[serde(rename = "DocumentEmbeddedFiles")]
        DocumentEmbeddedFiles,
}

/// Gets or sets the PDF standards compliance level for output documents.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_ComplianceEnum {
    #[serde(rename = "Pdf17")]
        Pdf17,
    #[serde(rename = "Pdf20")]
        Pdf20,
    #[serde(rename = "PdfA1a")]
        PdfA1a,
    #[serde(rename = "PdfA1b")]
        PdfA1b,
    #[serde(rename = "PdfA2a")]
        PdfA2a,
    #[serde(rename = "PdfA2u")]
        PdfA2u,
    #[serde(rename = "PdfA3a")]
        PdfA3a,
    #[serde(rename = "PdfA3u")]
        PdfA3u,
    #[serde(rename = "PdfA4")]
        PdfA4,
    #[serde(rename = "PdfA4f")]
        PdfA4f,
    #[serde(rename = "PdfA4Ua2")]
        PdfA4Ua2,
    #[serde(rename = "PdfUa1")]
        PdfUa1,
    #[serde(rename = "PdfUa2")]
        PdfUa2,
}

/// Gets or sets the option that controls the way CustomDocumentProperties are exported to PDF file.
/// The default value is None.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_CustomPropertiesExportEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Standard")]
        Standard,
    #[serde(rename = "Metadata")]
        Metadata,
}

/// Gets or sets the font embedding mode.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_FontEmbeddingModeEnum {
    #[serde(rename = "EmbedAll")]
        EmbedAll,
    #[serde(rename = "EmbedNonstandard")]
        EmbedNonstandard,
    #[serde(rename = "EmbedNone")]
        EmbedNone,
}

/// Gets or sets the option that controls how bookmarks in headers/footers are exported.
/// The default value is Aspose.Words.Saving.HeaderFooterBookmarksExportMode.All.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_HeaderFooterBookmarksExportModeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "First")]
        First,
    #[serde(rename = "All")]
        All,
}

/// Gets or sets the option that controls how the color space will be selected for the images in PDF document.
/// The default value is "Auto". If "SimpleCmyk" value is specified, ImageCompression option is ignored and Flate compression is used for all images in the document.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_ImageColorSpaceExportModeEnum {
    #[serde(rename = "Auto")]
        Auto,
    #[serde(rename = "SimpleCmyk")]
        SimpleCmyk,
}

/// Gets or sets the option that controls how the PDF document should be displayed when opened in the PDF reader.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_PageModeEnum {
    #[serde(rename = "UseNone")]
        UseNone,
    #[serde(rename = "UseOutlines")]
        UseOutlines,
    #[serde(rename = "UseThumbs")]
        UseThumbs,
    #[serde(rename = "FullScreen")]
        FullScreen,
    #[serde(rename = "UseOC")]
        UseOC,
    #[serde(rename = "UseAttachments")]
        UseAttachments,
}

/// Gets or sets the compression type to be used for all textual content in the document.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_TextCompressionEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Flate")]
        Flate,
}

/// Gets or sets the option that controls what type of zoom should be applied when a document is opened with a PDF viewer.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PdfSaveOptionsData_ZoomBehaviorEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "ZoomFactor")]
        ZoomFactor,
    #[serde(rename = "FitPage")]
        FitPage,
    #[serde(rename = "FitWidth")]
        FitWidth,
    #[serde(rename = "FitHeight")]
        FitHeight,
    #[serde(rename = "FitBox")]
        FitBox,
}