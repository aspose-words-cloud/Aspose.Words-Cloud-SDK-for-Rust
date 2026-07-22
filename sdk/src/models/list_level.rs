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

/// DTO container with a document list level.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListLevel {
    #[serde(flatten)]
    pub parent: LinkElement,
        /// Gets or sets the starting number for this list level.
            /// The default value is 1.
        #[serde(rename = "StartAt", skip_serializing_if = "Option::is_none")]
        pub r#start_at: Option<i32>,


        /// Gets or sets the number style for this list level.
        #[serde(rename = "NumberStyle", skip_serializing_if = "Option::is_none")]
        pub r#number_style: Option<ListLevel_NumberStyleEnum>,


        /// Gets or sets the number format for the list level.
            /// Among normal text characters, the string can contain placeholder characters \\x0000 to \\x0008 representing the numbers from the corresponding list levels. For example, the string "\\x0000.\\x0001)" will generate a list label that looks something like "1.5)". The number "1" is the current number from the 1st list level, the number "5" is the current number from the 2nd list level. Null is not allowed, but an empty string meaning no number is valid.
        #[serde(rename = "NumberFormat", skip_serializing_if = "Option::is_none")]
        pub r#number_format: Option<String>,


        /// Gets or sets the justification of the actual number of the list item.
            /// The list label is justified relative to the Aspose.Words.Lists.ListLevel.NumberPosition property.
        #[serde(rename = "Alignment", skip_serializing_if = "Option::is_none")]
        pub r#alignment: Option<ListLevel_AlignmentEnum>,


        /// Gets or sets a value indicating whether the level turns all inherited numbers to Arabic, false if it preserves their number style.
        #[serde(rename = "IsLegal", skip_serializing_if = "Option::is_none")]
        pub r#is_legal: Option<bool>,


        /// Gets or sets the list level, that must appear before the specified list level restarts numbering.
            /// The value of -1 means the numbering will continue.
        #[serde(rename = "RestartAfterLevel", skip_serializing_if = "Option::is_none")]
        pub r#restart_after_level: Option<i32>,


        /// Gets or sets the character inserted after the number for the list level.
        #[serde(rename = "TrailingCharacter", skip_serializing_if = "Option::is_none")]
        pub r#trailing_character: Option<ListLevel_TrailingCharacterEnum>,


        /// Gets or sets character formatting used for the list label.
        #[serde(rename = "Font", skip_serializing_if = "Option::is_none")]
        pub r#font: Option<Font>,


        /// Gets or sets the tab position (in points) for the list level.
            /// Has effect only when Aspose.Words.Lists.ListLevel.TrailingCharacter is a tab.
            /// Aspose.Words.Lists.ListLevel.NumberPosition Aspose.Words.Lists.ListLevel.TextPosition.
        #[serde(rename = "TabPosition", skip_serializing_if = "Option::is_none")]
        pub r#tab_position: Option<f64>,


        /// Gets or sets the position (in points) of the number or bullet for the list level.
            /// Aspose.Words.Lists.ListLevel.NumberPosition corresponds to LeftIndent plus FirstLineIndent of the paragraph. Aspose.Words.Lists.ListLevel.TextPosition Aspose.Words.Lists.ListLevel.TabPosition.
        #[serde(rename = "NumberPosition", skip_serializing_if = "Option::is_none")]
        pub r#number_position: Option<f64>,


        /// Gets or sets the position (in points) for the second line of wrapping text for the list level.
            /// Aspose.Words.Lists.ListLevel.TextPosition corresponds to LeftIndent of the paragraph.
            /// Aspose.Words.Lists.ListLevel.NumberPosition Aspose.Words.Lists.ListLevel.TabPosition.
        #[serde(rename = "TextPosition", skip_serializing_if = "Option::is_none")]
        pub r#text_position: Option<f64>,


        /// Gets or sets the paragraph style that is linked to this list level.
            /// This property is null when the list level is not linked to a paragraph style.
            /// This property can be set to null.
        #[serde(rename = "LinkedStyle", skip_serializing_if = "Option::is_none")]
        pub r#linked_style: Option<Style>,

}

impl Default for ListLevel {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#start_at: None,
            r#number_style: None,
            r#number_format: None,
            r#alignment: None,
            r#is_legal: None,
            r#restart_after_level: None,
            r#trailing_character: None,
            r#font: None,
            r#tab_position: None,
            r#number_position: None,
            r#text_position: None,
            r#linked_style: None,
        }
    }
}

