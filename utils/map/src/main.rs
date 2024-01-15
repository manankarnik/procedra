use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::WindowMode};
use bevy_egui::{
    egui::{self, epaint::Shadow, RichText, TextEdit, Visuals},
    EguiContexts, EguiPlugin,
};
use bevy_generative::{
    map::{Map, MapBundle, MapPlugin},
    noise::{FunctionName, Method, Region},
};
use egui::{ComboBox, DragValue, Slider};
#[cfg(target_arch = "wasm32")]
use image::{codecs::png::PngEncoder, imageops::FilterType, DynamicImage, ImageEncoder};
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

fn main() {
    let mut app = App::new();
    app.add_plugins(
        #[cfg(not(target_arch = "wasm32"))]
        {
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
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
    .add_plugins(MapPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, noise_gui)
    .add_systems(Update, image_gui)
    .add_systems(Update, export_gui)
    .add_systems(Update, colors_gui)
    .add_systems(Update, update_theme);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            #[cfg(target_arch = "wasm32")]
            parent.spawn(MapBundle {
                map: match recieve_asset() {
                    Some(map) => serde_json::from_str(&map).expect("Could not deserialize map"),
                    None => Map::default(),
                },
                ..default()
            });
            #[cfg(not(target_arch = "wasm32"))]
            parent.spawn(MapBundle::default());
        });
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

fn noise_gui(mut contexts: EguiContexts, mut query: Query<&mut Map>) {
    let mut map = query.single_mut();
    egui::Window::new("Noise")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.noise.seed));
                ui.label("Seed");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.noise.offset[0]));
                ui.label("X");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.noise.offset[1]));
                ui.label("Y");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.size[0]).clamp_range(1..=10000));
                ui.label("Width");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.size[1]).clamp_range(1..=10000));
                ui.label("Height");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.noise.scale).clamp_range(1..=10000));
                ui.label("Scale");
            });
            ComboBox::from_label("Method")
                .selected_text(map.noise.method.to_string())
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
                        ui.selectable_value(&mut map.noise.method, method, text);
                    }
                });
            ComboBox::from_label("Function")
                .selected_text(if let Some(function_name) = &map.noise.function.name {
                    function_name.to_string()
                } else {
                    "None".to_string()
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut map.noise.function.name, None, "None");
                    let function_names = [
                        FunctionName::BasicMulti,
                        FunctionName::Billow,
                        FunctionName::Fbm,
                        FunctionName::HybridMulti,
                        FunctionName::RidgedMulti,
                    ];
                    for function_name in function_names {
                        let text = function_name.to_string();
                        ui.selectable_value(
                            &mut map.noise.function.name,
                            Some(function_name),
                            text,
                        );
                    }
                });
            if let Some(_function_name) = &map.noise.function.name {
                ui.add(Slider::new(&mut map.noise.function.octaves, 0..=10).text("Octaves"));
                ui.add(
                    Slider::new(&mut map.noise.function.frequency, 0.0..=10.0).text("Frequency"),
                );
                ui.add(
                    Slider::new(&mut map.noise.function.lacunarity, 0.0..=30.0).text("Lacunarity"),
                );
                ui.add(
                    Slider::new(&mut map.noise.function.persistence, 0.01..=1.0)
                        .text("Persistence"),
                );
            }
        });
}

fn image_gui(mut contexts: EguiContexts, mut query: Query<&mut Map>) {
    let mut map = query.single_mut();
    egui::Window::new("Image")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.image_size[0]).clamp_range(1..=10000));
                ui.label("Width");
            });
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.image_size[1]).clamp_range(1..=10000));
                ui.label("Height");
            });
            ui.checkbox(&mut map.same_size, "Same size as Noise");
            ui.checkbox(&mut map.anti_aliasing, "Anti-aliasing");
        });
    if map.same_size {
        map.image_size = map.size;
    }
}

fn export_gui(
    #[allow(unused_variables)] images: Res<Assets<Image>>,
    mut contexts: EguiContexts,
    mut query: Query<(&mut Map, &UiImage)>,
) {
    #[allow(unused_variables)]
    let (mut map, ui_image) = query.single_mut();
    egui::Window::new("Export")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            #[cfg(target_arch = "wasm32")]
            if ui.button("Publish").clicked() {
                let thumbnail = images
                    .get(ui_image.texture.clone())
                    .expect("Image texture not found")
                    .clone()
                    .try_into_dynamic()
                    .expect("Could not convert to dynamic")
                    .resize(
                        200.min(map.image_size[0]),
                        200.min(map.image_size[1]),
                        if map.anti_aliasing {
                            FilterType::Triangle
                        } else {
                            FilterType::Nearest
                        },
                    )
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
                send_asset(
                    &serde_json::to_string::<Map>(&map).expect("Cannot serialize Map"),
                    &thumbnail_buffer,
                );
            }
            if ui.button("Download").clicked() {
                map.export = true;
            }
        });
}

fn colors_gui(mut contexts: EguiContexts, mut query: Query<&mut Map>) {
    let mut map = query.single_mut();
    let texture_id = contexts.add_image(map.noise.gradient.image.clone_weak());
    let mut min_pos = 0.0;
    egui::Window::new("Colors")
        .default_width(map.noise.gradient.size[0] as f32)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                texture_id,
                [
                    map.noise.gradient.size[0] as f32,
                    map.noise.gradient.size[1] as f32,
                ],
            )));
            ui.add(Slider::new(&mut map.noise.gradient.smoothness, 0.0..=1.0).text("Smoothness"));
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut map.noise.gradient.segments).clamp_range(0..=100));
                ui.label("Segments");
            });
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba_unmultiplied(&mut map.noise.base_color);
                ui.label("Base Color");
            });
            ui.separator();
            if ui.button("Add Region").clicked() {
                let index = map.noise.regions.len() + 1;
                map.noise.regions.push(Region {
                    label: format!("Region #{index}"),
                    position: 0.0,
                    color: [0, 0, 0, 255],
                    ..default()
                });
            }
            ui.separator();
            let regions_len = map.noise.regions.len();
            let mut regions_to_remove: Vec<usize> = Vec::with_capacity(regions_len);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, region) in map.noise.regions.iter_mut().enumerate() {
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
                map.noise.regions.remove(i);
            }
        });
}
