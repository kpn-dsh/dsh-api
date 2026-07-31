# Known issues

### Incorrect limit values for managed tenants

The openapi specification/schema for `LimitValue` is designed in such a way that the variant
of the enum can only be determined from the value of a field inside the enum. This makes it
impossible for `Typify` to generate a proper deserializable enum from it. Deserializing
any `LimitValue` enum will therefor always result in a `LimitValue::Cpu` variant, which again
results in incorrect output from the following generated functions:

* `get_tenant_limit(tenant, kind)`,
* `get_tenant_limits(tenant)`.

It is not likely that this will be changed in a future version of the openapi specification.
Instead of these generated methods you can use the derived methods below, which do a proper
conversion:

* `managed_tenant_limit(tenant, kind)`,
* `managed_tenant_limits(tenant)`.
