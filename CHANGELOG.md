# Changelog

All notable changes to the DSH Api Client project will be documented in this file.

## [Unreleased]

## [0.7.2] - yyyy-mm-dd

### Added

* Platform methods:
    * `tenant_private_domain`,
    * `tenant_proxy_private_bootstrap_servers`,
    * `tenant_proxy_private_schema_registry_host`,
    * `tenant_proxy_public_bootstrap_servers`,
    * `tenant_proxy_public_schema_registry_host`,
    * `tenant_public_domain`.
* Bucket methods.

### Changed

* Platform method `tenant_public_apps_domain` is deprecated.

## [0.7.1] - 2025-04-18

### Added

* Module for managed tenants and limits.
* Module for managed streams.
* Implementation for `Display` trait for `LimitValue`, `ManagedStream`, `ManagedStreamId`,
  `ManagedTenant` and `PathSpec` types.

### Changed

* Improvements on embedded token fetcher.
* Improved error logging.

### Fixed

* Patch for bug in open api specification version `1.9.0`.
  Permanent resolution (dsh platform) pending.
* `dsh-api-build` upgraded to version `0.6.2`.

## [0.7.0] - 2025-04-17

### Changed

* Removed dependency on `dsh_sdk`, embedded the token fetcher code.

## [0.6.1] - 2025-04-07

### Added

* DshApiClient methods:
    * `get_app_catalog_manifest`,
    * `get_app_catalog_manifests`,
    * `get_raw_manifest method`.

### Changed

* Improved manifest data structures.
* Added private domain to `prod-aws-lz-dsh` platform parameters.

### Fixed

* Remove obsolete remark in generated comments.

## [0.6.0] - 2025-03-06

### Breaking changes

* Shorter selector and generic method names.
* Renamed some app catalog manifest methods.
* Renamed some application methods.
* Deleted feature `appcatalog`, which is now always enabled.

### Changed

* Method descriptors sorted alphabetically by selector.

### Removed

* Removed deprecated function `api_version`.

## [0.5.2] - 2025-02-28

### Changed

* Identifier lists returned by some wrapped functions are now sorted.

## [0.5.1] - 2025-02-20

### Fixed

* Fixed bug with incorrect internal domain for a platform.
* Fixed bug in generated doc comments.

## [0.5.0] - 2025-02-19

### Breaking changes

* All API methods are now generated from the openapi specification.
  Hence, many methods now have a different name.
* Removed static default client factory.
* Renamed `api_version()` function to `openapi_version()`.
* Removed group and user id from tenant data.
* Updated `dsh_sdk` dependency to 0.6.1.

### Added

* Improved error handling for bad requests.
* Features:
    * `appcatalog` - Controls availability of app catalog operations.
    * `manage` - Controls availability of manage operations.
    * `robot` - Controls availability of robot operation.

### Fixed

* Fixed bug with wrong realm for prod-aws-dsh platform.

### Removed

* Feature `actual` is removed. Its capabilities are now all enabled.

## [0.4.0]

### Breaking change

* Platform module now reads the platform definitions from an internal configuration file
  or from an explicit given configuration file.
  The old platform enum data structure is no longer available.

### Added

* Support DSH open API specification version 1.9.0.
* Generic API function.
* Platform swagger url method.

### Changed

* Implementation of methods that provide the API and the openapi versions.
* Embedded logo and favicon in generated docs.
* Changed platform enum to struct.
* Dedicated readme file for creates.io.

## [0.3.1]

### Added

* Readme file for dsh-api subproject.

## [0.3.0]

> Note: The changes from version 0.2.0 to 0.3.0 are **not** backwards compatible.

### Added

* Expose openapi specification.
* Added vhost capability.
* Added Display implementations.

### Changed

* Changed license to Apache-2.0.
* Improved platform capabilites.

## [0.2.0]

### Added

* Feature 'actual' enables/disables actual configurations.
* Query processor capability.
* Display implementations for selected types.

### Changed

* Some naming.
* Improved documentation.
* Changed type of guid to u16.
* Improved error handling in client factory.
* Better handling of usage relations.
* Embedded generated code in API crate.

### Removed

* Macros.

## [0.1.0] - 2024-10-29

### Added

* Functions for app catalog manifests.
* Functions for application tasks.
* Functions for certificates.
* Functions for kafka proxies.
* Functions for stream topics.
* Functions for volumes.

### Changed

* New naming schema in API.

### Fixes

* Support DSH openapi specification version 1.8.0.

### Removed

All code and dependencies for the Trifonius engine are moved to their own project.

## [0.0.6] - 2024-08-20

### Added

* Functions for buckets.
* Functions for topics.

### Fixes

* Consistent naming convention on the DSH API.
* Moved generation of API code to this crate, for better control and one less dependency.
* Better separation of concerns between Trifonius engine and DSH API.
