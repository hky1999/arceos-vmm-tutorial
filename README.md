# ArceOS-VMM-Tutorial

Let's build a VMM (Virtual Machine Minotor or hypervisor) upon [ArceOS](https://github.com/rcore-os/arceos) unikernel!

## Preparation

Initialize the ArceOS repository using Git submodule.

```console
$ git submodule init && git submodule update
```

Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) to use `rust-objcopy` and `rust-objdump` tools:

```console
$ cargo install cargo-binutils
```

Your also need to install [musl-gcc](http://musl.cc/x86_64-linux-musl-cross.tgz) to build guest user applications.

## Build & Run Hypervisor

```console
$ cd arceos-vmm
$ make -C ../arceos/ A=$(pwd) run ACCEL=y [LOG=warn|info|debug|trace]
......
Booting from ROM..
Initialize IDT & GDT...

       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = x86_64
platform = x86_64-qemu-q35
target = x86_64-unknown-none
smp = 1
build_mode = release
log_level = warn

Starting virtualization...
Hardware support: true
VmxVcpu {
    guest_regs: GeneralRegisters {
        rax: 0x0,
        rcx: 0x0,
        rdx: 0x0,
        rbx: 0x0,
        _unused_rsp: 0x0,
        rbp: 0x0,
        rsi: 0x0,
        rdi: 0x0,
        r8: 0x0,
        r9: 0x0,
        r10: 0x0,
        r11: 0x0,
        r12: 0x0,
        r13: 0x0,
        r14: 0x0,
        r15: 0x0,
    },
    rip: 0x0,
    rsp: 0x0,
    rflags: 0x2,
    cr0: 0x30,
    cr3: 0x0,
    cr4: 0x2000,
    cs: 0x0,
    fs_base: 0x0,
    gs_base: 0x0,
    tss: 0x0,
}
......

