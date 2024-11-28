# async-lsm - an LSM library for Rust

All the existing Rust LSM's appear to be synchronous. This leads to a function colouring mismatch when using cloud blob
storage, as (most) all client SDKs for such systems are async.

Currently, this is an experiment.

## Goals

- Library for opening LSM storage on top of an Async storage abstraction
- Included storage implementations for local disk and common blob storage APIs
- Single process access to the storage - fencing and other distributed system concerns should be handled by the
  deployment layer of an application built on top of this library.
- Write-through caching
- Read caching
- Some sensible limits on value and key sizes
- Reads don't block writes
- Mutations don't block reads
- Mutations are serialised, but multiple mutations can be in flight at once