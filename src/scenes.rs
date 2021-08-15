use engine::EngineContext;
use scene_manager::{CallResult, SpritesData};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sprite::{Button, ButtonState, ImgSprite, TextSprite};
use {colors, VERSION};

#[enum_dispatch::enum_dispatch]
pub trait SceneT {
    fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult;
    fn button_click(&mut self, button_id: &str) -> Option<CallResult>;
    fn on_open(&mut self, context: &mut EngineContext);
    fn create_sprites(&mut self, context: &mut EngineContext) -> Option<SpritesData>;
    fn on_resize(&mut self, context: &mut EngineContext);
    fn on_update(&mut self, context: &mut EngineContext, elapsed_dime: f64);
}

#[derive(Hash, Eq, PartialEq)]
pub struct MainMenu;
impl SceneT for MainMenu {
    fn call(&mut self, _context: &mut EngineContext, _event: &Event) -> CallResult {
        CallResult::DoNothing
    }

    fn button_click(&mut self, button_id: &str) -> Option<CallResult> {
        match button_id {
            "exit" => Some(CallResult::SystemExit),
            "settings" => Some(CallResult::ChangeScene("empty_screen".to_ascii_lowercase())),
            _ => None,
        }
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, context: &mut EngineContext) -> Option<SpritesData> {
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        let logo_size = context.sprite_manager.image_size("res/img/logo.png");
        let version_size = context.sprite_manager.text_size(&*VERSION);
        let button_size = |text: &str| (context.sprite_manager.text_size(text).0 + 20, 30);
        let load_button_text = "[l] Load world";
        let load_button_size = button_size(load_button_text);
        let create_button_text = "[c] Create new world";
        let create_button_size = button_size(create_button_text);
        let settings_button_text = "[s] Settings";
        let settings_button_size = button_size(settings_button_text);
        let exit_button_text = "[x] Exit";
        let exit_button_size = button_size(exit_button_text);
        Some(SpritesData {
            img_sprites: vec![
                ImgSprite {
                    path: "res/img/bg.jpg".to_string(),
                    position: (
                        screen_center.0 - bg_size.0 as i32 / 2,
                        screen_center.1 - bg_size.1 as i32 / 2,
                    ),
                },
                ImgSprite {
                    path: "res/img/logo.png".to_string(),
                    position: (screen_center.0 - logo_size.0 as i32 / 2, 10),
                },
            ],
            text_sprites: vec![TextSprite {
                text: (*VERSION).to_string(),
                color: None,
                position: (
                    screen_center.0 + logo_size.0 as i32 / 2 - version_size.0 as i32 - 20,
                    logo_size.1 as i32 + 10,
                ),
            }],
            buttons: vec![
                Button {
                    id: "load_world".to_ascii_lowercase(),
                    key: Scancode::L,
                    text: load_button_text.to_string(),
                    size: load_button_size,
                    position: (screen_center.0 - load_button_size.0 as i32 / 2, 300),
                    state: ButtonState::Disabled,
                },
                Button {
                    id: "create_world".to_ascii_lowercase(),
                    key: Scancode::C,
                    text: create_button_text.to_string(),
                    size: create_button_size,
                    position: (screen_center.0 - create_button_size.0 as i32 / 2, 340),
                    state: ButtonState::Default,
                },
                Button {
                    id: "settings".to_ascii_lowercase(),
                    key: Scancode::S,
                    text: settings_button_text.to_string(),
                    size: settings_button_size,
                    position: (screen_center.0 - settings_button_size.0 as i32 / 2, 380),
                    state: ButtonState::Default,
                },
                Button {
                    id: "exit".to_ascii_lowercase(),
                    key: Scancode::X,
                    text: exit_button_text.to_string(),
                    size: exit_button_size,
                    position: (screen_center.0 - exit_button_size.0 as i32 / 2, 420),
                    state: ButtonState::Default,
                },
            ],
        })
    }

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, _context: &mut EngineContext, _elapsed_dime: f64) {}
}

#[derive(Hash, Eq, PartialEq)]
pub struct EmptyScreen;
impl SceneT for EmptyScreen {
    fn call(&mut self, _context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            }
            | Event::KeyDown {
                scancode: Some(Scancode::Escape),
                ..
            } => CallResult::ChangeScene("main_menu".to_ascii_lowercase()),
            _ => CallResult::DoNothing,
        }
    }

    fn button_click(&mut self, _button_id: &str) -> Option<CallResult> {
        None
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, _context: &mut EngineContext) -> Option<SpritesData> {
        None
    }

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, context: &mut EngineContext, _elapsed_dime: f64) {
        context
            .canvas
            .set_draw_color(colors::rgb(colors::DARK_SEA_GREEN));
        context.canvas.clear();
    }
}

#[enum_dispatch::enum_dispatch(SceneT)]
#[derive(Hash, Eq, PartialEq)]
pub enum Scene {
    MainMenu,
    EmptyScreen,
}