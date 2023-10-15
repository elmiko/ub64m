# UnBase64Manifest - ub64m

A tool to help me with kubernetes manifest wrangling. There is probably a
simpler/more elegant way to do this using standard cli tools, but where's
the fun in that? 😂

This tool will take a [JSON][json] or [YAML][yaml] document and decode
all base64 encoded strings in the values for any field. It will then
print the output as a new [YAML][yaml] file.

## Example uses

```
$ cat tests/data/encoded-sample.yaml
---
Name: Cloud Configuration
Version: 1
Private: true
Account:
  User: Qm9iCg==
  Zones:
    - ZWFzdC0xLWEK
    - ZWFzdC0xLWIK
    - c291dGgtMi1hCg==
  ProjectName: Ym9icy1iaWctcHJvamVjdAo=
  Token: c3VwZXItc2VjcmV0Cg==
$ ./target/debug/ub64m tests/data/encoded-sample.yaml
---
Name: Cloud Configuration
Version: 1
Private: true
Account:
  User: Bob
  Zones:
    - east-1-a
    - east-1-b
    - south-2-a
  ProjectName: bobs-big-project
  Token: super-secret
```

```
$ kubectl create -f tests/data/kubernetes-configmap.yaml
$ kubectl get cm cloud-config -o yaml
apiVersion: v1
data:
  projectName: Ym9icy1iaWctcHJvamVjdAo=
  token: c3VwZXItc2VjcmV0Cg==
  user: Qm9iCg==
kind: ConfigMap
metadata:
  creationTimestamp: "2023-10-15T23:50:15Z"
  name: cloud-config
  namespace: default
  resourceVersion: "1776"
  uid: 7aeee832-4421-472e-a27d-83b4dece0c7f
[mike@gamebox] devel ~/dev/ub64m
$ kubectl get cm cloud-config -o yaml | ./target/debug/ub64m -
---
apiVersion: v1
data:
  projectName: bobs-big-project
  token: super-secret
  user: Bob
kind: ConfigMap
metadata:
  creationTimestamp: "2023-10-15T23:50:15Z"
  name: cloud-config
  namespace: default
  resourceVersion: "1776"
  uid: 7aeee832-4421-472e-a27d-83b4dece0c7f
```

## Install using cargo

From `$HOME` in your terminal type the following:

```
cargo install ub64m
```

This will install the `ub64m` binary to the `.cargo/bin` directory, please
note that this will need to be in your `$PATH` for easier use.

[json]: https://json.org
[yaml]: https://yaml.org
