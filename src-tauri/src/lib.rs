mod app;

use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use app::AppState;
use std::sync::Mutex;

// declre appstate

// static appState: AppState;
static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| Mutex::new(AppState::new()));


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_file_panel_content_as_json,
             set_current_item_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[derive(Serialize, Deserialize, Debug)]
struct Item {
    display_name: String,
    display_ext: String,
    name: String,
    is_dir: bool,
    size: i64,
    modified: i64,
    full_path: String,
    link_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilePanelContent {
    items: Vec<Item>,
    current_item_index: i32,
    panel_index: i32,
    current_path: String,
}

#[tauri::command]
fn get_file_panel_content_as_json(index: i32) -> String {
    APP_STATE.lock().unwrap().get_file_panel_content_as_json(index)
}

#[tauri::command]
fn set_current_item_index(panel_index: i32, index: i32) {
    APP_STATE.lock().unwrap().set_current_item_index(panel_index, index)    
    //APP_STATE.set_current_item_index(panel_index, index)
}
/*

type FilePanelContent struct {
	Items            []*common.Item `json:"items"`
	CurrentItemIndex int            `json:"currentItemIndex"`
	PanelIndex       int            `json:"panelIndex"`
	CurrentPath      string         `json:"currentPath"`
}


func (a App) GetFilePanelContentAsJson(index int) string {
	return a.s.GetFilePanelContentAsJson(index)
}

func (c *App) SetCurrentItemIndex(panelIndex, index int) {
	c.s.SetCurrentItemIndex(panelIndex, index)
}

func (c *App) UpdateContent(panelIndex int) {
	c.s.UpdateContent(panelIndex)
}

func (c *App) MainAction(panelIndex int) {
	c.s.MainAction(panelIndex)
}

func (c *App) GoBack(panelIndex int) {
	c.s.GoBack(panelIndex)
}
*/