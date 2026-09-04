import os
import shutil
from pathlib import Path
import sys

def install_windows():
    # Definiamo la cartella globale di sistema C:\Program Files\todo_arch
    dest_dir = Path("C:/Program Files/todo_arch")
    
    try:
        dest_dir.mkdir(parents=True, exist_ok=True)
    except PermissionError:
        print("❌ Errore: Permessi insufficienti!")
        print("Esegui il terminale (CMD o PowerShell) COME AMMINISTRATORE e riprova.")
        sys.exit(1)

    # Percorso del binario cross-compilato su Linux
    src_binary = Path("target") / "x86_64-pc-windows-gnu" / "release" / "todo_tui.exe"
    
    # Fallback se lo lanci direttamente da Windows dopo build locale
    if not src_binary.exists():
        src_binary = Path("target") / "release" / "todo_tui.exe"

    if src_binary.exists():
        dest_binary = dest_dir / "todo_arch.exe"
        try:
            shutil.copy(src_binary, dest_binary)
            print("====================================================")
            print(f"🎉 Installazione GLOBALE completata su Windows!")
            print(f"Eseguibile copiato in: {dest_binary}")
            print("====================================================")
            print("Consiglio: Aggiungi 'C:\\Program Files\\todo_arch' al PATH di sistema.")
        except PermissionError:
            print("❌ Errore di scrittura in Program Files. Assicurati di essere Amministratore.")
    else:
        print("❌ Errore: todo_tui.exe non trovato. Verifica di aver fatto la build.")

if __name__ == "__main__":
    install_windows()

