use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
// 1. Definiamo come è fatto un singolo Task
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoItem {
    pub title: String,
    pub completed: bool,
}

// NUOVA: Identifica una cartella con un nome e i suoi task dedicati
#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub name: String,
    pub items: Vec<TodoItem>,
}

// AGGIORNATA: La lista principale adesso contiene un vettore di cartelle
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub folders: Vec<Folder>,
    pub current_folder_index: usize, // Tiene traccia di quale cartella stai guardando
}

impl TodoList {
    // Crea una lista vuota
    pub fn new() -> Self {
        Self {
            folders: vec![Folder {
                name: "Generale".to_string(),
                items: Vec::new(),
            }],
            current_folder_index: 0,
        }
    }

    // AGGIUNGI QUESTA FUNZIONE PROPRIO QUI SOTTO:
    pub fn create_folder(&mut self, name: String) {
        let new_folder = Folder {
            name,
            items: Vec::new(),
        };
        self.folders.push(new_folder);
    }

    // Aggiunge un nuovo task alla lista
    pub fn add_item(&mut self, title: String) {
        let new_item = TodoItem {
            title,
            completed: false,
        };
        self.folders[self.current_folder_index].items.push(new_item);
    }

    // Inverte lo stato del task (completato / da fare)
    pub fn toggle_item(&mut self, index: usize) {
        if let Some(item) = self.folders[self.current_folder_index].items.get_mut(index) {
            item.completed = !item.completed;
        }
    }

    // Rimuove un task in base all'indice
    pub fn remove_item(&mut self, index: usize) {
        if index < self.folders[self.current_folder_index].items.len() {
            self.folders[self.current_folder_index].items.remove(index);
        }
    }

    // Salva la lista su un file JSON
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::new());
        }
        let file = File::open(path)?;

        // Se il file JSON è strutturato male o vuoto, restituiamo una lista nuova pulita
        let mut list: Self = serde_json::from_reader(file).unwrap_or_else(|_| Self::new());

        // Controllo di sicurezza estremo: se le cartelle sono comunque 0, ne forziamo una
        if list.folders.is_empty() {
            list.folders.push(Folder {
                name: "Generale".to_string(),
                items: Vec::new(),
            });
            list.current_folder_index = 0;
        }

        Ok(list)
    }

    // Ottieni il percorso del file di salvataggio
    pub fn get_save_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "example", "todo_app") {
            let data_dir = proj_dirs.data_dir();
            std::fs::create_dir_all(data_dir).expect("Failed to create data directory");
            data_dir.join("todos.json")
        } else {
            PathBuf::from("todos.json") // Fallback se non si riesce a ottenere il percorso della directory dati
        }
    }
    pub fn add_item_to_current(&mut self, title: String) {
        if let Some(folder) = self.folders.get_mut(self.current_folder_index) {
            folder.items.push(TodoItem {
                title,
                completed: false,
            });
        }
    }
        // NUOVA: Rimuove la cartella attualmente selezionata (impedendo di rimanere a 0 cartelle)
    pub fn remove_current_folder(&mut self) {
        if self.folders.len() > 1 {
            self.folders.remove(self.current_folder_index);
            // Se eravamo sull'ultima cartella, arretriamo l'indice per evitare out-of-bounds
            if self.current_folder_index >= self.folders.len() {
                self.current_folder_index = self.folders.len() - 1;
            }
        } else {
            // Se è l'ultima cartella rimasta, svuota semplicemente i suoi task invece di eliminarla
            if let Some(folder) = self.folders.get_mut(0) {
                folder.items.clear();
                folder.name = "Generale".to_string();
            }
        }
    }

}
