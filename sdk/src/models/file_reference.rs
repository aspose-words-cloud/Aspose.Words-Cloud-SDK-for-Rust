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

use std::sync::OnceLock;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::{ApiClient, SdkError, SdkResult};

/// A local request file or a remote file in Aspose storage.
#[derive(Debug)]
pub struct FileReference {
    source: &'static str,
    reference: String,
    content: Option<Vec<u8>>,
    password: Option<String>,
    encrypted_password: OnceLock<String>,
}

impl FileReference {
    pub fn remote(path: impl Into<String>, password: Option<String>) -> Self {
        Self {
            source: "Storage",
            reference: path.into(),
            content: None,
            password,
            encrypted_password: OnceLock::new(),
        }
    }

    pub fn local(content: Vec<u8>, password: Option<String>) -> Self {
        Self {
            source: "Request",
            reference: Uuid::new_v4().to_string(),
            content: Some(content),
            password,
            encrypted_password: OnceLock::new(),
        }
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn content(&self) -> Option<&[u8]> {
        self.content.as_deref()
    }

    pub(crate) async fn prepare(&self, client: &ApiClient) -> SdkResult<()> {
        if self.encrypted_password.get().is_some() {
            return Ok(());
        }

        if let Some(password) = &self.password {
            let encrypted = client.encrypt_password(password).await?;
            let _already_initialized = self.encrypted_password.set(encrypted);
        }
        Ok(())
    }
}

impl Serialize for FileReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FileReference", 3)?;
        state.serialize_field("Source", self.source)?;
        state.serialize_field("Reference", &self.reference)?;
        if let Some(password) = self.encrypted_password.get() {
            state.serialize_field("EncryptedPassword", password)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for FileReference {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(serde::de::Error::custom(
            "FileReference cannot be deserialized from an API response",
        ))
    }
}

impl super::Model for FileReference {
    fn validate(&self) -> SdkResult<()> {
        if self.reference.is_empty() {
            return Err(SdkError::InvalidRequest(
                "file reference must not be empty".to_owned(),
            ));
        }
        Ok(())
    }

    fn collect_file_references<'a>(&'a self, output: &mut Vec<&'a FileReference>) {
        output.push(self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}