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

use std::collections::HashMap;
use std::sync::Arc;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use bytes::Bytes;
use futures_util::stream;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION};
use rsa::rand_core::OsRng;
use rsa::{BigUint, Pkcs1v15Encrypt, RsaPublicKey};
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{FileReference, Model};
use crate::request::{ApiRequestData, BodyPart, Request, RequestBody, ResponseData};
use crate::{Configuration, SdkError, SdkResult};

struct ClientState {
    auth_token: Mutex<Option<String>>,
    public_key: Mutex<Option<Arc<RsaPublicKey>>>,
}

/// Shared asynchronous HTTP client used by the generated API methods.
#[derive(Clone)]
pub struct ApiClient {
    http: reqwest::Client,
    configuration: Arc<Configuration>,
    state: Arc<ClientState>,
}

impl ApiClient {
    pub fn new(configuration: Configuration) -> SdkResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(configuration.timeout)
            .build()?;
        Ok(Self {
            http,
            configuration: Arc::new(configuration),
            state: Arc::new(ClientState {
                auth_token: Mutex::new(None),
                public_key: Mutex::new(None),
            }),
        })
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub(crate) fn build_url(
        &self,
        path: &str,
        query: &[(String, String)],
    ) -> SdkResult<String> {
        let root = self.configuration.api_root();
        let mut path = path.to_owned();
        while path.contains("//") {
            path = path.replace("//", "/");
        }
        let mut url = reqwest::Url::parse(&format!("{root}{path}")).map_err(|error| {
            SdkError::InvalidRequest(format!("failed to construct request URL: {error}"))
        })?;
        url.query_pairs_mut()
            .extend_pairs(query.iter().map(|(key, value)| (key, value)));
        Ok(url.to_string())
    }

    pub(crate) fn query_value<T: Serialize + ?Sized>(&self, value: &T) -> SdkResult<String> {
        match serde_json::to_value(value)? {
            Value::String(value) => Ok(value),
            Value::Bool(value) => Ok(value.to_string()),
            Value::Number(value) => Ok(value.to_string()),
            value => Ok(serde_json::to_string(&value)?),
        }
    }

    pub(crate) async fn add_model_part(
        &self,
        parts: &mut Vec<BodyPart>,
        name: impl Into<String>,
        model: &dyn Model,
    ) -> SdkResult<()> {
        model.validate()?;
        let mut files = Vec::new();
        model.collect_file_references(&mut files);
        for file in &files {
            file.prepare(self).await?;
        }
        parts.push(BodyPart {
            name: name.into(),
            filename: None,
            content_type: "application/json".to_owned(),
            data: serde_json::to_vec(model)?,
        });
        for file in files {
            if file.source() == "Request" {
                let content = file.content().ok_or_else(|| {
                    SdkError::InvalidRequest(format!(
                        "local file reference {} has no content",
                        file.reference()
                    ))
                })?;
                parts.push(BodyPart {
                    name: file.reference().to_owned(),
                    filename: None,
                    content_type: "application/octet-stream".to_owned(),
                    data: content.to_vec(),
                });
            }
        }
        Ok(())
    }

    pub(crate) fn add_json_part<T: Serialize + ?Sized>(
        &self,
        parts: &mut Vec<BodyPart>,
        name: impl Into<String>,
        value: &T,
    ) -> SdkResult<()> {
        parts.push(BodyPart {
            name: name.into(),
            filename: None,
            content_type: "application/json".to_owned(),
            data: serde_json::to_vec(value)?,
        });
        Ok(())
    }

    pub(crate) fn add_binary_part(
        &self,
        parts: &mut Vec<BodyPart>,
        name: impl Into<String>,
        value: &[u8],
    ) {
        parts.push(BodyPart {
            name: name.into(),
            filename: None,
            content_type: "application/octet-stream".to_owned(),
            data: value.to_vec(),
        });
    }

    pub(crate) fn add_text_part(
        &self,
        parts: &mut Vec<BodyPart>,
        name: impl Into<String>,
        value: &str,
    ) {
        parts.push(BodyPart {
            name: name.into(),
            filename: None,
            content_type: "text/plain".to_owned(),
            data: value.as_bytes().to_vec(),
        });
    }

