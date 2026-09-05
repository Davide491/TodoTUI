#!/bin/bash

# 1. Controlla i permessi di root
if [ "$EUID" -ne 0 ]; then
  echo "Execute with root privileges (e.g., sudo ./install.sh)"
  exit 1
fi

# 2. Definisci le dipendenze richieste
# Modifica questa lista con i pacchetti necessari al tuo programma
DEPENDENCIES="git"
WORKDIR=/tmp/todo-tui-install
DESTINATION=/usr/local/bin/
mkdir -p $WORKDIR

# 3. Rileva il gestore di pacchetti e installa le dipendenze
echo "Checking for package manager and installing dependencies..."

if command -v apt-get &> /dev/null; then
    apt-get update
    apt-get install -y $DEPENDENCIES
elif command -v dnf &> /dev/null; then
    dnf install -y $DEPENDENCIES
elif command -v pacman &> /dev/null; then
    pacman -S --noconfirm $DEPENDENCIES
else
    echo "Error: Unsupported package manager."
    exit 1
fi

cd $WORKDIR
git clone https://github.com/Davide491/TodoTUI.git
# 4. Copia i file del programma nel sistema
echo "Installazione dei file dell'applicazione..."
# Sostituisci 'mio_eseguibile' con il tuo file reale
cp TodoTUI/target/release/todo_tui $DESTINATION
chmod +x $DESTINATION
rm -rf $WORKDIR
echo "Installation completed. You can run the program using 'todo_tui'."


