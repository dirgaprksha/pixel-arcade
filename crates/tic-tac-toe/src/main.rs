use pixelate::prelude::*;

struct TicTacToe {
    assets: AssetManager,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            assets: AssetManager::new(),
        }
    }
}

impl Application for TicTacToe {
    fn on_render(&mut self, _window: &Window, renderer: &mut Renderer) {
        renderer.draw_rectangle(0, 0, 200, 200, [0, 225, 0, 225]);
    }
}

fn main() {
    let application = TicTacToe::new();
    let window_config = WindowConfiguration {
        title: "Tic Tac Toe".to_string(),
        ..Default::default()
    };

    let _ = Engine::run(application, window_config);
}
