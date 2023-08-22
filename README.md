# Tidy 🧹

A CLI tool to remove build directories and other clutter

## Installation

```sh
cargo install tidy
```

## Usage

Remove all [target directories](#target-directories) from the current working directory and all its
children

```sh
tidy
```

### Target Directories

The directories to remove

#### Default Values
`node_modules, target`

#### Setting values
```sh
tidy -t build, node_modules
```

### Ignore Directories

Directories that will not be searched. Will not prevent a directory from being deleted if
it is in the [target directories](#target-directories).

#### Default Values
`.cache, .local, .config`

#### Setting values
```sh
tidy -i .cache
```
