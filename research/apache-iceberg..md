# Apache iceberg

This is an LSM but optimised for slow change rates and schema'd data. The LSM manifest is not stored in the table, but
outside and the current design does not have a blob-storage backed 'Catalog' - requiring an additional stateful service
to run in the first place.

So - interesting, but doesn't quite fit the space I'm looking at.

OTOH some clouds like AWS offer native Iceberg support. Of course they also offer serverless SQL databases, so the use
case of 'give me just enough database without a server' is still met.
