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

### Downloading a specific folder

```bash
$ cloneit https://github.com/alok8bb/cloneit
```

```bash
$ cloneit https://github.com/alok8bb/cloneit/tree/master/src
```

### Downloading a specific file

```bash
$ cloneit https://github.com/alok8bb/cloneit/tree/master/src/main.rs
```

### Downloading a zip file - WIP

```bash
$ cloneit -z https://github.com/alok8bb/cloneit
```

# Todo

-   [ ] Downloading zipped directories
-   [ ] Uploading File
-   [ ] Advanced Error Handling
-   [ ] Code Refactoring
