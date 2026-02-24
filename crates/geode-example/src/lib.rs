use geode_rs::classes::*;
use geode_rs::*;

#[geode_main]
fn main() {
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
        if !MenuLayer::init(this) {
            return false;
        }

        self.my_custom_field = 67;

        log::debug!("MenuLayer::init hooked from Rust!");

        true
    }
}

#[modify(PlayLayer)]
struct MyPlayLayer {
    print_timer: f32,
}

#[modify(PlayLayer)]
impl MyPlayLayer {
    fn play_layer_ctor_1(&mut self, this: &mut PlayLayer) {
        log::info!("PlayLayer constructor hook!");
        PlayLayer::play_layer_ctor_1(this);
    }

    fn post_update(&mut self, this: &mut PlayLayer, dt: f32) {
        self.print_timer += dt;
        if self.print_timer > 1.0 {
            log::info!("PlayLayer post_update: currentTime {}", this.current_time); // Pretty sure this is actually broken in geode rn tho, the codegen offsets are correct
            self.print_timer = 0.0;
        }
        PlayLayer::post_update(this, dt);
    }
}
