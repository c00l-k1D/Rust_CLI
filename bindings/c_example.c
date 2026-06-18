/*
 * C example for using Rust CLI via FFI
 *
 * Compilation:
 *   Linux/macOS: gcc c_example.c -o c_example -L../target/release -lmy_cli
 *   Windows (MSVC): cl.exe c_example.c /link ../target/release/my_cli.lib
 *   Windows (MinGW): gcc c_example.c -o c_example -L../target/release -lmy_cli
 *
 * Execution:
 *   LD_LIBRARY_PATH=../target/release ./c_example
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Forward declarations of Rust FFI functions
extern char* process_ffi(const char* input);
extern void free_string(char* ptr);

int main() {
    printf("Rust CLI FFI Example (C)\n");
    printf("========================\n\n");
    
    // Example 1: Process simple string
    const char* input1 = "hello world";
    printf("Input: %s\n", input1);
    char* result1 = process_ffi(input1);
    printf("Output: %s\n\n", result1);
    free_string(result1);
    
    // Example 2: Process another string
    const char* input2 = "rust is awesome";
    printf("Input: %s\n", input2);
    char* result2 = process_ffi(input2);
    printf("Output: %s\n\n", result2);
    free_string(result2);
    
    // Example 3: Interactive mode
    printf("========================\n");
    printf("Interactive mode (type 'exit' to quit):\n");
    
    char buffer[256];
    while (1) {
        printf("> ");
        fflush(stdout);
        
        if (fgets(buffer, sizeof(buffer), stdin) == NULL) {
            break;
        }
        
        // Remove newline
        size_t len = strlen(buffer);
        if (len > 0 && buffer[len - 1] == '\n') {
            buffer[len - 1] = '\0';
        }
        
        if (strcmp(buffer, "exit") == 0) {
            printf("Goodbye!\n");
            break;
        }
        
        char* result = process_ffi(buffer);
        printf("Output: %s\n", result);
        free_string(result);
    }
    
    return 0;
}
