# dsh_api build helpers

This lib crate contains functions that are needed during the build phase of the `dsh_api` crate.

* Generate the client code
* Generate the generic client code
* Generate type structs from openapi specification

It has no real value in any other situation.
It is published to `crates.io` because the current capabilities of the rust `build.rs` system
are too limited for complex build strategies.
