// SPDX-License-Identifier: GPL-2.0

//! Rust Kernel Module hello world example.

use kernel::prelude::*;

module! {
    type: HelloRust,
    name: "hello_rust",
    author: "Your Name <your email address>",
    description: "Rust Kernel Module hello world example",
    license: "GPL",
}

struct HelloRust {
}

impl kernel::Module for HelloRust {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World Kernel Module from Rust\n");
        Ok(HelloRust { })
    }
}

impl Drop for HelloRust {
    fn drop(&mut self) {
        pr_info!("Goodbye Kernel Module from Rust\n");
    }
}