# UnBase64Manifest - ub64m

A tool to help me with kubernetes manifest wrangling. There is probably a
simpler/more elegant way to do this using standard cli tools, but where's
the fun in that? 😂

This tool will take a [JSON][json] or [YAML][yaml] document and decode
all base64 encoded strings in the values for any field. It will then
print the output as a new [YAML][yaml] file.

## Example use

```shell
$ cat tests/data/encoded-sample.yaml
---
Encoded: SGVsbG8gV29ybGQhCg==
SomethingElse:
  Name: Foo Bar!
  DoTheThing: true
  Value: 9000
$ ./target/debug/ub64m ./tests/data/encoded-sample.yaml 
---
Encoded: Hello World!
SomethingElse:
  Name: Foo Bar!
  DoTheThing: true
  Value: 9000
```

[json]: https://json.org
[yaml]: https://yaml.org
