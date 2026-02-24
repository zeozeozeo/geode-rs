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
    fn init(&mut self, this: *mut MenuLayer) -> bool {
        unsafe {
            if !MenuLayer::init(this) {
                return false;
            }

            self.my_custom_field = 67;

            log::debug!("MenuLayer::init hooked from Rust!");

            true
        }
    }
}

#[modify(PlayLayer)]
struct MyPlayLayer {}

#[modify(PlayLayer)]
impl MyPlayLayer {
    fn play_layer_ctor_1(&mut self, this: *mut PlayLayer) {
        log::info!("PlayLayer constructor hook!");
        unsafe { PlayLayer::play_layer_ctor_1(this) };
    }
}
