# Aspose.Words Cloud SDK for Rust

This generated workspace contains two crates:

- `sdk` is the publishable `aspose-words-cloud` library.
- `tests` contains integration tests and is excluded from publication.

The SDK is asynchronous and uses Tokio, Reqwest, Serde, Multer, and Thiserror.
All operations return `Result`; generated code does not panic on API, transport,
serialization, multipart, cryptographic, or file-system failures.

## Usage

```rust
use aspose_words_cloud::{Configuration, GetDocumentRequest, SdkResult, WordsApi};

#[tokio::main]
async fn main() -> SdkResult<()> {
    let configuration = Configuration::new("client-id", "client-secret");
    let api = WordsApi::new(configuration)?;
    let request = GetDocumentRequest::new("document.docx".to_owned());
    let document = api.get_document(request).await?;
    tokio::fs::write("document.docx", document).await?;
    Ok(())
}
```

Credentials should be supplied by the application and must not be committed to
source control. The generated examples read `ASPOSE_CLIENT_ID`,
`ASPOSE_CLIENT_SECRET`, and optionally `ASPOSE_BASE_URL` from the environment.

## Tests

The integration suite mirrors the Dart SDK scenarios. Put the credentials in
`settings/servercreds.json`; test documents are read from `test_data` and
example documents from `examples_data`. Then run from the workspace root:

```text
cargo test --workspace
```