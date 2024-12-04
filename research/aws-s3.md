# AWS S3 capabilities

PutObject and CompleteMultipartUpload support ETAG based atomic CAS, useful for ensuring correctness if two processes end up operating at once.

Multipart uploads do not time out, and can be queried.
