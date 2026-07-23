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

/// DTO container with a StructuredDocumentTagBaseDto.
#[derive(Debug, Deserialize, Serialize)]
pub struct StructuredDocumentTagBase {
    #[serde(flatten)]
    pub parent: NodeLink,
    /// Gets or sets Aspose.Words.Markup.SdtListItemCollection associated with this SDT.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.ComboBox or Aspose.Words.Markup.SdtType.DropDownList SDT types.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "ListItems", skip_serializing_if = "Option::is_none")]
    pub list_items: Option<Vec<StructuredDocumentTagListItem>>,

    /// Gets or sets a value indicating whether current state of the Checkbox SDT. Default value for this property.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.Checkbox SDT types.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "Checked", skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,

    /// Gets or sets the appearance of a structured document tag.
    #[serde(rename = "Appearance", skip_serializing_if = "Option::is_none")]
    pub appearance: Option<StructuredDocumentTagBaseAppearanceEnum>,

    /// Gets or sets the language format for the date displayed in this SDT.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "DateDisplayLocale", skip_serializing_if = "Option::is_none")]
    pub date_display_locale: Option<i32>,

    /// Gets or sets String that represents the format in which dates are displayed. Can not be null. The dates for English (U.S.) is "mm/dd/yyyy".
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "DateDisplayFormat", skip_serializing_if = "Option::is_none")]
    pub date_display_format: Option<String>,

    /// Gets or sets the full date and time last entered into this SDT.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
    /// For all other SDT types, an exception will occur.
    #[serde(
        rename = "FullDate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_optional_date_time"
    )]
    pub full_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Gets or sets the friendly name associated with this SDT. Can not be null.
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Gets or sets format in which the date for a date SDT is stored when the SDT is bound to an XML node in the document's data store.
    /// The default value is Aspose.Words.Markup.SdtDateStorageFormat.DateTime.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "DateStorageFormat", skip_serializing_if = "Option::is_none")]
    pub date_storage_format: Option<StructuredDocumentTagBaseDateStorageFormatEnum>,

    /// Gets or sets type of building block for this SDT. Can not be null.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.BuildingBlockGallery and Aspose.Words.Markup.SdtType.DocPartObj SDT types.
    /// It is read-only for SDT of the document part type.
    /// For all other SDT types, an exception will occur.
    #[serde(
        rename = "BuildingBlockGallery",
        skip_serializing_if = "Option::is_none"
    )]
    pub building_block_gallery: Option<String>,

    /// Gets or sets category of building block for this SDT node. Can not be null.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.BuildingBlockGallery and Aspose.Words.Markup.SdtType.DocPartObj SDT types.
    /// It is read-only for SDT of the document part type. For all other SDT types, an exception will occur.
    #[serde(
        rename = "BuildingBlockCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub building_block_category: Option<String>,

    /// Gets or sets a value indicating whether this SDT allows multiple lines of text.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.RichText and Aspose.Words.Markup.SdtType.PlainText SDT type.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "Multiline", skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,

    /// Gets or sets the color of the structured document tag.
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    pub color: Option<XmlColor>,

    /// Gets or sets the name of the style applied to the structured document tag.
    #[serde(rename = "StyleName", skip_serializing_if = "Option::is_none")]
    pub style_name: Option<String>,

    /// Gets or sets the type of calendar for this SDT. Default is Aspose.Words.Markup.SdtCalendarType.Default.
    /// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
    /// For all other SDT types, an exception will occur.
    #[serde(rename = "CalendarType", skip_serializing_if = "Option::is_none")]
    pub calendar_type: Option<StructuredDocumentTagBaseCalendarTypeEnum>,

    /// Gets or sets a value indicating whether this SDT shall be removed from the WordProcessingML document when its contents are modified.
    #[serde(rename = "IsTemporary", skip_serializing_if = "Option::is_none")]
    pub is_temporary: Option<bool>,

    /// Gets or sets Name of the Aspose.Words.BuildingBlocks.BuildingBlock containing placeholder text.
    /// Aspose.Words.BuildingBlocks.BuildingBlock with this name Aspose.Words.BuildingBlocks.BuildingBlock.Name has to be present in the Aspose.Words.Document.GlossaryDocument otherwise System.InvalidOperationException will occur.
    #[serde(rename = "PlaceholderName", skip_serializing_if = "Option::is_none")]
    pub placeholder_name: Option<String>,

    /// Gets or sets a value indicating whether, this property will prohibit a user from deleting this SDT.
    #[serde(rename = "LockContentControl", skip_serializing_if = "Option::is_none")]
    pub lock_content_control: Option<bool>,

    /// Gets or sets a value indicating whether, this property will prohibit a user from editing the contents of this SDT.
    #[serde(rename = "LockContents", skip_serializing_if = "Option::is_none")]
    pub lock_contents: Option<bool>,

    /// Gets or sets a value indicating whether the content of this SDT shall be interpreted to contain placeholder text (as opposed to regular text contents within the SDT).
    /// If set to true, this state shall be resumed (showing placeholder text) upon opening his document.
    #[serde(
        rename = "IsShowingPlaceholderText",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_showing_placeholder_text: Option<bool>,

    /// Gets or sets a tag associated with the current SDT node. Can not be null.
    /// A tag is an arbitrary string which applications can associate with SDT in order to identify it without providing a visible friendly name.
    #[serde(rename = "Tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Gets or sets a unique read-only persistent numerical Id for this SDT.
    /// Id attribute shall follow these rules:
    /// - The document shall retain SDT ids only if the whole document is cloned Aspose.Words.Document.Clone.
    /// - During Aspose.Words.DocumentBase.ImportNode(Aspose.Words.Node,System.Boolean)
    /// - Id shall be retained if import does not cause conflicts with other SDT Ids in the target document.
    /// - If multiple SDT nodes specify the same decimal number value for the Id attribute, then the first SDT in the document shall maintain this original Id, and all subsequent
    /// - SDT nodes shall have new identifiers assigned to them when the document is loaded.
    /// - During standalone SDT Aspose.Words.Markup.StructuredDocumentTag.Clone(System.Boolean,Aspose.Words.INodeCloningListener) operation new unique ID will be generated for the cloned SDT node.
    /// - If Id is not specified in the source document, then the SDT node shall have a new unique identifier assigned to it when the document is loaded.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Gets a string that represents the XML contained within the node in the Aspose.Words.SaveFormat.FlatOpc format.
    #[serde(rename = "WordOpenXML", skip_serializing_if = "Option::is_none")]
    pub word_open_xml: Option<String>,
}

