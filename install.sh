#!/bin/bash

# Target di sistema globale per i binari locali
DEST_DIR="/usr/local/bin"

echo "🔧 Compilazione della versione release nativa per Linux..."
cargo build --release -p todo_tui

BINARY_SRC="target/release/todo_tui"

if [ -f "$BINARY_SRC" ]; then
    echo "🔐 Richiesta privilegi di amministratore per installare in $DEST_DIR..."
    
    # Copia il file in /usr/local/bin usando sudo
    sudo cp "$BINARY_SRC" "$DEST_DIR/todo_arch"
    sudo chmod +x "$DEST_DIR/todo_arch"
    
    echo "===================================================="
    echo "🎉 Installazione GLOBALE completata su Linux!"
    echo "Eseguibile installato in: $DEST_DIR/todo_arch"
    echo "===================================================="
    echo "Ora tutti gli utenti possono lanciarlo scrivendo: todo_arch"
else
    echo "❌ Errore: Compilazione fallita o file non trovato."
fi

