use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MainMenuPlugin,
        ))
        .init_state::<GameState>()
        .run();
}

// State

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Game,
}




// Main Menu

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_menu);
    }
}

fn handle_menu_input(
    hover: On<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands
        .entity(hover.entity)
        .insert(BackgroundColor(Color::srgb(0.0, 0.5, 1.0)));

    next_state.set(GameState::Game);
}

fn setup_menu(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    commands.spawn(
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(button("Start")).observe(handle_menu_input);
        });
}

fn button(text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: auto(),
            height: auto(),
            padding: UiRect::axes(px(10), px(5)),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(text),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )]
    )
}






// 
