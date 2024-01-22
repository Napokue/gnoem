use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_gnoems)
        .add_systems(Update, greet_gnoems)
        .run();
}

fn add_gnoems(mut commands: Commands) {
    commands.spawn((Gnoem, Name("Willy".to_string())));
    commands.spawn((Gnoem, Name("Kaulana".to_string())));
    commands.spawn((Gnoem, Name("Buhle".to_string())));
}

fn greet_gnoems(query: Query<&Name, With<Gnoem>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

#[derive(Component)]
struct Gnoem;

#[derive(Component)]
struct Name(String);
