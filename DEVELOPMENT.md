# Гайд для разработчиков

## 🚀 Начало разработки

### 1. Клонирование и подготовка
```bash
cd /path/to/my-cli
cargo build          # Собрать проект
cargo test           # Запустить тесты
cargo run -- process "hello"  # Тестовый запуск
```

### 2. Понимание архитектуры

```
Входные данные (CLI/FFI)
        ↓
    main.rs (обработка команд)
        ↓
    processor.rs (основная логика) ← ✏️ РЕДАКТИРУЙТЕ ЗДЕСЬ
        ↓
    Вывод (JSON/текст или в FFI)
```

---

## 📝 Структура кода

### `src/main.rs`
- **Назначение:** Точка входа, обработка CLI команд
- **Не редактируйте:** Структура команд `Commands` enum
- **Когда редактировать:** Добавление новых команд (см. ниже)

### `src/lib.rs`
- **Назначение:** Экспорт модулей для использования как библиотеки
- **Не меняйте:** Оставьте как есть для FFI

### `src/processor.rs` ✏️ ОСНОВНОЙ ФАЙЛ ДЛЯ РЕДАКТИРОВАНИЯ
- **Назначение:** Основная бизнес-логика
- **Функция:** `process_data(input: &str) -> Result<String, ProcessError>`
- **Возвращает:** `Result<String, ProcessError>`

**Пример реализации:**
```rust
pub fn process_data(input: &str) -> Result<String, ProcessError> {
    // Валидация
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("Empty input".to_string()));
    }

    // Ваша логика здесь
    let result = transform_data(input)?;

    Ok(result)
}

fn transform_data(input: &str) -> Result<String, ProcessError> {
    // Вспомогательные функции
    todo!()
}
```

### `src/ffi.rs`
- **Назначение:** FFI функции для других языков
- **Не редактируйте:** Автоматическое преобразование из `process_data()`
- **Используется:** Python, C#, C и другими языками

---

## 🔄 Рабочий процесс разработки

### Шаг 1: Добавление новой функции

```rust
// src/processor.rs
pub fn new_feature(input: &str) -> Result<String, ProcessError> {
    // Реализация
    Ok(format!("Feature result: {}", input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        let result = new_feature("test").unwrap();
        assert!(result.contains("Feature result"));
    }
}
```

### Шаг 2: Тестирование локально

```bash
# Только ваш тест
cargo test new_feature

# Все тесты
cargo test

# С выводом
cargo test -- --nocapture
```

### Шаг 3: Проверка кода

```bash
# Проверка без сборки
cargo check

# Форматирование
cargo fmt

# Linting
cargo clippy
```

### Шаг 4: Сборка release версии

```bash
cargo build --release
# Результат: target/release/my-cli
```

---

## 🆕 Добавление новых команд CLI

Если нужна новая команда (например, `analyze`):

### 1. Добавить в `Commands` enum в `src/main.rs`

```rust
#[derive(Subcommand)]
enum Commands {
    Process {
        #[arg(value_name = "DATA")]
        data: String,
    },
    
    // НОВАЯ КОМАНДА ↓
    Analyze {
        #[arg(value_name = "FILE")]
        file: String,

        #[arg(short, long)]
        verbose: bool,
    },
}
```

### 2. Добавить обработчик в `main()` функции

```rust
fn main() {
    let args = Args::parse();
    
    match args.command {
        Commands::Process { data } => {
            // ... существующий код
        }

        // НОВЫЙ ОБРАБОТЧИК ↓
        Commands::Analyze { file, verbose } => {
            match process_analyze(&file, verbose) {
                Ok(result) => {
                    if args.json {
                        println!("{}", json!({ "status": "success", "result": result }));
                    } else {
                        println!("Analysis:\n{}", result);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
```

### 3. Реализовать логику в `src/processor.rs`

```rust
pub fn process_analyze(file: &str, verbose: bool) -> Result<String, ProcessError> {
    // Валидация
    if file.is_empty() {
        return Err(ProcessError::InvalidInput("File path cannot be empty".to_string()));
    }

    // Логика анализа
    if verbose {
        println!("Analyzing file: {}", file);
    }

    let result = format!("Analysis of {}", file);
    Ok(result)
}
```

### 4. Добавить тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_valid_file() {
        let result = process_analyze("test.txt", false).unwrap();
        assert!(result.contains("Analysis"));
    }

    #[test]
    fn test_analyze_empty_file() {
        let result = process_analyze("", false);
        assert!(result.is_err());
    }
}
```

---

## 🧪 Тестирование

### Структура тестов

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        // Обычный сценарий
        let result = process_data("valid input").unwrap();
        assert_eq!(result, "expected output");
    }

    #[test]
    fn test_error_case() {
        // Сценарий с ошибкой
        let result = process_data("");
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_panic_case() {
        // Сценарий, который должен вызвать паник
        panic!("Expected behavior");
    }
}
```

