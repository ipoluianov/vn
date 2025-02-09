use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_file_panel_content_as_json])
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
    let file_panel_content = FilePanelContent {
        items: vec![
            Item {
                name: "file1".to_string(),
                display_name: "file1".to_string(),
                display_ext: "".to_string(),
                is_dir: false,
                size: 123,
                modified: 123,
                full_path: "/file1".to_string(),
                link_path: "/file1".to_string(),
            },
            Item {
                name: "file2".to_string(),
                display_name: "file2".to_string(),
                display_ext: "".to_string(),
                is_dir: false,
                size: 123,
                modified: 123,
                full_path: "/file2".to_string(),
                link_path: "/file2".to_string(),
            },
        ],
        current_item_index: 0,
        panel_index: index,
        current_path: "/".to_string(),
    };
    serde_json::to_string(&file_panel_content).unwrap()
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