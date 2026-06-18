# Quick Start Guide

## 🚀 Быстрый старт для Rust CLI

### Шаг 1: Установка Rust

Если у вас ещё не установлен Rust:

**Вариант 1: Автоматическая установка (Windows)**
```batch
install-rust.bat
```

**Вариант 2: Ручная установка**
1. Посетите https://rustup.rs/
2. Скачайте установщик для Windows
3. Запустите `rustup-init.exe`
4. Следуйте инструкциям на экране
5. Перезагрузите терминал

**Проверка установки:**
```bash
rustc --version
cargo --version
```

---

### Шаг 2: Сборка проекта

```bash
# Debug версия (быстрая сборка, медленное выполнение)
cargo build

# Release версия (оптимизированная, готова к использованию)
cargo build --release

# Shared library для FFI
cargo build --release --lib
```

---

### Шаг 3: Запуск CLI

```bash
# Обработать данные
cargo run -- process "hello world"

# С JSON выводом
cargo run -- process "hello world" --json

# Эхо сообщение
cargo run -- echo "test"

# Интерактивный режим
cargo run -- interactive
```

---

### Шаг 4: Запуск тестов

```bash
cargo test
```

---

### Шаг 5: Использование из других языков

#### Python
```bash
# Сначала создайте shared library
cargo build --release --lib

# Затем запустите пример
python bindings/python_example.py
```

#### C
```bash
# Сборка библиотеки
cargo build --release --lib

# Компиляция C кода (Linux/macOS)
cd bindings
gcc c_example.c -o c_example -L../target/release -lmy_cli

# На Windows нужно использовать MSVC или MinGW
```

#### C#
```bash
# Убедитесь что shared library собрана
cargo build --release --lib

# Скопируйте DLL рядом с C# кодом
copy target\release\my_cli.dll bindings\

# Скомпилируйте C# проект
csc bindings/csharp_example.cs
```

---

## 📝 Основные команды

| Команда | Назначение |
|---------|-----------|
| `cargo build` | Собрать debug версию |
| `cargo build --release` | Собрать оптимизированную версию |
| `cargo run -- <args>` | Запустить приложение |
| `cargo test` | Запустить тесты |
| `cargo fmt` | Отформатировать код |
| `cargo clippy` | Проверить код на ошибки |
| `cargo doc --open` | Открыть документацию |

---

## 📂 Структура файлов

```
src/
├── main.rs          Точка входа CLI приложения
├── lib.rs           Экспорт модулей
├── processor.rs     Основная логика (ОТРЕДАКТИРУЙТЕ ЗДЕСЬ)
└── ffi.rs           FFI функции для других языков

bindings/           Примеры для других языков
├── python_example.py
├── c_example.c
└── csharp_example.cs

.vscode/            Конфигурация VS Code
├── settings.json   Параметры редактора
├── launch.json     Конфигурация отладки
└── tasks.json      Задачи сборки
```

---

## 🔧 Редактирование основной логики

Основная бизнес-логика находится в `src/processor.rs`:

```rust
pub fn process_data(input: &str) -> Result<String, ProcessError> {
    // Ваша логика здесь
    let result = format!("Processed: {}", input.to_uppercase());
    Ok(result)
}
```

Отредактируйте функцию `process_data()` и ваши изменения будут автоматически:
- Использоваться в CLI
- Доступны для FFI (Python, C#, C)
- Протестированы через `cargo test`

---

## 🐛 Отладка

**В VS Code:**
- Нажмите `F5` для запуска отладки
- Используйте конфигурации из `.vscode/launch.json`

**В терминале:**
```bash
# Запуск с выводом отладки
RUST_BACKTRACE=1 cargo run -- process "test"

# Полная трассировка
RUST_BACKTRACE=full cargo run -- process "test"
```

---

## 📦 Сборка для производства

```bash
# Оптимизированная версия
cargo build --release

# Бинарный файл находится в:
# target/release/my-cli.exe (Windows)
# target/release/my-cli (Linux/macOS)

# Поделитесь бинарным файлом с другими!
```

---

## ❓ Часто встречающиеся ошибки

### `cargo: command not found`
Rust не установлен. Запустите `install-rust.bat` или установите вручную с rustup.rs

### `error: could not compile 'my-cli'`
Проверьте синтаксис кода в `src/processor.rs`. Запустите:
```bash
cargo check
```

### FFI not working
1. Проверьте что library скомпилирована: `cargo build --release --lib`
2. Установите переменные окружения для path к библиотеке
3. На Windows убедитесь что DLL в том же каталоге

---

## 🎯 Следующие шаги

1. ✅ Установите Rust
2. ✅ Отредактируйте `src/processor.rs` с вашей логикой
3. ✅ Протестируйте: `cargo test`
4. ✅ Запустите: `cargo run --release`
5. ✅ Создайте FFI привязки для других языков

---

## 📚 Дополнительные ресурсы

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Clap CLI Framework](https://docs.rs/clap/latest/clap/)
