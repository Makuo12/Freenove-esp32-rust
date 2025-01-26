
# Introduction

## Content of this material

The goal of this material is to provide step by step process to getting through the various chapters from the freenove tutorial book (Freenove_Ultimate_Starter_Kit_for_ESP32) using the Rust programming language. The approach we would b taking is `no_std` (bare metal). To better understand this approach, see [Developing on Bare Metal (no_std)] chapter of [The Rust on ESP Book].

This book also draws inspiration from The Rust on ESP Book and no_std training rust books.

> Note that there are several examples covering the use of specific peripherals under the examples folder of `esp-hal`. See [`esp-hal/examples`].

If you would like to learn about `std` development, see [Using the Standard Library (std)] chapter of [The Rust on ESP Book] and,
[Embedded Rust on Espressif] training.

You can join the [esp-rs community](https://matrix.to/#/#esp-rs:matrix.org) on Matrix for all technical questions and issues! The community is open to everyone.

## The board

The board I will be using is the ESP32, but all ESP boards will work fine.

You can use any [SoC supported by `no_std`] but smaller code and configuration changes might be needed.

## The Freenove materials

In the materials folder you can see the Freenove tutorial materials which is provided in python and C. This materials would serve as references to what would be shared in the book.
To ensure that you have all the materials you will need you can order a complete tools kit [freenove tool kit](https://www.amazon.com/FREENOVE-Ultimate-ESP32-WROVER-Included-Compatible/dp/B0CJJJ7BCY/ref=sr_1_8?dib=eyJ2IjoiMSJ9.MYk0voOCzP8pMzF_jnFDtoIaJU4oJvRaxGvn51TD5sWLndAy-Fb86jsc0tPBI3JBprnhIUwq1EsEuEH0mnLwtkO1Khf7Mt6lzaYxmxVLOgAtZPMBW2QS-oOkHvewS1bWfIb65gNNbYPARzEY-36T_DBwCglmjnlR7f9lISzh3TZEpXrpt24c0cYFxSE_UF_DiMTu_l0Ba6nO6qkckSR12jLWdGM2RpBm2_yZ3UmKTMM.54VVLDW3vagvBVv2R-u8eVMgVOZSkHd_O1NS0aoa6lY&dib_tag=se&keywords=freenove&qid=1737893230&sr=8-8&th=1).

## Rust knowledge

- Basic Rust like [The Rust Book](https://doc.rust-lang.org/book/) Chapters 1 - 6, Chapter 4 Ownership, does not need to be fully understood.
- [The Rust on ESP Book](https://esp-rs.github.io/book/) is not required, but it is highly recommended, as it can help you understand the Rust on ESP ecosystem and many of the concepts that will be discussed during the training.
