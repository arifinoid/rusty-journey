# The Rusty Journey

I started my journey by going through the [official Rust documentation](https://doc.rust-lang.org/book/). This provided a solid foundation on the language's syntax, type system, and standard library. The documentation also has a lot of examples and exercises, which helped me get a feel for the language.

## Installation

The first step is to install Rust. We’ll download Rust through `rustup`, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.

## Installing `rustup` on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```sh
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

You might be prompted for your password. If the install is successful, the following line will appear:

```
Rust is installed now. Great!
```

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```sh
xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.
