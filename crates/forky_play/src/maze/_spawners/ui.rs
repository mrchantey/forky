use crate::maze::RespawnEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreUI;


pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::FlexStart,
				flex_direction: FlexDirection::ColumnReverse,
				align_items: AlignItems::FlexStart,
				padding: UiRect::all(Val::Px(10.)),

				..default()
			},
			color: Color::NONE.into(),
			..default()
		})
		.with_children(|parent| {
			// parent.spawn_bundle(NodeBundle::default());
			parent.spawn_bundle(TextBundle::from_section(
				"You're on a roll!",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 50.0,
					color: Color::WHITE,
					..default()
				},
			));
			parent
				.spawn_bundle(
					TextBundle::from_section(
						"Score:  0",
						TextStyle {
							font: asset_server.load("fonts/FiraSans-Bold.ttf"),
							font_size: 40.0,
							color: Color::WHITE,
							..default()
						},
					)
					.with_style(Style {
						margin: UiRect::new(
							Val::Px(0.),
							Val::Px(0.),
							Val::Px(10.0),
							Val::Px(10.0),
						),
						..default()
					}),
				)
				.insert(ScoreUI);
			parent.spawn_bundle(TextBundle::from_section(
				"\nClick on maze to start\n\nKeys:         I, J, K, L\nReset:       spacebar",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 20.0,
					color: Color::WHITE,
					..default()
				},
			));
		});
}


pub fn update(
	mut spawn_event: EventReader<RespawnEvent>,
	mut query: Query<&mut Text, With<ScoreUI>>,
) {
	for e in spawn_event.iter() {
		for mut text in query.iter_mut() {
			text.sections[0].value = format!("Score:  {}", e.level - 1);
		}
	}
}
