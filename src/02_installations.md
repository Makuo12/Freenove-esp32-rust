# Installations

## RISC-V and Xtensa Targets

The esp32 ism mostly based up of 2 chip architectures RISC-V and Xtensa. The RISC-V are usually single chip architectures while the Xtensa is dual core.
When setuping up a working environment with my esp32 in rust I usually just like using the espup tool kit as it simplifies installing and maintaining the components required to develop Rust applications for the Xtensa and RISC-V architectures.

### 1. Install `espup`

To install `espup`, run:

```shell
cargo install espup
```

### 2. Install Necessary Toolchains

Install all the necessary tools to develop Rust applications for all supported Espressif targets by running:

```shell
espup install
```

### 3. Setup environment varaibles

To setup environment varaibles checkout [the Rust ESP book Part 3](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).

### 4. Generate Project from template

Using templates adds a layers of convienence because it generates a working project you all the dependencies you need to get started.

To generate a template first you will need to install esp-generate:

esp-generate is project generation tool that can be used to generate an application with all the required configurations and dependencies

```shell
cargo install esp-generate
```

Once it is done installing all you now need to do is to run it and specify the chip

```shell
esp-generate --chip=esp32c6 your-project
```

To learn more about esp-generate checkout Generating Projects from Templates [The Rust on ESP Book](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/index.html)

### [Next](https://github.com/Makuo12/Freenove-esp32-rust/blob/main/src/03_Project_1.1_Blink.md)
