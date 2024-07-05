# Trifonius engine project

### Environment variables

In order to use the default token fetcher (which is also used in the examples in the
`engine/src/bin` directory), some environment variables must be set:

<table>
    <tr align="top">
        <th align="left">variable</th>
        <th align="left">description</th>
        <th align="left">example</th>
    </tr>
    <tr align="top">
        <td align="top"><code>TRIFONIUS_TARGET_PLATFORM</code></td>
        <td>
            <ul>
                <li><code>nplz</code>Non production landing zone</li>
                <li><code>poc</code>Proof of concept platform</li>
                <li><code>prod</code>Production landing zone</li>
                <li><code>prodaz</code></li>
                <li><code>prodlz</code></li>
            </ul>
        </td>
        <td align="top"><code>nlpz</code></td>
    </tr>
    <tr align="top">
        <td><code>TRIFONIUS_TARGET_TENANT</code></td>
        <td>Tenant name for the target tenant. The target tenant is the tenant whose resources 
            will be managed by Trifonius.</td>
        <td><code>greenbox-dev</code></td>
    </tr>
    <tr align="top">
        <td><code>TRIFONIUS_TARGET_TENANT_SECRET</code></td>
        <td>Secret api token for the target tenant.</td>
        <td><code>...</code></td>
    </tr>
    <tr align="top">
        <td><code>TRIFONIUS_TARGET_TENANT_USER</code></td>
        <td>Group id and user id for the target tenant.</td>
        <td><code>1903:1903</code></td>
    </tr>
</table>

This allows the creation of the default token fetcher, as is shown in the following code fragment:

```rust
use trifonius_engine::resource::resource_descriptor::ResourceType;
use trifonius_engine::resource::resource_registry::ResourceRegistry;
use trifonius_engine::DEFAULT_TARGET_CLIENT_FACTOR;

#[tokio::main]
async fn main() {
    let resource_registry = ResourceRegistry::create(&DEFAULT_TARGET_CLIENT_FACTOR).unwrap();
    let topic_descriptors = resource_registry.resource_descriptors_by_type(ResourceType::Topic).unwrap();
    for topic_descriptor in topic_descriptors {
        println!("{}", topic_descriptor);
    }
}

```
