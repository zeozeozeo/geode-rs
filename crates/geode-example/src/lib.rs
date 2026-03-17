use geode_egui::egui;
use geode_rs::classes::*;
use geode_rs::*;

geode_egui::install_hooks!();

#[geode_main]
fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("geode-rs-example")
            .with_max_level(log::LevelFilter::Debug),
    );

    #[cfg(not(target_os = "android"))]
    simple_logger::SimpleLogger::new().init().unwrap();

    log_settings();

    let mut demos = egui_demo_lib::DemoWindows::default();
    let mut show_demos = false;

    // lets render an egui window ingame:
    geode_egui::set_ui(move |ctx| {
        show_demo_ui(ctx, &mut show_demos, &mut demos);
    });

    log::info!("Example mod loaded from Rust!");
}

#[modify(MenuLayer)]
struct MyMenuLayer {
    my_custom_field: i32,
}

#[modify(MenuLayer)]
impl MyMenuLayer {
    fn init(&mut self, this: &mut MenuLayer) -> bool {
        if !this.init() {
            return false;
        }

        self.my_custom_field = 67;
        log::debug!("MenuLayer::init hooked from Rust!");

        true
    }
}

// Note: by default, #[modify] storage is unique per-hook. This means that even if the game
//       eventually constructs a second PlayLayer, `self` will still point to the same data.
//       If you want to have unique storage per instance of a class, use #[modify(Class, unique)]
//       However, this makes you responsible of hooking a destructor and calling Self::free(this)
//       to free up the storage slot when the class eventually gets destructed.

#[modify(PlayLayer, unique)] // unique storage per constructed instance of PlayLayer
struct MyPlayLayer {
    print_timer: f32,
}

#[modify(PlayLayer, unique)]
impl MyPlayLayer {
    fn play_layer_ctor(&mut self, this: &mut PlayLayer) {
        log::info!("PlayLayer constructor hook!");
        this.play_layer_ctor();
    }

    fn play_layer_dtor(&mut self, this: &mut PlayLayer) {
        Self::free(this); // release the #[modify] storage slot, or this is a memory leak!
        this.play_layer_dtor();
    }

    fn post_update(&mut self, this: &mut PlayLayer, dt: f32) {
        self.print_timer += dt;
        if self.print_timer > 1.0 {
            log::info!("PlayLayer post_update: currentTime {}", this.current_time); // Pretty sure this is actually broken in geode rn tho, the codegen offsets are correct
            self.print_timer = 0.0;
        }
        this.remove_all_checkpoints(); // disallow placing checkpoints
        this.post_update(dt);
    }
}

//
// egui ui below, ignore
//

#[derive(Clone)]
struct SettingRow {
    lookup_key: String,
    resolved_key: String,
    display_name: String,
    description: String,
    enable_if: String,
    should_enable: String,
    requires_restart: String,
}

fn collect_setting_rows(current_mod: &Mod) -> Vec<SettingRow> {
    ["demo-title", "demo-enabled", "demo-scale"]
        .into_iter()
        .map(|key| {
            if let Some(setting) = current_mod.get_setting(key) {
                SettingRow {
                    lookup_key: key.to_owned(),
                    resolved_key: setting.key().unwrap_or_default(),
                    display_name: setting.display_name().unwrap_or_default(),
                    description: setting.description().unwrap_or_default(),
                    enable_if: setting.enable_if().unwrap_or_else(|| "<none>".to_owned()),
                    should_enable: setting.should_enable().to_string(),
                    requires_restart: setting.requires_restart().to_string(),
                }
            } else {
                SettingRow {
                    lookup_key: key.to_owned(),
                    resolved_key: "<lookup failed>".to_owned(),
                    display_name: String::new(),
                    description: String::new(),
                    enable_if: String::new(),
                    should_enable: String::new(),
                    requires_restart: String::new(),
                }
            }
        })
        .collect()
}

