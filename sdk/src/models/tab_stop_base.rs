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

/// Base class for paragraph format tab stop DTO.
#[derive(Debug, Deserialize, Serialize)]
pub struct TabStopBase {
        /// Gets or sets the alignment of text at this tab stop.
        #[serde(rename = "Alignment", skip_serializing_if = "Option::is_none")]
        pub alignment: Option<TabStopBaseAlignmentEnum>,


        /// Gets or sets the type of the leader line displayed under the tab character.
        #[serde(rename = "Leader", skip_serializing_if = "Option::is_none")]
        pub leader: Option<TabStopBaseLeaderEnum>,


        /// Gets or sets the position of the tab stop in points.
        #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
        pub position: Option<f64>,

}

impl Default for TabStopBase {
    fn default() -> Self {
        Self {
            alignment: None,
            leader: None,
            position: None,
        }
    }
}

impl Model for TabStopBase {
    fn validate(&self) -> SdkResult<()> {
        if self.alignment.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Alignment in TabStopBase is required".to_owned(),
            ));
        }
        if self.leader.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Leader in TabStopBase is required".to_owned(),
            ));
        }
        if self.position.is_none() {
            return Err(SdkError::InvalidRequest(
                "property Position in TabStopBase is required".to_owned(),
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

/// Gets or sets the alignment of text at this tab stop.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TabStopBaseAlignmentEnum {
    #[serde(rename = "Left")]
        Left,
    #[serde(rename = "Center")]
        Center,
    #[serde(rename = "Right")]
        Right,
    #[serde(rename = "Decimal")]
        Decimal,
    #[serde(rename = "Bar")]
        Bar,
    #[serde(rename = "List")]
        List,
    #[serde(rename = "Clear")]
        Clear,
}

/// Gets or sets the type of the leader line displayed under the tab character.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TabStopBaseLeaderEnum {
    #[serde(rename = "None")]
        None,
    #[serde(rename = "Dots")]
        Dots,
    #[serde(rename = "Dashes")]
        Dashes,
    #[serde(rename = "Line")]
        Line,
    #[serde(rename = "Heavy")]
        Heavy,
    #[serde(rename = "MiddleDot")]
        MiddleDot,
}