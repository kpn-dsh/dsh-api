# DSH resource management API client

### Environment variables

<table>
    <tr valign="top">
        <th align="left">variable</th>
        <th align="left">description</th>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_PLATFORM</code></td>
        <td>
            Target platform on which the tenant's environment lives.
            <ul>
                <li><code>nplz</code>Non production landing zone</li>
                <li><code>poc</code>Proof of concept platform</li>
                <li><code>prod</code>Production landing zone</li>
                <li><code>prodaz</code></li>
                <li><code>prodlz</code></li>
            </ul>
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_TENANT</code></td>
        <td>Tenant id for the target tenant. The target tenant is the tenant whose resources 
            will be managed via the api.</td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_SECRET_[platform]_[tenant]</code></td>
        <td>
            Secret api token for the target tenant. 
            The placeholders <code>[platform]</code> and <code>[tenant]</code> 
            need to be substituted with the platform name and the tenant name in all capitals, 
            with hyphens (<code>-</code>) replaced by underscores (<code>_</code>).<br/>
            E.g. if the platform is <code>nplz</code> and the tenant name is 
            <code>greenbox-dev</code>, the environment variable must be
            <code>DSH_API_SECRET_NPLZ_GREENBOX_DEV</code>.
        </td>
    </tr>
    <tr valign="top">
        <td><code>DSH_API_GUID_[tenant]</code></td>
        <td>
            Group id and user id for the target tenant.
            The placeholder <code>[tenant]</code> needs to be substituted 
            with the tenant name in all capitals, with hyphens (<code>-</code>) 
            replaced by underscores (<code>_</code>).<br/>
            E.g. if the tenant name is <code>greenbox-dev</code>, the environment variable must be
            <code>DSH_API_GUID_GREENBOX_DEV</code>.
        </td>
    </tr>
</table>

E.g., for tenant `greenbox-dev` (gid/uid `1903`) at platform `nplz`, use:

```bash
> export DSH_API_PLATFORM=nplz && \
  export DSH_API_TENANT=greenbox-dev && \
  export DSH_API_GUID_GREENBOX_DEV=1903:1903 && \
  export DSH_API_SECRET_NPLZ_GREENBOX_DEV=..
```

## Features

The following features are defined:

<table>
    <tr valign="top">
        <th align="left">feature</th>
        <th align="left">description</th>
    </tr>
    <tr valign="top">
        <td><code>actual</code></td>
        <td>
            When this feature is enabled the library will include all the "actual" 
            method versions of the REST API. By default, these methods will not be included.
        </td>
    </tr>
    <tr valign="top">
        <td><code>generated</code></td>
        <td>
            When this feature is enabled the library will re-export the functions 
            generated from the REST API. By default, these methods will not be re-exported.
            Note that the generated functions can also be added explicitly 
            to the dependencies section in <code>Cargo.toml</code>.
        </td>
    </tr>
</table>

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
