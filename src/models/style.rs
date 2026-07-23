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

/// DTO container with a single document style.
#[derive(Debug, Deserialize, Serialize)]
pub struct Style {
    #[serde(flatten)]
    pub parent: LinkElement,
    /// Gets or sets the character formatting of the style.
    /// For list styles this property returns null.
    #[serde(rename = "Font", skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,

    /// Gets or sets a value indicating whether this style is one of the built-in styles in MS Word.
    #[serde(rename = "BuiltIn", skip_serializing_if = "Option::is_none")]
    pub built_in: Option<bool>,

    /// Gets or sets the name of the style to be applied automatically to a new paragraph inserted after a paragraph formatted with the specified style.
    /// This property is not used by Aspose.Words. The next paragraph style will only be applied automatically when you edit the document in MS Word.
    #[serde(
        rename = "NextParagraphStyleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_paragraph_style_name: Option<String>,

    /// Gets or sets the name of the style this style is based on.
    /// This will be an empty string if the style is not based on any other style and it can be set to an empty string.
    #[serde(rename = "BaseStyleName", skip_serializing_if = "Option::is_none")]
    pub base_style_name: Option<String>,

    /// Gets or sets a value indicating whether this style is shown in the Quick Style gallery inside MS Word UI.
    #[serde(rename = "IsQuickStyle", skip_serializing_if = "Option::is_none")]
    pub is_quick_style: Option<bool>,

    /// Gets or sets the name of the Style linked to this one. Returns Empty string if no styles are linked.
    #[serde(rename = "LinkedStyleName", skip_serializing_if = "Option::is_none")]
    pub linked_style_name: Option<String>,

    /// Gets or sets the style type (paragraph or character).
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<StyleTypeEnum>,

    /// Gets or sets a value indicating whether the style is one of the built-in Heading styles.
    #[serde(rename = "IsHeading", skip_serializing_if = "Option::is_none")]
    pub is_heading: Option<bool>,

    /// Gets or sets all aliases of this style. If style has no aliases then empty array of string is returned.
    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    /// Gets or sets the locale independent style identifier for a built-in style.
    /// For user defined (custom) styles, this property returns Aspose.Words.StyleIdentifier.User.
    /// Aspose.Words.Style.Name.
    #[serde(rename = "StyleIdentifier", skip_serializing_if = "Option::is_none")]
    pub style_identifier: Option<StyleStyleIdentifierEnum>,

    /// Gets or sets the name of the style.
    /// Cannot be an empty string. If there already is a style with such name in the collection, then this style will override it. All affected nodes will reference new style.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for Style {
    fn default() -> Self {
        let mut parent = LinkElement::default();
        Self {
            parent,
            font: None,
            built_in: None,
            next_paragraph_style_name: None,
            base_style_name: None,
            is_quick_style: None,
            linked_style_name: None,
            r#type: None,
            is_heading: None,
            aliases: None,
            style_identifier: None,
            name: None,
        }
    }
}

impl Deref for Style {
    type Target = LinkElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Model for Style {
    fn validate(&self) -> SdkResult<()> {
        self.parent.validate()?;
        if self.built_in.is_none() {
            return Err(SdkError::InvalidRequest(
                "property BuiltIn in Style is required".to_owned(),
            ));
        }
        if self.is_quick_style.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsQuickStyle in Style is required".to_owned(),
            ));
        }
        if self.r#type.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Type in Style is required".to_owned(),
            ));
        }
        if self.is_heading.is_none() {
            return Err(SdkError::InvalidRequest(
                "property IsHeading in Style is required".to_owned(),
            ));
        }
        if self.style_identifier.is_none() {
            return Err(SdkError::InvalidRequest(
                "property StyleIdentifier in Style is required".to_owned(),
            ));
        }
        if let Some(value) = &self.font {
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

/// Gets or sets the style type (paragraph or character).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StyleTypeEnum {
    #[serde(rename = "Paragraph")]
    Paragraph,
    #[serde(rename = "Character")]
    Character,
    #[serde(rename = "Table")]
    Table,
    #[serde(rename = "List")]
    List,
}

