use imgui::Ui;
use rfd::AsyncFileDialog;
use crate::chip8::Chip8;
impl Chip8 {
    pub async fn draw_menu_bar(&mut self, ui: &mut Ui, running: &mut bool) {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("File") {
                if ui.menu_item("Open ROM") {
                    let file = AsyncFileDialog::new()
                        .add_filter("text", &["ch8"])
                        .pick_file()
                        .await;

                    if let Some(file) = file {
                        let data = file.read().await;
                        self.load(&data[..]);
                    }
                }
                if ui.menu_item("Close ROM") {
                    self.unload();
                }
                if ui.menu_item("Quit") {
                    *running = false;
                }
                menu.end();
            }
            menu_bar.end();
        }
    }
}