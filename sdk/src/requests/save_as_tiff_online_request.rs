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

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::models::*;
use crate::responses::*;
use crate::request::{
    ApiRequestData, DynamicResponse, ProgressCallback, Request, ResponseData, TypedRequest,
};
use crate::{ApiClient, SdkResult};
use super::*;

/// Request parameters for the SaveAsTiffOnline operation.
pub struct SaveAsTiffOnlineRequest {
    /// The document.
    pub document: Vec<u8>,
    /// Tiff save options.
    pub save_options: TiffSaveOptionsData,
    /// Encoding that will be used to load an HTML (or TXT) document if the encoding is not specified in HTML.
    pub load_encoding: Option<String>,
    /// Password of protected Word document. Use the parameter to pass a password via SDK. SDK encrypts it automatically. We don't recommend to use the parameter to pass a plain password for direct call of API.
    pub password: Option<String>,
    /// Password of protected Word document. Use the parameter to pass an encrypted password for direct calls of API. See SDK code for encyption details.
    pub encrypted_password: Option<String>,
    /// The value indicates whether OpenType support is on.
    pub open_type_support: Option<bool>,
    /// The flag indicating whether to use antialiasing.
    pub use_anti_aliasing: Option<bool>,
    /// The flag indicating whether to use high quality.
    pub use_high_quality_rendering: Option<bool>,
    /// The level of brightness for the generated images.
    pub image_brightness: Option<f64>,
    /// The color mode for the generated images.
    pub image_color_mode: Option<String>,
    /// The contrast for the generated images.
    pub image_contrast: Option<f64>,
    /// The images numeral format.
    pub numeral_format: Option<String>,
    /// The number of pages to render.
    pub page_count: Option<i32>,
    /// The index of the page to start rendering.
    pub page_index: Option<i32>,
    /// The background image color.
    pub paper_color: Option<String>,
    /// The pixel format of the generated images.
    pub pixel_format: Option<String>,
    /// The resolution of the generated images.
    pub resolution: Option<f64>,
    /// The zoom factor for the generated images.
    pub scale: Option<f64>,
    /// The compression tipe.
    pub tiff_compression: Option<String>,
    /// The optional dml rendering mode. The default value is Fallback.
    pub dml_rendering_mode: Option<String>,
    /// The optional dml effects rendering mode. The default value is Simplified.
    pub dml_effects_rendering_mode: Option<String>,
    /// The optional TIFF binarization method. Possible values are: FloydSteinbergDithering, Threshold.
    pub tiff_binarization_method: Option<String>,
    /// The flag indicating whether to ZIP the output.
    pub zip_output: Option<bool>,
    /// Folder in filestorage with custom fonts.
    pub fonts_location: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl SaveAsTiffOnlineRequest {
    pub fn new(document: Vec<u8>, save_options: TiffSaveOptionsData) -> Self {
        Self {
            document,
            save_options,
            load_encoding: None,
            password: None,
            encrypted_password: None,
            open_type_support: None,
            use_anti_aliasing: None,
            use_high_quality_rendering: None,
            image_brightness: None,
            image_color_mode: None,
            image_contrast: None,
            numeral_format: None,
            page_count: None,
            page_index: None,
            paper_color: None,
            pixel_format: None,
            resolution: None,
            scale: None,
            tiff_compression: None,
            dml_rendering_mode: None,
            dml_effects_rendering_mode: None,
            tiff_binarization_method: None,
            zip_output: None,
            fonts_location: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_load_encoding(mut self, value: String) -> Self {
        self.load_encoding = Some(value);
        self
    }

    pub fn with_password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    pub fn with_encrypted_password(mut self, value: String) -> Self {
        self.encrypted_password = Some(value);
        self
    }

    pub fn with_open_type_support(mut self, value: bool) -> Self {
        self.open_type_support = Some(value);
        self
    }

    pub fn with_use_anti_aliasing(mut self, value: bool) -> Self {
        self.use_anti_aliasing = Some(value);
        self
    }

    pub fn with_use_high_quality_rendering(mut self, value: bool) -> Self {
        self.use_high_quality_rendering = Some(value);
        self
    }

    pub fn with_image_brightness(mut self, value: f64) -> Self {
        self.image_brightness = Some(value);
        self
    }

    pub fn with_image_color_mode(mut self, value: String) -> Self {
        self.image_color_mode = Some(value);
        self
    }

    pub fn with_image_contrast(mut self, value: f64) -> Self {
        self.image_contrast = Some(value);
        self
    }

    pub fn with_numeral_format(mut self, value: String) -> Self {
        self.numeral_format = Some(value);
        self
    }

    pub fn with_page_count(mut self, value: i32) -> Self {
        self.page_count = Some(value);
        self
    }

    pub fn with_page_index(mut self, value: i32) -> Self {
        self.page_index = Some(value);
        self
    }

    pub fn with_paper_color(mut self, value: String) -> Self {
        self.paper_color = Some(value);
        self
    }

    pub fn with_pixel_format(mut self, value: String) -> Self {
        self.pixel_format = Some(value);
        self
    }

    pub fn with_resolution(mut self, value: f64) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn with_scale(mut self, value: f64) -> Self {
        self.scale = Some(value);
        self
    }

    pub fn with_tiff_compression(mut self, value: String) -> Self {
        self.tiff_compression = Some(value);
        self
    }

    pub fn with_dml_rendering_mode(mut self, value: String) -> Self {
        self.dml_rendering_mode = Some(value);
        self
    }

    pub fn with_dml_effects_rendering_mode(mut self, value: String) -> Self {
        self.dml_effects_rendering_mode = Some(value);
        self
    }

    pub fn with_tiff_binarization_method(mut self, value: String) -> Self {
        self.tiff_binarization_method = Some(value);
        self
    }

    pub fn with_zip_output(mut self, value: bool) -> Self {
        self.zip_output = Some(value);
        self
    }

    pub fn with_fonts_location(mut self, value: String) -> Self {
        self.fonts_location = Some(value);
        self
    }

    pub fn with_send_progress(mut self, callback: ProgressCallback) -> Self {
        self.send_progress = Some(callback);
        self
    }

    pub fn with_receive_progress(mut self, callback: ProgressCallback) -> Self {
        self.receive_progress = Some(callback);
        self
    }

    async fn parse_response_data(response: ResponseData) -> SdkResult<SaveAsTiffOnlineResponse> {
        SaveAsTiffOnlineResponse::from_response(response).await
    }

}

#[async_trait]
impl TypedRequest for SaveAsTiffOnlineRequest {
    type Response = SaveAsTiffOnlineResponse;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for SaveAsTiffOnlineRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/online/put/saveAs/tiff".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        if let Some(value) = &self.load_encoding {
        query.push(("loadEncoding".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.password {
        query.push(("encryptedPassword".to_owned(), client.encrypt_password(value).await?));
        }
        if let Some(value) = &self.encrypted_password {
        query.push(("encryptedPassword".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.open_type_support {
        query.push(("openTypeSupport".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.use_anti_aliasing {
        query.push(("useAntiAliasing".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.use_high_quality_rendering {
        query.push(("useHighQualityRendering".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.image_brightness {
        query.push(("imageBrightness".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.image_color_mode {
        query.push(("imageColorMode".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.image_contrast {
        query.push(("imageContrast".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.numeral_format {
        query.push(("numeralFormat".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.page_count {
        query.push(("pageCount".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.page_index {
        query.push(("pageIndex".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.paper_color {
        query.push(("paperColor".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.pixel_format {
        query.push(("pixelFormat".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.resolution {
        query.push(("resolution".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.scale {
        query.push(("scale".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.tiff_compression {
        query.push(("tiffCompression".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.dml_rendering_mode {
        query.push(("dmlRenderingMode".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.dml_effects_rendering_mode {
        query.push(("dmlEffectsRenderingMode".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.tiff_binarization_method {
        query.push(("tiffBinarizationMethod".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.zip_output {
        query.push(("zipOutput".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.fonts_location {
        query.push(("fontsLocation".to_owned(), client.query_value(value)?));
        }
        client.add_binary_part(&mut body_parts, "Document", &self.document);
        client.add_model_part(&mut body_parts, "SaveOptions", &self.save_options).await?;

        let url = client.build_url(&path, &query)?;
        let body = client.request_body_from_parts(&mut headers, body_parts);
        Ok(ApiRequestData {
            method: reqwest::Method::PUT,
            url,
            headers,
            body,
            send_progress: self.send_progress.clone(),
            receive_progress: self.receive_progress.clone(),
        })
    }

    async fn parse_any(&self, response: ResponseData) -> SdkResult<DynamicResponse> {
        Ok(Box::new(Self::parse_response_data(response).await?))
    }
}