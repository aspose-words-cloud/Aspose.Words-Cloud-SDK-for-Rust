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

/// DTO container with a StructuredDocumentTag.
#[derive(Debug, Deserialize, Serialize)]
pub struct StructuredDocumentTagInsert {
    #[serde(flatten)]
    pub parent: StructuredDocumentTagBase,
        /// Gets or sets the level at which this SDT occurs in the document tree.
        #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
        pub level: Option<StructuredDocumentTagInsertLevelEnum>,


        /// Gets or sets type of this Structured document tag.
        #[serde(rename = "SdtType", skip_serializing_if = "Option::is_none")]
        pub sdt_type: Option<StructuredDocumentTagInsertSdtTypeEnum>,


        /// Gets or sets the position of the node that will be used to determine the placement of a new node.
        #[serde(rename = "Position", skip_serializing_if = "Option::is_none", default, deserialize_with = "deserialize_optional_model")]
        pub position: Option<ModelBox>,

}

impl Default for StructuredDocumentTagInsert {
    fn default() -> Self {
        let mut parent = StructuredDocumentTagBase::default();
        Self {
            parent,
            level: None,
            sdt_type: None,
            position: None,
        }
    }
}

impl Deref for StructuredDocumentTagInsert {
    type Target = StructuredDocumentTagBase;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for StructuredDocumentTagInsert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for StructuredDocumentTagInsert {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.level.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Level in StructuredDocumentTagInsert is required".to_owned(),
            ));
        }
        if self.sdt_type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property SdtType in StructuredDocumentTagInsert is required".to_owned(),
            ));
        }
        if let Some(value) = &self.position {
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

/// Gets or sets the level at which this SDT occurs in the document tree.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StructuredDocumentTagInsertLevelEnum {
    #[serde(rename = "Unknown")]
        Unknown,
    #[serde(rename = "Inline")]
        Inline,
    #[serde(rename = "Block")]
        Block,
    #[serde(rename = "Row")]
        Row,
    #[serde(rename = "Cell")]
        Cell,
}

/// Gets or sets type of this Structured document tag.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StructuredDocumentTagInsertSdtTypeEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Bibliography")]
        Bibliography,
    #[serde(rename = "Citation")]
        Citation,
    #[serde(rename = "Equation")]
        Equation,
    #[serde(rename = "DropDownList")]
        DropDownList,
    #[serde(rename = "ComboBox")]
        ComboBox,
    #[serde(rename = "Date")]
        Date,
    #[serde(rename = "BuildingBlockGallery")]
        BuildingBlockGallery,
    #[serde(rename = "DocPartObj")]
        DocPartObj,
    #[serde(rename = "Group")]
        Group,
    #[serde(rename = "Picture")]
        Picture,
    #[serde(rename = "RichText")]
        RichText,
    #[serde(rename = "PlainText")]
        PlainText,
    #[serde(rename = "Checkbox")]
        Checkbox,
    #[serde(rename = "RepeatingSection")]
        RepeatingSection,
    #[serde(rename = "RepeatingSectionItem")]
        RepeatingSectionItem,
    #[serde(rename = "EntityPicker")]
        EntityPicker,
}