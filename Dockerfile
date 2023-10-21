from ubuntu:23.04
RUN apt update
RUN apt upgrade
RUN apt dist-upgrade

# install Rust
# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN apt install rustc-1.62 rust-1.62-src rustfmt-1.62 \
    bindgen-0.56 llvm clang gcc make
# こっちはうまく行かなかった
# RUN apt install linux-headers-`uname -r`
RUN apt install linux-headers-generic
