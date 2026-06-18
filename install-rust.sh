#!/usr/bin/env bash
# Скрипт для установки Rust на Linux/macOS

set -e

echo "==========================================="
echo "Rust Installation Helper"
echo "==========================================="
echo ""

# Проверка установлена ли Rust
if command -v cargo &> /dev/null; then
    echo "Rust is already installed:"
    cargo --version
    rustc --version
    exit 0
fi

echo "Installing Rust..."
echo "Please wait..."
echo ""

# Загрузить и запустить установщик
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo ""
echo "==========================================="
echo "Installation complete!"
echo "==========================================="
echo ""
echo "To activate Rust in your current shell, run:"
echo "  source $HOME/.cargo/env"
echo ""
echo "Or close and reopen your terminal."
echo ""
echo "Verify installation:"
echo "  rustc --version"
echo "  cargo --version"
