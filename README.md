# Recase

Recase is a little utility to switch the casing-style of code from one style
to another. For example, it can convert between [camel case](https://en.wikipedia.org/wiki/Camel_case),
[snake case](https://en.wikipedia.org/wiki/Letter_case#Snake_case), and
[kebab case](https://en.wikipedia.org/wiki/Letter_case#Kebab_case). It reads
its input from `stdin`.

The valid case types are as follows:

* "snake"
* "camel"
* "kebab"

## Usage

To convert from snake-case to camel-case, reading from stdin:

```sh
$ recase snake camel
this_is_the_test
thisIsTheTest
^D
```

To convert a single string:

```sh
$ recase snake camel 'this_is_the_test'
thisIsTheTest
```

## License

Recase is licensed under the MIT license.
