use eframe::egui;
use image::RgbaImage;

struct Application {
    texture: Option<egui::TextureHandle>,
    regen: bool,
    scene_bounds: egui::Rect,
}

impl Application {
    fn new() -> Self {
        Self {
            texture: None,
            regen: false,
            scene_bounds: egui::Rect::ZERO,
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let egui_img = egui::ColorImage::example();
        match &mut self.texture {
            Some(texture) => texture.set(egui_img, egui::TextureOptions::default()),
            None => {
                self.texture =
                    Some(ctx.load_texture("goldsrctexture", egui_img, egui::TextureOptions::default()));
            }
        }
    }
}

impl eframe::App for Application {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });

        egui::Panel::left("config_panel").show(ui, |ui| {
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(8.0);
                ui.heading("Image Configuration");
                ui.add_space(16.0);
                if ui.button("Generate").clicked() {
                    self.regen = true;
                }
            });
        });

        if self.regen {
            self.update_texture(ui.ctx());
            self.regen = false;
        }

        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(texture) = &self.texture {
                let image = egui::Image::from_texture(texture).fit_to_original_size(1.0);
                let scene = egui::Scene::new().zoom_range(0.1..=20.0);

                // might use hook later for hover details
                let _hook = scene.show(ui, &mut self.scene_bounds, |ui| {
                    ui.add(image);
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.heading("No Image Generated");
                });
            }
        });
    }
}

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "goldsrctexturecreator",
        options,
        Box::new(|cc| Ok(Box::new(Application::new()))),
    )
}