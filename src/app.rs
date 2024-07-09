use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(TaskPoolPlugin::default());
        app.init_asset::<bevy::render::texture::Image>();
    }

    let add_player_fn = move |/* no mut? */ commands: Commands,
                              asset_server: Res<AssetServer>| {
        add_player(commands, asset_server);
    };
    app.add_systems(Startup, add_player_fn);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_bird_dark.png"),
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world_mut()).len()
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
fn get_player_has_texture(app: &mut App) -> bool {
    let mut query = app.world_mut().query::<(&Handle<Image>, &Player)>();
    let (handle, _) = query.single(app.world());
    handle.is_strong()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_default() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_player_has_a_texture() {
        let mut app = create_app();
        app.update();
        // I can see the player has a texture,
        // and here I want to test that
        assert!(get_player_has_texture(&mut app));
    }
}
