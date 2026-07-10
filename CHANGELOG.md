# Changelog

All notable changes to the DSH Api Client project will be documented in this file.

## [Unreleased]

## [0.10.1] - YYYY-MM-DD

### Added

### Changed

* Openapi specification version 1.13.0.

### Fixed

* Unused import warning (`FromStr`) in generated generic code.

## [0.10.0] - 2026-05-29

### Added

* Added `application::application_tasks` method.
* Added `secret` constants for system secret names.
* Added `secret::secret_with_status` method.
* Added `platform` methods:
    * `DshPlatform::consumer_group`.
    * `DshPlatform::from_domain`,
    * `DshPlatform::robot_client_id`,
    * `DshPlatform::robot_tenant_client_id`.
    * Custom serializer and deserializer.
* Added `platform::VhostZone` enum.
* Added `Display` and `new`implementations for `KafkaAclGroupTopic`.
* Added `proxy` diagram.

### Changed

* Platform proxy methods improved.
* Changed `platform` methods for generating `proxy` urls:
    * `DshPlatform::proxy_consumer_group_acl`
    * `DshPlatform::proxy_consumer_group`
    * `DshPlatform::proxy_schema_store_vhost`
    * `DshPlatform::proxy_vhost`
    * `DshPlatform::tenant_proxy_bootstrap_server`
    * `DshPlatform::tenant_proxy_bootstrap_servers`
    * `DshPlatform::tenant_proxy_private_bootstrap_servers`

### Deprecated

* Deprecated `platform` methods:
    * `DshPlatform::client_id`.
    * `DshPlatform::tenant_client_id`.
    * `DshPlatform::tenant_private_domain`.
    * `DshPlatform::tenant_proxy_private_bootstrap_servers`.
    * `DshPlatform::tenant_proxy_private_schema_store_host`.
    * `DshPlatform::tenant_proxy_public_bootstrap_servers`.
    * `DshPlatform::tenant_proxy_public_schema_store_host`.
    * `DshPlatform::tenant_public_domain`.

### Removed

* Removed deprecated method `DshPlatform::tenant_public_apps_domain`.

### Fixed

* Resolved bug with secrets dependants.
* Resolved bug with topic dependants.

## [0.9.0] - 2026-03-05

### Added

* Platform naming methods for proxy.
* Platform naming methods for messaging api.
* Module `nodepool`.
* Builder pattern to create type instances.
* Added `debug` and `trace` logging to the generated code.
* Added methods `secret_names`, `secret_names_non_system` and `secret_names_system`.
* Added function `secret_name`.
* Added `Trifonius` as a separate dependency.
* Added `secrets_with_dependant_certificates` method.
* Added `render_message` method to `Notification`.

### Changed

* New open api specification version `1.11.1`.
* Removed dependency on `Progenitor`.
* Secret methods and functions now use secret names instead of secret ids.
* Improved error handling and logging.
* Renamed `issuer_endpoint` field in platforms configuration to  `issuer-endpoint`
  (`issuer_endpoint` is still allowed but deprecated).
* Removed `lazy_static` dependency.
* Function `secret_id_to_secret_name` is deprecated.
* Changed `msrv` to `1.80.0`.
* Create and open api versions are returned as a `Version` reference.
* Set dependencies to latest versions.
* Resolved some known issues.
* Lots of refactoring.

### Fixed

* Fixed bug in the description of return type in method descriptors.

## [0.8.1] - 2026-01-15

### Added

* Module for proxies.
* Additional methods for certificates and proxies.
* Additional methods for secrets and proxies.
* Method to get secret dependants.
* `impl FromStr for DshPlatform`.
* `StringQueryProcessor`.
* Some derived trait implementations for some types.
* Platform client factory.

### Changed

* Changed serializer for `Version`.

## [0.8.0] - 2025-11-20

### Added

* Added `k8s-dev-aws-lz-dsh` (development platform for Klarrio) to default platform list.
* Implemented `Default` trait for selected types.
* Implemented `new()` function for selected types.
* Parsers for selected string formats.
* Added methods and functions for application, bucket, manifest, platform, stream and tenant.
* Added capability to use static access tokens instead of robot password and token fetcher.
* Added DSH Json Web Token struct.

### Changed

* New open api specification version `1.10.0` (skipped version `1.9.2`).
* Removed `prod-aws-lz-laas` from default platform list.
* Redundant patches removed.
* Platform method `tenant_public_apps_domain` deprecated.
* Struct `AccessToken` removed.

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
