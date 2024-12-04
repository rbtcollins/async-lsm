# GCS capabilities

GCS supports both resumable/streaming uploads and an AWS-like multipart upload
system.

Larger objects can be composed from a list of existing objects - this is the
native multipart-equivalent for GCS. The composition sources are unaltered.
