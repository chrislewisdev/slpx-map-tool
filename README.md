# Sleep Paradox Map Tool

This is a tool used to convert maps created in Tiled for use in our GBA Jam entry, [Sleep Paradox](https://github.com/chrislewisdev/sleep-paradox). To compile Sleep Paradox from scratch on your machine, you need to have this installed on your system. Aside from exporting tilemap data for use with Butano, it also exports data such as enemy spawn points and portals between different zones/rooms in the game.

## Installation

As long as you have Rust on your system, you can install using cargo:

```
cargo install --path .
```

## Usage

`slpx-map-tool <input-directory> --output-directory <output-directory>` will search for `.tmx` files in `input-directory` and create Butano-compatible tilemap header files in `output-directory`.
