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

/// base container class for save options data.
#[derive(Debug, Deserialize, Serialize)]
pub struct SaveOptionsData {
        /// Gets or sets a boolean value indicating whether to allow embedding fonts with PostScript outlines when embedding TrueType fonts in a document upon it is saved. The default value is false..
            /// Note, Word does not embed PostScript fonts, but can open documents with embedded fonts of this type.
            /// This option only works when Aspose.Words.Fonts.FontInfoCollection.EmbedTrueTypeFonts of the Aspose.Words.DocumentBase.FontInfos property is set to true.
            /// The default value is false.
        #[serde(rename = "AllowEmbeddingPostScriptFonts", skip_serializing_if = "Option::is_none")]
        pub r#allow_embedding_post_script_fonts: Option<bool>,


        /// Gets or sets CustomTimeZoneInfo.
        #[serde(rename = "CustomTimeZoneInfoData", skip_serializing_if = "Option::is_none")]
        pub r#custom_time_zone_info_data: Option<TimeZoneInfoData>,


        /// Gets or sets the value determining how 3D effects are rendered.
            /// The default value is Aspose.Words.Saving.Dml3DEffectsRenderingMode.Basic.
        #[serde(rename = "Dml3DEffectsRenderingMode", skip_serializing_if = "Option::is_none")]
        pub r#dml3_d_effects_rendering_mode: Option<SaveOptionsData_Dml3DEffectsRenderingModeEnum>,


        /// Gets or sets the value determining how DrawingML effects are rendered.
            /// { Simplified | None | Fine }.
            /// The default value is Simplified.
            /// This property is used when the document is exported to fixed page formats.
        #[serde(rename = "DmlEffectsRenderingMode", skip_serializing_if = "Option::is_none")]
        pub r#dml_effects_rendering_mode: Option<SaveOptionsData_DmlEffectsRenderingModeEnum>,


        /// Gets or sets the option that controls how DrawingML shapes are rendered.
            /// { Fallback | DrawingML }. The default value is Fallback.
            /// This property is used when the document is exported to fixed page formats.
        #[serde(rename = "DmlRenderingMode", skip_serializing_if = "Option::is_none")]
        pub r#dml_rendering_mode: Option<SaveOptionsData_DmlRenderingModeEnum>,


        /// Gets or sets the name of destination file.
        #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
        pub r#file_name: Option<String>,


        /// Gets or sets the value determining how ink (InkML) objects are rendered.
            /// The default value is Aspose.Words.Saving.ImlRenderingMode.InkML.
        #[serde(rename = "ImlRenderingMode", skip_serializing_if = "Option::is_none")]
        pub r#iml_rendering_mode: Option<SaveOptionsData_ImlRenderingModeEnum>,


        /// Gets or sets a value indicating whether the font attributes will be changed according to the character code being used.
        #[serde(rename = "UpdateAmbiguousTextFont", skip_serializing_if = "Option::is_none")]
        pub r#update_ambiguous_text_font: Option<bool>,


        /// Gets or sets a value determining whether the Aspose.Words.Properties.BuiltInDocumentProperties.CreatedTime property is updated before saving.
            /// The default value is false.
        #[serde(rename = "UpdateCreatedTimeProperty", skip_serializing_if = "Option::is_none")]
        pub r#update_created_time_property: Option<bool>,


        /// Gets or sets a value indicating whether fields should be updated before saving the document to a fixed page format. The default value is true.
        #[serde(rename = "UpdateFields", skip_serializing_if = "Option::is_none")]
        pub r#update_fields: Option<bool>,


        /// Gets or sets a value indicating whether the Aspose.Words.Properties.BuiltInDocumentProperties.LastPrinted property is updated before saving.
        #[serde(rename = "UpdateLastPrintedProperty", skip_serializing_if = "Option::is_none")]
        pub r#update_last_printed_property: Option<bool>,


        /// Gets or sets a value indicating whether the Aspose.Words.Properties.BuiltInDocumentProperties.LastSavedTime property is updated before saving.
            /// The default value is false.
        #[serde(rename = "UpdateLastSavedTimeProperty", skip_serializing_if = "Option::is_none")]
        pub r#update_last_saved_time_property: Option<bool>,


        /// Gets or sets a value indicating whether to zip output or not.
            /// The default value is false.
            /// When set to true, output files will be zipped.
        #[serde(rename = "ZipOutput", skip_serializing_if = "Option::is_none")]
        pub r#zip_output: Option<bool>,


        /// Gets the format of save.
        #[serde(rename = "SaveFormat", skip_serializing_if = "Option::is_none")]
        pub r#save_format: Option<String>,

}

impl Default for SaveOptionsData {
    fn default() -> Self {
        Self {
            r#allow_embedding_post_script_fonts: None,
            r#custom_time_zone_info_data: None,
            r#dml3_d_effects_rendering_mode: None,
            r#dml_effects_rendering_mode: None,
            r#dml_rendering_mode: None,
            r#file_name: None,
            r#iml_rendering_mode: None,
            r#update_ambiguous_text_font: None,
            r#update_created_time_property: None,
            r#update_fields: None,
            r#update_last_printed_property: None,
            r#update_last_saved_time_property: None,
            r#zip_output: None,
            r#save_format: None,
        }
    }
}

impl Model for SaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        if self.r#file_name.is_none() {
            return Err(SdkError::InvalidRequest(
                "property FileName in SaveOptionsData is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#custom_time_zone_info_data {
        value.validate()?;
        }












        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the value determining how 3D effects are rendered.
/// The default value is Aspose.Words.Saving.Dml3DEffectsRenderingMode.Basic.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SaveOptionsData_Dml3DEffectsRenderingModeEnum {
    #[serde(rename = "Basic")]
        Basic,
    #[serde(rename = "Advanced")]
        Advanced,
}

/// Gets or sets the value determining how DrawingML effects are rendered.
/// { Simplified | None | Fine }.
/// The default value is Simplified.
/// This property is used when the document is exported to fixed page formats.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SaveOptionsData_DmlEffectsRenderingModeEnum {
    #[serde(rename = "Simplified")]
        Simplified,
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Fine")]
        Fine,
}

/// Gets or sets the option that controls how DrawingML shapes are rendered.
/// { Fallback | DrawingML }. The default value is Fallback.
/// This property is used when the document is exported to fixed page formats.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SaveOptionsData_DmlRenderingModeEnum {
    #[serde(rename = "Fallback")]
        Fallback,
    #[serde(rename = "DrawingML")]
        DrawingML,
}

/// Gets or sets the value determining how ink (InkML) objects are rendered.
/// The default value is Aspose.Words.Saving.ImlRenderingMode.InkML.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SaveOptionsData_ImlRenderingModeEnum {
    #[serde(rename = "Fallback")]
        Fallback,
    #[serde(rename = "InkML")]
        InkML,
}