impl Default for StructuredDocumentTagBase {
    fn default() -> Self {
        let mut parent = NodeLink::default();
        Self {
            parent,
            list_items: None,
            checked: None,
            appearance: None,
            date_display_locale: None,
            date_display_format: None,
            full_date: None,
            title: None,
            date_storage_format: None,
            building_block_gallery: None,
            building_block_category: None,
            multiline: None,
            color: None,
            style_name: None,
            calendar_type: None,
            is_temporary: None,
            placeholder_name: None,
            lock_content_control: None,
            lock_contents: None,
            is_showing_placeholder_text: None,
            tag: None,
            id: None,
            word_open_xml: None,
        }
    }
}

impl Deref for StructuredDocumentTagBase {
    type Target = NodeLink;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for StructuredDocumentTagBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for StructuredDocumentTagBase {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(values) = &self.list_items {
            for value in values {
                value.validate()?;
            }
        }

        if let Some(value) = &self.color {
            value.validate()?;
        }

        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the appearance of a structured document tag.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StructuredDocumentTagBaseAppearanceEnum {
    #[serde(rename = "BoundingBox")]
    BoundingBox,
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Tags")]
    Tags,
    #[serde(rename = "Hidden")]
    Hidden,
}

/// Gets or sets format in which the date for a date SDT is stored when the SDT is bound to an XML node in the document's data store.
/// The default value is Aspose.Words.Markup.SdtDateStorageFormat.DateTime.
/// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
/// For all other SDT types, an exception will occur.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StructuredDocumentTagBaseDateStorageFormatEnum {
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "DateTime")]
    DateTime,
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Text")]
    Text,
}

/// Gets or sets the type of calendar for this SDT. Default is Aspose.Words.Markup.SdtCalendarType.Default.
/// Accessing this property will work only for Aspose.Words.Markup.SdtType.Date SDT type.
/// For all other SDT types, an exception will occur.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StructuredDocumentTagBaseCalendarTypeEnum {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Gregorian")]
    Gregorian,
    #[serde(rename = "GregorianArabic")]
    GregorianArabic,
    #[serde(rename = "GregorianMeFrench")]
    GregorianMeFrench,
    #[serde(rename = "GregorianUs")]
    GregorianUs,
    #[serde(rename = "GregorianXlitEnglish")]
    GregorianXlitEnglish,
    #[serde(rename = "GregorianXlitFrench")]
    GregorianXlitFrench,
    #[serde(rename = "Hebrew")]
    Hebrew,
    #[serde(rename = "Hijri")]
    Hijri,
    #[serde(rename = "Japan")]
    Japan,
    #[serde(rename = "Korea")]
    Korea,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Saka")]
    Saka,
    #[serde(rename = "Taiwan")]
    Taiwan,
    #[serde(rename = "Thai")]
    Thai,
}
