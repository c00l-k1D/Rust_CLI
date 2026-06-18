# Rust CLI - Cross-Language Integration

A high-performance CLI application written in Rust that can be easily integrated with other programming languages.

## Features

- ✅ Standalone CLI executable
- ✅ JSON output format for easy parsing
- ✅ FFI (Foreign Function Interface) bindings for C/C++
- ✅ Interactive mode
- ✅ Error handling
- ✅ Cross-platform support

## Project Structure

```
.
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── processor.rs     # Core processing logic
│   └── ffi.rs           # C FFI bindings
├── bindings/            # Language bindings (Python, Node.js, C#, etc.)
├── examples/            # Integration examples
└── Cargo.toml           # Rust project configuration
```

## Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

## Building

### Debug Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

### Build as Shared Library (for FFI)
```bash
cargo build --release --lib
```

## Usage

### As CLI

```bash
# Process data
cargo run -- process "hello world" --json

# Echo message
cargo run -- echo "hello"

# Interactive mode
cargo run -- interactive
```

### As Library (Rust)

```rust
use my_cli::processor::process_data;

fn main() {
    match process_data("input") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### As FFI (C/C++)

```c
#include <stdio.h>

// Declare FFI functions
extern char* process_ffi(const char* input);
extern void free_string(char* ptr);

int main() {
    const char* result = process_ffi("hello");
    printf("Result: %s\n", result);
    free_string((char*)result);
    return 0;
}
```

**Compile C code linking to Rust library:**
```bash
# Build Rust library
cargo build --release --lib

# On Linux/macOS:
gcc main.c -o main -L target/release -l my_cli

# On Windows (MSVC):
cl.exe main.c /link target/release/my_cli.lib

# On Windows (MinGW):
gcc main.c -o main -L target/release -l my_cli
```

## Language Bindings

### Python

```python
import ctypes
import os

# Load the shared library
lib_name = "my_cli"
if os.name == "nt":
    lib = ctypes.CDLL(f"target/release/{lib_name}.dll")
else:
    lib = ctypes.CDLL(f"target/release/lib{lib_name}.so")

# Define function signatures
lib.process_ffi.argtypes = [ctypes.c_char_p]
lib.process_ffi.restype = ctypes.c_char_p
lib.free_string.argtypes = [ctypes.c_char_p]

# Call the function
result = lib.process_ffi(b"hello")
print("Result:", result.decode("utf-8"))
lib.free_string(result)
```

### Node.js

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

const lib = ffi.Library('./target/release/my_cli', {
  process_ffi: ['string', ['string']],
  free_string: ['void', ['string']],
});

const result = lib.process_ffi('hello');
console.log('Result:', result);
```

### C#

```csharp
using System.Runtime.InteropServices;

public class MyCli {
    [DllImport("my_cli.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr process_ffi(string input);

    [DllImport("my_cli.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);

    static void Main() {
        IntPtr result = process_ffi("hello");
        string resultStr = Marshal.PtrToStringAnsi(result);
        Console.WriteLine($"Result: {resultStr}");
        free_string(result);
    }
}
```

## Testing

```bash
cargo test
```

## Formatting & Linting

```bash
cargo fmt         # Format code
cargo clippy      # Run linter
cargo fmt -- --check  # Check formatting without changes
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check linting: `cargo clippy`
7. Submit a pull request

## License

MIT License - see LICENSE file for details

## Performance

Built with optimization flags for maximum performance:
- Link-time optimization (LTO)
- Single code generation unit
- Release profile optimizations

## Troubleshooting

### Library not found on Linux
```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./target/release
```

### Library not found on macOS
```bash
export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:./target/release
```

### Library not found on Windows
Ensure the DLL is in the same directory as the executable or add it to PATH.

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Clap CLI Guide](https://docs.rs/clap/latest/clap/)
- [Serde Documentation](https://serde.rs/)
