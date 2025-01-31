# DSH resource management API client

This crate contains functions and definitions that provide support for using the functions
of the DSH resource management API.

## Examples

Three examples will demonstrate the use of the library.
To make the library available to your rust application add it to your dependencies:

```toml
[dependencies]
dsh_api = "0.4.0" 
```

### Minimal example

The first minimal example will print a list of all the applications that are deployed
in a tenant environment. This example requires that the tenant's name, group and user id,
platform and API secret are configured via environment variables as follows:.

```bash
> export DSH_API_PLATFORM=np-aws-lz-dsh
> export DSH_API_TENANT=my-tenant
> export DSH_API_GUID_MY_TENANT=1234
> export DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT=...
````

See the paragraph on environment variables for more details.
Then the following program will list all applications for this tenant on the given platform.

```rust
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
    for (application_id, application) in client.list_applications().await? {
        println!("{} -> {}", application_id, application);
    }
    Ok(())
}
```

### More elaborate example

In the next, slightly more elaborate example, the tenant parameters are given explicitly.
This example will list all the applications in the tenant environment that have been
configured to require a token in order to access the Kafka broker.
This is accomplished via the `find_applications()`
methods, that returns a list of all applications for which the provided predicate
evaluates to `true`.

```rust
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::dsh_api_tenant::DshApiTenant;
use dsh_api::platform::DshPlatform;
use dsh_api::types::Application;
use dsh_api::DshApiError;

#[tokio::main]
async fn main() -> Result<(), DshApiError> {
    let tenant = DshApiTenant::new(
        "my-tenant".to_string(),
        1234,
        DshPlatform::try_from("np-aws-lz-dsh")?
    );
    let secret = "...".to_string();

    let client_factory = DshApiClientFactory::create(tenant, secret)?;
    let client = client_factory.client().await?;
    let predicate = |application: &Application| application.needs_token;
    let applications = client.find_applications(&predicate).await?;
    for (application_id, application) in applications {
        println!("{} -> {}", application_id, application);
    }
    Ok(())
}
```

### Generic api example

The generic methods in the library make all rest api methods (`delete`, `get`, `head`, `patch`,
`post` and `put`) available for all paths in the OpenApi specification,
but the generic nature has some limitations due to the lack of abstract datatypes in rust.
The main limitation is that static type checking is not possible.
All parameters, data and results must be plain text or json.
The parameters and input data will be checked,
e.g. the provided json messages must be deserializable to the expected types defined in
the `dsh_api::types` module, but this will only be checked at run-time, not at compile-time.

The generic methods requires the `generic` feature to be enabled:

```toml
[dependencies]
dsh_api = { version = "0.4.0", features = ["generic"] }
```

The example below will add a new secret to the tenant's secret store.
To do this, first create a json formatted string as defined for the body in the OpenApi file
for the `POST /allocation/{tenant}/secret` path:

```json
{
  "name": "secret-name",
  "value": "secret-value"
}
```

where `secret-name` and `secret-value` will be the name and value of the new secret.

Now the `post` method can be called with the following parameters:

* `selector: &str` - this is either an identifier for the path of the requested method in the rest
  api
  (`secret`) or the explicit path (`/allocation/{tenant}/secret`).
* `parameters: &[&str]` - a vector with the expected path parameters for the api method,
  but __without__ the first path parameter (`{tenant}` or sometimes `{manager}`).
  The first path parameters is handled in a special way,
  since it identifies the tenant that is making the api call and is used for
  authorization/authentication. For the example below there are no path parameters,
  so the vector must be empty.
* `body: Option<String>` - a string that must deserialize to the expected data type
  (`dsh_api::types::Secret`).
  If the json string does not deserialize to a valid `dsh_api::types::Secret`
  the `post` method will return an `DshApiResult::Err`.

The example expects the same environment variables from the "Minimal example" to be set.

```rust
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api::DshApiResult;

