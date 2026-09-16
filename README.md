# img-search

A Rust CLI tool used to search for text inside images using OCR.

`img-search` recursively scans a directory for supported image files, extracts their text using **Tesseract OCR**, caches the OCR results, and returns the paths of images containing the specified search text.

## Prerequisites

* **Rust / Cargo**
* **Tesseract OCR**
* Tesseract language data for the languages you want to search

By default, `img-search` uses:

* English (`eng`)
* German (`deu`)

### Arch Linux

```bash
sudo pacman -S rust tesseract tesseract-data-eng tesseract-data-deu
```

### Debian / Ubuntu

```bash
sudo apt install cargo rustc tesseract-ocr tesseract-ocr-eng tesseract-ocr-deu
```

## Installation

### Clone the repository

```bash
git clone https://github.com/Vaenyx/img-search.git
cd img-search
```

### Build

```bash
cargo build --release
```

The compiled executable can then be found at:

```bash
target/release/img-search
```

Optionally, install it into your Cargo binary directory:

```bash
cargo install --path .
```

Afterwards, `img-search` can be called directly:

```bash
img-search [OPTIONS] <INPUT> [SEARCH_DIRECTORY]
```

## Usage

### Execution

```bash
img-search <INPUT> [SEARCH_DIRECTORY]
```

`INPUT` is the text to search for.

If no search directory is specified, the current directory is used.

The search is **case-insensitive**.

### Options

| Option                         | Description                                           |
| ------------------------------ | ----------------------------------------------------- |
| `<INPUT>`                      | Text to search for                                    |
| `[SEARCH_DIRECTORY]`           | Directory containing the images to search recursively |
| `-c, --cache-directory <PATH>` | Directory used to store OCR cache files               |
| `-l, --languages <LANGUAGES>`  | Tesseract OCR languages using ISO 639-2 codes         |
| `-t, --thread-count <COUNT>`   | Number of parallel OCR indexing threads               |
| `-h, --help`                   | Show help and exit                                    |
| `-V, --version`                | Show version and exit                                 |

The default OCR languages are:

```text
eng deu
```

The default thread count is:

```text
1
```

If no cache directory is specified, the system cache directory is used under:

```text
img-search
```

For example, on Linux this will usually be:

```text
~/.cache/img-search
```

## Supported Image Formats

`img-search` recursively indexes the following image formats:

```text
.png
.jpg
.jpeg
.webp
.tif
.tiff
```

## Examples

Search the current directory for images containing `invoice`:

```bash
img-search invoice
```

Search a specific directory:

```bash
img-search invoice ~/Pictures
```

Search for multiple words:

```bash
img-search "invoice number" ~/Documents/scans
```

Use only English OCR:

```bash
img-search invoice ~/Pictures -l eng
```

Use English and German OCR:

```bash
img-search Rechnung ~/Pictures -l eng -l deu
```

Use four OCR indexing threads:

```bash
img-search invoice ~/Pictures -t 4
```

Use a custom cache directory:

```bash
img-search invoice ~/Pictures -c ~/.cache/my-image-index
```

Combine multiple options:

```bash
img-search "order number" ~/Pictures -l eng -l deu -t 4
```

## Caching

OCR results are cached so that images do not need to be processed by Tesseract every time a search is performed.

For each indexed image, `img-search` stores:

* extracted OCR text
* the original image path
* the image modification timestamp

An image is automatically re-indexed when its modification timestamp changes.

This makes subsequent searches significantly faster because unchanged images can use the existing OCR cache.

## Output

Matching image paths are printed to stdout, one path per line.

Example:

```text
/home/user/Pictures/invoice-2026-01.jpg
/home/user/Pictures/scans/invoice-2026-02.png
/home/user/Pictures/archive/order.jpg
```

This makes the output easy to combine with other command-line tools.

For example:

```bash
img-search invoice ~/Pictures | xargs -d '\n' -n1 echo
```
