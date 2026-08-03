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

/// Insert document to document list.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListInsert {
        /// Gets or sets the option that controls how list should be restarted at each section.
            /// This option is supported only in RTF, DOC and DOCX document formats. This option will be written to DOCX only if Aspose.Words.Saving.OoxmlCompliance is higher then Aspose.Words.Saving.OoxmlCompliance.Ecma376_2006.
        #[serde(rename = "Template", skip_serializing_if = "Option::is_none")]
        pub template: Option<ListInsertTemplateEnum>,

}

impl Default for ListInsert {
    fn default() -> Self {
        Self {
            template: None,
        }
    }
}

impl Model for ListInsert {
    fn validate(&self) -> SdkResult<()> {
        if self.template.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Template in ListInsert is required".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the option that controls how list should be restarted at each section.
/// This option is supported only in RTF, DOC and DOCX document formats. This option will be written to DOCX only if Aspose.Words.Saving.OoxmlCompliance is higher then Aspose.Words.Saving.OoxmlCompliance.Ecma376_2006.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListInsertTemplateEnum {
    #[serde(rename = "BulletDefault")]
        BulletDefault,
    #[serde(rename = "BulletDisk")]
        BulletDisk,
    #[serde(rename = "BulletCircle")]
        BulletCircle,
    #[serde(rename = "BulletSquare")]
        BulletSquare,
    #[serde(rename = "BulletDiamonds")]
        BulletDiamonds,
    #[serde(rename = "BulletArrowHead")]
        BulletArrowHead,
    #[serde(rename = "BulletTick")]
        BulletTick,
    #[serde(rename = "NumberDefault")]
        NumberDefault,
    #[serde(rename = "NumberArabicDot")]
        NumberArabicDot,
    #[serde(rename = "NumberArabicParenthesis")]
        NumberArabicParenthesis,
    #[serde(rename = "NumberUppercaseRomanDot")]
        NumberUppercaseRomanDot,
    #[serde(rename = "NumberUppercaseLetterDot")]
        NumberUppercaseLetterDot,
    #[serde(rename = "NumberLowercaseLetterParenthesis")]
        NumberLowercaseLetterParenthesis,
    #[serde(rename = "NumberLowercaseLetterDot")]
        NumberLowercaseLetterDot,
    #[serde(rename = "NumberLowercaseRomanDot")]
        NumberLowercaseRomanDot,
    #[serde(rename = "OutlineNumbers")]
        OutlineNumbers,
    #[serde(rename = "OutlineLegal")]
        OutlineLegal,
    #[serde(rename = "OutlineBullets")]
        OutlineBullets,
    #[serde(rename = "OutlineHeadingsArticleSection")]
        OutlineHeadingsArticleSection,
    #[serde(rename = "OutlineHeadingsLegal")]
        OutlineHeadingsLegal,
    #[serde(rename = "OutlineHeadingsNumbers")]
        OutlineHeadingsNumbers,
    #[serde(rename = "OutlineHeadingsChapter")]
        OutlineHeadingsChapter,
}