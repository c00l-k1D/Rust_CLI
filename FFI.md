# FFI и интеграция с другими языками

## 📖 Основы FFI (Foreign Function Interface)

FFI позволяет вызывать Rust код из других языков программирования.

### Как это работает

```
Rust код (processor.rs)
        ↓
   FFI слой (ffi.rs)
        ↓
   C compatible interface
        ↓
   Другие языки
   (Python, C#, Node.js, etc.)
```

---

## 🔧 Сборка для FFI

### Шаг 1: Сборка Shared Library

```bash
# Linux/macOS
cargo build --release --lib
# Результат: target/release/libmy_cli.so (Linux)
#           target/release/libmy_cli.dylib (macOS)

# Windows
cargo build --release --lib
# Результат: target/release/my_cli.dll
```

### Шаг 2: Проверка артефактов

```bash
# Linux/macOS
ls -la target/release/libmy_cli.*

# Windows
dir target\release\my_cli.dll
```

---

## 🐍 Python интеграция

### Загрузка библиотеки

```python
import ctypes
import os
import platform

def load_lib():
    if platform.system() == "Windows":
        lib = ctypes.CDLL("target/release/my_cli.dll")
    elif platform.system() == "Darwin":  # macOS
        lib = ctypes.CDLL("target/release/libmy_cli.dylib")
    else:  # Linux
        lib = ctypes.CDLL("target/release/libmy_cli.so")
    
    return lib

lib = load_lib()
```

### Определение функций

```python
# Типы параметров
lib.process_ffi.argtypes = [ctypes.c_char_p]

# Тип возврата
lib.process_ffi.restype = ctypes.c_char_p

# Функция очистки памяти
lib.free_string.argtypes = [ctypes.c_char_p]
lib.free_string.restype = None
```

### Вызов функции

```python
# Кодировать строку в bytes
input_data = "hello".encode("utf-8")

# Вызвать функцию
result_ptr = lib.process_ffi(input_data)

# Декодировать результат
result = ctypes.string_at(result_ptr).decode("utf-8")

# Освободить память
lib.free_string(result_ptr)

print(result)
```

### Полный пример

```python
import ctypes

# Загрузить библиотеку
lib = ctypes.CDLL("target/release/libmy_cli.so")

# Определить функции
lib.process_ffi.argtypes = [ctypes.c_char_p]
lib.process_ffi.restype = ctypes.c_char_p
lib.free_string.argtypes = [ctypes.c_char_p]

def process(data):
    result = lib.process_ffi(data.encode("utf-8"))
    result_str = ctypes.string_at(result).decode("utf-8")
    lib.free_string(result)
    return result_str

print(process("hello world"))
```

### Ошибки и решения

| Ошибка | Причина | Решение |
|--------|---------|--------|
| `OSError: cannot open shared object file` | Library not found | Убедитесь что DLL/SO собрана, используйте полный path |
| `AttributeError: undefined symbol` | Функция не экспортирована | Проверьте `#[no_mangle]` в ffi.rs |
| Segmentation fault | Неправильная работа с памятью | Убедитесь что вызываете `free_string` |
| `TypeError: argument 1 must be bytes-like object` | Передали строку вместо bytes | Используйте `.encode()` |

---

## 🔗 C/C++ интеграция

### Объявление функций в C

```c
#include <stdint.h>

// Объявить Rust функции
extern char* process_ffi(const char* input);
extern void free_string(char* ptr);
```

### Вызов из C

```c
#include <stdio.h>

extern char* process_ffi(const char* input);
extern void free_string(char* ptr);

int main() {
    const char* result = process_ffi("hello");
    printf("Result: %s\n", result);
    free_string((char*)result);
    return 0;
}
```

### Компиляция на Linux/macOS

```bash
# Сборка Rust library
cargo build --release --lib

# Компиляция C кода
gcc main.c -o main -L target/release -lmy_cli

# Запуск
LD_LIBRARY_PATH=target/release ./main
```

### Компиляция на Windows (MSVC)

```bash
# Сборка Rust library
cargo build --release --lib

# Компиляция C кода
cl.exe main.c /link target/release/my_cli.lib

# Запуск (DLL должна быть в PATH или рядом)
main.exe
```

### Компиляция на Windows (MinGW)

```bash
# Сборка Rust library
cargo build --release --lib

# Компиляция C кода
gcc main.c -o main -L target/release -lmy_cli

# Запуск (установите PATH)
set PATH=%PATH%;%cd%\target\release
main.exe
```

### Обработка ошибок в C

```c
#include <stdio.h>
#include <string.h>

extern char* process_ffi(const char* input);
extern void free_string(char* ptr);

int main() {
    const char* input = "test";
    char* result = process_ffi(input);
    
    if (result == NULL) {
        fprintf(stderr, "Error: process_ffi returned NULL\n");
        return 1;
    }
    
    // Проверить на ошибку в результате
    if (strncmp(result, "Error:", 6) == 0) {
        fprintf(stderr, "Rust error: %s\n", result);
        free_string(result);
        return 1;
    }
    
    printf("Success: %s\n", result);
    free_string(result);
    return 0;
}
```

---

## 🔲 C# интеграция

### P/Invoke объявление

```csharp
using System.Runtime.InteropServices;

public class RustLib {
    // DLL должен быть в PATH или рядом с .exe
    private const string DLL_NAME = "my_cli";

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    public static extern IntPtr process_ffi(string input);

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_string(IntPtr ptr);
}
```

### Использование в C#

