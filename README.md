# lto-test

This is a test project for cross language LTO using the rust compiler, compiling a rust program which uses a very simple C function. Specifically, compiling it on windows using the msvc rust toolchain, and compiling the C code using the clang-cl.exe LLVM compiler, and linking it all together using the lld-link.exe LLVM linker.

This requires LLVM binaries in your PATH, and that the LLVM version matches the rust LLVM version. You can check that running `clang-cl --version`, and ensuring it says for example clang version 13.x.x and that the rust compiler `rustc --version --verbose`syas that it uses LLVM version: 13.x.x.

When I tried running this locally as of 18-12-2021 with rustc 1.58.0-nightly (8b09ba6a5 2021-11-09) and clang 13.0.0, it compiled but it did NOT inline symbol() into the rust code, but it did remove unused() from the binary.

You can compile this using build.bat, and then run the generated binary in target/release or run `cargo run --release` as normal.

This repository has a rust-toolchain.toml file for nightly which is what I was using, but it shouldn't be required, so feel free to remove it locally.

Resources I used to make this:

https://blog.llvm.org/2019/09/closing-gap-cross-language-lto-between.html
https://doc.rust-lang.org/rustc/linker-plugin-lto.html

Special thanks to Saethlin#0001 and WithinRafael#7014 from the unofficial rust community discord.
