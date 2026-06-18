# Справка по установке и использованию

## 📋 Оглавление
1. [Установка Rust](#установка-rust)
2. [Первоначальная настройка](#первоначальная-настройка)
3. [Разработка](#разработка)
4. [Интеграция с другими языками](#интеграция-с-другими-языками)
5. [Troubleshooting](#troubleshooting)

---

## Установка Rust

### Для Windows (Рекомендуется)

**Способ 1: Автоматический скрипт**
1. Откройте PowerShell или cmd.exe
2. Перейдите в директорию проекта
3. Запустите: `install-rust.bat`

**Способ 2: Ручная установка**
1. Откройте https://rustup.rs/ в браузере
2. Следуйте инструкциям установщика
3. Перезагрузите терминал после установки

**Способ 3: Через Chocolatey (если установлен)**
```powershell
choco install rust
```

### Для macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Для Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Первоначальная настройка

### 1. Проверка установки
```bash
rustc --version  # должно вывести версию компилятора
cargo --version  # должно вывести версию пакетного менеджера
```

### 2. Обновление Rust (рекомендуется)
```bash
rustup update
```

### 3. Установка рекомендуемых компонентов
```bash
rustup component add rustfmt clippy
```

---

## Разработка

### Сборка проекта

```bash
# Отладочная сборка (быстро, но медленное выполнение)
cargo build

# Release сборка (медленнее собирается, но быстро выполняется)
cargo build --release

# Shared library для FFI интеграции
cargo build --release --lib
```

### Запуск приложения

```bash
# Через cargo
cargo run -- process "hello world"
cargo run -- process "hello" --json
cargo run -- echo "test message"
cargo run -- interactive

# Или напрямую (после сборки)
./target/release/my-cli process "hello world"
```

### Форматирование и проверка кода

```bash
# Форматировать код по стандартам Rust
cargo fmt

# Проверить код на потенциальные ошибки
cargo clippy

# Проверить без выполнения (быстро)
cargo check
```

### Тестирование

```bash
# Запустить все тесты
cargo test

# Запустить с выводом println!
cargo test -- --nocapture

# Запустить один тест
cargo test test_process_data
```

### Отладка

```bash
# Запустить с информацией об ошибке
RUST_BACKTRACE=1 cargo run -- process "test"

# Полная информация
RUST_BACKTRACE=full cargo run -- process "test"
```

---

## Интеграция с другими языками

### Python

1. **Сборка библиотеки:**
```bash
cargo build --release --lib
```

2. **Использование:**
```python
import ctypes
import os

# Загрузить библиотеку
lib = ctypes.CDLL('./target/release/my_cli.dll')  # Windows
# или
# lib = ctypes.CDLL('./target/release/libmy_cli.so')  # Linux
# lib = ctypes.CDLL('./target/release/libmy_cli.dylib')  # macOS

lib.process_ffi.argtypes = [ctypes.c_char_p]
lib.process_ffi.restype = ctypes.c_char_p

result = lib.process_ffi(b"hello")
print(result.decode('utf-8'))
lib.free_string(result)
```

3. **Запуск примера:**
```bash
python bindings/python_example.py
```

### C / C++

1. **Сборка библиотеки:**
```bash
cargo build --release --lib
```

2. **Linux/macOS - Компиляция:**
```bash
gcc bindings/c_example.c -o c_example -L target/release -lmy_cli
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./target/release
./c_example
```

3. **Windows - Компиляция (MinGW):**
```bash
gcc bindings/c_example.c -o c_example -L target/release -lmy_cli
set PATH=%PATH%;%cd%\target\release
c_example.exe
```

4. **Windows - Компиляция (MSVC):**
```bash
cl.exe bindings/c_example.c /link target/release/my_cli.lib
```

### C#

1. **Сборка библиотеки:**
```bash
cargo build --release --lib
```

2. **Подготовка:**
```bash
# Скопируйте DLL рядом с вашей C# программой
copy target\release\my_cli.dll bindings\
```

3. **Компиляция:**
```bash
csc bindings/csharp_example.cs
```

4. **Запуск:**
```bash
csharp_example.exe
```

---

## Структура проекта

```
my-cli/
├── src/
│   ├── main.rs              # CLI приложение - команды и обработка аргументов
│   ├── lib.rs               # Экспорт модулей для библиотеки
│   ├── processor.rs         # ✏️ РЕДАКТИРУЙТЕ ЗДЕСЬ - основная логика
│   └── ffi.rs               # FFI функции для других языков (автоматически)
│
├── bindings/                # Примеры для других языков
│   ├── python_example.py
│   ├── c_example.c
│   └── csharp_example.cs
│
├── examples/                # Примеры использования (пока пусто)
│   └── (для ваших примеров)
│
├── .vscode/                 # Конфигурация VS Code
│   ├── settings.json        # Параметры редактора (Rust formatting, etc)
│   ├── launch.json          # Конфигурация отладки (F5)
│   └── tasks.json           # Задачи сборки (Ctrl+Shift+B)
│
├── .github/
│   └── copilot-instructions.md
│
├── Cargo.toml               # Конфигурация проекта (версия, зависимости)
├── Cargo.lock               # Блокировка версий (git-иное)
├── README.md                # Основная документация
├── QUICKSTART.md            # Быстрый старт (этот файл)
├── SETUP.md                 # Эта справка
├── install-rust.bat         # Скрипт установки Rust
├── LICENSE                  # MIT лицензия
└── .gitignore               # Что не коммитить в git
```

---

## Редактирование основной логики

Ваша основная задача - редактировать **`src/processor.rs`**:

```rust
pub fn process_data(input: &str) -> Result<String, ProcessError> {
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("Input cannot be empty".to_string()));
    }

    // ✏️ ЗАМЕНИТЕ ЭТУ СТРОКУ НА ВАШУ ЛОГИКУ
    let result = format!("Processed: {}", input.to_uppercase());
    Ok(result)
}
```

**После редактирования:**
1. Сохраните файл (Ctrl+S)
2. Запустите: `cargo test` (проверить тесты)
3. Запустите: `cargo run -- process "test"` (проверить результат)
4. Обновите тест в функции `#[cfg(test)]` если нужно

---

## VS Code интеграция

### Установленные расширения (рекомендуется)
- **rust-analyzer** - Rust языковой сервер
- **CodeLLDB** или **Rust Debugger** - Отладка Rust кода

### Сочетания клавиш
- `Ctrl+Shift+B` - Сборка проекта (запустит `cargo build --release`)
- `Ctrl+Shift+T` - Запустить тесты
- `F5` - Запустить отладчик
- `Ctrl+Shift+X` - Открыть рынок расширений

### Форматирование при сохранении
Включено автоматически в `.vscode/settings.json`:
- Код автоматически форматируется при сохранении (Ctrl+S)
- Используется `rustfmt` для форматирования

---

## Troubleshooting

### ❌ "cargo: command not found"
**Решение:** Rust не установлен. Запустите `install-rust.bat` или установите вручную.

### ❌ "error: could not compile"
**Решение:** 
1. Проверьте синтаксис: `cargo check`
2. Посмотрите сообщение об ошибке внимательнее
3. Убедитесь что используете правильные типы данных

### ❌ "could not find library"
**Решение:**
```bash
# Для Linux/macOS
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./target/release

# Для Windows - скопируйте DLL рядом с .exe
copy target\release\my_cli.dll .
```

### ❌ FFI вызовы вызывают segfault
**Решение:**
1. Убедитесь что `free_string()` вызывается для всех указателей
2. Проверьте что library скомпилирована в release режиме
3. Используйте примеры из `bindings/` как шаблон

### ❌ Медленная сборка
**Решение:**
```bash
# Используйте mold линкер (Linux) или lld (все платформы)
# Добавьте в ~/.cargo/config.toml:
[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### ❌ Большой размер бинарика
**Решение:**
```bash
# Используйте strip для удаления символов
strip target/release/my-cli

# Или добавьте в Cargo.toml:
[profile.release]
strip = true
```

---

## Быстрые команды

```bash
# Полный цикл разработки
cargo fmt && cargo clippy && cargo test && cargo build --release

# Только проверка
cargo check

# Только тесты
cargo test

# Только форматирование
cargo fmt

# Генерировать документацию
cargo doc --no-deps --open
```

---

## Полезные ссылки

- 📚 [Rust Book](https://doc.rust-lang.org/book/)
- 📦 [Crates.io](https://crates.io/) - Реестр пакетов Rust
- 🔗 [FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- ⚙️ [Cargo Reference](https://doc.rust-lang.org/cargo/)
- 🛠️ [Rustlings Exercises](https://github.com/rust-lang/rustlings)

---

## Получение помощи

1. **Ошибка компиляции?**
   - Прочитайте сообщение об ошибке внимательно
   - Гуглите текст ошибки + "rust"
   - Спросите на [r/rust](https://reddit.com/r/rust)

2. **Нужна документация?**
   - `cargo doc --open` - локальная документация
   - [docs.rs](https://docs.rs) - документация пакетов

3. **Проблема с FFI?**
   - Используйте примеры из `bindings/` как шаблон
   - Убедитесь что используете `unsafe` правильно
   - Проверьте memory safety

---

Удачи в разработке! 🦀