#[tokio::main]
async fn main() -> DshApiResult<()> {
    let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
    let secret_json = r#"{"name": "secret-name","value": "secret-value"}"#.to_string();
    client.post("secret", &[], Some(secret_json)).await
}
```

## Environment variables

Most library functions need at least the following parameters to run:

* platform - the platform that the resources reside on,
* tenant - this is the tenant that is making the function calls,
  needed for authentication and authorization,
* group and user id - needed for some special occasions,
* the rest api password for the tenant on the platform.

These parameters can be provided explicitly when creating an `DshApiClientFactory` object
(see "More elaborate example"), or they can be provided via environment variables.
In the latter case, the library functions use their default implementations and the library
gets the default value from the environment variables decribed below.

<table>
    <tr valign="top">
        <th align="left">variable</th>
        <th align="left">description</th>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_PLATFORM</code></td>
        <td>
            Target platform on which the tenant's environment lives. 
            The default list of platforms is:
            <ul>
                <li>
                    <code>np-aws-lz-dsh / nplz</code> 
                    - Staging platform for KPN internal tenants.
                </li>
                <li>
                    <code>poc-aws-dsh / poc</code> 
                    - Staging platform for non KPN tenants.
                </li>
                <li>
                    <code>prod-aws-dsh / prod</code> 
                    - Production platform for non KPN tenants.
                </li>
                <li>
                    <code>prod-aws-lz-dsh / prodlz</code> 
                    - Production platform for KPN internal tenants.
                </li>
                <li>
                    <code>prod-aws-lz-laas / prodls</code> 
                    - Production platform for logstash as a service.
                </li>
                <li>
                    <code>prod-azure-dsh / prodaz</code> 
                    - Production platform for non KPN tenants.
                </li>
            </ul>
            Note that this default list can be overridden by setting the environment variable 
            <code>DSH_API_PLATFORMS_FILE</code>.
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_TENANT</code></td>
        <td>
            Tenant id for the tenant that is making the api requests (the client tenant). 
            In some cases this is not the same tenant as the tenant whose resources 
            will be managed via the api. The latter will be called the target tenant.
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_PASSWORD_[platform]_[tenant]</code></td>
        <td>
            Secret api token for the client tenant. 
            For better security, the use of <code>DSH_API_PASSWORD_FILE_[platform]_[tenant]</code>
            is preferred over this variable.<br/>
            The placeholders <code>[platform]</code> and <code>[tenant]</code> 
            need to be substituted with the platform name and the tenant name in all capitals, 
            with hyphens (<code>-</code>) replaced by underscores (<code>_</code>).
            E.g. if the platform is <code>np-aws-lz-dsh</code> and the tenant name is 
            <code>my-tenant</code>, the environment variable must be
            <code>DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT</code>.<br/>
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_PASSWORD_FILE_[platform]_[tenant]</code></td>
        <td>
            This environment variable specifies a file containing the secret api 
            token/password for the client tenant.<br/>
            The placeholders <code>[platform]</code> and <code>[tenant]</code> 
            need to be substituted with the platform name and the tenant name in all capitals, 
            with hyphens (<code>-</code>) replaced by underscores (<code>_</code>).
            E.g. if the platform is <code>np-aws-lz-dsh</code> and the tenant name is 
            <code>my-tenant</code>, the environment variable must be
            <code>DSH_API_PASSWORD_FILE_NP_AWS_LZ_DSH_MY_TENANT</code>.
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_GUID_[tenant]</code></td>
        <td>
            Group id and user id for the client tenant.<br/>
            The placeholder <code>[tenant]</code> needs to be substituted 
            with the tenant name in all capitals, with hyphens (<code>-</code>) 
            replaced by underscores (<code>_</code>).
            E.g. if the tenant name is <code>my-tenant</code>, the environment variable must be
            <code>DSH_API_GUID_MY_TENANT</code>.
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_PLATFORMS_FILE</code></td>
        <td>
            Set this environment variable to override the default list of available platforms.
            The value of the environment variable must be the name 
            of the alternative platforms file. It can either be an absolute file name, 
            or a relative file name from the working directory of your application. 
            When this environment variable is set, the normal list of default platforms 
            will <em>not</em> be included. If you need these too, make sure that you also 
            include the default platforms in your platforms file.
            The default platforms file can be found 
            <a href="dsh-api/default-platforms.json">here</a>.
            <pre>
[
  {
    "name": "np-aws-lz-dsh",
    "description": "Staging platform for KPN internal tenants",
    "alias": "nplz",
    "is-production": false,
    "cloud-provider": "aws",
    "key-cloak-url": "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth",
    "realm": "dev-lz-dsh",
    "public-domain": "dsh-dev.dsh.np.aws.kpn.com",
    "private-domain": "dsh-dev.dsh.np.aws.kpn.org"
  },
  ...
]
            </pre>
            All these values are mandatory for each defined platform, 
            except <code>private-domain</code>. 
            When a private domain is not specified for a platform, 
            do not include the attribute in the json object.
        </td>
</tr>
</table>

E.g., for tenant `my-tenant` (gid/uid `1234`) at platform `np-aws-lz-dsh`, use:

```bash
> export DSH_API_PLATFORM=np-aws-lz-dsh && \
  export DSH_API_TENANT=my-tenant && \
  export DSH_API_GUID_MY_TENANT=1234 && \
  export DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT=..
```

## Features

The following features are defined:

* `actual` - When this feature is enabled the library will include all the "actual"
  method versions of the REST API. By default, these methods will not be included.
* `generic` - When this feature is enabled the library will also include the generic methods.
  This feature is disabled by default.

## Coding guidelines

Before pushing code to github, make sure that you adhere to the code formatting defined in
`rustfmt.toml` and that you have run the `clippy` linter. The following commands should
return without any remarks:

```bash
> cargo +nightly fmt --check
```

```bash
> cargo clippy
```

Consider configuring your IDE to automatically apply the formatting rules when saving a file. 
