#!/bin/bash
set -e

echo "[1/4] Pulizia e creazione cartella temporanea (dist)..."
rm -rf dist
mkdir -p dist

echo "[2/4] Cross-compilazione del binario Rust per Windows..."
cargo build --release --target x86_64-pc-windows-gnu

echo "[3/4] Copia del binario Windows in dist/..."
cp target/x86_64-pc-windows-gnu/release/todo_tui.exe dist/

echo "[4/4] Generazione dell'installer .exe nativo con NSIS..."
makensis installer.nsi

echo "--------------------------------------------------"
echo "Fatto! Trovi l'installer pronto in: dist/TodoTuiInstaller.exe"
