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
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::{ApiClient, SdkResult};

pub type ProgressCallback = Arc<dyn Fn(u64, u64) + Send + Sync>;
pub type DynamicResponse = Box<dyn Any + Send + Sync>;

#[doc(hidden)]
pub struct ApiRequestData {
    pub(crate) method: reqwest::Method,
    pub(crate) url: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: RequestBody,
    pub(crate) send_progress: Option<ProgressCallback>,
    pub(crate) receive_progress: Option<ProgressCallback>,
}

#[doc(hidden)]
pub struct ResponseData {
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug)]
pub(crate) struct BodyPart {
    pub name: String,
    pub filename: Option<String>,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub(crate) enum RequestBody {
    Empty,
    Bytes(Vec<u8>),
    Multipart(Vec<BodyPart>),
}

impl RequestBody {
    pub(crate) fn len(&self) -> u64 {
        match self {
            Self::Empty => 0,
            Self::Bytes(value) => value.len() as u64,
            Self::Multipart(parts) => parts.iter().map(|part| part.data.len() as u64).sum(),
        }
    }
}

#[async_trait]
pub trait Request: Send + Sync {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData>;

    async fn parse_any(&self, response: ResponseData) -> SdkResult<DynamicResponse>;
}

#[async_trait]
pub trait TypedRequest: Request {
    type Response: Send + Sync + 'static;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response>;
}