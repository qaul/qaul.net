# libqaul Include Header File

The `libqaul.h` header file was autmatically generated 
with the cargo subcommand cbindgen.

Install cbindgen to automatically create the C/C++11 headers:

```sh
cargo install cbindgen
```

create libqaul.h header file:

```sh
# move to rust folder
cd rust/include

# create the C header file with cbindgen
cbindgen ../libqaul/src/api/c.rs -l c > libqaul.h
```
