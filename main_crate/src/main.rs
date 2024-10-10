use bevy::{
    asset::load_internal_asset,
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::{mesh::MeshVertexBufferLayoutRef, render_resource::*},
};

pub const MAIN_SHADER: Handle<Shader> = Handle::weak_from_u128(1716607672170466);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MyExtension>,
        >::default());

    #[cfg(feature = "activate_shader")]
    app.add_plugins(shader_crate::plugin);

    load_internal_asset!(
        app,
        MAIN_SHADER,
        "shaders/extended_material.wgsl",
        Shader::from_wgsl
    );

    app.add_systems(Startup, setup).run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
) {
    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(1.0)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial::default(),
            extension: MyExtension {
                extended_color: LinearRgba::BLACK,
            },
        }),
        ..default()
    });

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyExtension {
    #[uniform(100)]
    extended_color: LinearRgba,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        MAIN_SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            extend_shader_defs(&mut fragment.shader_defs);
        }

        // extend_shader_defs(&mut descriptor.vertex.shader_defs);

        Ok(())
    }
}

fn extend_shader_defs(shader_defs: &mut Vec<ShaderDefVal>) {
    #[cfg(feature = "activate_shader")]
    shader_defs.push("OPTIONAL_SHADER".into());
}
