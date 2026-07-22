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

/// Represents the page setup properties of a section.
/// PageSetup object contains all the page setup attributes of a section (left margin, bottom margin, paper size, and so on) as properties.
#[derive(Debug, Deserialize, Serialize)]
pub struct PageSetup {
    #[serde(flatten)]
    pub parent: LinkElement,
        /// Gets or sets a value indicating whether this section contains bidirectional (complex scripts) text.
            /// When true, the columns in this section are laid out from right to left.
        #[serde(rename = "Bidi", skip_serializing_if = "Option::is_none")]
        pub r#bidi: Option<bool>,


        /// Gets or sets a value indicating whether the page border is positioned relative to intersecting texts and objects.
        #[serde(rename = "BorderAlwaysInFront", skip_serializing_if = "Option::is_none")]
        pub r#border_always_in_front: Option<bool>,


        /// Gets or sets the option that controls which pages the page border is printed on.
        #[serde(rename = "BorderAppliesTo", skip_serializing_if = "Option::is_none")]
        pub r#border_applies_to: Option<PageSetup_BorderAppliesToEnum>,


        /// Gets or sets the value, that indicates whether the specified page border is measured from the edge of the page or from the text it surrounds.
        #[serde(rename = "BorderDistanceFrom", skip_serializing_if = "Option::is_none")]
        pub r#border_distance_from: Option<PageSetup_BorderDistanceFromEnum>,


        /// Gets or sets the distance (in points) between the bottom edge of the page and the bottom boundary of the body text.
        #[serde(rename = "BottomMargin", skip_serializing_if = "Option::is_none")]
        pub r#bottom_margin: Option<f64>,


        /// Gets or sets a value indicating whether a different header or footer is used on the first page.
        #[serde(rename = "DifferentFirstPageHeaderFooter", skip_serializing_if = "Option::is_none")]
        pub r#different_first_page_header_footer: Option<bool>,


        /// Gets or sets the paper tray (bin) to use for the first page of a section.
            /// The value is implementation (printer) specific.
        #[serde(rename = "FirstPageTray", skip_serializing_if = "Option::is_none")]
        pub r#first_page_tray: Option<i32>,


        /// Gets or sets the distance (in points) between the footer and the bottom of the page.
        #[serde(rename = "FooterDistance", skip_serializing_if = "Option::is_none")]
        pub r#footer_distance: Option<f64>,


        /// Gets or sets the amount of extra space added to the margin for document binding.
        #[serde(rename = "Gutter", skip_serializing_if = "Option::is_none")]
        pub r#gutter: Option<f64>,


        /// Gets or sets the distance (in points) between the header and the top of the page.
        #[serde(rename = "HeaderDistance", skip_serializing_if = "Option::is_none")]
        pub r#header_distance: Option<f64>,


        /// Gets or sets the distance (in points) between the left edge of the page and the left boundary of the body text.
        #[serde(rename = "LeftMargin", skip_serializing_if = "Option::is_none")]
        pub r#left_margin: Option<f64>,


        /// Gets or sets the numeric increment for line numbers.
        #[serde(rename = "LineNumberCountBy", skip_serializing_if = "Option::is_none")]
        pub r#line_number_count_by: Option<i32>,


        /// Gets or sets the distance between the right edge of line numbers and the left edge of the document.
            /// Set this property to zero for automatic distance between the line numbers and text of the document.
        #[serde(rename = "LineNumberDistanceFromText", skip_serializing_if = "Option::is_none")]
        pub r#line_number_distance_from_text: Option<f64>,


        /// Gets or sets the way line numbering runs  that is, whether it starts over at the beginning of a new page or section or runs continuously.
        #[serde(rename = "LineNumberRestartMode", skip_serializing_if = "Option::is_none")]
        pub r#line_number_restart_mode: Option<PageSetup_LineNumberRestartModeEnum>,


        /// Gets or sets the starting line number.
        #[serde(rename = "LineStartingNumber", skip_serializing_if = "Option::is_none")]
        pub r#line_starting_number: Option<i32>,


