use bevy::prelude::*;
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MainCamera, MousePosPlugin, MousePosWorld};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_prototype_lyon::prelude::*;
use block::{BindEntityTree, ObjectData, Orientation, PartialForm, WrappedExpr};
use block_to_expr::block_to_expr;
use expr::{Binding, Expr, BindTree};
use hashdb::{LinkArena, TypeStore};
use mouseover::{BottomHover, HoverState, TopHover};
use placing::place_expr;

mod expr;
mod mouseover;
mod name;
mod block;
mod placing;
mod parse;
mod ui;
mod block_to_expr;

const IMAGE_SIZE: f32 = 300.0;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
	#[default]
	Default,
	PlacingObject,
	WiringObject,
}

fn main() {
	println!("Hello, langjam #0003!");
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PanCamPlugin::default())
		.add_plugin(MousePosPlugin::SingleCamera)
		.add_plugin(ShapePlugin)
		.add_startup_system(setup)
		.add_startup_system(ui::ui_setup)
		.add_state(AppState::Default)
		.add_system_set(SystemSet::on_update(AppState::Default).with_system(input_system).with_system(block_input))

		.add_system_set(SystemSet::on_update(AppState::PlacingObject).with_system(placing::placing_system))

		.add_system_set(SystemSet::on_update(AppState::WiringObject).with_system(wiring_system))
		.add_system_set(SystemSet::on_exit(AppState::WiringObject).with_system(connecting_system))

		.add_system(block::data_update).add_system(block::expr_update).add_system(block::hover_update)
		.add_system(mouseover::mouseover_system)
		.add_system(state_change_detect)
		.add_system(ui::button_system)
    	.add_system(bevy::window::exit_on_window_close_system)
		.add_system(exprs_forming_system).add_system(reform_system)
		.init_resource::<GameState>()
		.run();
}

fn setup(mut commands: Commands) {
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(MainCamera)
		.insert(PanCam { track_mouse: true, ..default() });
}

fn state_change_detect(app_state: Res<State<AppState>>, mut previous: Local<AppState>) {
	let current = app_state.current();
	if *previous != *current {
		info!("State changed: {:?}", app_state.current());
		*previous = current.clone();
	}
}

#[derive(Default)]
pub struct GameState {
	placing_orientation: Orientation,
	placing_index: f32,
	update_placing_expr: Option<WrappedExpr>,
	just_pressed: bool,
}

// System for triggering things based on keyboard input
fn input_system(
	mut commands: Commands,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	keyboard_input: Res<Input<KeyCode>>,
	mut mouse_input: ResMut<Input<MouseButton>>,
	mut other_objects: Query<(Entity, &mut ObjectData, &mut block::WrappedExpr, &HoverState)>,
	mut expr_text: Query<&mut Text, (With<ui::ExpressionText>, Without<ui::ReducedText>)>,
	mut red_expr_text: Query<&mut Text, (With<ui::ReducedText>, Without<ui::ExpressionText>)>,
) {
	if keyboard_input.just_pressed(KeyCode::F) {
		place_expr(commands, &mut app_state, &mut state, WrappedExpr::LAMBDA);
	} else if keyboard_input.just_pressed(KeyCode::V) {
		place_expr(commands, &mut app_state, &mut state, WrappedExpr::VARIABLE);
	} else if keyboard_input.just_pressed(KeyCode::A) {
		place_expr(commands, &mut app_state, &mut state, WrappedExpr::APPLICATION);
	} else if keyboard_input.just_pressed(KeyCode::R) {
		for (h_entity, mut h_data, mut h_expr, h_hover_state) in other_objects.iter_mut() {
			if let HoverState::Yes { side, .. } = h_hover_state {
				let text: &mut Text = &mut expr_text.iter_mut().next().unwrap();
				let red_text: &mut Text = &mut red_expr_text.iter_mut().next().unwrap();
			  match block_to_expr(&h_expr) {
					Ok(expr) => {
						text.sections[0].value = format!("{}", &expr);
						let arena = LinkArena::new();
						match expr.reduce(&arena) {
							Ok(red) => {
								red_text.sections[0].value = format!("{}", &red);
							}
							Err(_) => {
								red_text.sections[0].value = "unreducable".into();
							}
						}
						
					},
					Err(_) => {
						text.sections[0].value = "malformed expression".into()
					},
				};
			  break
			}
		  }
	} /* else if mouse_input.clear_just_pressed(MouseButton::Left) {
		app_state.push(AppState::WiringObject).unwrap();
	} */
}

