use std::i8;

use bevy::color::palettes::tailwind::*;
use bevy::picking::pointer::PointerInteraction;
#[warn(dead_code)]
#[warn(unused_imports)]
use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            Wireframe2dPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let square_mesh = meshes.add(Rectangle::new(100.0, 100.0));
    let circle_mesh = meshes.add(Circle::new(50.0));
    let triangle_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 50.0,
        Vec2::new(-50.0, -50.0),
        Vec2::new(50.0, -50.0),
    ));

    let color = Color::srgb(255., 255., 255.);

    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));

    let mut px: f32 = 150.;
    let mut py: f32 = 150.;

    for _x in 1..4 {
        for _y in 1..4 {
            commands
                .spawn((
                    Mesh2d(square_mesh.clone()),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(px, py, 0.),
                    BoardSquare {
                        state: SquareState::Empty,
                        x: px,
                        y: py,
                    },
                ))
                .observe(add_symbol::<Pointer<Press>>(
                    circle_mesh.clone(),
                    triangle_mesh.clone(),
                    pressed_matl.clone(),
                ));

            px -= 150.;
        }
        py -= 150.;
        px = 150.;
    }
}

enum SquareState {
    Empty,
    X,
    O,
}

#[derive(Component)]
struct BoardSquare {
    state: SquareState,
    x: f32,
    y: f32,
}

fn change_color_on_click<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

fn add_symbol<E: EntityEvent>(
    o_shape: Handle<Mesh>,
    x_shape: Handle<Mesh>,
    mat: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut BoardSquare>, Commands) {
    move |event, mut query, mut commands| {
        if let Ok(mut board_square) = query.get_mut(event.event_target()) {
            match board_square.state {
                SquareState::Empty => {
                    println!("Clicked Empty Square ");
                    board_square.state = SquareState::X;
                    commands.spawn((
                        Mesh2d(x_shape.clone()),
                        MeshMaterial2d(mat.clone()),
                        Transform::from_xyz(board_square.x, board_square.y, 0.),
                    ));
                }
                SquareState::X => {
                    println!("Clicked X Square");
                }

                _ => {
                    println!("Square Clicked!")
                }
            }
        }
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
