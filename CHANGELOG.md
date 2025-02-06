# Changelog

All notable changes to the DSH Api Client project will be documented in this file.

## [Unreleased]

## [0.5.0] - yyy-mm-dd

### Breaking changes

* All API methods are now generated from the openapi specification.
  Hence, many methods now have a different name.
* Renamed `api_version()` function to `openapi_version()`.

### Added

* Improved error handling for bad requests.
* Features:
    * `appcatalog` - Controls availability of app catalog operations.
    * `manage` - Controls availability of manage operations.
    * `robot` - Controls availability of robot operation.

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
