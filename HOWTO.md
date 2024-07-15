# HOWTO

To run in debug mode:
```sh
RUST_LOG=debug cargo run
```

To run in build mode:
```sh
cargo run
```

To compile shaders:
```sh
cd src/shaders
slangc gradient.slang -emit-spirv-directly -g2 -profile glsl_460 -target spirv -o gradient.spv -entry main
```