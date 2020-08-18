mod app;
mod imgui_wgpu;
mod nes;

use app::NESApp;
use std::rc::Rc;

fn main() {
    let app = Rc::new(NESApp::new());
    app.run()
}
