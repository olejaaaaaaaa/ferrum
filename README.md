[![CI Status](https://github.com/olejaaaaaaaa/ferrum/actions/workflows/rust.yml/badge.svg)](https://github.com/olejaaaaaaaa/ferrum/actions)

# Ferrum

Ferrum is an experiment in creating a 2D/3D engine with graphical techniques from AAA engines

# Preview

https://github.com/user-attachments/assets/c23b24b1-c1c0-49cd-8238-643bca31abc4

# The main goals of the engine:

    - Be productive and not waste resources In vain, but
    the emphasis is on architecture, not on Highest
    possible performance

    - Provide a RenderGraph for easy description complex
    graphic techniques

    - Support for only one vulkan api through the library
    ash >= 0.38. No RHI or additional layers for Using
    multiple GAPIs

    - Scripting support and ability to create scripts for
    different languages. The main emphasis will be is aimed
    at Rust/Lua scripts

    - Be well-documented and have tests

## Installation

```bash
  git clone https://github.com/olejaaaaaaaa/ferrum.git
```

## Cube with texture Example

```bash
    cd ~/ferrum/examples/cube
    cargo run
```

## Supported OS

The project is developed and tested mainly on Windows 10, to work on other OS, changes in the engine may be required

## License

[MIT](https://choosealicense.com/licenses/mit/) or [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)