### Запуск тестов

```bash
# Все тесты
cargo test

# Один тест
cargo test test_happy_path

# С выводом println!
cargo test -- --nocapture

# Параллельно (обычно)
cargo test -- --test-threads=1  # Последовательно

# Игнорируемые тесты
cargo test -- --ignored
```

---

## 📦 Зависимости

### Текущие зависимости

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # CLI аргументы
serde = { version = "1.0", features = ["derive"] }  # Сериализация
serde_json = "1.0"                                   # JSON
thiserror = "1.0"                                    # Error handling
```

### Добавление новой зависимости

```bash
# Добавить из командной строки
cargo add rand --version "^0.8"

# Или редактировать Cargo.toml вручную
[dependencies]
rand = "0.8"
```

---

## 🐛 Отладка

### Debug печать

```rust
// Простой вывод
println!("Value: {}", x);

// С форматированием
println!("Debug: {:#?}", complex_struct);

// Условный debug
if std::env::var("DEBUG").is_ok() {
    eprintln!("Debug output");
}
```

### Запуск с backtrace

```bash
RUST_BACKTRACE=1 cargo run -- process "test"
RUST_BACKTRACE=full cargo run -- process "test"
```

### Использование dbg! макроса

```rust
let x = 42;
dbg!(x);  // Выведет: [src/processor.rs:10] x = 42

let result = expensive_operation();
let x = dbg!(process_data(result)?);
```

---

## 📈 Оптимизация

### Профиль для разработки

```bash
# Debug build - быстро, но медленно выполняется
cargo build

# Запуск: slow
cargo run
```

### Профиль для release

```bash
# Release - медленно собирается, но быстро выполняется
cargo build --release

# Запуск: fast
cargo run --release
```

### Дополнительные флаги оптимизации

В `Cargo.toml`:
```toml
[profile.release]
opt-level = 3           # Максимальная оптимизация
lto = true              # Link-time optimization
codegen-units = 1       # Лучшая оптимизация (медленнее)
strip = true            # Удалить символы отладки
```

---

## 🔐 Error Handling

### Определение ошибок

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Использование ошибок

```rust
pub fn process_data(input: &str) -> Result<String, ProcessError> {
    // Возвращение ошибки
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("Cannot be empty".to_string()));
    }

    // Оператор ? автоматически преобразует ошибку
    let file_content = std::fs::read_to_string(input)?;

    // Преобразование ошибки
    let number: i32 = input
        .parse()
        .map_err(|_| ProcessError::InvalidInput("Not a number".to_string()))?;

    Ok(format!("Processed: {}", number))
}
```

---

## 📚 Документация кода

### Документирование функций

```rust
/// Обрабатывает входные данные и возвращает результат.
///
/// # Arguments
/// * `input` - Входная строка для обработки
///
/// # Returns
/// * `Ok(String)` - Обработанные данные
/// * `Err(ProcessError)` - Описание ошибки
///
/// # Example
/// ```
/// use my_cli::processor::process_data;
///
/// let result = process_data("hello").unwrap();
/// assert!(!result.is_empty());
/// ```
///
/// # Panics
/// Паникует если... (или может не паниковать)
pub fn process_data(input: &str) -> Result<String, ProcessError> {
    // Реализация
}
```

### Генерирование документации

```bash
# Генерировать и открыть в браузере
cargo doc --open

# Без зависимостей
cargo doc --no-deps --open

# Только для вашего проекта
cargo doc --lib --open
```

---

## 🚀 Continuous Integration

### GitHub Actions (пример)

```yaml
name: Rust

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo build --release
```

---

## ✨ Лучшие практики

1. **Валидируйте входные данные** - Проверяйте input в начале функции
2. **Используйте Result** - Не паникуйте, возвращайте ошибки
3. **Пишите тесты** - Каждая функция должна иметь тест
4. **Форматируйте код** - `cargo fmt` перед commit
5. **Проверяйте linter** - `cargo clippy` перед push
6. **Обновляйте документацию** - Коммитируйте вместе с кодом
7. **Используйте типы** - Rust типы помогают избежать ошибок
8. **Избегайте unsafe** - Используйте только когда необходимо (в FFI)

---

## 🔗 Полезные команды

```bash
# Полный цикл разработки
cargo fmt && cargo clippy && cargo test && cargo build --release

# Проверка перед push
cargo fmt --check && cargo clippy && cargo test

# Профилирование (cargo flamegraph)
cargo install flamegraph
cargo flamegraph -- process "data"

# Анализ бинарика (cargo bloat)
cargo install cargo-bloat
cargo bloat --release

# Проверка безопасности (cargo audit)
cargo install cargo-audit
cargo audit
```

---

Удачи в разработке! 🦀
