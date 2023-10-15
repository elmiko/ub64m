# UnBase64Manifest - ub64m

A tool to help me with kubernetes manifest wrangling. There is probably a
simpler/more elegant way to do this using standard cli tools, but where's
the fun in that? 😂

This tool will take a [JSON][json] or [YAML][yaml] document and decode
all base64 encoded strings in the values for any field. It will then
print the output as a new [YAML][yaml] file.

## Example use

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

## Install using cargo

From `$HOME` in your terminal type the following:

```
cargo install ub64m
```

This will install the `ub64m` binary to the `.cargo/bin` directory, please
note that this will need to be in your `$PATH` for easier use.

[json]: https://json.org
[yaml]: https://yaml.org
