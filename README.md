# cloneit

A cli tool to download specific GitHub directories or files.

# Installation

### From git

```bash
git clone https://github.com/alok8bb/cloneit
cd cloneit
cargo install build --release
```

# Usage

```
cloneit 1.0
Alok P <alok8bb@gmail.com>
Download specific GitHub directories or files

USAGE:
    cloneit [FLAGS] <URL>

ARGS:
    <URL>    URL to the GitHub directory

FLAGS:
    -h, --help       Prints help information
    -l               Generate download link to zipped file
    -V, --version    Prints version information
    -z               Download zipped directory
```

# Examples

### Downloading

```bash
$ cloneit https://github.com/alok8bb/cloneit
```

```bash
$ cloneit https://github.com/alok8bb/cloneit/tree/master/src
```

```bash
$ cloneit https://github.com/alok8bb/cloneit/tree/master/src/main.rs
```

### Zipped File - WIP

```bash
$ cloneit -z https://github.com/alok8bb/cloneit
```

# Todo

-   [ ] Downloading zipped directories
-   [ ] Uploading File
-   [ ] Advanced Error Handling
-   [ ] Code Refactoring
