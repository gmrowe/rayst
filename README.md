# Rayst

A toy ray tracer written in Rust.

## Description

This repo contains a ray tracer implemented in a TDD style going through the book [The Ray Tracer Callenge:
A Test-Driven Guide to Your First 3D Renderer][1] by Jamis Buck.

## Getting Started

### Dependencies

* No external dependancies required

### Installing

If you have a rust compiler and Cargo installed:

* clone the repository
* cd into the newly created directory and execute:
```
cargo build --release
```

### Executing program

* after building, execute
```
./target/release/rayst
```
Program should output `scene.ppm` which can be opened in many image viewer progams including
Preview on macOS and gthumb on Linux. 

[1]: http://raytracerchallenge.com/