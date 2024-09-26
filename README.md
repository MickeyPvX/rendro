# Rendro - A Jinja template rendering CLI tool written in Rust

## Description

* Rendro is a CLI tool designed to take Jinja templates (*.j2 files by default) and render them using environment variables.  Any templates with the file extension `.j2` will have the extension stripped (i.e. `my_template.yaml.j2` -> `my_template.yaml`), otherwise the filename will be left as-is.

EXAMPLE:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: test-configmap
data:

  HOMEPATH: {{ env.homepath }}
```

Would, assuming an environment variable of `HOMEPATH` is set, become:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: test-configmap
data:

  HOMEPATH: YOUR_HOMEPATH_VAR_HERE
```

## CLI Commands

### `render`

#### Arguments

* `input_dir`
  * Short: `-i`
  * Long: `--input-dir`
  * Default: `./src/templates`

* `output_dir`
  * Short: `-o`
  * Long: `-output-dir`
  * Default: `./src/templates/temp`

* `file_extension`
  * Short: `-x`
  * Long: `--file-extension`
  * Default: `j2`

## Custom Filters

### `b64encode`

* This filter will be recognized in templates, and will convert the variable into base64
  * Example: `{{ 'Hello, world!' | b64encode }}` -> `SGVsbG8sIHdvcmxkIQ0K`

## Usage

```bash
rendro render
```

* Using Rendro with no arguments will default to any `.j2` file found in the `./src/templates` directory being rendered and stored in the `./src/templates/temp` directory

```bash
rendro render -i ./my-templates -o ./my-rendered-templates -x yaml
```

* This example will find any `.yaml` file in `./my-templates`, render them, and store the rendered templates in `./my-rendered-templates`
