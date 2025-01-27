# bevy_mouse_tracking_plugin

<!-- cargo-rdme start -->

[![CI](https://github.com/JoJoJet/bevy-mouse-tracking/actions/workflows/ci.yml/badge.svg)](https://github.com/JoJoJet/bevy-mouse-tracking/workflows/ci.yml)
[![bevy_mouse_tracking on crates.io](https://img.shields.io/crates/v/bevy_mouse_tracking_plugin.svg)](https://crates.io/crates/bevy_mouse_tracking_plugin)
[![bevy_mouse_tracking docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/bevy_mouse_tracking_plugin)

Tracking the mouse in `bevy` is kind of annoying.
You gotta use [`Events`], and [`EventReader`]s, and even then, they only
get called when the mouse actually *moves*.

[`Events`]: bevy::ecs::event::Events
[`EventReader`]: bevy::ecs::event::EventReader

This crate aims to make this as easy as possible, by providing a
static [resource](bevy::ecs::system::Res) that tracks the mouse position every frame.

This crate also supports more complex use cases such as multiple cameras, which are discussed further down.

## Basics

First, add the plugin to your app:

```rust
use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePosPlugin;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(MousePosPlugin::SingleCamera);
```

Now, you can access the resource in your [`System`]s:

[`System`]: bevy::ecs::system::System

```rust
use bevy_mouse_tracking_plugin::MousePos;
fn dbg_mouse(mouse: Res<MousePos>) {
    eprintln!("{}", *mouse);
}
```
...and don't forget to add the system to your app:
```rust
    .add_plugin(MousePosPlugin::SingleCamera)
    .add_system(dbg_mouse);

```

This will print the screen-space location of the mouse on every frame.

However, we can do better than just screen-space: we support automatic
transformation to world-space coordinates via the [`MousePosWorld`] resource.

```rust
use bevy_mouse_tracking_plugin::MousePosWorld;
fn dbg_world(mouse: Res<MousePosWorld>) {
    eprintln!("{}", *mouse);
}
```

This will print the world-space location of the mouse on every frame.  
Note that this is only supported for two-dimensional, orthographic camera,
but pull requests for 3D support are welcome!

## Multiple cameras

You may notice that if you try to use this plugin in an app that has multiple cameras, it crashes!

```rust

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(MousePosPlugin::SingleCamera)
    .add_startup_system(setup)
    .run();

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
```

This panics with the following output:

```text
thread 'main' panicked at 'cannot identify main camera -- consider adding the MainCamera component to one of the cameras', src\mouse_pos.rs:163:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This is because the plugin doesn't know which of the two cameras to use when figuring out
the values of the `MousePos` and `MousePosWorld` resources. Let's take the panic message's advice.

```rust
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera); // added this line
    commands.spawn_bundle(UiCameraBundle::default());
```

### Queries

If you want to get mouse tracking information relative to each camera individually,
simply [query](bevy::ecs::system::Query) for a `MousePos` or `MousePosWorld` as a
_component_ instead of as a resource.

```rust

App::new()
    // plugins omitted...
    .add_system(dbg_for_each);

fn dbg_for_each(mouse_pos: Query<&MousePosWorld>) {
    for pos in mouse_pos.iter() {
        // This prints the mouse position twice per frame:
        // once relative to the UI camera, and once relative to the physical camera.
        eprintln!("{}", *pos);
    }
}
```

If you want the mouse position for a specific camera, you can add query filters as always.  
Note that as of `bevy 0.6`, the only way to tell the difference between a UI camera and
an orthographic camera is by checking for the [`Frustum`] component.

[`Frustum`]: bevy::render::primitives::Frustum

```rust


use bevy::render::primitives::Frustum;
fn dbg_ui_pos(mouse_pos: Query<&MousePosWorld, Without<Frustum>>) {
    // query for the UI camera, which doesn't have a Frustum component.
    let pos = mouse_pos.single();
    eprintln!("{}", *pos);
}
```

### No main camera

Let's say you have multiple cameras in your app, and you want to treat them all equally,
without declaring any one of them as the main camera.  
Change the plugin to this:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(MousePosPlugin::MultiCamera) // SingleCamera -> MultiCamera
    .add_startup_system(setup)
    // ...

```

Now, you can add as many cameras as you want, without having to worry about marking any
of them as the main camera.  
Note that `MousePos` and `MousePosWorld` will no longer be accessible as global resources
-- you can only access them by `Query`ing camera entities.

## Mouse motion

This crate supports a resource that tracks mouse motion, via [`MouseMotionPlugin`].
The motion can be accessed from any system in a [`MouseMotion`] resource.

[`Res`]: bevy::ecs::system::Res

## Crate name

As a final aside: the name of this crate is intentionally verbose.
This is because I didn't want to steal a crate name, especially since
it is very likely that this crate will eventually be made redundant by
future updates to `bevy`.  
I recommend renaming the crate in your `Cargo.toml`:
```toml
[dependencies]
mouse_tracking = { package = "bevy_mouse_tracking_plugin", version = "..." }
```

<!-- cargo-rdme end -->

License: MIT