        /// Gets or sets the orientation of the page.
            /// Changing Orientation swaps PageWidth and PageHeight.
        #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
        pub r#orientation: Option<PageSetup_OrientationEnum>,


        /// Gets or sets the paper tray (bin) to be used for all but the first page of a section.
            /// The value is implementation (printer) specific.
        #[serde(rename = "OtherPagesTray", skip_serializing_if = "Option::is_none")]
        pub r#other_pages_tray: Option<i32>,


        /// Gets or sets the height of the page in points.
        #[serde(rename = "PageHeight", skip_serializing_if = "Option::is_none")]
        pub r#page_height: Option<f64>,


        /// Gets or sets the page number format.
        #[serde(rename = "PageNumberStyle", skip_serializing_if = "Option::is_none")]
        pub r#page_number_style: Option<PageSetup_PageNumberStyleEnum>,


        /// Gets or sets the starting page number of the section.
            /// The RestartPageNumbering property, if set to false, will override the PageStartingNumber property so that page numbering can continue from the previous section.
        #[serde(rename = "PageStartingNumber", skip_serializing_if = "Option::is_none")]
        pub r#page_starting_number: Option<i32>,


        /// Gets or sets the width of the page in points.
        #[serde(rename = "PageWidth", skip_serializing_if = "Option::is_none")]
        pub r#page_width: Option<f64>,


        /// Gets or sets the paper size.
            /// Setting this property updates PageWidth and PageHeight values. Setting this value to Custom does not change existing values.
        #[serde(rename = "PaperSize", skip_serializing_if = "Option::is_none")]
        pub r#paper_size: Option<PageSetup_PaperSizeEnum>,


        /// Gets or sets a value indicating whether page numbering restarts at the beginning of the section.
            /// If set to false, the RestartPageNumbering property will override the PageStartingNumber property so that page numbering can continue from the previous section.
        #[serde(rename = "RestartPageNumbering", skip_serializing_if = "Option::is_none")]
        pub r#restart_page_numbering: Option<bool>,


        /// Gets or sets the distance (in points) between the right edge of the page and the right boundary of the body text.
        #[serde(rename = "RightMargin", skip_serializing_if = "Option::is_none")]
        pub r#right_margin: Option<f64>,


        /// Gets or sets a value indicating whether Microsoft Word uses gutters for the section based on a right-to-left language or a left-to-right language.
        #[serde(rename = "RtlGutter", skip_serializing_if = "Option::is_none")]
        pub r#rtl_gutter: Option<bool>,


        /// Gets or sets the type of section break for the specified object.
        #[serde(rename = "SectionStart", skip_serializing_if = "Option::is_none")]
        pub r#section_start: Option<PageSetup_SectionStartEnum>,


        /// Gets or sets a value indicating whether endnotes are printed at the end of the next section that doesn't suppress endnotes. Suppressed endnotes are printed before the endnotes in that section.
        #[serde(rename = "SuppressEndnotes", skip_serializing_if = "Option::is_none")]
        pub r#suppress_endnotes: Option<bool>,


        /// Gets or sets the distance (in points) between the top edge of the page and the top boundary of the body text.
        #[serde(rename = "TopMargin", skip_serializing_if = "Option::is_none")]
        pub r#top_margin: Option<f64>,


        /// Gets or sets the vertical alignment of text on each page in the document.or section.
        #[serde(rename = "VerticalAlignment", skip_serializing_if = "Option::is_none")]
        pub r#vertical_alignment: Option<PageSetup_VerticalAlignmentEnum>,

}

