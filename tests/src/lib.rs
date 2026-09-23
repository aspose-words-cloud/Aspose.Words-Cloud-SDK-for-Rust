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

#![allow(warnings)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![forbid(unsafe_code)]
#![allow(dead_code, unused_mut)]
// The source-language test expressions are converted uniformly so the full
// generated suite remains maintainable as the API schema changes.
#![allow(
    clippy::field_reassign_with_default,
    clippy::unnecessary_cast,
    clippy::unnecessary_to_owned,
    clippy::useless_conversion
)]

#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod test_context;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod batch_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod encoding_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod examples_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod readme_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/append_document_tests.rs"]
mod append_document_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "bookmark/bookmark_tests.rs"]
mod bookmark_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "report/build_report_tests.rs"]
mod build_report_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/comment_tests.rs"]
mod comment_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/compare_document_tests.rs"]
mod compare_document_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "compatibility/compatibility_tests.rs"]
mod compatibility_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/compress_document_tests.rs"]
mod compress_document_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/convert_document_tests.rs"]
mod convert_document_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/custom_xml_parts_tests.rs"]
mod custom_xml_parts_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/document_tests.rs"]
mod document_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document_properties/document_properties_tests.rs"]
mod document_properties_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document_protection/document_protection_tests.rs"]
mod document_protection_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/document_statistics_tests.rs"]
mod document_statistics_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/document_with_format_tests.rs"]
mod document_with_format_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "drawing/drawing_objects_tests.rs"]
mod drawing_objects_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "mail_merge/execute_mail_merge_tests.rs"]
mod execute_mail_merge_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "mail_merge/execute_template_tests.rs"]
mod execute_template_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "mail_merge/execute_template_with_field_options_tests.rs"]
mod execute_template_with_field_options_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "field/field_tests.rs"]
mod field_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "storage/file_tests.rs"]
mod file_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "storage/folder_tests.rs"]
mod folder_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "font/font_tests.rs"]
mod font_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "footnote/footnote_tests.rs"]
mod footnote_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "field/form_field_tests.rs"]
mod form_field_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "header_footer/header_footer_tests.rs"]
mod header_footer_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "hyperlink/hyperlink_tests.rs"]
mod hyperlink_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "info/info_tests.rs"]
mod info_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "lists/lists_tests.rs"]
mod lists_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/load_web_document_tests.rs"]
mod load_web_document_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "macros/macros_tests.rs"]
mod macros_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "mail_merge/mail_merge_fileds_tests.rs"]
mod mail_merge_fileds_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "math_object/math_object_tests.rs"]
mod math_object_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "page_setup/page_setup_tests.rs"]
mod page_setup_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "paragraph/paragraph_tests.rs"]
mod paragraph_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/password_encryption_tests.rs"]
mod password_encryption_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "range/range_tests.rs"]
mod range_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/revisions_tests.rs"]
mod revisions_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "run/run_tests.rs"]
mod run_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "sections/section_tests.rs"]
mod section_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/signature_tests.rs"]
mod signature_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "document/split_document_to_format_tests.rs"]
mod split_document_to_format_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "structured_document_tag/structured_document_tag_tests.rs"]
mod structured_document_tag_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "styles/styles_tests.rs"]
mod styles_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "table/table_tests.rs"]
mod table_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "table/table_border_tests.rs"]
mod table_border_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "text/text_tests.rs"]
mod text_tests;
#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[path = "watermark/watermark_tests.rs"]
mod watermark_tests;