fn log_settings() {
    // we can use geode apis:
    let Some(current_mod) = Mod::get() else {
        log::warn!("Settings API demo: Mod::get() returned None");
        return;
    };

    log::info!(
        "Settings API demo: mod={} has_settings={} keys={:?}",
        current_mod.id(),
        current_mod.has_settings(),
        current_mod.setting_keys()
    );

    if let Some(manager) = current_mod.settings_manager() {
        log::info!(
            "Settings API demo: restart_required={}",
            manager.restart_required()
        );

        for key in ["demo-title", "demo-enabled", "demo-scale"] {
            match manager.get(key) {
                Some(setting) => log::info!(
                    "Settings API demo: key={} display_name={:?} description={:?} enable_if={:?} should_enable={} requires_restart={}",
                    setting.key().unwrap_or_default(),
                    setting.name(),
                    setting.description(),
                    setting.enable_if(),
                    setting.should_enable(),
                    setting.requires_restart()
                ),
                None => log::warn!("Settings API demo: missing setting `{key}`"),
            }
        }
    } else {
        log::warn!("Settings API demo: settings_manager() returned None");
    }
}

fn show_demo_ui(
    ctx: &egui::Context,
    show_demos: &mut bool,
    demos: &mut egui_demo_lib::DemoWindows,
) {
    egui::Window::new("geode-egui + geode-rs")
        .default_pos(egui::pos2(24.0, 24.0))
        .default_size((525.0, 305.0))
        .show(ctx, |ui| {
            ui.heading("Hello from Rust");
            ui.label("This window is rendered through cocos + egui_glow, using geode-rs.");
            ui.checkbox(show_demos, "Show demos");

            ui.separator();
            ui.heading("Mod settings");

            if let Some(current_mod) = Mod::get() {
                ui.label(format!("Mod ID: {}", current_mod.id()));
                ui.label(format!("Has settings: {}", current_mod.has_settings()));
                ui.label(format!("Setting keys: {:?}", current_mod.setting_keys()));

                if let Some(manager) = current_mod.settings_manager() {
                    ui.separator();
                    ui.label(format!(
                        "Manager restart_required: {}",
                        manager.restart_required()
                    ));
                    ui.label(format!(
                        "Manager.get(\"demo-enabled\") succeeded: {}",
                        manager.get("demo-enabled").is_some()
                    ));
                }

                let rows = collect_setting_rows(current_mod);
                ui.separator();

                egui::ScrollArea::both().show(ui, |ui| {
                    egui_extras::TableBuilder::new(ui)
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Lookup");
                            });
                            header.col(|ui| {
                                ui.strong("Key");
                            });
                            header.col(|ui| {
                                ui.strong("Name");
                            });
                            header.col(|ui| {
                                ui.strong("Description");
                            });
                            header.col(|ui| {
                                ui.strong("Enable-if");
                            });
                            header.col(|ui| {
                                ui.strong("Enabled");
                            });
                            header.col(|ui| {
                                ui.strong("Restart");
                            });
                        })
                        .body(|mut body| {
                            for row in rows {
                                body.row(20.0, |mut row_ui| {
                                    row_ui.col(|ui| {
                                        ui.label(row.lookup_key);
                                    });
                                    row_ui.col(|ui| {
                                        ui.label(row.resolved_key);
                                    });
                                    row_ui.col(|ui| {
                                        ui.label(row.display_name);
                                    });
                                    row_ui.col(|ui| {
                                        ui.label(row.description);
                                    });
                                    row_ui.col(|ui| {
                                        ui.label(row.enable_if);
                                    });
                                    row_ui.col(|ui| {
                                        ui.label(row.should_enable);
                                    });
                                    row_ui.col(|ui| {
                                        ui.label(row.requires_restart);
                                    });
                                });
                            }
                        })
                });
            } else {
                ui.label("Mod::get() unavailable");
            }
        });

    if *show_demos {
        demos.ui(ctx);
    }
}