fn block_input(
	mut commands: Commands,
	mut keyboard_input: ResMut<Input<KeyCode>>,
	objects: Query<(Entity, &ObjectData, &WrappedExpr, &HoverState, Option<&TopHover>, Option<&BottomHover>)>,
	mut app_state: ResMut<State<AppState>>,
) {
	for (entity, data, expr, state, top, bottom) in objects.iter() {
		match (state, top, bottom) {
			(HoverState::Yes { .. }, Some(_), None) => {
				if keyboard_input.clear_just_pressed(KeyCode::C) {
					if let Some(port) = match expr { WrappedExpr::Lambda { .. } => Some(PortType::Lambda), WrappedExpr::Variable { .. } => Some(PortType::Variable), _ => None } {
						commands.spawn().insert(Wire { from: entity, start: data.location, end: Vec2::ZERO, port }).insert(ActiveWire);
						app_state.push(AppState::WiringObject).unwrap();
					}
				}
			}
			(HoverState::Yes { .. }, None, Some(_)) => {
				
			}
			(HoverState::Yes { .. }, Some(_), Some(_)) => {}
			(HoverState::Yes { .. }, None, None) => {}
			(HoverState::No, None, None) => {}
			_ => { /* panic!("Invalid Hover component configuration: {entity:?}, {state:?}, {top:?}, {bottom:?}") */ }
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum PortType {
	Lambda,
	Variable,
}
impl PortType {
	pub fn swap(self) -> Self {
		match self { Self::Lambda => Self::Variable, Self::Variable => Self::Lambda }
	}
}

#[derive(Component)]
struct Wire {
	from: Entity,
	port: PortType,
	start: Vec2,
	end: Vec2,
}
#[derive(Component, Debug, Clone, Copy)]
struct ActiveWire;

#[derive(Component, Debug, Clone)]
struct FormConnection(Entity, PortType);

#[derive(Component, Debug, Clone)]
struct TriggerReform;

fn connecting_system(
	mut commands: Commands,
	mut objects: Query<(Entity, &mut ObjectData, &mut WrappedExpr, &FormConnection), Added<FormConnection>>
) {
	for (entity, data, mut expr, conn) in objects.iter_mut() {
		match (&mut *expr, conn.1) {
			(WrappedExpr::Variable { formed: (_, bind_tree) }, PortType::Variable) => {
				*bind_tree = BindTree::end(conn.0, &LeakStore);
				commands.entity(entity).insert(TriggerReform);
				debug!("Set bind {entity:?} : {:?} to {:?}", *expr, conn.0);
			},
			(WrappedExpr::Lambda { is_bound, .. }, PortType::Lambda) => {
				*is_bound = true;
				commands.entity(entity).insert(TriggerReform);
				debug!("Set bind {entity:?} : {:?} to {:?}", *expr, conn.0);
			},
			_ => { error!("Invalid connection") }
		}
		commands.entity(entity).remove::<FormConnection>();
	}
}
fn reform_system(mut commands: Commands, mut objects: Query<(Entity, &ObjectData, &mut WrappedExpr), Added<TriggerReform>>) {
	for (entity, data, mut expr) in objects.iter_mut() {
		debug!("Unformed: {:?}", entity);
		expr.unform(); // Remove formed fields
		commands.entity(entity).remove::<TriggerReform>().remove::<Formed>();
		if let WrappedExpr::Variable { .. } = *expr { commands.entity(entity).insert(Formed); }
		if let Some(parent) = data.parent {
			commands.entity(parent).insert(TriggerReform).remove::<Formed>();
		}
	}
}

// System for wiring things up
fn wiring_system(
	mut commands: Commands,
	mut app_state: ResMut<State<AppState>>,
	// mut state: ResMut<GameState>,
	mut top_hover: Query<(Entity, &ObjectData, &WrappedExpr), With<TopHover>>,
	mut mouse: ResMut<Input<MouseButton>>,
	mut keyboard: ResMut<Input<KeyCode>>,
	mut wire: Query<(Entity, &mut Wire, Option<&mut Path>), With<ActiveWire>>,
	mouse_pos: Res<MousePosWorld>,
) {
	// Start wiring if there is an active wire
	if let Ok((wire_entity, mut wire, mut path)) = wire.get_single_mut() {
		wire.end = Vec2::new(mouse_pos.x, mouse_pos.y);
		if let Ok((entity, data, expr)) = top_hover.get_single_mut() {
			if mouse.clear_just_pressed(MouseButton::Left) {
				match (expr, wire.port) {
					(WrappedExpr::Variable { .. }, PortType::Lambda) |
					(WrappedExpr::Lambda { .. }, PortType::Variable) => {
						commands.entity(wire.from).insert(FormConnection(entity, wire.port));
						commands.entity(entity).insert(FormConnection(wire.from, wire.port.swap()));
						wire.end = data.location;
						app_state.pop().unwrap();
						commands.entity(wire_entity).remove::<ActiveWire>();
						debug!("Wired {:?} to {:?}:({expr:?})", wire.from, entity)
					}
					_ => {},
				}
			}
		}
		if keyboard.clear_just_pressed(KeyCode::Escape) {
			commands.entity(wire_entity).despawn();
			app_state.pop().unwrap();
		}

		// Build line
		let mut path_builder = PathBuilder::new();
		path_builder.move_to(Vec2::ZERO);
		path_builder.line_to(wire.end - wire.start);
		let line = path_builder.build();
		if let Some(path) = &mut path {
			**path = line;
		} else {
			commands.spawn().insert_bundle(GeometryBuilder::build_as(
				&line,
				DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
				Transform::from_translation(wire.start.extend(1000.0)),
			));
		}
	}
}

// *I like to leak it leak it, I like to leak it leak it, I like to leak it leak it, I like to, LEAK IT*
struct LeakStore;
impl TypeStore<'static> for LeakStore {
    fn add<T: hashdb::TypeStorable>(&self, val: T) -> &'static T {
        Box::leak(Box::new(val))
    }
}
#[derive(Component)]
struct Formed;

fn exprs_forming_system(
	mut commands: Commands,
	formed: Query<(Entity, &ObjectData, &mut WrappedExpr), (With<HoverState>, With<Formed>)>,
	mut unformed: Query<(Entity, &mut WrappedExpr), Without<Formed>>,
) {
	for (f_entity, data, f_wexpr) in formed.iter() {
		if let WrappedExpr::Variable { formed: (f_expr, mut f_bind_tree) }
		| WrappedExpr::Lambda { formed: Some((f_expr, mut f_bind_tree)), .. }
		| WrappedExpr::Application { formed: Some((f_expr, mut f_bind_tree)), .. } = *f_wexpr {
			if let Some(Ok((entity, mut wexpr))) = data.parent.map(|parent|unformed.get_mut(parent)) {
				match &mut *wexpr {
					WrappedExpr::Lambda {
						expr_entity: Some(expr_entity),
						formed, ..
					} if *expr_entity == f_entity => {
						let bind = f_bind_tree.pop_binding(&LeakStore, &entity, &LeakStore).unwrap();
						debug!("Using bind: {:?}", bind);
						*formed = Some((LeakStore.add(Expr::Lambda { bind, expr: f_expr }), f_bind_tree));
						commands.entity(entity).insert(Formed);
						info!("Entity {:?} formed expression: {:?}", entity, *formed);
					}
					WrappedExpr::Application {
						func_entity: Some(func_entity),
						args_entity: Some(args_entity),
						partial_formed: Some((partial_expr, partial_tree, partial_form)),
						formed,
					} if formed.is_none() => match partial_form {
						PartialForm::Func if *args_entity == f_entity => {
							*formed = Some((
								Expr::app(*partial_expr, f_expr, &LeakStore),
								BindEntityTree::branch(*partial_tree, f_bind_tree, &LeakStore)
							));
							commands.entity(entity).insert(Formed);
							info!("Entity {:?} formed expression: {:?}", entity, *formed);
						}
						PartialForm::Args if *func_entity == f_entity => {
							*formed = Some((
								Expr::app(f_expr, *partial_expr, &LeakStore),
								BindEntityTree::branch(f_bind_tree, *partial_tree, &LeakStore)
							));
							commands.entity(entity).insert(Formed);
							info!("Entity {:?} formed expression: {:?}", entity, *formed);
						}
						_ => { warn!("Entity {:?} Couldn't form partial form with expression {:?}", entity, *wexpr); }
					}
					WrappedExpr::Application {
						func_entity: Some(func_entity),
						args_entity: _,
						partial_formed,
						formed: None,
					} if partial_formed.is_none() && *func_entity == f_entity => {
						*partial_formed = Some((f_expr, f_bind_tree, PartialForm::Func));
						info!("Entity {:?} formed partial expression for: {:?}", entity, PartialForm::Func);
					}
					WrappedExpr::Application {
						func_entity: _,
						args_entity: Some(args_entity),
						partial_formed,
						formed: None,
					} if partial_formed.is_none() && *args_entity == f_entity => {
						*partial_formed = Some((f_expr, f_bind_tree, PartialForm::Args));
						info!("Entity {:?} formed partial expression for: {:?}", entity, PartialForm::Args);
					}
					WrappedExpr::Variable { .. } => {
						commands.entity(entity).insert(Formed);
						warn!("Variable not set to Formed for some reason");
					}
					_ => { warn!("Entity {:?} couldn't form expression: {:?}", entity, *wexpr); }
				}
			}
		} else {
			warn!("Entity {f_entity:?} did not have formed field but had Formed component");
			commands.entity(f_entity).remove::<Formed>();
		}
	}
}