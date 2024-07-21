[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Lints](https://github.com/mrLSD/llvm-lib-rs/actions/workflows/lints.yml/badge.svg)](https://github.com/mrLSD/llvm-lib-rs/actions/workflows/lints.yml)
[![Crates.io version](https://img.shields.io/crates/v/llvm-lib.svg?style=flat-square)](https://crates.io/crates/llvm-lib)
<!-- [![Tests](https://github.com/mrLSD/z-rose/actions/workflows/tests.yml/badge.svg)](https://github.com/mrLSD/llvm-lib-rs/actions/workflows/tests.yml) -->

<div style="text-align: center;">
    <h1>mrLSD<code>/llvm-lib-rs</code></h1>
</div>

LLVM library with safety and flexibility in mind, without over complexity based on `LLVM-C` API
provided be `llvm-sys` crate.

## Overview

The `llvm-lib-rs` provides a robust and comprehensive interface to the LLVM Compiler Infrastructure,
leveraging the `LLVM-C API` to offer a blend of safety, flexibility, and extendability. This library
is designed to serve as a powerful tool for developers looking to create backends for compilers, enabling
them to harness the full potential of **LLVM** in a secure and user-friendly manner.

## Safety

Safety is a paramount concern in the design of this library. By building on the `LLVM-C API`, we ensure that
interactions
with the **LLVM** infrastructure are conducted in a type-safe and memory-safe manner. The library employs Rust’s
stringent
safety guarantees to prevent common issues such as null pointer dereferencing, buffer overflows, and memory leaks. This
commitment to safety allows developers to focus on the functionality of their compiler backends without worrying about
underlying security vulnerabilities.

## Flexibility

Flexibility is another core attribute of the llvm-lib-rs`. The library provides a rich set of APIs that cover a wide
range of LLVM’s capabilities, from module management and inline assembly to debugging metadata and function iteration.
Developers can easily access and manipulate **LLVM** constructs, enabling the creation of highly customized and
optimized
compiler backends. The library’s design ensures that it can adapt to various use cases and requirements, making it an
ideal choice for a diverse set of compiler development projects.

## Extendability

The 'llvm-lib-rs' is built with extendability in mind. It is designed to be easily extendable, allowing developers to
add
new functionalities and support for additional **LLVM** features as needed. The modular structure of the library
facilitates
the incorporation of new components, ensuring that it can grow and evolve alongside the **LLVM** ecosystem. This
extendability ensures that the library remains relevant and useful as **LLVM** continues to advance and expand its
capabilities.

## Why LLVM?

**LLVM** (Low-Level Virtual Machine) is a powerful and versatile compiler infrastructure that provides a collection of
modular and reusable compiler and toolchain technologies. It is widely used in the development of modern compilers,
providing a framework for optimizing intermediate representations and generating machine code for various target
architectures. LLVM’s ability to support multiple languages and platforms, coupled with its extensive optimization
capabilities, makes it an essential tool for compiler developers. By leveraging **LLVM**, developers can create highly
efficient and portable compilers that meet the demands of today’s diverse computing environments.

## Design

The llvm-lib-rs library adheres to the structure of the LLVM C API, ensuring easy navigation through the extensive LLVM
functions. Logical elements are grouped into modules, providing a clear organizational structure. Within these modules,
Rust structures are introduced to wrap LLVM types, implementing corresponding functions for the wrapped LLVM types. This
approach enhances flexibility and usability while maintaining the original LLVM code structure. The design avoids
unnecessary complexity in the code and documentation, remaining fully aligned with the LLVM API. This alignment allows
developers to easily navigate the llvm-lib-rs library using existing LLVM-C documentation.

### Safety Considerations

When implementing functions using the `LLVM-C API` through `FFI` (Foreign Function Interface), all unsafe operations
and data types are managed separately and wrapped in new types and structures. This separation ensures that unsafe code
is isolated and not exposed in the final **API**, guaranteeing safety without introducing excessive complexity.
By encapsulating unsafe operations within safe Rust abstractions, the library maintains a clean and understandable
codebase while preventing unsafe memory interactions. This design choice provides a robust layer of protection, ensuring
that users of the `llvm-lib-rs` library can work with **LLVM** functionalities securely and confidently.

## Status

Based on [llvm-sys](https://crates.io/crates/llvm-sys) rust crate.

Support LLVM:

-[x] llvm-18

Development in progress and API suppose to be unstable before completion.

### LICENSE: [MIT](LICENSE)
