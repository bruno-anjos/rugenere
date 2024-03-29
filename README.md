# rugenere [![GitHub license](https://img.shields.io/badge/license-GNU-blue.svg)](https://github.com/bruno-anjos/rugenere/blob/master/LICENSE) ![version](https://img.shields.io/badge/version-1.1.0-yellow) ![dependencies](https://img.shields.io/badge/clap-2.33-orange)

**rugenere** is a simple [vigenère](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) cipher tool written in rust. It can encode and decode text either from the standard input or a file. It can also write the output to a file.

### [crates.io](https://crates.io/crates/rugenere)

| **Branches** | **Build Status** |
|:--------:|:------------:|
|  master  |[![Build Status](https://travis-ci.org/bruno-anjos/rugenere.svg?branch=master)](https://travis-ci.org/bruno-anjos/rugenere)|
| dev      |[![Build Status](https://travis-ci.org/bruno-anjos/rugenere.svg?branch=dev)](https://travis-ci.org/bruno-anjos/rugenere)|

## Installing

To install this tool just clone the git repository by running the following command (have in mind that this will clone the repository to your current directory).

```bash
> git clone https://github.com/bruno-anjos/rugenere.git
> cd rugenere
> cargo build --release
```

## Examples

I made it easier to run this tool by creating a link to the binary in the target directory.

### help

Run **rugenere** with the `--help` to get helpful information

```bash
> ./rugenere --help
rugenere 1.0
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

### encode

Run **rugenere** with the `m` flag set to `encode`

```bash
> ./rugenere TESTKEY -m encode
THis Is AN exaMPlE
Result: MLal Sw YG iptWTjX
```

### decode

Run **rugenere** with the `m` flag set to `decode`

```bash
> ./rugenere TESTKEY -m decode
MLal Sw YG iptWTjX
Result: THis Is AN exaMPlE
```

### with input file

Run **rugenere** with the `i` flag followed by the file name

```bash
> ./rugenere TESTKEY -m encode -i input_test
Result: MlAL sw hNWl tX IvTQheo
```

### with output file

Run **rugenere** with the `o` flag followed by the file name

```bash
> ./rugenere TESTKEY -m encode -i input_test -o output_test
> cat output_test
MlAL sw hNWl tX IvTQheo
```

## Built with

- [clap](https://clap.rs/) - rust library to parse CLI arguments

## Contributing

If you detect any bug or find any way to improve the code, please make a [pull request](https://github.com/bruno-anjos/rugenere/pulls) or submit an [issue](https://github.com/bruno-anjos/rugenere/issues).

## License

This project is licensed under the GNU License - see the LICENSE.md file for details
