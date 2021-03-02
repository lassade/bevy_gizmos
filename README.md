# Gizmos for bevy

The way NEO would see Matrix if he was a gamedev; 🕶️

## Design Goals

**BTW** This is what I want not what I got

1. Non Intrusive, it doesn't affect your system scheduling and if you don't want them they are gone;

```rust
fn my_system(gizmos: Res<GizmosCommandBuffer>, ...) {
    gizmos.draw(|context| {
        // If gizmos are disabled this won't run
        let points = generete_your_expensive_spline(too_many_points);
        context.line_list(point);
    })
}
```

2. Immediate and Persistent Mode's

```rust
fn startup_system(commands: &mut Commands) {
    commands.spawn(PbrBundle { ... }) // The object
        .with_children(|c| c.spawn(
            GizmoBundle { // And his gizmo
                gizmo: Gizmo { ... },
            })
        );
}
```

3. Fast TM

Ideally it shouldn't drop FPS while active, unless it's a user fault.

4. Gizmos for days

Have a lot of pre-made gizmos, enough to cover basic and some extra, to name a few: capsule, spline, arrow_handle, billboards;

### Other alternatives

[bevy_prototype_debug_lines](https://github.com/Toqozz/bevy_debug_lines)
