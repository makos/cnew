# cnew

## Create basic C program directory structures

### Preface

This was created mostly for my personal usage. It can be customized by delving
into the source code. I didn't try to make it too pretty or clever, it just works.

### Usage

````
cnew [OPTIONS] NAME

Options:
    -h -- help text
    -v -- version info
````

`cnew test` will create a directory named `test` with the following structure:

````
test/ 
    - .git/
    - .gitignore
    - include/
    - src/
        - main.c
    - Makefile
````

The Makefile contains basic directives for a small project.
It also initializes an empty git repository in the directory.

### License & copyright
This is free software, licensed under the MIT license.

&copy; 2018 Mateusz Makowski