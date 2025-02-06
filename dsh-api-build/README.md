# dsh_api build helpers

This lib crate contains functions that are needed during the build phase of the `dsh_api` crate.

* Generate the generic client code
* Generate Progenitor client from an openapi specification
* Update the openapi specification

It has no real value in any other situation.
It is published to `crates.io` because the current capabilities of the rust `build.rs` system
are too limited for complex build strategies.
