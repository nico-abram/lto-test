cd cbuild/src
clang-cl -flto=thin -Fo../out/test.obj -O2 -c test.c
cd ../out
llvm-lib /OUT:test.lib .\test.obj
cd ../..
cargo clean
cargo build --release