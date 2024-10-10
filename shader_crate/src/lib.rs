use bevy::{
    app::App,
    asset::{load_internal_asset, Handle},
    prelude::Shader,
};

pub const OPTIONAL_SHADER: Handle<Shader> = Handle::weak_from_u128(7212405635289860);

pub fn plugin(app: &mut App) {
    load_internal_asset!(
        app,
        OPTIONAL_SHADER,
        "shaders/optional_shader.wgsl",
        Shader::from_wgsl
    );
}
