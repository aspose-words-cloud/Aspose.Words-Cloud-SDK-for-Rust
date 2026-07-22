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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{SdkError, SdkResult};

use super::*;

/// DTO for field options.
#[derive(Debug, Deserialize, Serialize)]
pub struct FieldOptions {
        /// Gets or sets Current User.
        #[serde(rename = "CurrentUser", skip_serializing_if = "Option::is_none")]
        pub current_user: Option<UserInformation>,


        /// Gets or sets Custom Toc Style Separator.
        #[serde(rename = "CustomTocStyleSeparator", skip_serializing_if = "Option::is_none")]
        pub custom_toc_style_separator: Option<String>,


        /// Gets or sets Default Document Author.
        #[serde(rename = "DefaultDocumentAuthor", skip_serializing_if = "Option::is_none")]
        pub default_document_author: Option<String>,


        /// Gets or sets Field Index Format.
        #[serde(rename = "FieldIndexFormat", skip_serializing_if = "Option::is_none")]
        pub field_index_format: Option<FieldOptionsFieldIndexFormatEnum>,


        /// Gets or sets Field Update Culture Name.
            /// It is used for all fields if FieldUpdateCultureSource is FieldCode.
        #[serde(rename = "FieldUpdateCultureName", skip_serializing_if = "Option::is_none")]
        pub field_update_culture_name: Option<String>,


        /// Gets or sets Field Update Culture Source.
        #[serde(rename = "FieldUpdateCultureSource", skip_serializing_if = "Option::is_none")]
        pub field_update_culture_source: Option<FieldOptionsFieldUpdateCultureSourceEnum>,


        /// Gets or sets File Name.
        #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
        pub file_name: Option<String>,


        /// Gets or sets if Bidi Text Supported OnUpdate.
        #[serde(rename = "IsBidiTextSupportedOnUpdate", skip_serializing_if = "Option::is_none")]
        pub is_bidi_text_supported_on_update: Option<bool>,


        /// Gets or sets if Legacy Number Format.
        #[serde(rename = "LegacyNumberFormat", skip_serializing_if = "Option::is_none")]
        pub legacy_number_format: Option<bool>,


        /// Gets or sets PreProcess Culture Name.
            /// It is a culture code for DOC fields.
        #[serde(rename = "PreProcessCultureName", skip_serializing_if = "Option::is_none")]
        pub pre_process_culture_name: Option<String>,


        /// Gets or sets Template Name.
        #[serde(rename = "TemplateName", skip_serializing_if = "Option::is_none")]
        pub template_name: Option<String>,


        /// Gets or sets if Use Invariant Culture Number Format.
        #[serde(rename = "UseInvariantCultureNumberFormat", skip_serializing_if = "Option::is_none")]
        pub use_invariant_culture_number_format: Option<bool>,


        /// Gets or sets BuiltIn Templates Paths.
        #[serde(rename = "BuiltInTemplatesPaths", skip_serializing_if = "Option::is_none")]
        pub built_in_templates_paths: Option<Vec<String>>,

}

impl Default for FieldOptions {
    fn default() -> Self {
        Self {
            current_user: None,
            custom_toc_style_separator: None,
            default_document_author: None,
            field_index_format: None,
            field_update_culture_name: None,
            field_update_culture_source: None,
            file_name: None,
            is_bidi_text_supported_on_update: None,
            legacy_number_format: None,
            pre_process_culture_name: None,
            template_name: None,
            use_invariant_culture_number_format: None,
            built_in_templates_paths: None,
        }
    }
}

impl Model for FieldOptions {
    fn validate(&self) -> SdkResult<()> {
        if let Some(value) = &self.current_user {
        value.validate()?;
        }












        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets Field Index Format.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FieldOptionsFieldIndexFormatEnum {
    #[serde(rename = "Template")]
        Template,
    #[serde(rename = "Classic")]
        Classic,
    #[serde(rename = "Fancy")]
        Fancy,
    #[serde(rename = "Modern")]
        Modern,
    #[serde(rename = "Bulleted")]
        Bulleted,
    #[serde(rename = "Formal")]
        Formal,
    #[serde(rename = "Simple")]
        Simple,
}

/// Gets or sets Field Update Culture Source.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FieldOptionsFieldUpdateCultureSourceEnum {
    #[serde(rename = "CurrentThread")]
        CurrentThread,
    #[serde(rename = "FieldCode")]
        FieldCode,
}