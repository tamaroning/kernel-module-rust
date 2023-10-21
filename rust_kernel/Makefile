NAME=hello_rust

ifndef KERNELRELEASE
ifndef KDIR
KDIR:=/lib/modules/`uname -r`/build
endif
PWD := $(shell pwd)

rust_flags=CROSS_COMPILE=x86_64-linux-gnu- HOSTRUSTC=rustc-1.62 RUSTC=rustc-1.62 BINDGEN=bindgen-0.56 RUSTFMT=rustfmt-1.62 RUST_LIB_SRC=/usr/src/rustc-1.62.1/library

all:
	@$(MAKE) $(rust_flags) -C $(KDIR) M=$(PWD) modules

install:
	@$(MAKE) $(rust_flags) -C $(KDIR) M=$(PWD) modules_install

clean:
	@rm -f *.o *.ko *.mod* .*.cmd *.d Module.symvers modules.order
	@rm -rf .tmp_versions

else
obj-m := $(NAME).o
endif
