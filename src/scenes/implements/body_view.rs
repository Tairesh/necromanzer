use app::App;
use colors::Colors;
use game::human::body::Freshness;
use game::map::item::{Item, ItemView};
use game::World;
use geometry::Vec2;
use scenes::scene_impl::SceneImpl;
use scenes::transition::SomeTransitions;
use scenes::{back_btn, easy_back};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::{Context, Event};
use ui::alert::Alert;
use ui::label::{ItemDisplay, Label};
use ui::meshy::HoverableMesh;
use ui::position::{Horizontal, Position, Vertical};
use ui::traits::Positionate;
use ui::{BunchOfSprites, SomeSprites};

pub struct BodyView {
    sprites: BunchOfSprites,
    world: Rc<RefCell<World>>,
    alert: Rc<RefCell<Alert>>,
    window_size: (i32, i32),
}

impl BodyView {
    pub fn new(unit_id: usize, app: &App, ctx: &mut Context) -> Self {
        let world = app.world.as_ref().unwrap().borrow();
        let avatar = world.get_unit(unit_id);
        let window_size = app.window_size;
        let alert = Rc::new(RefCell::new(Alert::new(
            window_size.0 as f32,
            window_size.1 as f32,
            app.assets.alert.clone(),
            Position::by_left_top(0.0, 0.0),
        )));
        let back_btn = back_btn(
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowBottomByBottom { offset: -20.0 },
            },
            &app.assets,
        );
        let mut name = Label::new(
            avatar.character.name.clone(),
            app.assets.fonts.header2.clone(),
            Colors::LIGHT_GOLDEN_ROD_YELLOW,
            Position::by_left_top(20.0, 20.0),
        );
        let name_size = name.calc_size(ctx);
        let gender = Label::new(
            format!(
                "({}, {})",
                avatar.character.age_name(),
                avatar.character.age
            ),
            app.assets.fonts.header2.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(30.0 + name_size.x, 20.0),
        );
        let wear = Label::new(
            "Wear:",
            app.assets.fonts.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::by_right_top(-20.0, 50.0),
        );
        let body = Label::new(
            "Body:",
            app.assets.fonts.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::by_left_top(20.0, 50.0),
        );
        let mut sprites: BunchOfSprites = Vec::with_capacity(avatar.body.wear.len() + 4);
        sprites.push(alert.clone());
        sprites.push(Rc::new(RefCell::new(name)));
        sprites.push(Rc::new(RefCell::new(gender)));
        sprites.push(Rc::new(RefCell::new(wear)));
        sprites.push(Rc::new(RefCell::new(body)));
        sprites.push(back_btn);
        let mut y = 0;
        avatar.body.wear.iter().for_each(|i| {
            y += 35;
            let mut disp = ItemDisplay::new(
                Some(i),
                app.assets.fonts.default2.clone(),
                Colors::LIGHT_GRAY,
                app.assets.tileset.clone(),
                Vec2::new(2.0, 2.0),
                Position::by_right_top(-20.0, 60.0 + y as f32),
            );
            let size = disp.calc_size(ctx) + Vec2::new(10.0, 8.0);
            sprites.push(Rc::new(RefCell::new(HoverableMesh::new(
                Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(0.0, 0.0, size.x, size.y),
                )
                .unwrap(),
                Colors::TRANSPARENT,
                Colors::WHITE_SMOKE.with_alpha(0.2),
                size,
                Position::by_right_top(-15.0, 57.0 + y as f32),
            ))));
            sprites.push(Rc::new(RefCell::new(disp)));
        });
        let mut y = 0;
        avatar.body.parts.iter().for_each(|(key, item)| {
            y += 35;
            let mut name = key.clone();
            name.push(':');
            sprites.push(Rc::new(RefCell::new(Label::new(
                name,
                app.assets.fonts.default2.clone(),
                Colors::LIGHT_GRAY,
                Position::by_left_top(20.0, 60.0 + y as f32),
            ))));
            let color = if let Item::BodyPart(bp) = item {
                match bp.data.freshness {
                    Freshness::Fresh => Colors::LIGHT_PINK,
                    Freshness::Rotten => Colors::LIME_GREEN,
                    Freshness::Skeletal => Colors::WARM_IVORY,
                }
            } else {
                Colors::LIGHT_CORAL
            };
            let mut bp = Label::new(
                item.name(),
                app.assets.fonts.default2.clone(),
                color,
                Position::by_left_top(170.0, 60.0 + y as f32),
            );
            let size = bp.calc_size(ctx) + Vec2::new(10.0, 6.0);
            sprites.push(Rc::new(RefCell::new(HoverableMesh::new(
                Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(0.0, 0.0, size.x, size.y),
                )
                .unwrap(),
                Colors::TRANSPARENT,
                Colors::WHITE_SMOKE.with_alpha(0.2),
                size,
                Position::by_left_top(165.0, 57.0 + y as f32),
            ))));
            sprites.push(Rc::new(RefCell::new(bp)));
        });
        Self {
            world: app.clone_world(),
            sprites,
            alert,
            window_size,
        }
    }
}

impl SceneImpl for BodyView {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(event, false)
    }

    fn on_resize(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        self.alert.borrow_mut().set_size(ctx, window_size);
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }
}
