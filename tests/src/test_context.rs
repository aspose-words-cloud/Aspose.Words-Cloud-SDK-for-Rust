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

use std::path::{Path, PathBuf};

use aspose_words_cloud::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub type TestResult<T> = Result<T, TestError>;

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Sdk(#[from] SdkError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error("test assertion failed: {0}")]
    Assertion(String),
}

pub struct TestContext {
    api: WordsApi,
    examples_data: PathBuf,
    test_data: PathBuf,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ServerCredentials {
    client_id: String,
    client_secret: String,
    base_url: String,
}

pub fn create_random_guid() -> String {
    Uuid::new_v4().to_string()
}

pub async fn read_text_file(path: String) -> TestResult<String> {
    let sdk_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    Ok(tokio::fs::read_to_string(sdk_root.join("test_data").join(path)).await?)
}

impl TestContext {
    pub async fn from_settings() -> TestResult<Self> {
        let sdk_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let credentials_json =
            tokio::fs::read_to_string(sdk_root.join("settings/servercreds.json")).await?;
        let credentials: ServerCredentials = serde_json::from_str(&credentials_json)?;
        let configuration = Configuration::new(credentials.client_id, credentials.client_secret)
            .with_base_url(credentials.base_url);
        Ok(Self {
            api: WordsApi::new(configuration)?,
            examples_data: sdk_root.join("examples_data"),
            test_data: sdk_root.join("test_data"),
        })
    }

    pub fn api(&self) -> &WordsApi {
        &self.api
    }

    pub fn remote_base_test_data_folder(&self) -> &str {
        "Temp/SdkTests/Rust/TestData"
    }

    pub fn base_test_out_path(&self) -> &str {
        "TestOut/Rust"
    }

    pub fn examples_data_dir(&self) -> &Path {
        &self.examples_data
    }

    pub fn create_random_guid(&self) -> String {
        create_random_guid()
    }

    pub async fn load_binary_file(&self, path: impl AsRef<Path>) -> TestResult<Vec<u8>> {
        Ok(tokio::fs::read(self.test_data.join(path)).await?)
    }

    pub async fn load_text_file(&self, path: impl AsRef<Path>) -> TestResult<String> {
        Ok(tokio::fs::read_to_string(self.test_data.join(path)).await?)
    }

    pub async fn upload_file(&self, local_path: String, remote_path: String) -> TestResult<()> {
        let content = self.load_binary_file(local_path).await?;
        let request = UploadFileRequest::new(content, remote_path);
        let result = self.api.upload_file(request).await?;
        ensure(
            result.r#errors.as_ref().is_none_or(Vec::is_empty),
            "upload response contains errors",
        )?;
        ensure(
            result.r#uploaded.as_ref().is_some_and(|files| files.len() == 1),
            "upload response does not contain exactly one file",
        )
    }
}

pub fn serialize_result<T: Serialize>(value: &T) -> TestResult<Value> {
    Ok(serde_json::to_value(value)?)
}

pub fn ensure(condition: bool, message: impl Into<String>) -> TestResult<()> {
    if condition {
        Ok(())
    } else {
        Err(TestError::Assertion(message.into()))
    }
}

pub fn assert_not_null(root: &Value, path: &str) -> TestResult<()> {
    ensure(
        json_path(root, path).is_some_and(|value| !value.is_null()),
        format!("{path} is null or missing"),
    )
}

pub fn assert_null(root: &Value, path: &str) -> TestResult<()> {
    ensure(
        json_path(root, path).is_none_or(Value::is_null),
        format!("{path} is not null"),
    )
}

pub fn assert_bool(root: &Value, path: &str, expected: bool) -> TestResult<()> {
    ensure(
        json_path(root, path).and_then(Value::as_bool) == Some(expected),
        format!("{path} is not {expected}"),
    )
}

pub fn assert_length(root: &Value, path: &str, expected: usize) -> TestResult<()> {
    let actual = json_path(root, path).and_then(|value| match value {
        Value::Array(value) => Some(value.len()),
        Value::Object(value) => Some(value.len()),
        Value::String(value) => Some(value.chars().count()),
        _ => None,
    });
    ensure(
        actual == Some(expected),
        format!("{path} length is {actual:?}, expected {expected}"),
    )
}

pub fn assert_string(
    root: &Value,
    path: &str,
    expected: impl AsRef<str>,
) -> TestResult<()> {
    let expected = expected.as_ref();
    ensure(
        json_path(root, path).and_then(Value::as_str) == Some(expected),
        format!("{path} does not equal {expected:?}"),
    )
}

pub fn assert_integer(root: &Value, path: &str, expected: i64) -> TestResult<()> {
    ensure(
        json_path(root, path).and_then(Value::as_i64) == Some(expected),
        format!("{path} does not equal {expected}"),
    )
}

pub fn assert_float(root: &Value, path: &str, expected: f64) -> TestResult<()> {
    let actual = json_path(root, path).and_then(Value::as_f64);
    ensure(
        actual.is_some_and(|actual| (actual - expected).abs() < f64::EPSILON),
        format!("{path} does not equal {expected}"),
    )
}

fn json_path<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = root;
    for segment in path.split('.') {
        let (name, index) = match segment.split_once('[') {
            Some((name, remainder)) => {
                let index = remainder.strip_suffix(']')?.parse::<usize>().ok()?;
                (name, Some(index))
            }
            None => (segment, None),
        };
        if !name.is_empty() {
            current = current.get(name)?;
        }
        if let Some(index) = index {
            current = current.get(index)?;
        }
    }
    Some(current)
}