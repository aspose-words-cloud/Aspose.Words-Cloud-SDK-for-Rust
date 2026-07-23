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

/// Represents a document list levels.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListLevelUpdate {
    /// Gets or sets the starting number for this list level.
    /// The default value is 1.
    #[serde(rename = "StartAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,

    /// Gets or sets the number style for this list level.
    #[serde(rename = "NumberStyle", skip_serializing_if = "Option::is_none")]
    pub number_style: Option<ListLevelUpdateNumberStyleEnum>,

    /// Gets or sets the number format for the list level.
    /// Among normal text characters, the string can contain placeholder characters \\x0000 to \\x0008 representing the numbers from the corresponding list levels. For example, the string "\\x0000.\\x0001)" will generate a list label that looks something like "1.5)". The number "1" is the current number from the 1st list level, the number "5" is the current number from the 2nd list level. Null is not allowed, but an empty string meaning no number is valid.
    #[serde(rename = "NumberFormat", skip_serializing_if = "Option::is_none")]
    pub number_format: Option<String>,

    /// Gets or sets the justification of the actual number of the list item.
    /// The list label is justified relative to the Aspose.Words.Lists.ListLevel.NumberPosition
    /// property.
    #[serde(rename = "Alignment", skip_serializing_if = "Option::is_none")]
    pub alignment: Option<ListLevelUpdateAlignmentEnum>,

    /// Gets or sets a value indicating whether the level turns all inherited numbers to Arabic, false if it preserves their number style.
    #[serde(rename = "IsLegal", skip_serializing_if = "Option::is_none")]
    pub is_legal: Option<bool>,

    /// Gets or sets the list level that must appear before the specified list level restarts numbering.
    /// The value of -1 means the numbering will continue.
    #[serde(rename = "RestartAfterLevel", skip_serializing_if = "Option::is_none")]
    pub restart_after_level: Option<i32>,

    /// Gets or sets the character to be inserted after the number for the list level.
    #[serde(rename = "TrailingCharacter", skip_serializing_if = "Option::is_none")]
    pub trailing_character: Option<ListLevelUpdateTrailingCharacterEnum>,

    /// Gets or sets the tab position (in points) for the list level.
    /// Has effect only when Aspose.Words.Lists.ListLevel.TrailingCharacter is a tab.
    /// Aspose.Words.Lists.ListLevel.NumberPosition Aspose.Words.Lists.ListLevel.TextPosition.
    #[serde(rename = "TabPosition", skip_serializing_if = "Option::is_none")]
    pub tab_position: Option<f64>,

    /// Gets or sets the position (in points) of the number or bullet for the list level.
    /// Aspose.Words.Lists.ListLevel.NumberPosition corresponds to LeftIndent plus FirstLineIndent of the paragraph. Aspose.Words.Lists.ListLevel.TextPosition Aspose.Words.Lists.ListLevel.TabPosition.
    #[serde(rename = "NumberPosition", skip_serializing_if = "Option::is_none")]
    pub number_position: Option<f64>,

    /// Gets or sets the position (in points) for the second line of wrapping text for the list level.
    /// Aspose.Words.Lists.ListLevel.TextPosition corresponds to LeftIndent of the paragraph.
    /// Aspose.Words.Lists.ListLevel.NumberPosition Aspose.Words.Lists.ListLevel.TabPosition.
    #[serde(rename = "TextPosition", skip_serializing_if = "Option::is_none")]
    pub text_position: Option<f64>,
}

impl Default for ListLevelUpdate {
    fn default() -> Self {
        Self {
            start_at: None,
            number_style: None,
            number_format: None,
            alignment: None,
            is_legal: None,
            restart_after_level: None,
            trailing_character: None,
            tab_position: None,
            number_position: None,
            text_position: None,
        }
    }
}

impl Model for ListLevelUpdate {
    fn validate(&self) -> SdkResult<()> {
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, _output: &mut Vec<&'a FileReference>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the number style for this list level.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListLevelUpdateNumberStyleEnum {
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
    Gb1,
    #[serde(rename = "GB2")]
    Gb2,
    #[serde(rename = "GB3")]
    Gb3,
    #[serde(rename = "GB4")]
    Gb4,
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
/// The list label is justified relative to the Aspose.Words.Lists.ListLevel.NumberPosition
/// property.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListLevelUpdateAlignmentEnum {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Center")]
    Center,
    #[serde(rename = "Right")]
    Right,
}

/// Gets or sets the character to be inserted after the number for the list level.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ListLevelUpdateTrailingCharacterEnum {
    #[serde(rename = "Tab")]
    Tab,
    #[serde(rename = "Space")]
    Space,
    #[serde(rename = "Nothing")]
    Nothing,
}
