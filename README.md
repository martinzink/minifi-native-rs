# MiNiFi Native Rust

This repository provides a safe, idiomatic, and high-performance Rust framework for building native extensions (processors) for [Apache NiFi MiNiFi C++](https://github.com/apache/nifi-minifi-cpp)

It is designed to offer a robust developer experience, allowing you to write powerful and reliable data processing components in safe Rust.

The framework completely encapsulates the unsafe C FFI (Foreign Function Interface) boundary, providing a pure Rust API that is fully mockable for unit testing.

## Project Philosophy
 - **Safety First**: Leverage Rust's compile-time guarantees to prevent common bugs like null pointers, buffer overflows, and data races.
 - **Zero-Cost Abstraction**: The safe API wrapper is designed to compile down with zero runtime overhead compared to writing raw C FFI code.
 - **Ergonomics**: Provide a clean, idiomatic Rust API that is a pleasure to use. Developers should not need to think about unsafe code or C++ interoperability.
 - **Testability**: Every component of a processor's logic should be unit-testable in a pure Rust environment, without needing a C++ host.
 - **Cross Platform**: The library should work on all platforms that are supported by [Apache NiFi MiNiFi C++](https://github.com/apache/nifi-minifi-cpp)
   - macOS (aarch64)
   - Linux (x86_64, aarch64)
   - Windows (x86_64)


The project is structured as a Cargo workspace with a clear, layered architecture:
## [minifi-native-sys](minifi-native-sys)
Contains the raw, unsafe FFI bindings to the minifi-c.h C API.
## [minifi-native](minifi-native)
Provides the public, safe, and idiomatic Rust API. This is the crate that developers will use to build their processors.
#### API Traits
Pure Rust traits (Processor, ProcessSession, Logger, etc.) that define the abstract behavior of the MiNiFi environment.
#### FFI Wrappers
Concrete structs (CffiSession, CffiLogger, etc.) that implement the API traits by calling the unsafe functions from minifi-native-sys.
#### Thread safety
The trait system differentiates between thread-safe (&self) and single-threaded (&mut self) processors at compile time.
#### Automatic Registration:
Processors self-register when the library is loaded by the C++ host, using the ctor crate.
#### Comprehensive Mocking:
A full suite of mock objects allows for fast and reliable unit testing of all processor logic.


## [reference-extension](reference-extension)
A concrete example of a processor extension built using the minifi-native crate.
  - Demonstrates how to implement the Processor traits. Shows how to define properties and relationships.
  - Uses a #[ctor] function for self-registration, so the C++ host can load it without needing to call an explicit initialization function.
  - Includes comprehensive unit tests using the mocking framework.
  - Includes integration testing that verifies the processor works as expected in a real MiNiFi environment.
     

#### Use the Library in the MiNiFi C++ Application
Copy the shared library (.so, .dll, or .dylib) to the MiNiFi C++ application's extensions/ directory.