    pub(crate) fn request_body_from_parts(
        &self,
        headers: &mut Vec<(String, String)>,
        mut parts: Vec<BodyPart>,
    ) -> RequestBody {
        if parts.len() == 1 {
            if let Some(part) = parts.pop() {
                // Preserve the canonical spelling for batch serialization: the
                // service distinguishes content headers before adding request headers.
                headers.push(("Content-Type".to_owned(), part.content_type));
                return RequestBody::Bytes(part.data);
            }
        }
        if parts.is_empty() {
            RequestBody::Empty
        } else {
            RequestBody::Multipart(parts)
        }
    }

    pub async fn execute(&self, request: &dyn Request) -> SdkResult<ResponseData> {
        let token = self.auth_token(false).await?;
        let data = request.build(self).await?;
        let response = self.send_once(data, &token).await?;
        if response.status == reqwest::StatusCode::UNAUTHORIZED {
            let token = self.auth_token(true).await?;
            let data = request.build(self).await?;
            return self.finish_response(self.send_once(data, &token).await?).await;
        }
        self.finish_response(response).await
    }

    async fn auth_token(&self, force_refresh: bool) -> SdkResult<String> {
        let mut token = self.state.auth_token.lock().await;
        if !force_refresh {
            if let Some(value) = token.as_ref() {
                return Ok(value.clone());
            }
        }

        let url = format!(
            "{}/v4.0/words/connect/token",
            self.configuration.base_url.trim_end_matches('/')
        );
        let response = self
            .http
            .post(url)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", self.configuration.client_id.as_str()),
                ("client_secret", self.configuration.client_secret.as_str()),
            ])
            .send()
            .await?;
        let status = response.status();
        let body = response.bytes().await?;
        if !status.is_success() {
            let message = if status == reqwest::StatusCode::BAD_REQUEST {
                "invalid server credentials".to_owned()
            } else {
                String::from_utf8_lossy(&body).into_owned()
            };
            return Err(SdkError::Http {
                status: status.as_u16(),
                message,
            });
        }

        let value: Value = serde_json::from_slice(&body)?;
        let token_type = value
            .get("token_type")
            .and_then(Value::as_str)
            .ok_or_else(|| SdkError::InvalidResponse("token_type is missing".to_owned()))?;
        let access_token = value
            .get("access_token")
            .and_then(Value::as_str)
            .ok_or_else(|| SdkError::InvalidResponse("access_token is missing".to_owned()))?;
        let value = format!("{token_type} {access_token}");
        *token = Some(value.clone());
        Ok(value)
    }

    pub async fn encrypt_password(&self, password: &str) -> SdkResult<String> {
        let key = self.public_key().await?;
        let encrypted = key
            .encrypt(&mut OsRng, Pkcs1v15Encrypt, password.as_bytes())
            .map_err(|error| SdkError::Crypto(error.to_string()))?;
        Ok(BASE64.encode(encrypted))
    }

    async fn public_key(&self) -> SdkResult<Arc<RsaPublicKey>> {
        let mut key = self.state.public_key.lock().await;
        if let Some(value) = key.as_ref() {
            return Ok(value.clone());
        }

        let configured = self
            .configuration
            .rsa_exponent
            .as_ref()
            .zip(self.configuration.rsa_modulus.as_ref())
            .map(|(exponent, modulus)| (exponent.clone(), modulus.clone()));
        let (exponent, modulus) = match configured {
            Some(value) => value,
            None => self.fetch_public_key().await?,
        };
        let exponent = BASE64
            .decode(exponent)
            .map_err(|error| SdkError::Crypto(error.to_string()))?;
        let modulus = BASE64
            .decode(modulus)
            .map_err(|error| SdkError::Crypto(error.to_string()))?;
        let value = Arc::new(
            RsaPublicKey::new(
                BigUint::from_bytes_be(&modulus),
                BigUint::from_bytes_be(&exponent),
            )
            .map_err(|error| SdkError::Crypto(error.to_string()))?,
        );
        *key = Some(value.clone());
        Ok(value)
    }

    async fn fetch_public_key(&self) -> SdkResult<(String, String)> {
        let url = self.build_url("/words/encryption/publickey", &[])?;
        let token = self.auth_token(false).await?;
        let mut response = self
            .http
            .get(&url)
            .header(AUTHORIZATION, &token)
            .send()
            .await?;
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            let token = self.auth_token(true).await?;
            response = self
                .http
                .get(url)
                .header(AUTHORIZATION, token)
                .send()
                .await?;
        }
        let status = response.status();
        let body = response.bytes().await?;
        if !status.is_success() {
            return Err(SdkError::Http {
                status: status.as_u16(),
                message: String::from_utf8_lossy(&body).into_owned(),
            });
        }
        let value: Value = serde_json::from_slice(&body)?;
        let exponent = value
            .get("Exponent")
            .and_then(Value::as_str)
            .ok_or_else(|| SdkError::InvalidResponse("RSA exponent is missing".to_owned()))?;
        let modulus = value
            .get("Modulus")
            .and_then(Value::as_str)
            .ok_or_else(|| SdkError::InvalidResponse("RSA modulus is missing".to_owned()))?;
        Ok((exponent.to_owned(), modulus.to_owned()))
    }

    async fn send_once(&self, data: ApiRequestData, token: &str) -> SdkResult<RawResponse> {
        let body_length = data.body.len();
        let mut request = self
            .http
            .request(data.method.clone(), &data.url)
            .header(AUTHORIZATION, token)
            .header("x-aspose-client", "rust sdk")
            .header(
                "x-aspose-client-version",
                "26.7",
            );
        for (name, value) in data.headers {
            let name = HeaderName::from_bytes(name.as_bytes()).map_err(|error| {
                SdkError::InvalidRequest(format!("invalid header name: {error}"))
            })?;
            let value = HeaderValue::from_str(&value).map_err(|error| {
                SdkError::InvalidRequest(format!("invalid header value: {error}"))
            })?;
            request = request.header(name, value);
        }
        request = match data.body {
            RequestBody::Empty => request,
            RequestBody::Bytes(body) => request.body(body),
            RequestBody::Multipart(parts) => {
                let mut form = reqwest::multipart::Form::new();
                for part in parts {
                    let mut value = reqwest::multipart::Part::bytes(part.data)
                        .mime_str(&part.content_type)?;
                    if let Some(filename) = part.filename {
                        value = value.file_name(filename);
                    }
                    form = form.part(part.name, value);
                }
                request.multipart(form)
            }
        };
        if let Some(callback) = data.send_progress {
            callback(body_length, body_length);
        }
        if self.configuration.debug_mode {
            eprintln!("CALL: {} {}", data.method, data.url);
        }
        let response = request.send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let expected_length = response.content_length().unwrap_or_default();
        let body = response.bytes().await?.to_vec();
        if let Some(callback) = data.receive_progress {
            callback(body.len() as u64, expected_length);
        }
        if self.configuration.debug_mode {
            eprintln!("RESPONSE: {} {}", status.as_u16(), data.url);
        }
        Ok(RawResponse {
            status,
            headers,
            body,
        })
    }

    async fn finish_response(&self, response: RawResponse) -> SdkResult<ResponseData> {
        if !response.status.is_success() {
            return Err(SdkError::Http {
                status: response.status.as_u16(),
                message: String::from_utf8_lossy(&response.body).into_owned(),
            });
        }
        let mut headers = HashMap::new();
        for (name, value) in &response.headers {
            let value = value.to_str().map_err(|error| {
                SdkError::InvalidResponse(format!("invalid response header: {error}"))
            })?;
            headers.insert(name.as_str().to_ascii_lowercase(), value.to_owned());
        }
        Ok(ResponseData {
            headers,
            body: response.body,
        })
    }
}

