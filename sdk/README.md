# Aspose.Words Cloud SDK for Rust

An asynchronous, strongly typed Rust client for the Aspose.Words Cloud API.

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

The client uses Tokio and Reqwest for asynchronous I/O, Serde for typed JSON,
Multer for multipart parsing, and Thiserror for recoverable errors. API model
inheritance is represented by composition. Dynamic dispatch is limited to
schema fields that are explicitly polymorphic.