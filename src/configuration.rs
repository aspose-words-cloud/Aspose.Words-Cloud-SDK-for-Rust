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

use std::time::Duration;

/// Configuration for the Aspose.Words Cloud API client.
#[derive(Clone)]
pub struct Configuration {
    pub client_id: String,
    pub client_secret: String,
    pub base_url: String,
    pub timeout: Duration,
    pub debug_mode: bool,
    pub rsa_exponent: Option<String>,
    pub rsa_modulus: Option<String>,
}

impl Configuration {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            base_url: "https://api.aspose.cloud".to_owned(),
            timeout: Duration::from_secs(300),
            debug_mode: false,
            rsa_exponent: None,
            rsa_modulus: None,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_owned();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_debug_mode(mut self, debug_mode: bool) -> Self {
        self.debug_mode = debug_mode;
        self
    }

    pub fn with_rsa_public_key(
        mut self,
        exponent: impl Into<String>,
        modulus: impl Into<String>,
    ) -> Self {
        self.rsa_exponent = Some(exponent.into());
        self.rsa_modulus = Some(modulus.into());
        self
    }

    pub(crate) fn api_root(&self) -> String {
        format!("{}/v4.0", self.base_url.trim_end_matches('/'))
    }
}