struct RawResponse {
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    body: Vec<u8>,
}

pub(crate) struct MultipartPart {
    pub name: Option<String>,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub headers: HashMap<String, String>,
    pub data: Vec<u8>,
}

pub(crate) async fn parse_multipart(
    content_type: &str,
    data: Vec<u8>,
) -> SdkResult<Vec<MultipartPart>> {
    let media_type = content_type.parse::<mime::Mime>().map_err(|error| {
        SdkError::InvalidResponse(format!("invalid multipart content type: {error}"))
    })?;
    if media_type.type_() != mime::MULTIPART {
        return Err(SdkError::InvalidResponse(
            "response content type is not multipart".to_owned(),
        ));
    }
    let boundary = media_type
        .get_param("boundary")
        .map(|value| value.as_str().to_owned())
        .ok_or_else(|| {
            SdkError::InvalidResponse("multipart boundary is missing".to_owned())
        })?;
    let source = stream::once(async move { Ok::<Bytes, std::io::Error>(Bytes::from(data)) });
    let mut multipart = multer::Multipart::new(source, boundary);
    let mut result = Vec::new();
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().map(str::to_owned);
        let filename = field.file_name().map(str::to_owned);
        let content_type = field.content_type().map(ToString::to_string);
        let mut headers = HashMap::new();
        for (header_name, header_value) in field.headers() {
            let value = header_value.to_str().map_err(|error| {
                SdkError::InvalidResponse(format!("invalid multipart header: {error}"))
            })?;
            headers.insert(
                header_name.as_str().to_ascii_lowercase(),
                value.to_owned(),
            );
        }
        let data = field.bytes().await?.to_vec();
        result.push(MultipartPart {
            name,
            filename,
            content_type,
            headers,
            data,
        });
    }
    Ok(result)
}

