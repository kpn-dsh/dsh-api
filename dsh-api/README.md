# DSH resource management API client

This crate contains functions and definitions that provide support for using the functions
of the DSH resource management API.

## Example

A small example will demonstrate the use of the library.
More details and more elaborate examples can be found in the project's repository on
[github](https://github.com/kpn-dsh/dsh-api).

To make the library available to your rust application add it to your dependencies:

```toml
[dependencies]
dsh_api = "0.7.2" 
```

The example will print a list of all the applications that are deployed
in a tenant environment. This example requires that the tenant's name,
platform and API secret are configured via environment variables as follows:.

```bash
> export DSH_API_PLATFORM=np-aws-lz-dsh
> export DSH_API_TENANT=my-tenant
> export DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT=...
````

Then the following program will list all applications for this tenant on the given platform.

```rust
use dsh_api::dsh_api_client_factory::DshApiClientFactory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = DshApiClientFactory::default().client().await?;
    for (application_id, application) in client.list_applications().await? {
        println!("{} -> {}", application_id, application);
    }
    Ok(())
}
```

## Features

By enabling/disabling the features described below you have control over what's included
in your library and what's not. All features are disabled by default.
The following features are defined:

* `generic` - Enables the generic methods.
* `manage` - Enables the manage methods.
* `robot` - Enables the robot operation.

---

## Changelog

See [CHANGELOG.md](../CHANGELOG.md) for all changes per version.

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for more information on how to contribute to this project.

## License

See [LICENSE](../LICENSE.md) for more information on the license for this project.

## Security

See [SECURITY.md](../SECURITY.md) for more information on the security policy for this project.

---
_Copyright (c) Koninklijke KPN N.V._
