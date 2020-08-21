# nu_plugin_id3

The `id3` is a plugin for [nushell](https://www.nushell.sh) that helps you read the [id3](https://en.wikipedia.org/wiki/ID3) metadata of your mp3 files.
This is project is experimental and still under development.
The public API for the user (flags, command input, etc...) may change while [nushell](https://www.nushell.sh) is still pre `1.0`.

![Example of id3](images/nushell.gif "Example of id3")

This project is still lacking many features
- Ability to parse Id3V1 tags
- Show tag version information
- Eliminate non-mp3 files from the output (nu command `compact` removes empty rows)
- Missing several Id3V2 fields such as comments, data recorded, etc...
- Ability to write to the tag (mutate artist, album, comments, lyrics, etc...)
- Ability to convert from older Id3V1 to Id3V2.X

## Installation

This project depends on a minimum of [nushell](https://www.nushell.sh) `0.10.0` to be installed as that is when support for plugin names containing numbers started.

### Cargo
`cargo install nu_plugin_id3`

## Build from Source
- Clone this repository and navigate to the project directory
- `cargo install --path .`
- You may need to restart nushell for it to recognize this plugin.

## Usage

`id3` will recursively walk a directory and try to parse id3 tags for each file it finds.
The only parameter `id3` takes is an optional path.
If no optional path is given, `id3` will search the current directory.
You may specify an arbitrary directory or path to search.

```
// Searches the present working directory
id3
```

```
// Searches all directories inside of ~/mp3s/fugazi
id3 ~/mp3s/fugazi
```

To take advantage of some fun features of [nushell](https://www.nushell.sh) like viewing images in your terminal,
you'll need to have the `binaryview` plugin installed with [nushell](https://www.nushell.sh).
You can install this via `cargo install nu_plugin_binaryview`.

```
// :)
id3 ~/mp3s/fugazi/repeater | get pictures | first | get data
```
