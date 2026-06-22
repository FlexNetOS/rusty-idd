# Provider Matrix

| Provider / Tool | Intended role | May store secrets? | Notes |
|---|---|---:|---|
| GitHub Actions secrets | CI secret injection | yes | Prefer environment-scoped secrets for deployment |
| GitHub Actions variables | Non-secret CI config | no | Do not store credentials here |
| OIDC | Short-lived cloud auth | no long-lived secret | Preferred over static cloud keys where supported |
| SOPS | Encrypted secrets in Git | encrypted only | Requires key-management decision |
| Infisical | Central secret management | yes | Use explicit project/env mapping |
| OpenBao/Vault | Central secret management | yes | Use policies and short TTL credentials |
| direnv/mise | Local developer env loading | no by default | Keep local-only values out of Git |
| Docker/Compose env_file | Local/runtime config | maybe | Treat `.env` as local-only unless encrypted |

Decision rule: do not add a provider until the env/secrets contract lists the key namespace, source of truth, rotation model, and CI/local resolution path.
