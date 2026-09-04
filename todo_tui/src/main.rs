use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use std::io;
use todo_core::TodoList;

enum InputMode {
    Normal,
    Editing,
    FolderEditing,
    FolderDeletingConfirm,
}

enum ActivePane {
    Explorer,
    Tasks,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let save_path = TodoList::get_save_path();
    let mut todo_list = TodoList::load_from_file(&save_path).unwrap_or_else(|_| TodoList::new());

    if todo_list.folders.is_empty() {
        todo_list = TodoList::new();
    }

    let mut folder_state = ListState::default();
    folder_state.select(Some(todo_list.current_folder_index));

    let mut task_state = ListState::default();
    if !todo_list.folders[todo_list.current_folder_index]
        .items
        .is_empty()
    {
        task_state.select(Some(0));
    }

    let mut input_mode = InputMode::Normal;
    let mut active_pane = ActivePane::Tasks;
    let mut input_text = String::new();

    let mut app_result = Ok(());
    loop {
        let draw_result = terminal.draw(|frame| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            let content_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(75),
                ])
                .split(main_layout[0]);

            let folder_items: Vec<ListItem> = todo_list.folders.iter().map(|f| {
                ListItem::new(format!("📁 {}", f.name))
            }).collect();

            let explorer_border_color = match active_pane {
                ActivePane::Explorer => Color::Green,
                _ => Color::White,
            };

            let explorer_widget = List::new(folder_items)
                .block(Block::default()
                    .title(" EXPLORER ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(explorer_border_color)))
                .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            let current_folder = &todo_list.folders[todo_list.current_folder_index];
            let task_items: Vec<ListItem> = current_folder.items.iter().map(|item| {
                let status = if item.completed { "[X]" } else { "[ ]" };
                ListItem::new(format!("{} {}", status, item.title))
            }).collect();

            let tasks_border_color = match active_pane {
                ActivePane::Tasks => Color::Green,
                _ => Color::White,
            };

            let list_title = format!(" CARTELLA: {} ", current_folder.name.to_uppercase());
            let tasks_widget = List::new(task_items)
                .block(Block::default()
                    .title(list_title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(tasks_border_color)))
                .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            let input_block = Block::default()
                .title(match input_mode {
                    InputMode::FolderEditing => " Nuova Cartella ",
                    _ => " Nuovo Task ",
                })
                .borders(Borders::ALL)
                .border_style(match input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                    InputMode::FolderEditing => Style::default().fg(Color::Magenta),
                    InputMode::FolderDeletingConfirm => Style::default().fg(Color::Red),
                });
            let input_widget = Paragraph::new(input_text.as_str()).block(input_block);

            let help_text = match input_mode {
                InputMode::Normal => "←/→: Pannello | ↑/↓: Naviga | Spazio: Fatto | a: Task | c: Cartella | d: Elimina task| Shift+D: Elimina Cartella | q: Esci",
                InputMode::Editing | InputMode::FolderEditing => "Digita... | Invio: Conferma | Esc: Annulla",
                InputMode::FolderDeletingConfirm => "CONFERMA ELIMINAZIONE CARTELLA? Premi 'y' per Sì / 'n' per No",
            };
            let help_widget = Paragraph::new(help_text).block(Block::default().title(" Comandi ").borders(Borders::ALL));

            frame.render_stateful_widget(explorer_widget, content_layout[0], &mut folder_state);
            frame.render_stateful_widget(tasks_widget, content_layout[1], &mut task_state);
            frame.render_widget(input_widget, main_layout[1]);
            frame.render_widget(help_widget, main_layout[2]);

            if let InputMode::FolderDeletingConfirm = input_mode {
                let popup_area = Rect {
                    x: frame.area().width / 4,
                    y: frame.area().height / 3,
                    width: frame.area().width / 2,
                    height: 5,
                };
                let popup_block = Block::default()
                    .title(" ATTENZIONE ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
                let popup_text = Paragraph::new(format!(
                    "\n Vuoi davvero eliminare la cartella '{}'?\n I task al suo interno andranno persi. [y/n]",
                    current_folder.name
                ))
                .block(popup_block)
                .alignment(ratatui::layout::Alignment::Center);

                frame.render_widget(Clear, popup_area);
                frame.render_widget(popup_text, popup_area);
            }
        });

