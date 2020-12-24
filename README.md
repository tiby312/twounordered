A crate that provides the user with two fast "vec-like" vecs that are backed by
a single vec. The caveat is that the operations like push and truncate
may rearrange the order of the other vec in an unspecified way.

