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

/// Represents Words document DTO.
#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
        /// Gets or sets the list of links that originate from this document.
        #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
        pub r#links: Option<Vec<Link>>,


        /// Gets or sets the document properties.
        #[serde(rename = "DocumentProperties", skip_serializing_if = "Option::is_none")]
        pub r#document_properties: Option<DocumentProperties>,


        /// Gets or sets the name of the file.
        #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
        pub r#file_name: Option<String>,


        /// Gets or sets the file size.
        #[serde(rename = "FileSize", skip_serializing_if = "Option::is_none")]
        pub r#file_size: Option<i32>,


        /// Gets or sets a value indicating whether the document is encrypted and requires a password to open.
        #[serde(rename = "IsEncrypted", skip_serializing_if = "Option::is_none")]
        pub r#is_encrypted: Option<bool>,


        /// Gets or sets a value indicating whether the document contains a digital signature. This property merely informs that a digital signature is present on a document, but it does not specify whether the signature is valid or not.
        #[serde(rename = "IsSigned", skip_serializing_if = "Option::is_none")]
        pub r#is_signed: Option<bool>,


        /// Gets or sets the original format of the document.
        #[serde(rename = "SourceFormat", skip_serializing_if = "Option::is_none")]
        pub r#source_format: Option<Document_SourceFormatEnum>,

}

impl Default for Document {
    fn default() -> Self {
        Self {
            r#links: None,
            r#document_properties: None,
            r#file_name: None,
            r#file_size: None,
            r#is_encrypted: None,
            r#is_signed: None,
            r#source_format: None,
        }
    }
}

impl Model for Document {
    fn validate(&self) -> SdkResult<()> {
        if self.r#is_encrypted.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsEncrypted in Document is required".to_owned(),
            ));
        }
        if self.r#is_signed.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsSigned in Document is required".to_owned(),
            ));
        }
        if self.r#source_format.is_none() {
            return Err(SdkError::InvalidRequest(
                "property SourceFormat in Document is required".to_owned(),
            ));
        }
        if let Some(values) = &self.r#links {
        for value in values {
        value.validate()?;
        }
        }
        if let Some(value) = &self.r#document_properties {
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

/// Gets or sets the original format of the document.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Document_SourceFormatEnum {
    #[serde(rename = "Unknown")]
        Unknown,
    #[serde(rename = "Doc")]
        Doc,
    #[serde(rename = "Dot")]
        Dot,
    #[serde(rename = "DocPreWord60")]
        DocPreWord60,
    #[serde(rename = "Docx")]
        Docx,
    #[serde(rename = "Docm")]
        Docm,
    #[serde(rename = "Dotx")]
        Dotx,
    #[serde(rename = "Dotm")]
        Dotm,
    #[serde(rename = "FlatOpc")]
        FlatOpc,
    #[serde(rename = "Rtf")]
        Rtf,
    #[serde(rename = "WordML")]
        WordML,
    #[serde(rename = "Html")]
        Html,
    #[serde(rename = "Mhtml")]
        Mhtml,
    #[serde(rename = "Epub")]
        Epub,
    #[serde(rename = "Text")]
        Text,
    #[serde(rename = "Odt")]
        Odt,
    #[serde(rename = "Ott")]
        Ott,
    #[serde(rename = "Pdf")]
        Pdf,
    #[serde(rename = "Xps")]
        Xps,
    #[serde(rename = "Tiff")]
        Tiff,
    #[serde(rename = "Svg")]
        Svg,
    #[serde(rename = "Azw3")]
        Azw3,
}