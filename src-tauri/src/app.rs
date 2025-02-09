use serde::{Deserialize, Serialize};


pub struct AppState {
    file_panels: Vec<FilePanel>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            file_panels: vec![
                FilePanel {
                    items: vec![
                        Item {
                            name: "file1".to_string(),
                            display_name: "file1".to_string(),
                            display_ext: "".to_string(),
                            is_dir: false,
                            size: "".to_string(),
                            modified: 123,
                            full_path: "/file1".to_string(),
                            link_path: "/file1".to_string(),
                        },
                        Item {
                            name: "file2".to_string(),
                            display_name: "file2".to_string(),
                            display_ext: "".to_string(),
                            is_dir: false,
                            size: "".to_string(),
                            modified: 123,
                            full_path: "/file2".to_string(),
                            link_path: "/file2".to_string(),
                        },
                    ],
                    current_item_index: 0,
                    panel_index: 0,
                    current_path: "/".to_string(),
                },
                FilePanel {
                    items: vec![
                        Item {
                            name: "file3".to_string(),
                            display_name: "file3".to_string(),
                            display_ext: "".to_string(),
                            is_dir: false,
                            size: "".to_string(),
                            modified: 123,
                            full_path: "/file3".to_string(),
                            link_path: "/file3".to_string(),
                        },
                        Item {
                            name: "file4".to_string(),
                            display_name: "file4".to_string(),
                            display_ext: "".to_string(),
                            is_dir: false,
                            size: "".to_string(),
                            modified: 123,
                            full_path: "/file4".to_string(),
                            link_path: "/file4".to_string(),
                        },
                    ],
                    current_item_index: 0,
                    panel_index: 1,
                    current_path: "/".to_string(),
                },
            ],
        }
    }

    fn load_file_panel_content(&mut self, panel_index: i32) {
        let panel_index_usize = panel_index as usize;
        if panel_index_usize >= self.file_panels.len() {
            return;
        }
        let panel = &mut self.file_panels[panel_index_usize];
        let path = &panel.current_path;
        let items = std::fs::read_dir(path).unwrap();
        panel.items.clear();
        for item in items {
            
            // if it is link then get the link path
            // if it is a file then get the file path
            // if it is a directory then get the directory path

            //let isLink = item.unwrap().file_type().unwrap().is_symlink();


            let item = item.unwrap();
            let metadata = item.metadata().unwrap();
            let is_dir = metadata.is_dir();
            let name = item.file_name().into_string().unwrap();
            let display_name = name.clone();
            let display_ext = "".to_string();
            let size = format_file_size(metadata.len());
            let modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
            let full_path = item.path().to_str().unwrap().to_string();
            let link_path = "".to_string();
            panel.items.push(Item {
                name,
                display_name,
                display_ext,
                is_dir,
                size,
                modified,
                full_path,
                link_path,
            });
        }
    }

    pub fn get_file_panel_content_as_json(&mut self, index: i32) -> String {
        self.load_file_panel_content(index);
        let index_usize = index as usize;
        if index_usize >= self.file_panels.len() {
            return "".to_string();
        }
        serde_json::to_string(&self.file_panels[index_usize]).unwrap()
    }

    pub fn set_current_item_index(&mut self, panel_index: i32, index: i32) {
        let panel_index_usize = panel_index as usize;
        if panel_index_usize >= self.file_panels.len() {
            return;
        }
        let panel = &mut self.file_panels[panel_index_usize];
        if index < 0 || index >= panel.items.len() as i32 {
            return;
        }
        panel.current_item_index = index;
    }



}

pub fn format_file_size( bytes: u64) -> String {
    let result = format_number(&bytes.to_string());
    result
}

pub fn format_number(s: &str) -> String {
    // example: 1234567 -> 1 234 567
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count == 3 {
            result.push(' ');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    result.chars().rev().collect()
}


/*

*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    name: String,
    display_name: String,
    display_ext: String,
    is_dir: bool,
    size: String,
    modified: u64,
    full_path: String,
    link_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePanel {
    items: Vec<Item>,
    current_item_index: i32,
    panel_index: i32,
    current_path: String,
}

