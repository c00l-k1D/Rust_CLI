#!/usr/bin/env python3
"""
Python example for using Rust CLI via FFI

Requirements:
    - First build the Rust library: cargo build --release --lib
    - Install ctypes (built-in to Python)
"""

import ctypes
import os
import sys
import platform

def get_library_path():
    """Get the path to the compiled library based on the OS"""
    base_path = os.path.join(os.path.dirname(__file__), "..", "target", "release")
    
    if platform.system() == "Windows":
        return os.path.join(base_path, "my_cli.dll")
    elif platform.system() == "Darwin":  # macOS
        return os.path.join(base_path, "libmy_cli.dylib")
    else:  # Linux
        return os.path.join(base_path, "libmy_cli.so")


def load_library():
    """Load the Rust library"""
    lib_path = get_library_path()
    
    if not os.path.exists(lib_path):
        print(f"Error: Library not found at {lib_path}")
        print("Please build the library first: cargo build --release --lib")
        sys.exit(1)
    
    lib = ctypes.CDLL(lib_path)
    
    # Define function signatures
    lib.process_ffi.argtypes = [ctypes.c_char_p]
    lib.process_ffi.restype = ctypes.c_char_p
    lib.free_string.argtypes = [ctypes.c_char_p]
    lib.free_string.restype = None
    
    return lib


def process_data(lib, data: str) -> str:
    """Process data using the Rust CLI library"""
    # Call the FFI function
    result_ptr = lib.process_ffi(data.encode("utf-8"))
    
    # Convert C string to Python string
    result_str = ctypes.string_at(result_ptr).decode("utf-8")
    
    # Free the memory allocated by Rust
    lib.free_string(result_ptr)
    
    return result_str


def main():
    print("Loading Rust library...")
    lib = load_library()
    
    print("Rust CLI FFI Example (Python)")
    print("=" * 40)
    
    # Example 1: Process simple string
    test_data = "hello world"
    print(f"\nInput: {test_data}")
    result = process_data(lib, test_data)
    print(f"Output: {result}")
    
    # Example 2: Process another string
    test_data = "rust is awesome"
    print(f"\nInput: {test_data}")
    result = process_data(lib, test_data)
    print(f"Output: {result}")
    
    # Example 3: Interactive mode
    print("\n" + "=" * 40)
    print("Interactive mode (type 'exit' to quit):")
    while True:
        user_input = input("> ")
        if user_input.lower() == "exit":
            print("Goodbye!")
            break
        result = process_data(lib, user_input)
        print(f"Output: {result}")


if __name__ == "__main__":
    main()
