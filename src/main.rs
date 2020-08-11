use bevy::prelude::*;


fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}

///adds components
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Jack Mera".to_string())))
        .spawn((Person, Name("Marie Cavendish".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) {
    //updates timer with the time elapsed since last update
    timer.0.tick(time.delta_seconds);

    //check to see if the timer has finished. if it has, print message
    if timer.0.finished {
        for(_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }

        timer.0.reset();
    }
}
struct Person;

struct Name(String);

struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(GreetTimer(Timer::from_seconds(2.0)))
        .add_startup_system(add_people.system())
        .add_system(greet_people.system());
    }
}


