# y2hcl

a CLI Tool to convert Yaml values file (e.g. Helm Chart Values) to the following format:
- Helm cli set values, e.g. `--set some.var="somevalue"`
- Terraform `helm_release` provider `set_value`
- Terraform `helm_release` provider `sensitive_value`
- HCL variable of type map, e.g. `some_map = {"some.var" = "somevalue"}`


## Installation

You can install this using the cargo install command:

```sh
$ cargo install y2hcl
```

## Usage

```sh
$ y2hcl --help

Convert Helm Chart Values yaml to Terraform helm_release set_value, Helm Cli --set values etc.

Usage: y2hcl --filename <FILENAME> --output-format <OUTPUT_FORMAT>

Options:
  -f, --filename <FILENAME>            Filename or full path to YAML formated Helm Chart values
  -o, --output-format <OUTPUT_FORMAT>  Output format is how we want our Helm Chart values to look like. Supported format are `hcl_map`, `set_value`, `sensitive_value`, `helm_cli`
  -h, --help                           Print help
  -V, --version                        Print version

```


## Examples

For example, a file `values.yaml` contains the following:

```yaml
livenessProbe:
  httpGet:
    path: /user/login
    port: http
  initialDelaySeconds: 120
```


1. YAML values to `HCL Map`

```sh
$ y2hcl -f values.yaml -o hcl_map

hcl_map = {
  "livenessProbe.httpGet.path" = "/user/login"
  "livenessProbe.httpGet.port" = "http"
  "livenessProbe.initialDelaySeconds" = "120"
}
```

2. YAML values to `helm_release` `set_value`

```sh
$ y2hcl -f values.yaml -o set_value


    set_value = {
      name = "livenessProbe.httpGet.path"
      value = "/user/login"
    }

    set_value = {
      name = "livenessProbe.httpGet.port"
      value = "http"
    }

    set_value = {
      name = "livenessProbe.initialDelaySeconds"
      value = "120"
    }
```

3. YAML values to `helm_release` `sensitive_value`

```sh
$ y2hcl -f values.yaml -o sensitive_value


    sensitive_value = {
      name = "livenessProbe.httpGet.path"
      value = "/user/login"
    }

    sensitive_value = {
      name = "livenessProbe.httpGet.port"
      value = "http"
    }

    sensitive_value = {
      name = "livenessProbe.initialDelaySeconds"
      value = "120"
    }
```


 4. YAML values to `Helm CLI` set values

 ```sh
$  y2hcl -f values.yaml -o helm_cli


--set livenessProbe.httpGet.path="/user/login" \
--set livenessProbe.httpGet.port="http" \
--set livenessProbe.initialDelaySeconds="120" \
```
