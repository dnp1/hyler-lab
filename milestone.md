### V1

- CORS
- Download/Upload Streaming
- Easy Serialize/Deserialize Request/Response Bodies (native suppor form JSON, TOML, etc) -- use serde
   * Parsing according to Content-type
   * Returning according to Accept or Content-type
- Pretty validation of request body (each field is invalid and why)
- Good performance
- Non-block
- As Static as possible