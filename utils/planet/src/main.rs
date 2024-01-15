use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::WindowMode,
};
use bevy_egui::{
    egui::{self, epaint::Shadow, RichText, TextEdit, Visuals},
    EguiContexts, EguiPlugin,
};
use bevy_generative::{
    noise::{FunctionName, Method, Region},
    planet::{Planet, PlanetBundle, PlanetPlugin},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use egui::{ComboBox, DragValue, Slider};
#[cfg(target_arch = "wasm32")]
use image::{codecs::png::PngEncoder, DynamicImage, ImageEncoder};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(raw_module = "../../lib/components/generate/publish-popup.svelte")]
extern "C" {
    fn send_asset(asset: &str, thumbnail: &[u8]);
    fn dark_theme() -> bool;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(raw_module = "../../routes/generate/[util]/+page.svelte")]
extern "C" {
    fn recieve_asset() -> Option<String>;
}

#[derive(Component)]
struct Thumbnail {
    image_handle: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(
            #[cfg(not(target_arch = "wasm32"))]
            {
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        transparent: true,
                        resizable: true,
                        fit_canvas_to_parent: true,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
            },
            #[cfg(target_arch = "wasm32")]
            {
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-canvas".into()),
                        resizable: true,
                        fit_canvas_to_parent: true,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
            },
        )
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PlanetPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, thumbnail_camera_transform)
        .add_systems(Update, update_theme)
        .add_systems(Update, noise_gui)
        .add_systems(Update, colors_gui)
        .add_systems(Update, thumbnail_gui)
        .add_systems(Update, export_gui)
        .run();
}

fn setup(mut images: ResMut<Assets<Image>>, mut commands: Commands) {
    let size = Extent3d {
        width: 400,
        height: 400,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    let image_handle = images.add(image);
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        Thumbnail { image_handle },
    ));
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    #[cfg(target_arch = "wasm32")]
    commands.spawn(PlanetBundle {
        planet: match recieve_asset() {
            Some(planet) => serde_json::from_str(&planet).expect("Could not deserialize planet"),
            None => Planet::default(),
        },
        ..default()
    });
    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn(PlanetBundle::default());
}

fn update_theme(mut contexts: EguiContexts) {
    contexts.ctx_mut().style_mut(|style| {
        style.visuals = Visuals {
            window_shadow: Shadow::NONE,
            ..Visuals::dark()
        };
    });
    #[cfg(target_arch = "wasm32")]
    if dark_theme() {
        contexts.ctx_mut().style_mut(|style| {
            style.visuals = Visuals {
                window_shadow: Shadow::NONE,
                ..Visuals::dark()
            };
        });
    } else {
        contexts.ctx_mut().style_mut(|style| {
            style.visuals = Visuals {
                window_shadow: Shadow::NONE,
                ..Visuals::light()
            };
        });
    }
}

fn noise_gui(mut contexts: EguiContexts, mut query: Query<&mut Planet>) {
    let mut planet = query.single_mut();
    egui::Window::new("Noise")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut planet.seed));
                ui.label("Seed");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut planet.offset[0]));
                ui.label("X");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut planet.offset[1]));
                ui.label("Y");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut planet.scale).clamp_range(1..=10000));
                ui.label("Scale");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut planet.resolution).clamp_range(1..=10000));
                ui.label("Resolution");
            });
            ui.checkbox(&mut planet.wireframe, "Wireframe");
            ComboBox::from_label("Method")
                .selected_text(planet.method.to_string())
                .show_ui(ui, |ui| {
                    let methods = [
                        Method::OpenSimplex,
                        Method::Perlin,
                        Method::PerlinSurflet,
                        Method::Simplex,
                        Method::SuperSimplex,
                        Method::Value,
                        Method::Worley,
                    ];
                    for method in methods {
                        let text = method.to_string();
                        ui.selectable_value(&mut planet.method, method, text);
                    }
                });
            ComboBox::from_label("Function")
                .selected_text(if let Some(function_name) = &planet.function.name {
                    function_name.to_string()
                } else {
                    "None".to_string()
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut planet.function.name, None, "None");
                    let function_names = [
                        FunctionName::BasicMulti,
                        FunctionName::Billow,
                        FunctionName::Fbm,
                        FunctionName::HybridMulti,
                        FunctionName::RidgedMulti,
                    ];
                    for function_name in function_names {
                        let text = function_name.to_string();
                        ui.selectable_value(&mut planet.function.name, Some(function_name), text);
                    }
                });
            if let Some(_function_name) = &planet.function.name {
                ui.add(Slider::new(&mut planet.function.octaves, 0..=10).text("Octaves"));
                ui.add(Slider::new(&mut planet.function.frequency, 0.0..=10.0).text("Frequency"));
                ui.add(Slider::new(&mut planet.function.lacunarity, 0.0..=30.0).text("Lacunarity"));
                ui.add(
                    Slider::new(&mut planet.function.persistence, 0.01..=1.0).text("Persistence"),
                );
            }
        });
}