/// Gets or sets the locale independent style identifier for a built-in style.
/// For user defined (custom) styles, this property returns Aspose.Words.StyleIdentifier.User.
/// Aspose.Words.Style.Name.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StyleStyleIdentifierEnum {
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Heading1")]
    Heading1,
    #[serde(rename = "Heading2")]
    Heading2,
    #[serde(rename = "Heading3")]
    Heading3,
    #[serde(rename = "Heading4")]
    Heading4,
    #[serde(rename = "Heading5")]
    Heading5,
    #[serde(rename = "Heading6")]
    Heading6,
    #[serde(rename = "Heading7")]
    Heading7,
    #[serde(rename = "Heading8")]
    Heading8,
    #[serde(rename = "Heading9")]
    Heading9,
    #[serde(rename = "Index1")]
    Index1,
    #[serde(rename = "Index2")]
    Index2,
    #[serde(rename = "Index3")]
    Index3,
    #[serde(rename = "Index4")]
    Index4,
    #[serde(rename = "Index5")]
    Index5,
    #[serde(rename = "Index6")]
    Index6,
    #[serde(rename = "Index7")]
    Index7,
    #[serde(rename = "Index8")]
    Index8,
    #[serde(rename = "Index9")]
    Index9,
    #[serde(rename = "Toc1")]
    Toc1,
    #[serde(rename = "Toc2")]
    Toc2,
    #[serde(rename = "Toc3")]
    Toc3,
    #[serde(rename = "Toc4")]
    Toc4,
    #[serde(rename = "Toc5")]
    Toc5,
    #[serde(rename = "Toc6")]
    Toc6,
    #[serde(rename = "Toc7")]
    Toc7,
    #[serde(rename = "Toc8")]
    Toc8,
    #[serde(rename = "Toc9")]
    Toc9,
    #[serde(rename = "NormalIndent")]
    NormalIndent,
    #[serde(rename = "FootnoteText")]
    FootnoteText,
    #[serde(rename = "CommentText")]
    CommentText,
    #[serde(rename = "Header")]
    Header,
    #[serde(rename = "Footer")]
    Footer,
    #[serde(rename = "IndexHeading")]
    IndexHeading,
    #[serde(rename = "Caption")]
    Caption,
    #[serde(rename = "TableOfFigures")]
    TableOfFigures,
    #[serde(rename = "EnvelopeAddress")]
    EnvelopeAddress,
    #[serde(rename = "EnvelopeReturn")]
    EnvelopeReturn,
    #[serde(rename = "FootnoteReference")]
    FootnoteReference,
    #[serde(rename = "CommentReference")]
    CommentReference,
    #[serde(rename = "LineNumber")]
    LineNumber,
    #[serde(rename = "PageNumber")]
    PageNumber,
    #[serde(rename = "EndnoteReference")]
    EndnoteReference,
    #[serde(rename = "EndnoteText")]
    EndnoteText,
    #[serde(rename = "TableOfAuthorities")]
    TableOfAuthorities,
    #[serde(rename = "Macro")]
    Macro,
    #[serde(rename = "ToaHeading")]
    ToaHeading,
    #[serde(rename = "List")]
    List,
    #[serde(rename = "ListBullet")]
    ListBullet,
    #[serde(rename = "ListNumber")]
    ListNumber,
    #[serde(rename = "List2")]
    List2,
    #[serde(rename = "List3")]
    List3,
    #[serde(rename = "List4")]
    List4,
    #[serde(rename = "List5")]
    List5,
    #[serde(rename = "ListBullet2")]
    ListBullet2,
    #[serde(rename = "ListBullet3")]
    ListBullet3,
    #[serde(rename = "ListBullet4")]
    ListBullet4,
    #[serde(rename = "ListBullet5")]
    ListBullet5,
    #[serde(rename = "ListNumber2")]
    ListNumber2,
    #[serde(rename = "ListNumber3")]
    ListNumber3,
    #[serde(rename = "ListNumber4")]
    ListNumber4,
    #[serde(rename = "ListNumber5")]
    ListNumber5,
    #[serde(rename = "Title")]
    Title,
    #[serde(rename = "Closing")]
    Closing,
    #[serde(rename = "Signature")]
    Signature,
    #[serde(rename = "DefaultParagraphFont")]
    DefaultParagraphFont,
    #[serde(rename = "BodyText")]
    BodyText,
    #[serde(rename = "BodyTextInd")]
    BodyTextInd,
    #[serde(rename = "ListContinue")]
    ListContinue,
    #[serde(rename = "ListContinue2")]
    ListContinue2,
    #[serde(rename = "ListContinue3")]
    ListContinue3,
    #[serde(rename = "ListContinue4")]
    ListContinue4,
    #[serde(rename = "ListContinue5")]
    ListContinue5,
    #[serde(rename = "MessageHeader")]
    MessageHeader,
    #[serde(rename = "Subtitle")]
    Subtitle,
    #[serde(rename = "Salutation")]
    Salutation,
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "BodyText1I")]
    BodyText1I,
    #[serde(rename = "BodyText1I2")]
    BodyText1I2,
    #[serde(rename = "NoteHeading")]
    NoteHeading,
    #[serde(rename = "BodyText2")]
    BodyText2,
    #[serde(rename = "BodyText3")]
    BodyText3,
    #[serde(rename = "BodyTextInd2")]
    BodyTextInd2,
    #[serde(rename = "BodyTextInd3")]
    BodyTextInd3,
    #[serde(rename = "BlockText")]
    BlockText,
    #[serde(rename = "Hyperlink")]
    Hyperlink,
    #[serde(rename = "FollowedHyperlink")]
    FollowedHyperlink,
    #[serde(rename = "Strong")]
    Strong,
    #[serde(rename = "Emphasis")]
    Emphasis,
    #[serde(rename = "DocumentMap")]
    DocumentMap,
    #[serde(rename = "PlainText")]
    PlainText,
    #[serde(rename = "EmailSignature")]
    EmailSignature,
    #[serde(rename = "HtmlTopOfForm")]
    HtmlTopOfForm,
    #[serde(rename = "HtmlBottomOfForm")]
    HtmlBottomOfForm,
    #[serde(rename = "NormalWeb")]
    NormalWeb,
    #[serde(rename = "HtmlAcronym")]
    HtmlAcronym,
    #[serde(rename = "HtmlAddress")]
    HtmlAddress,
    #[serde(rename = "HtmlCite")]
    HtmlCite,
    #[serde(rename = "HtmlCode")]
    HtmlCode,
    #[serde(rename = "HtmlDefinition")]
    HtmlDefinition,
    #[serde(rename = "HtmlKeyboard")]
    HtmlKeyboard,
    #[serde(rename = "HtmlPreformatted")]
    HtmlPreformatted,
    #[serde(rename = "HtmlSample")]
    HtmlSample,
    #[serde(rename = "HtmlTypewriter")]
    HtmlTypewriter,
    #[serde(rename = "HtmlVariable")]
    HtmlVariable,
    #[serde(rename = "TableNormal")]
    TableNormal,
    #[serde(rename = "CommentSubject")]
    CommentSubject,
    #[serde(rename = "NoList")]
    NoList,
    #[serde(rename = "OutlineList1")]
    OutlineList1,
    #[serde(rename = "OutlineList2")]
    OutlineList2,
    #[serde(rename = "OutlineList3")]
    OutlineList3,
    #[serde(rename = "TableSimple1")]
    TableSimple1,
    #[serde(rename = "TableSimple2")]
    TableSimple2,
    #[serde(rename = "TableSimple3")]
    TableSimple3,
    #[serde(rename = "TableClassic1")]
    TableClassic1,
    #[serde(rename = "TableClassic2")]
    TableClassic2,
    #[serde(rename = "TableClassic3")]
    TableClassic3,
    #[serde(rename = "TableClassic4")]
    TableClassic4,
    #[serde(rename = "TableColorful1")]
    TableColorful1,
    #[serde(rename = "TableColorful2")]
    TableColorful2,
    #[serde(rename = "TableColorful3")]
    TableColorful3,
    #[serde(rename = "TableColumns1")]
    TableColumns1,
    #[serde(rename = "TableColumns2")]
    TableColumns2,
    #[serde(rename = "TableColumns3")]
    TableColumns3,
    #[serde(rename = "TableColumns4")]
    TableColumns4,
    #[serde(rename = "TableColumns5")]
    TableColumns5,
    #[serde(rename = "TableGrid1")]
    TableGrid1,
    #[serde(rename = "TableGrid2")]
    TableGrid2,
    #[serde(rename = "TableGrid3")]
    TableGrid3,
    #[serde(rename = "TableGrid4")]
    TableGrid4,
    #[serde(rename = "TableGrid5")]
    TableGrid5,
    #[serde(rename = "TableGrid6")]
    TableGrid6,
    #[serde(rename = "TableGrid7")]
    TableGrid7,
    #[serde(rename = "TableGrid8")]
    TableGrid8,
    #[serde(rename = "TableList1")]
    TableList1,
    #[serde(rename = "TableList2")]
    TableList2,
    #[serde(rename = "TableList3")]
    TableList3,
    #[serde(rename = "TableList4")]
    TableList4,
    #[serde(rename = "TableList5")]
    TableList5,
    #[serde(rename = "TableList6")]
    TableList6,
    #[serde(rename = "TableList7")]
    TableList7,
    #[serde(rename = "TableList8")]
    TableList8,
    #[serde(rename = "Table3DEffects1")]
    Table3DEffects1,
    #[serde(rename = "Table3DEffects2")]
    Table3DEffects2,
    #[serde(rename = "Table3DEffects3")]
    Table3DEffects3,
    #[serde(rename = "TableContemporary")]
    TableContemporary,
    #[serde(rename = "TableElegant")]
    TableElegant,
    #[serde(rename = "TableProfessional")]
    TableProfessional,
    #[serde(rename = "TableSubtle1")]
    TableSubtle1,
    #[serde(rename = "TableSubtle2")]
    TableSubtle2,
    #[serde(rename = "TableWeb1")]
    TableWeb1,
    #[serde(rename = "TableWeb2")]
    TableWeb2,
    #[serde(rename = "TableWeb3")]
    TableWeb3,
    #[serde(rename = "BalloonText")]
    BalloonText,
    #[serde(rename = "TableGrid")]
    TableGrid,
    #[serde(rename = "TableTheme")]
    TableTheme,
    #[serde(rename = "PlaceholderText")]
    PlaceholderText,
    #[serde(rename = "NoSpacing")]
    NoSpacing,
    #[serde(rename = "LightShading")]
    LightShading,
    #[serde(rename = "LightList")]
    LightList,
    #[serde(rename = "LightGrid")]
    LightGrid,
    #[serde(rename = "MediumShading1")]
    MediumShading1,
    #[serde(rename = "MediumShading2")]
    MediumShading2,
    #[serde(rename = "MediumList1")]
    MediumList1,
    #[serde(rename = "MediumList2")]
    MediumList2,
    #[serde(rename = "MediumGrid1")]
    MediumGrid1,
    #[serde(rename = "MediumGrid2")]
    MediumGrid2,
    #[serde(rename = "MediumGrid3")]
    MediumGrid3,
    #[serde(rename = "DarkList")]
    DarkList,
    #[serde(rename = "ColorfulShading")]
    ColorfulShading,
    #[serde(rename = "ColorfulList")]
    ColorfulList,
    #[serde(rename = "ColorfulGrid")]
    ColorfulGrid,
    #[serde(rename = "LightShadingAccent1")]
    LightShadingAccent1,
    #[serde(rename = "LightListAccent1")]
    LightListAccent1,
    #[serde(rename = "LightGridAccent1")]
    LightGridAccent1,
    #[serde(rename = "MediumShading1Accent1")]
    MediumShading1Accent1,
    #[serde(rename = "MediumShading2Accent1")]
    MediumShading2Accent1,
    #[serde(rename = "MediumList1Accent1")]
    MediumList1Accent1,
    #[serde(rename = "Revision")]
    Revision,
    #[serde(rename = "ListParagraph")]
    ListParagraph,
    #[serde(rename = "Quote")]
    Quote,
    #[serde(rename = "IntenseQuote")]
    IntenseQuote,
    #[serde(rename = "MediumList2Accent1")]
    MediumList2Accent1,
    #[serde(rename = "MediumGrid1Accent1")]
    MediumGrid1Accent1,
    #[serde(rename = "MediumGrid2Accent1")]
    MediumGrid2Accent1,
    #[serde(rename = "MediumGrid3Accent1")]
    MediumGrid3Accent1,
    #[serde(rename = "DarkListAccent1")]
    DarkListAccent1,
    #[serde(rename = "ColorfulShadingAccent1")]
    ColorfulShadingAccent1,
    #[serde(rename = "ColorfulListAccent1")]
    ColorfulListAccent1,
    #[serde(rename = "ColorfulGridAccent1")]
    ColorfulGridAccent1,
    #[serde(rename = "LightShadingAccent2")]
    LightShadingAccent2,
    #[serde(rename = "LightListAccent2")]
    LightListAccent2,
    #[serde(rename = "LightGridAccent2")]
    LightGridAccent2,
    #[serde(rename = "MediumShading1Accent2")]
    MediumShading1Accent2,
    #[serde(rename = "MediumShading2Accent2")]
    MediumShading2Accent2,
    #[serde(rename = "MediumList1Accent2")]
    MediumList1Accent2,
    #[serde(rename = "MediumList2Accent2")]
    MediumList2Accent2,
    #[serde(rename = "MediumGrid1Accent2")]
    MediumGrid1Accent2,
    #[serde(rename = "MediumGrid2Accent2")]
    MediumGrid2Accent2,
    #[serde(rename = "MediumGrid3Accent2")]
    MediumGrid3Accent2,
    #[serde(rename = "DarkListAccent2")]
    DarkListAccent2,
    #[serde(rename = "ColorfulShadingAccent2")]
    ColorfulShadingAccent2,
    #[serde(rename = "ColorfulListAccent2")]
    ColorfulListAccent2,
    #[serde(rename = "ColorfulGridAccent2")]
    ColorfulGridAccent2,
    #[serde(rename = "LightShadingAccent3")]
    LightShadingAccent3,
    #[serde(rename = "LightListAccent3")]
    LightListAccent3,
    #[serde(rename = "LightGridAccent3")]
    LightGridAccent3,
    #[serde(rename = "MediumShading1Accent3")]
    MediumShading1Accent3,
    #[serde(rename = "MediumShading2Accent3")]
    MediumShading2Accent3,
    #[serde(rename = "MediumList1Accent3")]
    MediumList1Accent3,
    #[serde(rename = "MediumList2Accent3")]
    MediumList2Accent3,
    #[serde(rename = "MediumGrid1Accent3")]
    MediumGrid1Accent3,
    #[serde(rename = "MediumGrid2Accent3")]
    MediumGrid2Accent3,
    #[serde(rename = "MediumGrid3Accent3")]
    MediumGrid3Accent3,
    #[serde(rename = "DarkListAccent3")]
    DarkListAccent3,
    #[serde(rename = "ColorfulShadingAccent3")]
    ColorfulShadingAccent3,
    #[serde(rename = "ColorfulListAccent3")]
    ColorfulListAccent3,
    #[serde(rename = "ColorfulGridAccent3")]
    ColorfulGridAccent3,
    #[serde(rename = "LightShadingAccent4")]
    LightShadingAccent4,
    #[serde(rename = "LightListAccent4")]
    LightListAccent4,
    #[serde(rename = "LightGridAccent4")]
    LightGridAccent4,
    #[serde(rename = "MediumShading1Accent4")]
    MediumShading1Accent4,
    #[serde(rename = "MediumShading2Accent4")]
    MediumShading2Accent4,
    #[serde(rename = "MediumList1Accent4")]
    MediumList1Accent4,
    #[serde(rename = "MediumList2Accent4")]
    MediumList2Accent4,
    #[serde(rename = "MediumGrid1Accent4")]
    MediumGrid1Accent4,
    #[serde(rename = "MediumGrid2Accent4")]
    MediumGrid2Accent4,
    #[serde(rename = "MediumGrid3Accent4")]
    MediumGrid3Accent4,
    #[serde(rename = "DarkListAccent4")]
    DarkListAccent4,
    #[serde(rename = "ColorfulShadingAccent4")]
    ColorfulShadingAccent4,
    #[serde(rename = "ColorfulListAccent4")]
    ColorfulListAccent4,
    #[serde(rename = "ColorfulGridAccent4")]
    ColorfulGridAccent4,
    #[serde(rename = "LightShadingAccent5")]
    LightShadingAccent5,
    #[serde(rename = "LightListAccent5")]
    LightListAccent5,
    #[serde(rename = "LightGridAccent5")]
    LightGridAccent5,
    #[serde(rename = "MediumShading1Accent5")]
    MediumShading1Accent5,
    #[serde(rename = "MediumShading2Accent5")]
    MediumShading2Accent5,
    #[serde(rename = "MediumList1Accent5")]
    MediumList1Accent5,
    #[serde(rename = "MediumList2Accent5")]
    MediumList2Accent5,
    #[serde(rename = "MediumGrid1Accent5")]
    MediumGrid1Accent5,
    #[serde(rename = "MediumGrid2Accent5")]
    MediumGrid2Accent5,
    #[serde(rename = "MediumGrid3Accent5")]
    MediumGrid3Accent5,
    #[serde(rename = "DarkListAccent5")]
    DarkListAccent5,
    #[serde(rename = "ColorfulShadingAccent5")]
    ColorfulShadingAccent5,
    #[serde(rename = "ColorfulListAccent5")]
    ColorfulListAccent5,
    #[serde(rename = "ColorfulGridAccent5")]
    ColorfulGridAccent5,
    #[serde(rename = "LightShadingAccent6")]
    LightShadingAccent6,
    #[serde(rename = "LightListAccent6")]
    LightListAccent6,
    #[serde(rename = "LightGridAccent6")]
    LightGridAccent6,
    #[serde(rename = "MediumShading1Accent6")]
    MediumShading1Accent6,
    #[serde(rename = "MediumShading2Accent6")]
    MediumShading2Accent6,
    #[serde(rename = "MediumList1Accent6")]
    MediumList1Accent6,
    #[serde(rename = "MediumList2Accent6")]
    MediumList2Accent6,
    #[serde(rename = "MediumGrid1Accent6")]
    MediumGrid1Accent6,
    #[serde(rename = "MediumGrid2Accent6")]
    MediumGrid2Accent6,
    #[serde(rename = "MediumGrid3Accent6")]
    MediumGrid3Accent6,
    #[serde(rename = "DarkListAccent6")]
    DarkListAccent6,
    #[serde(rename = "ColorfulShadingAccent6")]
    ColorfulShadingAccent6,
    #[serde(rename = "ColorfulListAccent6")]
    ColorfulListAccent6,
    #[serde(rename = "ColorfulGridAccent6")]
    ColorfulGridAccent6,
    #[serde(rename = "SubtleEmphasis")]
    SubtleEmphasis,
    #[serde(rename = "IntenseEmphasis")]
    IntenseEmphasis,
    #[serde(rename = "SubtleReference")]
    SubtleReference,
    #[serde(rename = "IntenseReference")]
    IntenseReference,
    #[serde(rename = "BookTitle")]
    BookTitle,
    #[serde(rename = "Bibliography")]
    Bibliography,
    #[serde(rename = "TocHeading")]
    TocHeading,
    #[serde(rename = "PlainTable1")]
    PlainTable1,
    #[serde(rename = "PlainTable2")]
    PlainTable2,
    #[serde(rename = "PlainTable3")]
    PlainTable3,
    #[serde(rename = "PlainTable4")]
    PlainTable4,
    #[serde(rename = "PlainTable5")]
    PlainTable5,
    #[serde(rename = "TableGridLight")]
    TableGridLight,
    #[serde(rename = "GridTable1Light")]
    GridTable1Light,
    #[serde(rename = "GridTable2")]
    GridTable2,
    #[serde(rename = "GridTable3")]
    GridTable3,
    #[serde(rename = "GridTable4")]
    GridTable4,
    #[serde(rename = "GridTable5Dark")]
    GridTable5Dark,
    #[serde(rename = "GridTable6Colorful")]
    GridTable6Colorful,
    #[serde(rename = "GridTable7Colorful")]
    GridTable7Colorful,
    #[serde(rename = "GridTable1LightAccent1")]
    GridTable1LightAccent1,
    #[serde(rename = "GridTable2Accent1")]
    GridTable2Accent1,
    #[serde(rename = "GridTable3Accent1")]
    GridTable3Accent1,
    #[serde(rename = "GridTable4Accent1")]
    GridTable4Accent1,
    #[serde(rename = "GridTable5DarkAccent1")]
    GridTable5DarkAccent1,
    #[serde(rename = "GridTable6ColorfulAccent1")]
    GridTable6ColorfulAccent1,
    #[serde(rename = "GridTable7ColorfulAccent1")]
    GridTable7ColorfulAccent1,
    #[serde(rename = "GridTable1LightAccent2")]
    GridTable1LightAccent2,
    #[serde(rename = "GridTable2Accent2")]
    GridTable2Accent2,
    #[serde(rename = "GridTable3Accent2")]
    GridTable3Accent2,
    #[serde(rename = "GridTable4Accent2")]
    GridTable4Accent2,
    #[serde(rename = "GridTable5DarkAccent2")]
    GridTable5DarkAccent2,
    #[serde(rename = "GridTable6ColorfulAccent2")]
    GridTable6ColorfulAccent2,
    #[serde(rename = "GridTable7ColorfulAccent2")]
    GridTable7ColorfulAccent2,
    #[serde(rename = "GridTable1LightAccent3")]
    GridTable1LightAccent3,
    #[serde(rename = "GridTable2Accent3")]
    GridTable2Accent3,
    #[serde(rename = "GridTable3Accent3")]
    GridTable3Accent3,
    #[serde(rename = "GridTable4Accent3")]
    GridTable4Accent3,
    #[serde(rename = "GridTable5DarkAccent3")]
    GridTable5DarkAccent3,
    #[serde(rename = "GridTable6ColorfulAccent3")]
    GridTable6ColorfulAccent3,
    #[serde(rename = "GridTable7ColorfulAccent3")]
    GridTable7ColorfulAccent3,
    #[serde(rename = "GridTable1LightAccent4")]
    GridTable1LightAccent4,
    #[serde(rename = "GridTable2Accent4")]
    GridTable2Accent4,
    #[serde(rename = "GridTable3Accent4")]
    GridTable3Accent4,
    #[serde(rename = "GridTable4Accent4")]
    GridTable4Accent4,
    #[serde(rename = "GridTable5DarkAccent4")]
    GridTable5DarkAccent4,
    #[serde(rename = "GridTable6ColorfulAccent4")]
    GridTable6ColorfulAccent4,
    #[serde(rename = "GridTable7ColorfulAccent4")]
    GridTable7ColorfulAccent4,
    #[serde(rename = "GridTable1LightAccent5")]
    GridTable1LightAccent5,
    #[serde(rename = "GridTable2Accent5")]
    GridTable2Accent5,
    #[serde(rename = "GridTable3Accent5")]
    GridTable3Accent5,
    #[serde(rename = "GridTable4Accent5")]
    GridTable4Accent5,
    #[serde(rename = "GridTable5DarkAccent5")]
    GridTable5DarkAccent5,
    #[serde(rename = "GridTable6ColorfulAccent5")]
    GridTable6ColorfulAccent5,
    #[serde(rename = "GridTable7ColorfulAccent5")]
    GridTable7ColorfulAccent5,
    #[serde(rename = "GridTable1LightAccent6")]
    GridTable1LightAccent6,
    #[serde(rename = "GridTable2Accent6")]
    GridTable2Accent6,
    #[serde(rename = "GridTable3Accent6")]
    GridTable3Accent6,
    #[serde(rename = "GridTable4Accent6")]
    GridTable4Accent6,
    #[serde(rename = "GridTable5DarkAccent6")]
    GridTable5DarkAccent6,
    #[serde(rename = "GridTable6ColorfulAccent6")]
    GridTable6ColorfulAccent6,
    #[serde(rename = "GridTable7ColorfulAccent6")]
    GridTable7ColorfulAccent6,
    #[serde(rename = "ListTable1Light")]
    ListTable1Light,
    #[serde(rename = "ListTable2")]
    ListTable2,
    #[serde(rename = "ListTable3")]
    ListTable3,
    #[serde(rename = "ListTable4")]
    ListTable4,
    #[serde(rename = "ListTable5Dark")]
    ListTable5Dark,
    #[serde(rename = "ListTable6Colorful")]
    ListTable6Colorful,
    #[serde(rename = "ListTable7Colorful")]
    ListTable7Colorful,
    #[serde(rename = "ListTable1LightAccent1")]
    ListTable1LightAccent1,
    #[serde(rename = "ListTable2Accent1")]
    ListTable2Accent1,
    #[serde(rename = "ListTable3Accent1")]
    ListTable3Accent1,
    #[serde(rename = "ListTable4Accent1")]
    ListTable4Accent1,
    #[serde(rename = "ListTable5DarkAccent1")]
    ListTable5DarkAccent1,
    #[serde(rename = "ListTable6ColorfulAccent1")]
    ListTable6ColorfulAccent1,
    #[serde(rename = "ListTable7ColorfulAccent1")]
    ListTable7ColorfulAccent1,
    #[serde(rename = "ListTable1LightAccent2")]
    ListTable1LightAccent2,
    #[serde(rename = "ListTable2Accent2")]
    ListTable2Accent2,
    #[serde(rename = "ListTable3Accent2")]
    ListTable3Accent2,
    #[serde(rename = "ListTable4Accent2")]
    ListTable4Accent2,
    #[serde(rename = "ListTable5DarkAccent2")]
    ListTable5DarkAccent2,
    #[serde(rename = "ListTable6ColorfulAccent2")]
    ListTable6ColorfulAccent2,
    #[serde(rename = "ListTable7ColorfulAccent2")]
    ListTable7ColorfulAccent2,
    #[serde(rename = "ListTable1LightAccent3")]
    ListTable1LightAccent3,
    #[serde(rename = "ListTable2Accent3")]
    ListTable2Accent3,
    #[serde(rename = "ListTable3Accent3")]
    ListTable3Accent3,
    #[serde(rename = "ListTable4Accent3")]
    ListTable4Accent3,
    #[serde(rename = "ListTable5DarkAccent3")]
    ListTable5DarkAccent3,
    #[serde(rename = "ListTable6ColorfulAccent3")]
    ListTable6ColorfulAccent3,
    #[serde(rename = "ListTable7ColorfulAccent3")]
    ListTable7ColorfulAccent3,
    #[serde(rename = "ListTable1LightAccent4")]
    ListTable1LightAccent4,
    #[serde(rename = "ListTable2Accent4")]
    ListTable2Accent4,
    #[serde(rename = "ListTable3Accent4")]
    ListTable3Accent4,
    #[serde(rename = "ListTable4Accent4")]
    ListTable4Accent4,
    #[serde(rename = "ListTable5DarkAccent4")]
    ListTable5DarkAccent4,
    #[serde(rename = "ListTable6ColorfulAccent4")]
    ListTable6ColorfulAccent4,
    #[serde(rename = "ListTable7ColorfulAccent4")]
    ListTable7ColorfulAccent4,
    #[serde(rename = "ListTable1LightAccent5")]
    ListTable1LightAccent5,
    #[serde(rename = "ListTable2Accent5")]
    ListTable2Accent5,
    #[serde(rename = "ListTable3Accent5")]
    ListTable3Accent5,
    #[serde(rename = "ListTable4Accent5")]
    ListTable4Accent5,
    #[serde(rename = "ListTable5DarkAccent5")]
    ListTable5DarkAccent5,
    #[serde(rename = "ListTable6ColorfulAccent5")]
    ListTable6ColorfulAccent5,
    #[serde(rename = "ListTable7ColorfulAccent5")]
    ListTable7ColorfulAccent5,
    #[serde(rename = "ListTable1LightAccent6")]
    ListTable1LightAccent6,
    #[serde(rename = "ListTable2Accent6")]
    ListTable2Accent6,
    #[serde(rename = "ListTable3Accent6")]
    ListTable3Accent6,
    #[serde(rename = "ListTable4Accent6")]
    ListTable4Accent6,
    #[serde(rename = "ListTable5DarkAccent6")]
    ListTable5DarkAccent6,
    #[serde(rename = "ListTable6ColorfulAccent6")]
    ListTable6ColorfulAccent6,
    #[serde(rename = "ListTable7ColorfulAccent6")]
    ListTable7ColorfulAccent6,
    #[serde(rename = "SmartLink")]
    SmartLink,
    #[serde(rename = "Mention")]
    Mention,
    #[serde(rename = "SmartHyperlink")]
    SmartHyperlink,
    #[serde(rename = "Hashtag")]
    Hashtag,
    #[serde(rename = "UnresolvedMention")]
    UnresolvedMention,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Nil")]
    Nil,
}
