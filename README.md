# rugenere

**rugenere** is a simple [vigenère](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) cipher tool written in rust. It can encode and decode text either from the standard input or a file. It can also write the output to a file.

## Installing

To install this tool just clone the git repository by running the following command (have in mind that this will clone the repository to your current directory).

```bash
> git clone https://github.com/bruno-anjos/rugenere.git
```

## Running

I made it easier to run this tool by creating a link to the binary in the target directory.

### Examples

#### help

Run **rugenere** with the `--help` to get helpful information

```bash
> ./rugenere --help
rugenere 0.1
Bruno Anjos <bruno.vale.anjos@gmail.com>
Vigenére cipher encoder and decoder.

USAGE:
    rugenere [OPTIONS] <key> --mode <mode>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      file name to read content from
    -m, --mode <mode>        sets mode to encode or decode [possible values: encode, decode]
    -o, --output <output>    file name to write content to

ARGS:
    <key>    key used to encode or decode the content
```

#### encode

Run **rugenere** with the `m` flag set to `encode`

```bash
> ./rugenere TESTKEY -m encode                                                     ~/git/rugenere(master✗)@banjos-pc
THis Is AN exaMPlE
Result: MLal Mq EF obyFTdX
```

#### decode

Run **rugenere** with the `m` flag set to `decode`

```bash
> ./rugenere TESTKEY -m decode                                                     ~/git/rugenere(master✗)@banjos-pc
MLal Mq EF obyFTdX
Result: THis Is AN exaMPlE
```

#### with input file

Run **rugenere** with the `i` flag followed by the file name

```bash
> ./rugenere TESTKEY -m encode -i input_test                                       ~/git/rugenere(master✗)@banjos-pc
Result: mlal mq ef obyftdx
```

#### with output file

Run **rugenere** with the `o` flag followed by the file name

```bash
> ./rugenere TESTKEY -m encode -i input_test -o output_test
> cat output_test                                                                  ~/git/rugenere(master✗)@banjos-pc
mlal mq ef obyftdx
```

## Built with

- [clap](https://clap.rs/) - rust library to parse CLI arguments