impl Deref for ListLevel {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for ListLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for ListLevel {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.r#start_at.is_none() {
            return Err(SdkError::InvalidRequest(
                "property StartAt in ListLevel is required".to_owned(),
            ));
        }
        if self.r#number_style.is_none() {
            return Err(SdkError::InvalidRequest(
                "property NumberStyle in ListLevel is required".to_owned(),
            ));
        }
        if self.r#alignment.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Alignment in ListLevel is required".to_owned(),
            ));
        }
        if self.r#is_legal.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsLegal in ListLevel is required".to_owned(),
            ));
        }
        if self.r#restart_after_level.is_none() {
            return Err(SdkError::InvalidRequest(
                "property RestartAfterLevel in ListLevel is required".to_owned(),
            ));
        }
        if self.r#trailing_character.is_none() {
            return Err(SdkError::InvalidRequest(
                "property TrailingCharacter in ListLevel is required".to_owned(),
            ));
        }
        if self.r#tab_position.is_none() {
            return Err(SdkError::InvalidRequest(
                "property TabPosition in ListLevel is required".to_owned(),
            ));
        }
        if self.r#number_position.is_none() {
            return Err(SdkError::InvalidRequest(
                "property NumberPosition in ListLevel is required".to_owned(),
            ));
        }
        if self.r#text_position.is_none() {
            return Err(SdkError::InvalidRequest(
                "property TextPosition in ListLevel is required".to_owned(),
            ));
        }
        if let Some(value) = &self.r#font {
        value.validate()?;
        }



        if let Some(value) = &self.r#linked_style {
        value.validate()?;
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the number style for this list level.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListLevel_NumberStyleEnum {
    #[serde(rename = "Arabic")]
        Arabic,
    #[serde(rename = "UppercaseRoman")]
        UppercaseRoman,
    #[serde(rename = "LowercaseRoman")]
        LowercaseRoman,
    #[serde(rename = "UppercaseLetter")]
        UppercaseLetter,
    #[serde(rename = "LowercaseLetter")]
        LowercaseLetter,
    #[serde(rename = "Ordinal")]
        Ordinal,
    #[serde(rename = "Number")]
        Number,
    #[serde(rename = "OrdinalText")]
        OrdinalText,
    #[serde(rename = "Hex")]
        Hex,
    #[serde(rename = "ChicagoManual")]
        ChicagoManual,
    #[serde(rename = "Kanji")]
        Kanji,
    #[serde(rename = "KanjiDigit")]
        KanjiDigit,
    #[serde(rename = "AiueoHalfWidth")]
        AiueoHalfWidth,
    #[serde(rename = "IrohaHalfWidth")]
        IrohaHalfWidth,
    #[serde(rename = "ArabicFullWidth")]
        ArabicFullWidth,
    #[serde(rename = "ArabicHalfWidth")]
        ArabicHalfWidth,
    #[serde(rename = "KanjiTraditional")]
        KanjiTraditional,
    #[serde(rename = "KanjiTraditional2")]
        KanjiTraditional2,
    #[serde(rename = "NumberInCircle")]
        NumberInCircle,
    #[serde(rename = "DecimalFullWidth")]
        DecimalFullWidth,
    #[serde(rename = "Aiueo")]
        Aiueo,
    #[serde(rename = "Iroha")]
        Iroha,
    #[serde(rename = "LeadingZero")]
        LeadingZero,
    #[serde(rename = "Bullet")]
        Bullet,
    #[serde(rename = "Ganada")]
        Ganada,
    #[serde(rename = "Chosung")]
        Chosung,
    #[serde(rename = "GB1")]
        GB1,
    #[serde(rename = "GB2")]
        GB2,
    #[serde(rename = "GB3")]
        GB3,
    #[serde(rename = "GB4")]
        GB4,
    #[serde(rename = "Zodiac1")]
        Zodiac1,
    #[serde(rename = "Zodiac2")]
        Zodiac2,
    #[serde(rename = "Zodiac3")]
        Zodiac3,
    #[serde(rename = "TradChinNum1")]
        TradChinNum1,
    #[serde(rename = "TradChinNum2")]
        TradChinNum2,
    #[serde(rename = "TradChinNum3")]
        TradChinNum3,
    #[serde(rename = "TradChinNum4")]
        TradChinNum4,
    #[serde(rename = "SimpChinNum1")]
        SimpChinNum1,
    #[serde(rename = "SimpChinNum2")]
        SimpChinNum2,
    #[serde(rename = "SimpChinNum3")]
        SimpChinNum3,
    #[serde(rename = "SimpChinNum4")]
        SimpChinNum4,
    #[serde(rename = "HanjaRead")]
        HanjaRead,
    #[serde(rename = "HanjaReadDigit")]
        HanjaReadDigit,
    #[serde(rename = "Hangul")]
        Hangul,
    #[serde(rename = "Hanja")]
        Hanja,
    #[serde(rename = "Hebrew1")]
        Hebrew1,
    #[serde(rename = "Arabic1")]
        Arabic1,
    #[serde(rename = "Hebrew2")]
        Hebrew2,
    #[serde(rename = "Arabic2")]
        Arabic2,
    #[serde(rename = "HindiLetter1")]
        HindiLetter1,
    #[serde(rename = "HindiLetter2")]
        HindiLetter2,
    #[serde(rename = "HindiArabic")]
        HindiArabic,
    #[serde(rename = "HindiCardinalText")]
        HindiCardinalText,
    #[serde(rename = "ThaiLetter")]
        ThaiLetter,
    #[serde(rename = "ThaiArabic")]
        ThaiArabic,
    #[serde(rename = "ThaiCardinalText")]
        ThaiCardinalText,
    #[serde(rename = "VietCardinalText")]
        VietCardinalText,
    #[serde(rename = "NumberInDash")]
        NumberInDash,
    #[serde(rename = "LowercaseRussian")]
        LowercaseRussian,
    #[serde(rename = "UppercaseRussian")]
        UppercaseRussian,
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Custom")]
        Custom,
}

/// Gets or sets the justification of the actual number of the list item.
/// The list label is justified relative to the Aspose.Words.Lists.ListLevel.NumberPosition property.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListLevel_AlignmentEnum {
    #[serde(rename = "Left")]
        Left,
    #[serde(rename = "Center")]
        Center,
    #[serde(rename = "Right")]
        Right,
}

/// Gets or sets the character inserted after the number for the list level.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListLevel_TrailingCharacterEnum {
    #[serde(rename = "Tab")]
        Tab,
    #[serde(rename = "Space")]
        Space,
    #[serde(rename = "Nothing")]
        Nothing,
}