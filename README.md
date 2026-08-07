# Aspose.Words Cloud SDK for Rust

This package contains Aspose.Words Cloud SDK for Rust.
This SDK allows you to work with Aspose.Words Cloud REST APIs in your Rust applications quickly and easily, with zero initial cost.

[Aspose.Words Cloud](https://products.aspose.cloud/words/family "Aspose.Words Cloud")
[API Reference](https://apireference.aspose.cloud/words/)

## Key Features

- Conversion between various document-related formats (20+ formats supported), including PDF<->Word conversion
- Mail merge and reports generation
- Splitting Word documents
- Accessing Word document metadata and statistics
- Find and replace
- Watermarks and protection
- Full read and write access to the Document Object Model, including sections, paragraphs, text, images, tables, headers, footers, and many others

## How to use the SDK?

The complete source code is available in this repository. You can either use it directly in your project or add the published crate as a dependency (recommended). For more details, visit the [SDK documentation](https://docs.aspose.cloud/display/wordscloud/Available+SDKs).

### Prerequisites

To use Aspose.Words Cloud SDK for Rust, register an account with [Aspose Cloud](https://www.aspose.cloud/) and create an application in the [Cloud Dashboard](https://dashboard.aspose.cloud/#/apps) to obtain a Client ID and Client Secret. A free quota is available. For more details, see [Aspose Cloud Pricing](https://purchase.aspose.cloud/cloud/pricing/).

Rust 1.88 or newer is required.

## Installation & Usage

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
aspose-words-cloud = "26.8.0"
tokio = { version = "1", features = ["fs", "macros", "rt-multi-thread"] }
```

## Getting Started

```rust
use aspose_words_cloud::*;

#[tokio::main]
async fn main() -> SdkResult<()> {
    let configuration = Configuration::new("ClientId", "ClientSecret");
    let words_api = WordsApi::new(configuration)?;

    // Upload a document to cloud storage.
    let file_content = tokio::fs::read("./test_data/Common/test_doc.docx").await?;
    let upload_request =
        UploadFileRequest::new(file_content, "fileStoredInCloud.docx".to_owned());
    words_api.upload_file(upload_request).await?;

    // Save the document as PDF in cloud storage.
    let mut save_options = PdfSaveOptionsData::default();
    save_options.r#file_name = Some("destStoredInCloud.pdf".to_owned());
    let save_request = SaveAsRequest::new(
        "fileStoredInCloud.docx".to_owned(),
        save_options.into(),
    );
    words_api.save_as(save_request).await?;
    Ok(())
}
```

[Tests](tests) contain additional examples of using the SDK. Integration tests read credentials from `settings/servercreds.json`, test documents from `test_data`, and example documents from `examples_data`.

Run the test suite from the repository root:

```text
cargo test --manifest-path tests/Cargo.toml
```

## Dependencies

- Referenced crates are listed in [Cargo.toml](Cargo.toml).

## Licensing

All Aspose.Words Cloud SDKs, helper scripts, and templates are licensed under the [MIT License](LICENSE).

## Contact Us

Your feedback is very important to us. Feel free to contact us using our [Support Forums](https://forum.aspose.cloud/c/words).

## Resources

[Website](https://www.aspose.cloud/)
[Product Home](https://products.aspose.cloud/words/family)
[API Reference](https://apireference.aspose.cloud/words/)
[Documentation](https://docs.aspose.cloud/display/wordscloud/Home)
[Blog](https://blog.aspose.cloud/category/words/)

## Other languages

We generate our SDKs in different languages, so check whether yours is available in our [SDK list](https://github.com/aspose-words-cloud).

If you do not find your language in the list, request it from us or use the raw REST API as described in the [cURL documentation](https://products.aspose.cloud/words/curl).