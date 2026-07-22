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

/// DTO container with an OfficeMath object.
#[derive(Debug, Deserialize, Serialize)]
pub struct OfficeMathObject {
    #[serde(flatten)]
    pub parent: OfficeMathLink,
        /// Gets or sets the content of a footnote.
        #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
        pub content: Option<StoryChildNodes>,


        /// Gets or sets the display format type of the OfficeMath object. This display format defines whether an equation is displayed inline with the text or displayed on its own line.
            /// Display format type has effect for top level Office Math only.Returned display format type is always Inline for nested Office Math.
        #[serde(rename = "DisplayType", skip_serializing_if = "Option::is_none")]
        pub display_type: Option<OfficeMathObjectDisplayTypeEnum>,


        /// Gets or sets the justification of the OfficeMath object.
            /// Justification cannot be set to the Office Math with display format type Inline.Inline justification cannot be set to the Office Math with display format type Display.Corresponding DisplayType has to be set before setting Office Math justification.
        #[serde(rename = "Justification", skip_serializing_if = "Option::is_none")]
        pub justification: Option<OfficeMathObjectJustificationEnum>,


        /// Gets or sets the type of the OfficeMath object.
        #[serde(rename = "MathObjectType", skip_serializing_if = "Option::is_none")]
        pub math_object_type: Option<OfficeMathObjectMathObjectTypeEnum>,

}

impl Default for OfficeMathObject {
    fn default() -> Self {
        let mut parent = OfficeMathLink::default();
        Self {
            parent,
            content: None,
            display_type: None,
            justification: None,
            math_object_type: None,
        }
    }
}

impl Deref for OfficeMathObject {
    type Target = OfficeMathLink;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for OfficeMathObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for OfficeMathObject {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if let Some(value) = &self.content {
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

/// Gets or sets the display format type of the OfficeMath object. This display format defines whether an equation is displayed inline with the text or displayed on its own line.
/// Display format type has effect for top level Office Math only.Returned display format type is always Inline for nested Office Math.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OfficeMathObjectDisplayTypeEnum {
    #[serde(rename = "Display")]
        Display,
    #[serde(rename = "Inline")]
        Inline,
}

/// Gets or sets the justification of the OfficeMath object.
/// Justification cannot be set to the Office Math with display format type Inline.Inline justification cannot be set to the Office Math with display format type Display.Corresponding DisplayType has to be set before setting Office Math justification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OfficeMathObjectJustificationEnum {
    #[serde(rename = "CenterGroup")]
        CenterGroup,
    #[serde(rename = "Default")]
        Default,
    #[serde(rename = "Center")]
        Center,
    #[serde(rename = "Left")]
        Left,
    #[serde(rename = "Right")]
        Right,
    #[serde(rename = "Inline")]
        Inline,
}

/// Gets or sets the type of the OfficeMath object.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OfficeMathObjectMathObjectTypeEnum {
    #[serde(rename = "OMath")]
        OMath,
    #[serde(rename = "OMathPara")]
        OMathPara,
    #[serde(rename = "Accent")]
        Accent,
    #[serde(rename = "Bar")]
        Bar,
    #[serde(rename = "BorderBox")]
        BorderBox,
    #[serde(rename = "Box")]
        Box,
    #[serde(rename = "Delimiter")]
        Delimiter,
    #[serde(rename = "Degree")]
        Degree,
    #[serde(rename = "Argument")]
        Argument,
    #[serde(rename = "Array")]
        Array,
    #[serde(rename = "Fraction")]
        Fraction,
    #[serde(rename = "Denominator")]
        Denominator,
    #[serde(rename = "Numerator")]
        Numerator,
    #[serde(rename = "Function")]
        Function,
    #[serde(rename = "FunctionName")]
        FunctionName,
    #[serde(rename = "GroupCharacter")]
        GroupCharacter,
    #[serde(rename = "Limit")]
        Limit,
    #[serde(rename = "LowerLimit")]
        LowerLimit,
    #[serde(rename = "UpperLimit")]
        UpperLimit,
    #[serde(rename = "Matrix")]
        Matrix,
    #[serde(rename = "MatrixRow")]
        MatrixRow,
    #[serde(rename = "NAry")]
        NAry,
    #[serde(rename = "Phantom")]
        Phantom,
    #[serde(rename = "Radical")]
        Radical,
    #[serde(rename = "SubscriptPart")]
        SubscriptPart,
    #[serde(rename = "SuperscriptPart")]
        SuperscriptPart,
    #[serde(rename = "PreSubSuperscript")]
        PreSubSuperscript,
    #[serde(rename = "Subscript")]
        Subscript,
    #[serde(rename = "SubSuperscript")]
        SubSuperscript,
    #[serde(rename = "Superscript")]
        Superscript,
    #[serde(rename = "None")]
        None,
}