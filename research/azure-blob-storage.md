# Azure blob capabilities

Page blobs permit random write access (used for VHDs). Conditional via If-Match and others.

Append blobs permit appending up to 4MiB. Conditional via x-ms-blob-condition-appendpos=old-size

Block blobs support multi-part upload - pub block *N + put block list to finalise. Uncommitted blocks discard after one
week or a commit that doesn't include the block.

Blobs support leases, which can be used to provide fencing.
