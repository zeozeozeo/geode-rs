use std::sync::atomic::{AtomicBool, Ordering};

use geode_egui::egui;
use geode_rs::classes::PlayLayer;
use geode_rs::classes::{
    CCDirector, CCLabelBMFont, CCLayer, CCLayerColor, CCMenu, CCMenuItemFont, MenuLayer,
};
use geode_rs::inherit::Obj;
use geode_rs::loader::Mod;
use geode_rs::types::{CCPoint, ccColor4B};
use geode_rs::{geode_main, inherit, modify, spr};

geode_egui::install_hooks!();

static RUST_POPUP_SHOWN: AtomicBool = AtomicBool::new(false);

#[inherit(CCLayer)]
struct RustPopup {}

impl RustPopup {
    fn create() -> Obj<Self> {
        let mut popup =
            Obj::from_non_null(Self::try_alloc_uninit().expect("failed to allocate RustPopup"));
        popup.cc_layer_ctor();
        Self::init_fields(popup.as_non_null());
        assert!(popup.init_popup(), "RustPopup::init_popup failed");
        Self::autorelease(popup.as_non_null());
        popup
    }

    fn init_popup(&mut self) -> bool {
        if !self.init() {
            return false;
        }

        let background_color = ccColor4B {
            r: 0,
            g: 0,
            b: 0,
            a: 170,
        };

        let mut background = CCLayerColor::create_2(&background_color, 310.0, 170.0);
        let mut title = CCLabelBMFont::create_1("Rust Cocos Popup", "bigFont.fnt");
        let mut body =
            CCLabelBMFont::create_1("A Cocos class inherited and built in Rust.", "bigFont.fnt");
        let mut menu = CCMenu::create();
        let mut close = CCMenuItemFont::create("OK");

        background.set_position(&CCPoint {
            x: -155.0,
            y: -85.0,
        });
        title.set_position(&CCPoint { x: 0.0, y: 40.0 });
        body.set_position(&CCPoint { x: 0.0, y: 8.0 });
        menu.set_position(&CCPoint { x: 0.0, y: 0.0 });
        close.set_position(&CCPoint { x: 0.0, y: -62.0 });

        title.set_scale(0.65);
        body.set_scale(0.38);
        close.set_scale(0.8);

        self.add_child(&mut background);
        self.add_child(&mut title);
        self.add_child(&mut body);
        self.add_child(&mut menu);
        menu.add_child(&mut close);

        true
    }
}

fn show_rust_popup_once(parent: &mut MenuLayer) {
    if RUST_POPUP_SHOWN.load(Ordering::SeqCst) {
        return;
    }

    let mut popup = RustPopup::create();
    let mut director = CCDirector::shared_director();
    let size = director.get_win_size();
    popup.set_position(&CCPoint {
        x: size.width * 0.5,
        y: size.height * 0.5,
    });
    parent.add_child_2(&mut popup, 1000, 0);

    RUST_POPUP_SHOWN.store(true, Ordering::SeqCst);
}

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
    let _sprite_name = spr!("MySprite.png");

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
        show_rust_popup_once(this);

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
        egui::Window::new("egui demo")
            .default_pos(egui::pos2(560.0, 24.0))
            .default_size((520.0, 420.0))
            .show(ctx, |ui| demos.ui(ui));
    }
}
