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

/// Request parameters for the SaveAsTiff operation.
pub struct SaveAsTiffRequest {
    /// The filename of the input document.
    pub r#name: String,
    /// Tiff save options.
    pub r#save_options: TiffSaveOptionsData,
    /// Original document folder.
    pub r#folder: Option<String>,
    /// Original document storage.
    pub r#storage: Option<String>,
    /// Encoding that will be used to load an HTML (or TXT) document if the encoding is not specified in HTML.
    pub r#load_encoding: Option<String>,
    /// Password of protected Word document. Use the parameter to pass a password via SDK. SDK encrypts it automatically. We don't recommend to use the parameter to pass a plain password for direct call of API.
    pub r#password: Option<String>,
    /// Password of protected Word document. Use the parameter to pass an encrypted password for direct calls of API. See SDK code for encyption details.
    pub r#encrypted_password: Option<String>,
    /// The value indicates whether OpenType support is on.
    pub r#open_type_support: Option<bool>,
    /// The flag indicating whether to use antialiasing.
    pub r#use_anti_aliasing: Option<bool>,
    /// The flag indicating whether to use high quality.
    pub r#use_high_quality_rendering: Option<bool>,
    /// The level of brightness for the generated images.
    pub r#image_brightness: Option<f64>,
    /// The color mode for the generated images.
    pub r#image_color_mode: Option<String>,
    /// The contrast for the generated images.
    pub r#image_contrast: Option<f64>,
    /// The images numeral format.
    pub r#numeral_format: Option<String>,
    /// The number of pages to render.
    pub r#page_count: Option<i32>,
    /// The index of the page to start rendering.
    pub r#page_index: Option<i32>,
    /// The background image color.
    pub r#paper_color: Option<String>,
    /// The pixel format of the generated images.
    pub r#pixel_format: Option<String>,
    /// The resolution of the generated images.
    pub r#resolution: Option<f64>,
    /// The zoom factor for the generated images.
    pub r#scale: Option<f64>,
    /// The compression tipe.
    pub r#tiff_compression: Option<String>,
    /// The optional dml rendering mode. The default value is Fallback.
    pub r#dml_rendering_mode: Option<String>,
    /// The optional dml effects rendering mode. The default value is Simplified.
    pub r#dml_effects_rendering_mode: Option<String>,
    /// The optional TIFF binarization method. Possible values are: FloydSteinbergDithering, Threshold.
    pub r#tiff_binarization_method: Option<String>,
    /// The flag indicating whether to ZIP the output.
    pub r#zip_output: Option<bool>,
    /// Folder in filestorage with custom fonts.
    pub r#fonts_location: Option<String>,
    pub send_progress: Option<ProgressCallback>,
    pub receive_progress: Option<ProgressCallback>,
}

impl SaveAsTiffRequest {
    pub fn new(r#name: String, r#save_options: TiffSaveOptionsData) -> Self {
        Self {
            r#name,
            r#save_options,
            r#folder: None,
            r#storage: None,
            r#load_encoding: None,
            r#password: None,
            r#encrypted_password: None,
            r#open_type_support: None,
            r#use_anti_aliasing: None,
            r#use_high_quality_rendering: None,
            r#image_brightness: None,
            r#image_color_mode: None,
            r#image_contrast: None,
            r#numeral_format: None,
            r#page_count: None,
            r#page_index: None,
            r#paper_color: None,
            r#pixel_format: None,
            r#resolution: None,
            r#scale: None,
            r#tiff_compression: None,
            r#dml_rendering_mode: None,
            r#dml_effects_rendering_mode: None,
            r#tiff_binarization_method: None,
            r#zip_output: None,
            r#fonts_location: None,
            send_progress: None,
            receive_progress: None,
        }
    }

    pub fn with_folder(mut self, value: String) -> Self {
        self.r#folder = Some(value);
        self
    }

    pub fn with_storage(mut self, value: String) -> Self {
        self.r#storage = Some(value);
        self
    }

    pub fn with_load_encoding(mut self, value: String) -> Self {
        self.r#load_encoding = Some(value);
        self
    }

    pub fn with_password(mut self, value: String) -> Self {
        self.r#password = Some(value);
        self
    }

    pub fn with_encrypted_password(mut self, value: String) -> Self {
        self.r#encrypted_password = Some(value);
        self
    }

    pub fn with_open_type_support(mut self, value: bool) -> Self {
        self.r#open_type_support = Some(value);
        self
    }

    pub fn with_use_anti_aliasing(mut self, value: bool) -> Self {
        self.r#use_anti_aliasing = Some(value);
        self
    }

