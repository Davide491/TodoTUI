# Nome dell'applicazione e del file installer finale
Name "Todo TUI"
OutFile "dist\TodoTuiInstaller.exe"

# Cartella di installazione predefinita su Windows (C:\Program Files\todo_tui)
InstallDir "$PROGRAMFILES\todo_tui"

# Richiede i privilegi di amministratore su Windows per installare/rimuovere
RequestExecutionLevel admin

# Pagine dell'installer guidato
Page directory
Page instfiles

# Pagine del disinstallatore
UninstPage uninstConfirm
UninstPage instfiles

# --- SEZIONE DI INSTALLAZIONE ---
Section "Installazione Principale"
    SetOutPath $INSTDIR
    
    # Prende il file copiato in dist/ e lo inserisce nella cartella finale
    File "dist\todo_tui.exe"
    
    # Crea il disinstallatore eseguibile dentro la cartella dell'app
    WriteUninstaller "$INSTDIR\uninstall.exe"
    
    # Crea i collegamenti nel Menu Start per l'app e per la disinstallazione
    CreateDirectory "$SMPROGRAMS\Todo TUI"
    CreateShortCut "$SMPROGRAMS\Todo TUI\Todo TUI.lnk" "$INSTDIR\todo_tui.exe"
    CreateShortCut "$SMPROGRAMS\Todo TUI\Disinstalla Todo TUI.lnk" "$INSTDIR\uninstall.exe"
    
    # REGISTRO: Aggiunge la cartella dell'app al PATH dell'utente (fondamentale per le TUI)
    WriteRegExpandStr HKCU "Environment" "Path" "$%Path%;$INSTDIR"
    
    # REGISTRO: Aggiunge l'app alla schermata "Installazione Applicazioni" di Windows
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TodoTUI" "DisplayName" "Todo TUI (Rust)"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TodoTUI" "UninstallString" '"$INSTDIR\uninstall.exe"'
SectionEnd

# --- SEZIONE DI DISINSTALLAZIONE ---
Section "Uninstall"
    # Elimina i file installati e il disinstallatore stesso
    Delete "$INSTDIR\todo_tui.exe"
    Delete "$INSTDIR\uninstall.exe"
    RMDir "$INSTDIR"
    
    # Rimuove le scorciatoie dal Menu Start
    Delete "$SMPROGRAMS\Todo TUI\Todo TUI.lnk"
    Delete "$SMPROGRAMS\Todo TUI\Disinstalla Todo TUI.lnk"
    RMDir "$SMPROGRAMS\Todo TUI"
    
    # Nota: Rimuovere dinamicamente una stringa dal PATH via NSIS puro può essere complesso,
    # ma rimuovendo la chiave di disinstallazione puliamo l'elenco dei programmi di Windows
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TodoTUI"
SectionEnd