```csharp
using System;
using System.Runtime.InteropServices;

class Program {
    [DllImport("my_cli", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    private static extern IntPtr process_ffi(string input);

    [DllImport("my_cli", CallingConvention = CallingConvention.Cdecl)]
    private static extern void free_string(IntPtr ptr);

    static void Main() {
        string input = "hello world";
        IntPtr result = process_ffi(input);
        
        if (result != IntPtr.Zero) {
            string output = Marshal.PtrToStringAnsi(result);
            Console.WriteLine("Result: " + output);
            free_string(result);
        }
    }
}
```

### Компиляция C# на Windows

```bash
# Убедитесь что DLL скопирована рядом с кодом
copy target\release\my_cli.dll .

# Компиляция
csc example.cs

# Запуск
example.exe
```

---

## 🔌 Node.js интеграция

### Использование node-ffi

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

const lib = ffi.Library('./target/release/my_cli', {
    process_ffi: ['string', ['string']],
    free_string: ['void', ['string']],
});

const result = lib.process_ffi('hello world');
console.log('Result:', result);
```

### Установка зависимостей

```bash
npm install ffi-napi ref-napi
```

### Полный пример (Node.js)

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

const lib = ffi.Library('./target/release/my_cli', {
    process_ffi: ['string', ['string']],
    free_string: ['void', ['string']],
});

function process(data) {
    try {
        const result = lib.process_ffi(data);
        return result;
    } catch (e) {
        console.error('Error:', e);
        return null;
    }
}

// Использование
const result = process('hello');
console.log(result);
```

---

## 🔍 Отладка FFI

### Проверка экспортированных символов

#### Linux/macOS
```bash
nm target/release/libmy_cli.so | grep process_ffi
nm target/release/libmy_cli.dylib | grep process_ffi
```

#### Windows (MSVC)
```bash
dumpbin /exports target/release/my_cli.dll
```

#### Windows (MinGW)
```bash
nm target/release/my_cli.dll | grep process_ffi
```

### Логирование в FFI функции

```rust
// src/ffi.rs
#[no_mangle]
pub unsafe extern "C" fn process_ffi(input: *const c_char) -> *mut c_char {
    eprintln!("DEBUG: process_ffi called");
    
    if input.is_null() {
        eprintln!("DEBUG: input is null");
        let err = CString::new("Null pointer").unwrap();
        return err.into_raw();
    }

    match CStr::from_ptr(input).to_str() {
        Ok(input_str) => {
            eprintln!("DEBUG: processing input: {}", input_str);
            // ... rest of function
        }
        Err(_) => {
            eprintln!("DEBUG: UTF-8 conversion failed");
            let err = CString::new("Invalid UTF-8").unwrap();
            err.into_raw()
        }
    }
}
```

### Запуск с отладкой

```bash
# Linux/macOS
RUST_BACKTRACE=full ./program

# Windows PowerShell
$env:RUST_BACKTRACE = 'full'; .\program.exe

# Windows cmd.exe
set RUST_BACKTRACE=full
program.exe
```

---

## 🚨 Типичные ошибки

### 1. Забыли `#[no_mangle]`

```rust
// ❌ НЕПРАВИЛЬНО - символ не экспортирован
pub extern "C" fn process_ffi(input: *const c_char) -> *mut c_char {
    // ...
}

// ✅ ПРАВИЛЬНО
#[no_mangle]
pub extern "C" fn process_ffi(input: *const c_char) -> *mut c_char {
    // ...
}
```

### 2. Неправильно работаете с памятью

```rust
// ❌ НЕПРАВИЛЬНО - утечка памяти
#[no_mangle]
pub unsafe extern "C" fn process_ffi(input: *const c_char) -> *mut c_char {
    let s = CString::new("result").unwrap();
    // Опасно! Вызывающая сторона не может освободить эту память правильно
    s.as_ptr() as *mut c_char
}

// ✅ ПРАВИЛЬНО
#[no_mangle]
pub unsafe extern "C" fn process_ffi(input: *const c_char) -> *mut c_char {
    let s = CString::new("result").unwrap();
    s.into_raw()  // Передает ownership, вызывающая сторона должна вызвать free_string
}
```

### 3. Не освобождаете память

```python
# ❌ НЕПРАВИЛЬНО - утечка памяти
result = lib.process_ffi(b"hello")
print(result)
# Забыли free_string!

# ✅ ПРАВИЛЬНО
result = lib.process_ffi(b"hello")
print(result)
lib.free_string(result)  # Освобождаем память
```

### 4. Неправильные типы параметров

```python
# ❌ НЕПРАВИЛЬНО
lib.process_ffi("hello")  # String вместо bytes!

# ✅ ПРАВИЛЬНО
lib.process_ffi(b"hello")  # bytes
# или
lib.process_ffi("hello".encode("utf-8"))  # bytes
```

---

## 📊 Сравнение методов интеграции

| Метод | Скорость | Простота | Типизация |
|-------|----------|----------|-----------|
| CLI + JSON | Медленная | Простая | Слабая |
| FFI (Python) | Быстрая | Средняя | Средняя |
| FFI (C) | Очень быстрая | Сложная | Сильная |
| FFI (C#) | Быстрая | Средняя | Сильная |

---

## 🎯 Когда использовать FFI

✅ **Используйте FFI когда:**
- Нужна высокая производительность
- Работаете с бинарными данными
- Интегрируете с существующей C/C++ кодой
- Нужна максимальная скорость

❌ **Не используйте FFI когда:**
- Простое решение через CLI достаточно
- Нужна простота и читаемость
- Не критична скорость
- FFI усложнит поддержку кода

---

## 📚 Дополнительные ресурсы

- [Rust FFI Official Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [ctypes Documentation (Python)](https://docs.python.org/3/library/ctypes.html)
- [P/Invoke Documentation (C#)](https://docs.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)
- [ctypes Examples](https://docs.python.org/3/library/ctypes.html#fundamental-data-types)

---

Удачи в интеграции! 🦀
