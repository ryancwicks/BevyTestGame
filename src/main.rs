use bevy::prelude::*;

const BOARD_WIDTH: u32 = 75;
const BOARD_HEIGHT: u32 = 20;

#[derive(Bundle)]
struct Element{
    #[bundle]
    text: Text2dBundle,
    rectangle: Rectangle,
    intensity: Intensity
}

#[derive(Component)]
struct Intensity(f32);

#[derive(Component)]
struct RippleTimer{timer: Timer}

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct RippleEvent {
    pos: Position,
    timer: RippleTimer
}

impl RippleEvent {
    fn value_at_pos_time (&self, x: f32, y: f32, time_s: f32) -> f32 {
        let r2 = (self.pos.x - x).powf(2.0) + (self.pos.y-y).powf(2.0) ;
        100.0* (-0.5*time_s).exp() * (4.0*time_s).sin()/(r2/10.0+0.1).sqrt()
    }
}

#[derive(Component)]
struct Rectangle { //Center point rectange around the text. 
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl Rectangle {
    fn is_inside(&self, x: f32, y: f32, cx: f32, cy: f32) -> bool {
        ((x - cx) > (self.x - self.width/2.0)) && 
           ((x - cx) < (self.x + self.width/2.0 )) && 
           ((y - cy) > (self.y - self.height/2.0)) &&
           ((y - cy) < (self.y + self.height/2.0)) 
    }
}

enum ElementValue {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Central,
}

impl ElementValue {
    fn value(&self) -> String {
        match self {
            ElementValue::Left => "\u{2551}",
            ElementValue::Right => "\u{2551}",
            ElementValue::Top => "\u{2550}",
            ElementValue::Bottom => "\u{2550}",
            ElementValue::TopLeft => "\u{2554}",
            ElementValue::TopRight => "\u{2557}",
            ElementValue::BottomLeft => "\u{255A}",
            ElementValue::BottomRight => "\u{255D}",
            ElementValue::Central => "0"
        }.to_string()
    }
}

fn get_position(x: u32, y: u32, width: f32, height: f32) -> (f32, f32, f32) {
    let elem_width = width/BOARD_WIDTH as f32;
    let elem_height = height/BOARD_HEIGHT as f32;

    (x as f32 * elem_width - width/2.0 + elem_width/2.0, y as f32 * elem_height - height/2.0 + elem_height/2.0, 1.0)
}

fn add_board(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<Windows>) {
    let window = window.get_primary().unwrap();

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font,
        font_size: 15.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //Corners
    let (x, y, z) = get_position(0, BOARD_HEIGHT-1, window.width(), window.height());
    commands.spawn_bundle(Text2dBundle{
        text: Text::with_section(ElementValue::TopLeft.value(), text_style.clone(), text_alignment),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    });
    let (x, y, z) = get_position(BOARD_WIDTH-1, BOARD_HEIGHT-1, window.width(), window.height());
    commands.spawn_bundle(Text2dBundle{
        text: Text::with_section(ElementValue::TopRight.value(), text_style.clone(), text_alignment),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    });
    let (x, y, z) = get_position(0, 0, window.width(), window.height());
    commands.spawn_bundle(Text2dBundle{
        text: Text::with_section(ElementValue::BottomLeft.value(), text_style.clone(), text_alignment),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    });
    let (x, y, z) = get_position(BOARD_WIDTH-1, 0, window.width(), window.height());
    commands.spawn_bundle(Text2dBundle{
        text: Text::with_section(ElementValue::BottomRight.value(), text_style.clone(), text_alignment),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    });

    //Sides
    for x_i in 1..BOARD_WIDTH-1 {
        let (x, y, z) = get_position(x_i, 0, window.width(), window.height());
        commands.spawn_bundle(Text2dBundle{
            text: Text::with_section(ElementValue::Bottom.value(), text_style.clone(), text_alignment),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }); 
        let (x, y, z) = get_position(x_i, BOARD_HEIGHT-1, window.width(), window.height());
        commands.spawn_bundle(Text2dBundle{
            text: Text::with_section(ElementValue::Top.value(), text_style.clone(), text_alignment),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }); 
    }
    for y_i in 1..BOARD_HEIGHT-1 {
        let (x, y, z) = get_position(0, y_i, window.width(), window.height());
        commands.spawn_bundle(Text2dBundle{
            text: Text::with_section(ElementValue::Left.value(), text_style.clone(), text_alignment),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }); 
        let (x, y, z) = get_position(BOARD_WIDTH-1, y_i, window.width(), window.height());
        commands.spawn_bundle(Text2dBundle{
            text: Text::with_section(ElementValue::Right.value(), text_style.clone(), text_alignment),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }); 
    }

    let width = window.width()/BOARD_WIDTH as f32;
    let height = window.height()/BOARD_HEIGHT as f32;
    for x_i in 1..BOARD_WIDTH-1 {
        for y_i in 1..BOARD_HEIGHT-1 {
            let (x, y, z) = get_position(x_i, y_i, window.width(), window.height());
            commands.spawn_bundle(Element {
                text: Text2dBundle{
                    text: Text::with_section(ElementValue::Central.value(), text_style.clone(), text_alignment),
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                },
                rectangle: Rectangle {x:x, y:y, width: width, height:height},
                intensity: Intensity(0.0),
            });
        }
    }
}

// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(mut commands: Commands, mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>, mut query: Query<(&mut Text, &Rectangle, &mut Intensity)> ) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(vec) = win.cursor_position() {
            for (_text, rect, _intensity) in query.iter_mut() {
                if rect.is_inside(vec.x, vec.y, win.width()/2.0, win.height()/2.0){
                    commands.spawn().insert(RippleEvent {
                        pos: Position{x: vec.x-win.width()/2.0, y: vec.y-win.height()/2.0},
                        timer: RippleTimer{timer: Timer::from_seconds(15.0, false)}
                    });
                }
            }
        }
    }
}

fn ripple_update(mut commands: Commands, time: Res<Time>, mut event_query: Query<(Entity, &mut RippleEvent)>, mut element_query: Query<(&mut Text, &Rectangle, &mut Intensity)> ) {
    for (entity, mut event) in event_query.iter_mut() {
        event.timer.timer.tick(time.delta());
        if event.timer.timer.just_finished() {
            commands.entity(entity).despawn();
        } else {
            for (_text, rect, mut intensity) in element_query.iter_mut() {
                intensity.0 = intensity.0 + event.value_at_pos_time(rect.x, rect.y, event.timer.timer.elapsed_secs());
            }
        }
    }
    for (mut text, _rect, mut intensity) in element_query.iter_mut() {
        let value = intensity.0 as i32;
        if value == 0 {
            text.sections[0].value = ElementValue::Central.value();
        } else {
            text.sections[0].value = value.to_string();
        }
        intensity.0 = 0.0;
    }

}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app//.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_board)
            .add_system(mouse_click_system)
            .add_system(ripple_update);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}