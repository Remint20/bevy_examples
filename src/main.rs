use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::options::{Backends, WgpuOptions},
};

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

// fn startup_system() {
//     println!("startup system run");
// }

// fn normal_system() {
//     println!("normal system run")
// }

// fn normal_startup_stage_system() {
//     println!("normal startup stage system run");
// }

// fn before_startup_stage_system() {
//     println!("before startup statge system run");
// }

// fn after_startup_stage_system() {
//     println!("after startup statge system run");
// }

// fn normal_stage_system() {
//     println!("normal stage system run");
// }

// fn before_stage_system() {
//     println!("before statge system run");
// }

// fn after_stage_system() {
//     println!("after statge system run");
// }