impl Default for PageSetup {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            r#bidi: None,
            r#border_always_in_front: None,
            r#border_applies_to: None,
            r#border_distance_from: None,
            r#bottom_margin: None,
            r#different_first_page_header_footer: None,
            r#first_page_tray: None,
            r#footer_distance: None,
            r#gutter: None,
            r#header_distance: None,
            r#left_margin: None,
            r#line_number_count_by: None,
            r#line_number_distance_from_text: None,
            r#line_number_restart_mode: None,
            r#line_starting_number: None,
            r#orientation: None,
            r#other_pages_tray: None,
            r#page_height: None,
            r#page_number_style: None,
            r#page_starting_number: None,
            r#page_width: None,
            r#paper_size: None,
            r#restart_page_numbering: None,
            r#right_margin: None,
            r#rtl_gutter: None,
            r#section_start: None,
            r#suppress_endnotes: None,
            r#top_margin: None,
            r#vertical_alignment: None,
        }
    }
}

impl Deref for PageSetup {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for PageSetup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for PageSetup {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        self.parent.collect_file_references(output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Gets or sets the option that controls which pages the page border is printed on.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_BorderAppliesToEnum {
    #[serde(rename = "AllPages")]
        AllPages,
    #[serde(rename = "FirstPage")]
        FirstPage,
    #[serde(rename = "OtherPages")]
        OtherPages,
}

/// Gets or sets the value, that indicates whether the specified page border is measured from the edge of the page or from the text it surrounds.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_BorderDistanceFromEnum {
    #[serde(rename = "Text")]
        Text,
    #[serde(rename = "PageEdge")]
        PageEdge,
}

/// Gets or sets the way line numbering runs  that is, whether it starts over at the beginning of a new page or section or runs continuously.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_LineNumberRestartModeEnum {
    #[serde(rename = "RestartPage")]
        RestartPage,
    #[serde(rename = "RestartSection")]
        RestartSection,
    #[serde(rename = "Continuous")]
        Continuous,
}

/// Gets or sets the orientation of the page.
/// Changing Orientation swaps PageWidth and PageHeight.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_OrientationEnum {
    #[serde(rename = "Portrait")]
        Portrait,
    #[serde(rename = "Landscape")]
        Landscape,
}

/// Gets or sets the page number format.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_PageNumberStyleEnum {
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

/// Gets or sets the paper size.
/// Setting this property updates PageWidth and PageHeight values. Setting this value to Custom does not change existing values.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_PaperSizeEnum {
    #[serde(rename = "A3")]
        A3,
    #[serde(rename = "A4")]
        A4,
    #[serde(rename = "A5")]
        A5,
    #[serde(rename = "B4")]
        B4,
    #[serde(rename = "B5")]
        B5,
    #[serde(rename = "Executive")]
        Executive,
    #[serde(rename = "Folio")]
        Folio,
    #[serde(rename = "Ledger")]
        Ledger,
    #[serde(rename = "Legal")]
        Legal,
    #[serde(rename = "Letter")]
        Letter,
    #[serde(rename = "EnvelopeDL")]
        EnvelopeDL,
    #[serde(rename = "Quarto")]
        Quarto,
    #[serde(rename = "Statement")]
        Statement,
    #[serde(rename = "Tabloid")]
        Tabloid,
    #[serde(rename = "Paper10x14")]
        Paper10x14,
    #[serde(rename = "Paper11x17")]
        Paper11x17,
    #[serde(rename = "Number10Envelope")]
        Number10Envelope,
    #[serde(rename = "JisB4")]
        JisB4,
    #[serde(rename = "JisB5")]
        JisB5,
    #[serde(rename = "Custom")]
        Custom,
}

/// Gets or sets the type of section break for the specified object.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_SectionStartEnum {
    #[serde(rename = "Continuous")]
        Continuous,
    #[serde(rename = "NewColumn")]
        NewColumn,
    #[serde(rename = "NewPage")]
        NewPage,
    #[serde(rename = "EvenPage")]
        EvenPage,
    #[serde(rename = "OddPage")]
        OddPage,
}

/// Gets or sets the vertical alignment of text on each page in the document.or section.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PageSetup_VerticalAlignmentEnum {
    #[serde(rename = "Top")]
        Top,
    #[serde(rename = "Center")]
        Center,
    #[serde(rename = "Justify")]
        Justify,
    #[serde(rename = "Bottom")]
        Bottom,
}