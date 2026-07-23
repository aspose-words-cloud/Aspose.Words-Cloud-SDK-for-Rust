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

/// DTO container with a single document list.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListInfo {
    #[serde(flatten)]
    pub parent: LinkElement,
    /// Gets or sets the unique identifier of the list.
    /// You do not normally need to use this property. But if you use it, you normally do so in conjunction with the Aspose.Words.Lists.ListCollection.GetListByListId(System.Int32) method to find a list by its identifier.
    #[serde(rename = "ListId", skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i32>,

    /// Gets or sets a value indicating whether the list contains 9 levels; false when 1 level.
    /// The lists that you create with Aspose.Words are always multi-level lists and contain 9 levels. Microsoft Word 2003 and later always create multi-level lists with 9 levels. But in some documents, created with earlier versions of Microsoft Word you might encounter lists that have 1 level only.
    #[serde(rename = "IsMultiLevel", skip_serializing_if = "Option::is_none")]
    pub is_multi_level: Option<bool>,

    /// Gets or sets a value indicating whether list should be restarted at each section. The default value is false.
    /// This option is supported only in RTF, DOC and DOCX document formats. This option will be written to DOCX only if Aspose.Words.Saving.OoxmlCompliance is higher then Aspose.Words.Saving.OoxmlCompliance.Ecma376_2006.
    #[serde(
        rename = "IsRestartAtEachSection",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_restart_at_each_section: Option<bool>,

    /// Gets or sets a value indicating whether this list is a definition of a list style.
    /// When this property is true, the Aspose.Words.Lists.List.Style property returns the list style that this list defines. By modifying properties of a list that defines a list style, you modify The properties of the list style. A list that is a definition of a list style cannot be applied directly to paragraphs to make them numbered. Aspose.Words.Lists.List.Style Aspose.Words.Lists.List.IsListStyleReference.
    #[serde(
        rename = "IsListStyleDefinition",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_list_style_definition: Option<bool>,

    /// Gets or sets a value indicating whether this list is a reference to a list style.
    /// Note, modifying properties of a list that is a reference to list style has no effect. The list formatting specified in the list style itself always takes precedence. Aspose.Words.Lists.List.Style Aspose.Words.Lists.List.IsListStyleDefinition.
    #[serde(
        rename = "IsListStyleReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_list_style_reference: Option<bool>,

    /// Gets or sets the list style that this list references or defines.
    /// If this list is not associated with a list style, the property will return null. A list could be a reference to a list style, in this case Aspose.Words.Lists.List.IsListStyleReference will be true. A list could be a definition of a list style, in this case Aspose.Words.Lists.List.IsListStyleDefinition will be true. Such a list cannot be applied to paragraphs in the document directly.
    #[serde(rename = "Style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,

    /// Gets or sets the collection of list levels for this list.
    /// Use this property to access and modify formatting individual to each level of the list.
    #[serde(rename = "ListLevels", skip_serializing_if = "Option::is_none")]
    pub list_levels: Option<ListLevels>,
}

impl Default for ListInfo {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            list_id: None,
            is_multi_level: None,
            is_restart_at_each_section: None,
            is_list_style_definition: None,
            is_list_style_reference: None,
            style: None,
            list_levels: None,
        }
    }
}

impl Deref for ListInfo {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for ListInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for ListInfo {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.list_id.is_none() {
            return Err(SdkError::InvalidRequest(
                "property ListId in ListInfo is required".to_owned(),
            ));
        }
        if self.is_multi_level.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsMultiLevel in ListInfo is required".to_owned(),
            ));
        }
        if self.is_restart_at_each_section.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsRestartAtEachSection in ListInfo is required".to_owned(),
            ));
        }
        if self.is_list_style_definition.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsListStyleDefinition in ListInfo is required".to_owned(),
            ));
        }
        if self.is_list_style_reference.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsListStyleReference in ListInfo is required".to_owned(),
            ));
        }
        if let Some(value) = &self.style {
            value.validate()?;
        }
        if let Some(value) = &self.list_levels {
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