fn colors_gui(mut contexts: EguiContexts, mut query: Query<&mut Planet>) {
    let mut planet = query.single_mut();
    let texture_id = contexts.add_image(planet.gradient.image.clone_weak());
    let mut min_pos = 0.0;
    egui::Window::new("Colors")
        .default_width(planet.gradient.size[0] as f32)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                texture_id,
                [
                    planet.gradient.size[0] as f32,
                    planet.gradient.size[1] as f32,
                ],
            )));
            ui.add(Slider::new(&mut planet.gradient.smoothness, 0.0..=1.0).text("Smoothness"));
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut planet.gradient.segments).clamp_range(0..=100));
                ui.label("Segments");
            });
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba_unmultiplied(&mut planet.base_color);
                ui.label("Base Color");
            });
            ui.add(Slider::new(&mut planet.height_exponent, 0.1..=2.5).text("Height Exponent"));
            ui.add(Slider::new(&mut planet.sea_percent, 0.0..=100.0).text("Sea Level"));
            ui.separator();
            if ui.button("Add Region").clicked() {
                let index = planet.regions.len() + 1;
                planet.regions.push(Region {
                    label: format!("Region #{index}"),
                    position: 0.0,
                    color: [0, 0, 0, 255],
                    ..default()
                });
            }
            ui.separator();
            let regions_len = planet.regions.len();
            let mut regions_to_remove: Vec<usize> = Vec::with_capacity(regions_len);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, region) in planet.regions.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&format!("Region #{}", i + 1)).size(16.0));
                        if ui.button("Remove").clicked() {
                            regions_to_remove.push(i);
                        }
                    });
                    ui.vertical(|ui| {
                        ui.label("Label");
                        ui.add(TextEdit::singleline(&mut region.label).desired_width(200.0));
                    });

                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut region.position).clamp_range(min_pos..=100.0));
                        ui.label("Position");
                    });
                    min_pos = region.position;

                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba_unmultiplied(&mut region.color);
                        ui.label("Color");
                    });
                    if i != regions_len - 1 {
                        ui.separator();
                    }
                }
            });
            for i in regions_to_remove {
                planet.regions.remove(i);
            }
        });
}

fn thumbnail_camera_transform(
    camera: Query<&Transform, With<PanOrbitCamera>>,
    mut thumbnail_camera_transform: Query<
        &mut Transform,
        (With<Thumbnail>, Without<PanOrbitCamera>),
    >,
) {
    let mut thumbnail_camera_transform = thumbnail_camera_transform.single_mut();
    *thumbnail_camera_transform = camera.single().clone();
}

fn thumbnail_gui(mut contexts: EguiContexts, thumbnail: Query<&Thumbnail>) {
    let thumbnail = thumbnail.single();

    let texture_id = contexts.add_image(thumbnail.image_handle.clone());
    egui::Window::new("Thumbnail").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
            texture_id, [200.0; 2],
        )));
    });
}

fn export_gui(
    #[allow(unused_variables)] images: Res<Assets<Image>>,
    mut contexts: EguiContexts,
    mut planet: Query<&mut Planet>,
    thumbnail: Query<&Thumbnail>,
) {
    let mut planet = planet.single_mut();
    #[allow(unused_variables)]
    let thumbnail = thumbnail.single();
    egui::Window::new("Export").show(contexts.ctx_mut(), |ui| {
        #[cfg(target_arch = "wasm32")]
        if ui.button("Publish").clicked() {
            {
                let planet =
                    serde_json::to_string::<Planet>(&planet).expect("Cannot serialize Planet");

                let thumbnail = images
                    .get(thumbnail.image_handle.clone())
                    .expect("Could not find thumbnail")
                    .clone()
                    .try_into_dynamic()
                    .expect("Could not convert to dynamic")
                    // .resize(200, 200, image::imageops::FilterType::Triangle)
                    .to_rgba8();
                let mut thumbnail_buffer: Vec<u8> = vec![];
                let png_encoder = PngEncoder::new(&mut thumbnail_buffer);
                let color_type = DynamicImage::from(thumbnail.clone()).color();
                png_encoder
                    .write_image(
                        &thumbnail,
                        thumbnail.width(),
                        thumbnail.height(),
                        color_type,
                    )
                    .expect("Failed to write to png");
                info!("{:?}", thumbnail);
                send_asset(&planet, &thumbnail_buffer);
            }
        }
        if ui.button("Export").clicked() {
            planet.export = true
        }
    });
}
