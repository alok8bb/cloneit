# cloneit

A cli tool to download specific GitHub directories or files.

# Installation

### From git

```bash
git clone https://github.com/alok8bb/cloneit
cd cloneit
bash ./install.sh
```

# Usage

```bash
cloneit 1.0.0
Alok P <alok8bb@gmail.com>
Download specific GitHub directories or files

USAGE:
    cloneit [OPTIONS] <url>...

ARGS:
    <url>...
            URL to the GitHub directory or file. You can pass a single URL or multiple
            comma-delimited URLs e.g.

            https://github.com/fpic/linpeace.py,https://github.com/s0xf/r2gihdra.c,https://github.com/fpic/defpol/master

OPTIONS:
    -h, --help
            Print help information

    -V, --version
            Print version information

    -z
            Download zipped directory
```

# Examples

### Download a specific folder

```bash
$ cloneit https://github.com/alok8bb/cloneit
```

```bash
$ cloneit https://github.com/alok8bb/cloneit/tree/master/src
```

### Download a file

```bash
$ cloneit https://github.com/alok8bb/cloneit/tree/master/src/main.rs
```

### Downloading and zip the folder/file

Thanks to [@winterrdog](https://github.com/winterrdog) for implementing this feature.

```bash
$ cloneit -z https://github.com/alok8bb/cloneit
```
