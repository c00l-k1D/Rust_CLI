using System;
using System.Runtime.InteropServices;

/// <summary>
/// C# example for using Rust CLI via FFI
/// 
/// Build:
///   dotnet build
///
/// Run:
///   dotnet run
///
/// Note: The Rust library must be built first:
///   cargo build --release --lib
///
/// On Windows, ensure my_cli.dll is accessible (same directory as executable or in PATH)
/// On Linux/macOS, set LD_LIBRARY_PATH or DYLD_LIBRARY_PATH
/// </summary>
class Program {
    // Define the P/Invoke declarations
    [DllImport("my_cli", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    public static extern IntPtr process_ffi(string input);

    [DllImport("my_cli", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);

    static void Main() {
        Console.WriteLine("Rust CLI FFI Example (C#)");
        Console.WriteLine("==========================\n");

        // Example 1: Process simple string
        string input1 = "hello world";
        Console.WriteLine($"Input: {input1}");
        IntPtr result1 = process_ffi(input1);
        string output1 = Marshal.PtrToStringAnsi(result1);
        Console.WriteLine($"Output: {output1}\n");
        free_string(result1);

        // Example 2: Process another string
        string input2 = "rust is awesome";
        Console.WriteLine($"Input: {input2}");
        IntPtr result2 = process_ffi(input2);
        string output2 = Marshal.PtrToStringAnsi(result2);
        Console.WriteLine($"Output: {output2}\n");
        free_string(result2);

        // Example 3: Interactive mode
        Console.WriteLine("==========================");
        Console.WriteLine("Interactive mode (type 'exit' to quit):");

        while (true) {
            Console.Write("> ");
            string input = Console.ReadLine();

            if (input?.ToLower() == "exit") {
                Console.WriteLine("Goodbye!");
                break;
            }

            if (!string.IsNullOrEmpty(input)) {
                IntPtr result = process_ffi(input);
                string output = Marshal.PtrToStringAnsi(result);
                Console.WriteLine($"Output: {output}");
                free_string(result);
            }
        }
    }
}
