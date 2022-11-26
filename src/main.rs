use bevy::prelude::*;

pub const COLOR: Color = Color::rgb(1.0, 1.0, 1.0); 

fn main() {
	App::new()
		.insert_resource(ClearColor(COLOR))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Doctor Plague".to_string(),
				width: 1000.0,
				height: 666.0,
				..Default::default()
			},
			..Default::default()
		}))
        .run();
}
