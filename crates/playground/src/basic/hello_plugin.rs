use bevy::prelude::*;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(GreetTimer(Timer::from_seconds(2., true)))
			.add_startup_system(add_people)
			.add_system(greet_people);
	}
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct FavFood(String);

struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
	commands
		.spawn()
		.insert(Person)
		.insert(FavFood("Chicken".to_string()));
	commands
		.spawn()
		.insert(Person)
		.insert(FavFood("Pizza".to_string()));
	commands
		.spawn()
		.insert(Person)
		.insert(FavFood("Burger".to_string()));
}

fn greet_people(
	time: Res<Time>,
	mut timer: ResMut<GreetTimer>,
	query: Query<&FavFood, With<Person>>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		println!("gday!");
		for food in query.iter() {
			println!("hello {}!", food.0)
		}
	}
}


pub fn add(a: f32, b: f32) -> f32 { a + b }
