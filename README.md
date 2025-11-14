# Universal Media Converter

Universal Media Converter (UMC) is a program that allows you to easily and quickly convert media files from one format to another (e.g., WEBA to MP3, MKV to MP4, etc.).

## Why ?

- If you don’t want to upload your files to a website just to convert them, you can use this tool to do it locally.
- You can use FFmpeg directly, but it can be complex and difficult to remember because of all its parameters. UMC provides an easy-to-understand interface for fast conversions with default settings — and for advanced users, additional parameters are also available.

## How to run it ?

This project uses Tauri + Vite + VueJS, so you need to install some prerequisites before running it.

### Prerequisites

- Install [Node.js](https://nodejs.org/en/download). This project uses [Bun](https://bun.sh/) as a JavaScript runtime instead of `npm`.
- You should also install [Rust](https://rust-lang.org/tools/install/).

### Installation and running

**1. Download the project**

```bash
# Clone the Github repository
git clone https://github.com/paveldevpro/universal-media-converter.git
```

**2. Install and run**

```bash
# Install the dependencies
bun install

# Run the program
bun tauri dev
```

## Status

This project is currently a work in progress and is **not yet usable**. But feel free to help us to make a version that works.
