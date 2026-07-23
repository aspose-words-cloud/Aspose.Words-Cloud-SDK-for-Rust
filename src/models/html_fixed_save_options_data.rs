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

/// Container class for fixed html save options.
#[derive(Debug, Deserialize, Serialize)]
pub struct HtmlFixedSaveOptionsData {
    #[serde(flatten)]
    pub parent: FixedPageSaveOptionsData,
    /// Gets or sets the prefix which is added to all class names in style.css file.
    /// The default value is "aw".
    #[serde(
        rename = "CssClassNamesPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_class_names_prefix: Option<String>,

    /// Gets or sets the character encoding.
    #[serde(rename = "Encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Gets or sets a value indicating whether the CSS (Cascading Style Sheet) should be embedded into the Html document.
    #[serde(rename = "ExportEmbeddedCss", skip_serializing_if = "Option::is_none")]
    pub export_embedded_css: Option<bool>,

    /// Gets or sets a value indicating whether fonts should be embedded into the Html document in Base64 format.
    #[serde(
        rename = "ExportEmbeddedFonts",
        skip_serializing_if = "Option::is_none"
    )]
    pub export_embedded_fonts: Option<bool>,

    /// Gets or sets a value indicating whether images should be embedded into the Html document in Base64 format.
    #[serde(
        rename = "ExportEmbeddedImages",
        skip_serializing_if = "Option::is_none"
    )]
    pub export_embedded_images: Option<bool>,

    /// Gets or sets a value indicating whether form fields are exported as interactive items (as 'input' tag) rather than converted to text or graphics.
    #[serde(rename = "ExportFormFields", skip_serializing_if = "Option::is_none")]
    pub export_form_fields: Option<bool>,

    /// Gets or sets the export format of fonts.
    #[serde(rename = "FontFormat", skip_serializing_if = "Option::is_none")]
    pub font_format: Option<HtmlFixedSaveOptionsDataFontFormatEnum>,

    /// Gets or sets a prefix that is prepended to all generated element IDs in the output document.
    /// The default value is null and no prefix is prepended.
    /// If the prefix is specified, it can contain only letters, digits, underscores, and hyphens,
    /// and must start with a letter.
    #[serde(rename = "IdPrefix", skip_serializing_if = "Option::is_none")]
    pub id_prefix: Option<String>,

    /// Gets or sets the horizontal alignment of pages in the HTML document.
    /// The default value is HtmlFixedHorizontalPageAlignment.Center.
    #[serde(
        rename = "PageHorizontalAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub page_horizontal_alignment: Option<HtmlFixedSaveOptionsDataPageHorizontalAlignmentEnum>,

    /// Gets or sets the margin around pages in HTML document.
    /// The margins value is measured in points and should be equal to or greater than 0.
    /// The default value is 10 points.
    /// Depends on the value of PageHorizontalAlignment property:
    /// Defines top, bottom and left page margins if the value is Left.
    /// Defines top, bottom and right page margins if the value is Right.
    /// Defines top and bottom page margins if the value is Center.
    #[serde(rename = "PageMargins", skip_serializing_if = "Option::is_none")]
    pub page_margins: Option<f64>,

    /// Gets or sets the physical folder where resources are saved when exporting the document.
    #[serde(rename = "ResourcesFolder", skip_serializing_if = "Option::is_none")]
    pub resources_folder: Option<String>,

    /// Gets or sets the name of the folder used to construct resource URIs.
    #[serde(
        rename = "ResourcesFolderAlias",
        skip_serializing_if = "Option::is_none"
    )]
    pub resources_folder_alias: Option<String>,

    /// Gets or sets a value indicating whether "@font-face" CSS rules should be placed into a separate file "fontFaces.css" when a document is being saved with external stylesheet (that is, when Aspose.Words.Saving.HtmlFixedSaveOptions.ExportEmbeddedCss is false). The default value is false, all CSS rules are written into single file "styles.css".
    /// Setting this property to true restores the old behavior (separate files) for compatibility with legacy code.
    #[serde(
        rename = "SaveFontFaceCssSeparately",
        skip_serializing_if = "Option::is_none"
    )]
    pub save_font_face_css_separately: Option<bool>,

    /// Gets or sets a value indicating whether to show border around pages.
    #[serde(rename = "ShowPageBorder", skip_serializing_if = "Option::is_none")]
    pub show_page_border: Option<bool>,

    /// Gets or sets a value indicating whether fonts from target machine must be used to display the document. If this flag is set to true, Aspose.Words.Saving.HtmlFixedSaveOptions.FontFormat and Aspose.Words.Saving.HtmlFixedSaveOptions.ExportEmbeddedFonts properties do not have effect, also Aspose.Words.Saving.HtmlFixedSaveOptions.ResourceSavingCallback is not fired for fonts. The default value is false.
    #[serde(
        rename = "UseTargetMachineFonts",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_target_machine_fonts: Option<bool>,
}

impl Default for HtmlFixedSaveOptionsData {
    fn default() -> Self {
        let mut parent = FixedPageSaveOptionsData::default();
        parent.save_format = Some("htmlfixed".to_owned());
        Self {
            parent,
            css_class_names_prefix: None,
            encoding: None,
            export_embedded_css: None,
            export_embedded_fonts: None,
            export_embedded_images: None,
            export_form_fields: None,
            font_format: None,
            id_prefix: None,
            page_horizontal_alignment: None,
            page_margins: None,
            resources_folder: None,
            resources_folder_alias: None,
            save_font_face_css_separately: None,
            show_page_border: None,
            use_target_machine_fonts: None,
        }
    }
}

impl Deref for HtmlFixedSaveOptionsData {
    type Target = FixedPageSaveOptionsData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for HtmlFixedSaveOptionsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for HtmlFixedSaveOptionsData {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the export format of fonts.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HtmlFixedSaveOptionsDataFontFormatEnum {
    #[serde(rename = "Woff")]
    Woff,
    #[serde(rename = "Ttf")]
    Ttf,
}

/// Gets or sets the horizontal alignment of pages in the HTML document.
/// The default value is HtmlFixedHorizontalPageAlignment.Center.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HtmlFixedSaveOptionsDataPageHorizontalAlignmentEnum {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Center")]
    Center,
    #[serde(rename = "Right")]
    Right,
}
