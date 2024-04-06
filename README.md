# pngpaste

Paste images from your clipboard into a PNG file.

## Installation

You can install pngpaste by running the install script which will download
the [latest release](https://github.com/mskelton/pngpaste/releases/latest).

```bash
curl -LSfs https://go.mskelton.dev/pngpaste/install | sh
```

Or you can build from source.

```bash
git clone git@github.com:mskelton/pngpaste.git
cd pngpaste
cargo install --path .
```

## Usage

To use `pngpaste`, simply provide the name of the file where you want to write
the contents of your clipboard.

```bash
pngpaste output.png
```

You can also use the `--stdout` flag to print to stdout instead of a file.

```bash
pngpaste --stdout
```
