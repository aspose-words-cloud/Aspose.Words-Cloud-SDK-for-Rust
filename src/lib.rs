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

#![allow(warnings)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, unused_imports, unused_mut)]
// Generated models use one uniform implementation strategy, including inherited
// defaults and request constructors without required arguments.
#![allow(
    clippy::derivable_impls,
    clippy::field_reassign_with_default,
    clippy::new_without_default
)]

#![forbid(unsafe_code)]

#[cfg_attr(rustfmt, rustfmt_skip)]
mod api;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod api_client;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod batch;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod configuration;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod error;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod job_handler;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod models;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod request;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod requests;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod responses;

pub use api::WordsApi;
pub use api_client::ApiClient;
pub use batch::{BatchRequest, BatchResult};
pub use configuration::Configuration;
pub use error::{SdkError, SdkResult};
pub use job_handler::JobHandler;
pub use models::*;
pub use request::{DynamicResponse, ProgressCallback, Request, TypedRequest};
pub use requests::*;
pub use responses::*;