        if let Err(e) = draw_result {
            app_result = Err(e);
            break;
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    let current_folder = &todo_list.folders[todo_list.current_folder_index];
                    let current_index = task_state.selected();

                    match input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => {
                                let _ = todo_list.save_to_file(&save_path);
                                break;
                            }
                            KeyCode::Char('a') => input_mode = InputMode::Editing,
                            KeyCode::Char('c') => input_mode = InputMode::FolderEditing,
                            KeyCode::Char('D') => input_mode = InputMode::FolderDeletingConfirm,

                            KeyCode::Left => active_pane = ActivePane::Explorer,
                            KeyCode::Right => active_pane = ActivePane::Tasks,

                            KeyCode::Down => match active_pane {
                                ActivePane::Explorer => {
                                    if let Some(i) = folder_state.selected() {
                                        if i < todo_list.folders.len() - 1 {
                                            folder_state.select(Some(i + 1));
                                            todo_list.current_folder_index = i + 1;
                                            task_state.select(
                                                if todo_list.folders[i + 1].items.is_empty() {
                                                    None
                                                } else {
                                                    Some(0)
                                                },
                                            );
                                        }
                                    }
                                }
                                ActivePane::Tasks => {
                                    if let Some(i) = current_index {
                                        if i < current_folder.items.len() - 1 {
                                            task_state.select(Some(i + 1));
                                        }
                                    }
                                }
                            },
                            KeyCode::Up => match active_pane {
                                ActivePane::Explorer => {
                                    if let Some(i) = folder_state.selected() {
                                        if i > 0 {
                                            folder_state.select(Some(i - 1));
                                            todo_list.current_folder_index = i - 1;
                                            task_state.select(
                                                if todo_list.folders[i - 1].items.is_empty() {
                                                    None
                                                } else {
                                                    Some(0)
                                                },
                                            );
                                        }
                                    }
                                }
                                ActivePane::Tasks => {
                                    if let Some(i) = current_index {
                                        if i > 0 {
                                            task_state.select(Some(i - 1));
                                        }
                                    }
                                }
                            },
                            KeyCode::Char(' ') => {
                                if let ActivePane::Tasks = active_pane {
                                    if let Some(i) = current_index {
                                        todo_list.toggle_item(i);
                                    }
                                }
                            }
                            KeyCode::Char('d') => {
                                if let ActivePane::Tasks = active_pane {
                                    if let Some(i) = current_index {
                                        todo_list.remove_item(i);
                                        let updated_folder =
                                            &todo_list.folders[todo_list.current_folder_index];
                                        if updated_folder.items.is_empty() {
                                            task_state.select(None);
                                        } else if i >= updated_folder.items.len() {
                                            task_state.select(Some(updated_folder.items.len() - 1));
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                if !input_text.trim().is_empty() {
                                    todo_list.add_item_to_current(input_text.drain(..).collect());
                                    if task_state.selected().is_none() {
                                        task_state.select(Some(0));
                                    }
                                }
                                input_mode = InputMode::Normal;
                            }
                            KeyCode::Esc => {
                                input_text.clear();
                                input_mode = InputMode::Normal;
                            }
                            KeyCode::Char(c) => input_text.push(c),
                            KeyCode::Backspace => {
                                input_text.pop();
                            }
                            _ => {}
                        },
                        InputMode::FolderEditing => match key.code {
                            KeyCode::Enter => {
                                if !input_text.trim().is_empty() {
                                    let folder_name = input_text.drain(..).collect();
                                    todo_list.create_folder(folder_name);
                                    let new_idx = todo_list.folders.len() - 1;
                                    todo_list.current_folder_index = new_idx;
                                    folder_state.select(Some(new_idx));
                                    task_state.select(None);
                                }
                                input_mode = InputMode::Normal;
                            }
                            KeyCode::Esc => {
                                input_text.clear();
                                input_mode = InputMode::Normal;
                            }
                            KeyCode::Char(c) => input_text.push(c),
                            KeyCode::Backspace => {
                                input_text.pop();
                            }
                            _ => {}
                        },
                        InputMode::FolderDeletingConfirm => match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                todo_list.remove_current_folder();
                                folder_state.select(Some(todo_list.current_folder_index));
                                let current_folder =
                                    &todo_list.folders[todo_list.current_folder_index];
                                task_state.select(if current_folder.items.is_empty() {
                                    None
                                } else {
                                    Some(0)
                                });
                                input_mode = InputMode::Normal;
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }
    ratatui::restore();
    app_result
}
