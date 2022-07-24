//! Systems and structs to place Blocks

use std::f32::consts::FRAC_1_SQRT_2;

use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{MainCamera, MousePosWorld};

use crate::{AppState, GameState, block::{WrappedExpr, Object, ObjectData, Orientation}, mouseover::{HoverState, Side, TopHover}};

#[derive(Component, Default, Clone)]
pub struct Placing;

pub fn place_expr(
	mut commands: Commands,
	app_state: &mut State<AppState>,
	state: &mut GameState,
	expr: WrappedExpr,
) {
	info!("Placing: {:?}", expr);
	match app_state.current() {
		AppState::Default => {
			state.just_pressed = true; // Prevent a single mouse click
			commands
				.spawn_bundle(Object { expr, ..default() })
				.insert(Placing);
			app_state.push(AppState::PlacingObject).unwrap();
			state.placing_index += 1.0;
		}
		AppState::PlacingObject => {
			state.just_pressed = true; // Prevent a single mouse click
			state.update_placing_expr = Some(expr);
		}
		_ => {},
	}
	
}

// System for placing blocks on the canvas and inside other blocks
pub fn placing_system(
	mut commands: Commands,
	mut mouse: ResMut<Input<MouseButton>>,
	mouse_pos: Res<MousePosWorld>,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	mut placing: Query<(Entity, &mut ObjectData, &mut WrappedExpr, Option<&mut Sprite>), With<Placing>>,
	mut top_hover: Query<(Entity, &mut ObjectData, &mut WrappedExpr, &HoverState), (Without<Placing>, With<TopHover>)>,
	keyboard_input: Res<Input<KeyCode>>,
	camera_proj: Query<&OrthographicProjection, With<MainCamera>>,
	asset_server: Res<AssetServer>,
) {
	if state.just_pressed {
		state.just_pressed = false;
		mouse.clear_just_pressed(MouseButton::Left);
	}
	// Fetch data on block-to-place
	let (entity, mut data, mut expr, sprite) = placing.single_mut();

	if let Some(new_expr) = state.update_placing_expr.take() {
		*expr = new_expr;
	}
	
	if let Ok((h_entity, mut h_data, mut h_expr, HoverState::Yes { side, .. })) = top_hover.get_single_mut() {
		// Make sure we can place block
		if let Some((side, expr_slot)) = match (&mut *h_expr, side) {
			(WrappedExpr::Lambda { expr_entity, .. }, Side::First) if expr_entity.is_none() => {
				h_data.flip = true; // Make sure the dot is on the right side of the Function block texture
				Some((Side::First, expr_entity))
			},
			(WrappedExpr::Lambda { expr_entity, .. }, Side::Second) if expr_entity.is_none() => Some((Side::Second, expr_entity)),
			(WrappedExpr::Application { func_entity, .. }, Side::First) if func_entity.is_none() => {
				Some((Side::First, func_entity))
			}
			(WrappedExpr::Application { args_entity, .. }, Side::Second) if args_entity.is_none() => {
				Some((Side::Second, args_entity))
			}
			(_, _) => None,
		} {
			let size = (h_data.size * FRAC_1_SQRT_2) * 0.90;
			data.orientation = h_data.orientation.swap();
			data.size = size;

			let half_h_size_oriented = match h_data.orientation {
				Orientation::Horizontal => Vec2::new(h_data.size / 4.0, 0.0),
				Orientation::Vertical => Vec2::new(0.0, h_data.size / 4.0),
			};
			let relative_loc = match side {
				Side::First => -half_h_size_oriented,
				Side::Second => half_h_size_oriented,
			};
			data.location = h_data.location + relative_loc;

			// Place block inside another block
			if mouse.clear_just_pressed(MouseButton::Left) {
				state.just_pressed = true;
				*expr_slot = Some(entity);
				data.parent = Some(h_entity);
				// commands.entity(h_entity).add_child(entity); // DONT DO THIS, YOUR LIFE WILL BE PAINNNN
				commands.entity(entity).remove::<Placing>().insert(HoverState::No);
				app_state.pop().unwrap();
				return;
			}
		}
	} else {
		data.size = camera_proj.iter().next().unwrap().scale * 300.0; // Scale block-to-place with size
		data.location = Vec2::new(mouse_pos.x, mouse_pos.y); // Move block-to-place to mouse cursor
		data.orientation = state.placing_orientation; // Set orientation based on game state

		// Place block on blank canvas (if there are no objects in scene)
		if mouse.clear_just_pressed(MouseButton::Left) && top_hover.is_empty() {
			state.just_pressed = true;
			commands.entity(entity).remove::<Placing>().insert(HoverState::No);
			app_state.pop().unwrap();
		}
	}
	// Generate / Update visuals from Object data
	if sprite.is_none() {
		commands.entity(entity).insert_bundle(SpriteBundle {
			sprite: data.gen_sprite(),
			transform: data.gen_transform(state.placing_index),
			texture: ObjectData::gen_texture(&expr, &*asset_server),
			..default()
		});
	}

	// Press R to rotate while placing
	if keyboard_input.just_pressed(KeyCode::R) {
		state.placing_orientation = state.placing_orientation.swap();
	}
	// Press Escape to stop placing block
	if keyboard_input.just_pressed(KeyCode::Escape) {
		commands.entity(entity).despawn();
		app_state.pop().unwrap();
	}
	// Change placing WrappedExpr variant
	if keyboard_input.just_pressed(KeyCode::A) {
		*expr = WrappedExpr::APPLICATION;
	}
	if keyboard_input.just_pressed(KeyCode::F) {
		*expr = WrappedExpr::LAMBDA;
	}
	if keyboard_input.just_pressed(KeyCode::V) {
		*expr = WrappedExpr::VARIABLE;
	}
}
