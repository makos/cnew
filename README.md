# cnew

## Create basic C program directory structures

## Usage

````
cnew [OPTIONS] NAME

Options:
    -h -- help text
    -v -- version info
````

`cnew test` will create a directory named `test` with the following structure:

````
program_name ->
        - /include/
        - /src/main.c
        - /Makefile
        - /.gitignore
        - /.git
````

As you can see, it also initializes an empty git repository in the directory.

## License & copyright
This is free software, licensed under the MIT license. Do as you wish with it.

2018 Mateusz Makowski