    pub fn with_use_high_quality_rendering(mut self, value: bool) -> Self {
        self.r#use_high_quality_rendering = Some(value);
        self
    }

    pub fn with_image_brightness(mut self, value: f64) -> Self {
        self.r#image_brightness = Some(value);
        self
    }

    pub fn with_image_color_mode(mut self, value: String) -> Self {
        self.r#image_color_mode = Some(value);
        self
    }

    pub fn with_image_contrast(mut self, value: f64) -> Self {
        self.r#image_contrast = Some(value);
        self
    }

    pub fn with_numeral_format(mut self, value: String) -> Self {
        self.r#numeral_format = Some(value);
        self
    }

    pub fn with_page_count(mut self, value: i32) -> Self {
        self.r#page_count = Some(value);
        self
    }

    pub fn with_page_index(mut self, value: i32) -> Self {
        self.r#page_index = Some(value);
        self
    }

    pub fn with_paper_color(mut self, value: String) -> Self {
        self.r#paper_color = Some(value);
        self
    }

    pub fn with_pixel_format(mut self, value: String) -> Self {
        self.r#pixel_format = Some(value);
        self
    }

    pub fn with_resolution(mut self, value: f64) -> Self {
        self.r#resolution = Some(value);
        self
    }

    pub fn with_scale(mut self, value: f64) -> Self {
        self.r#scale = Some(value);
        self
    }

    pub fn with_tiff_compression(mut self, value: String) -> Self {
        self.r#tiff_compression = Some(value);
        self
    }

    pub fn with_dml_rendering_mode(mut self, value: String) -> Self {
        self.r#dml_rendering_mode = Some(value);
        self
    }

    pub fn with_dml_effects_rendering_mode(mut self, value: String) -> Self {
        self.r#dml_effects_rendering_mode = Some(value);
        self
    }

    pub fn with_tiff_binarization_method(mut self, value: String) -> Self {
        self.r#tiff_binarization_method = Some(value);
        self
    }

    pub fn with_zip_output(mut self, value: bool) -> Self {
        self.r#zip_output = Some(value);
        self
    }

    pub fn with_fonts_location(mut self, value: String) -> Self {
        self.r#fonts_location = Some(value);
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

    async fn parse_response_data(response: ResponseData) -> SdkResult<SaveResponse> {
        Ok(serde_json::from_slice::<SaveResponse>(&response.body)?)
    }

}

#[async_trait]
impl TypedRequest for SaveAsTiffRequest {
    type Response = SaveResponse;

    async fn parse_response(&self, response: ResponseData) -> SdkResult<Self::Response> {
        Self::parse_response_data(response).await
    }
}

#[async_trait]
impl Request for SaveAsTiffRequest {
    async fn build(&self, client: &ApiClient) -> SdkResult<ApiRequestData> {
        let mut path = "/words/{name}/saveAs/tiff".to_owned();
        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body_parts = Vec::new();

        let value = client.query_value(&self.r#name)?;
        path = path.replace("{name}", &value);
        if let Some(value) = &self.r#folder {
        query.push(("folder".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#storage {
        query.push(("storage".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#load_encoding {
        query.push(("loadEncoding".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#password {
        query.push(("encryptedPassword".to_owned(), client.encrypt_password(value).await?));
        }
        if let Some(value) = &self.r#encrypted_password {
        query.push(("encryptedPassword".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#open_type_support {
        query.push(("openTypeSupport".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#use_anti_aliasing {
        query.push(("useAntiAliasing".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#use_high_quality_rendering {
        query.push(("useHighQualityRendering".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#image_brightness {
        query.push(("imageBrightness".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#image_color_mode {
        query.push(("imageColorMode".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#image_contrast {
        query.push(("imageContrast".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#numeral_format {
        query.push(("numeralFormat".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#page_count {
        query.push(("pageCount".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#page_index {
        query.push(("pageIndex".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#paper_color {
        query.push(("paperColor".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#pixel_format {
        query.push(("pixelFormat".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#resolution {
        query.push(("resolution".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#scale {
        query.push(("scale".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#tiff_compression {
        query.push(("tiffCompression".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#dml_rendering_mode {
        query.push(("dmlRenderingMode".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#dml_effects_rendering_mode {
        query.push(("dmlEffectsRenderingMode".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#tiff_binarization_method {
        query.push(("tiffBinarizationMethod".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#zip_output {
        query.push(("zipOutput".to_owned(), client.query_value(value)?));
        }
        if let Some(value) = &self.r#fonts_location {
        query.push(("fontsLocation".to_owned(), client.query_value(value)?));
        }
        client.add_model_part(&mut body_parts, "Body", &self.r#save_options).await?;

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