pub(crate) async fn parse_files_collection(
    content_type: Option<&str>,
    filename: Option<&str>,
    data: Vec<u8>,
) -> SdkResult<HashMap<String, Vec<u8>>> {
    let mut result = HashMap::new();
    if content_type.is_some_and(|value| value.starts_with("multipart/mixed")) {
        let content_type = content_type.ok_or_else(|| {
            SdkError::InvalidResponse("multipart content type is missing".to_owned())
        })?;
        for part in parse_multipart(content_type, data).await? {
            let key = part.filename.or(part.name).unwrap_or_default();
            result.insert(key, part.data);
        }
    } else {
        result.insert(filename.unwrap_or_default().to_owned(), data);
    }
    Ok(result)
}

pub(crate) fn encode_multipart(
    parts: Vec<BodyPart>,
    boundary: &str,
) -> SdkResult<Vec<u8>> {
    let mut body = Vec::new();
    for part in parts {
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        body.extend_from_slice(format!("Content-Type: {}\r\n", part.content_type).as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{}\"", part.name).as_bytes(),
        );
        if let Some(filename) = part.filename {
            body.extend_from_slice(format!("; filename=\"{filename}\"").as_bytes());
        }
        body.extend_from_slice(b"\r\n\r\n");
        body.extend_from_slice(&part.data);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    Ok(body)
}

pub(crate) fn prepare_batch_part(
    mut data: ApiRequestData,
    api_root: &str,
) -> SdkResult<Vec<u8>> {
    let prefix = format!("{api_root}/words/");
    let relative_url = data.url.strip_prefix(&prefix).ok_or_else(|| {
        SdkError::InvalidRequest("batch request URL is outside the Words API".to_owned())
    })?;
    let mut body = Vec::new();
    body.extend_from_slice(format!("{} {relative_url} \r\n", data.method).as_bytes());
    let request_body = match data.body {
        RequestBody::Empty => Vec::new(),
        RequestBody::Bytes(value) => value,
        RequestBody::Multipart(parts) => {
            let boundary = Uuid::new_v4().to_string();
            data.headers.push((
                "Content-Type".to_owned(),
                format!("multipart/form-data; boundary=\"{boundary}\""),
            ));
            encode_multipart(parts, &boundary)?
        }
    };
    for (name, value) in data.headers {
        body.extend_from_slice(format!("{name}: {value}\r\n").as_bytes());
    }
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(&request_body);
    Ok(body)
}

pub(crate) async fn parse_http_part(data: Vec<u8>) -> SdkResult<(u16, ResponseData)> {
    let separator = b"\r\n\r\n";
    let header_end = data
        .windows(separator.len())
        .position(|window| window == separator)
        .ok_or_else(|| SdkError::InvalidResponse("HTTP part has no header separator".to_owned()))?;
    let header_bytes = data.get(..header_end).ok_or_else(|| {
        SdkError::InvalidResponse("HTTP part header range is invalid".to_owned())
    })?;
    let header_text = String::from_utf8_lossy(header_bytes);
    let mut lines = header_text.lines();
    let status_line = lines
        .next()
        .ok_or_else(|| SdkError::InvalidResponse("HTTP part status is missing".to_owned()))?;
    let status_text = if status_line.starts_with("HTTP/") {
        status_line.split_whitespace().nth(1)
    } else {
        status_line.split_whitespace().next()
    }
    .ok_or_else(|| SdkError::InvalidResponse("HTTP part status is invalid".to_owned()))?;
    let status = status_text.parse::<u16>().map_err(|error| {
        SdkError::InvalidResponse(format!("HTTP part status is invalid: {error}"))
    })?;
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
        }
    }
    let body_start = header_end + separator.len();
    let body = data
        .get(body_start..)
        .ok_or_else(|| SdkError::InvalidResponse("HTTP part body range is invalid".to_owned()))?
        .to_vec();
    Ok((status, ResponseData { headers, body }))
}