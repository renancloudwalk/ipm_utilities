# IPM utilities

For now it only extracts the IP0040T1 from a T067



## Compiling to Windows Server from OSX:
Assuming you have brew, install mingw-w64 and copy the following objects:

```
brew install mingw-w64

rustup target add x86_64-pc-windows-gnu

cp /usr/local/Cellar/mingw-w64/8.0.0/toolchain-x86_64/x86_64-w64-mingw32/lib/crt2.o /usr/local/Cellar/mingw-w64/8.0.0/toolchain-x86_64/x86_64-w64-mingw32/lib/dllcrt2.o /usr/local/Cellar/mingw-w64/8.0.0/toolchain-x86_64/x86_64-w64-mingw32/lib/libmsvcrt.a ~/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-pc-windows-gnu/lib/


cargo build --target x86_64-pc-windows-gnu
```
