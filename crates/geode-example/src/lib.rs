use geode_rs::classes::*;
use geode_rs::*;

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
