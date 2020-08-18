#![allow(unused_variables)]
#![allow(unused)]

use crate::imgui_wgpu::Renderer;
use crate::nes;
use futures::executor::block_on;
use glob::glob;
use imgui::*;
use imgui_winit_support;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;
use wgpu::{Device, Queue};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn find_roms() -> glob::Paths {
    let exe_path = std::env::current_exe();
    let rom_path = exe_path.unwrap().parent().unwrap().join("../../roms");

    glob(rom_path.join("**/*.nes").to_str().unwrap()).unwrap()
}

fn to_rgb01(color: [i32; 4]) -> [f32; 4] {
    [
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
        color[3] as f32 / 255.0,
    ]
}

struct UiTexture {
    size: (usize, usize),
    data: Vec<u8>,
    texture_id: TextureId,
}

impl UiTexture {
    fn new(renderer: &mut Renderer, device: &Device, size: (usize, usize)) -> Self {
        let texture_id = renderer.create_texture(&device, size.0 as u32, size.1 as u32);
        UiTexture {
            size: size,
            data: vec![0; size.0 * size.1 * 4],
            texture_id: texture_id,
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer, device: &Device, mut queue: &mut Queue) {
        // Uploaded updated screen texture data
        renderer.update_texture(
            self.texture_id,
            &device,
            &mut queue,
            &self.data,
            self.size.0 as u32,
            self.size.1 as u32,
        );
    }

    pub fn get_size(&self, scale: f32) -> [f32; 2] {
        [(self.size.0 as f32) * scale, (self.size.1 as f32) * scale]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: u32) {
        let r = ((c & 0xFF0000) >> 16) as u8;
        let g = ((c & 0xFF00) >> 8) as u8;
        let b = ((c & 0xFF) >> 0) as u8;

        let x0 = x * 4;
        let y0 = y * 4;
        let pos = y0 * self.size.0;
        self.data[pos + x0..pos + x0 + 4].copy_from_slice(&[r, g, b, 0xFF]);
    }
}

// Screen is used to store and update screen buffer and draw it as window with a texture
struct ScreenBuffer {
    ui_scale: f32,
    ui_color: [f32; 4],
    texture: UiTexture,
}

impl ScreenBuffer {
    fn new(renderer: &mut Renderer, device: &Device) -> Self {
        ScreenBuffer {
            ui_scale: 3.0_f32,
            ui_color: [0.09_f32, 0.6_f32, 0.0_f32, 1.0_f32],
            texture: UiTexture::new(renderer, device, nes::ppu::SCREEN_SIZE),
        }
    }

    fn draw_ui(&mut self, ui: &imgui::Ui) {
        // Screen window
        let window = imgui::Window::new(im_str!("Screen")).always_auto_resize(true);
        window
            .position([370.0, 5.0], Condition::Once)
            .build(&ui, || {
                Image::new(
                    self.texture.texture_id,
                    self.texture.get_size(self.ui_scale),
                )
                //.tint_col(self.ui_color)
                .build(&ui);
                ui.drag_float(im_str!("Scale"), &mut self.ui_scale).build();
                ui.same_line(0.0);
                imgui::ColorEdit::new(im_str!("Color"), &mut self.ui_color).build(&ui);
            });
    }

    fn update(
        &mut self,
        ppu: &nes::ppu::Ppu,
        renderer: &mut Renderer,
        device: &Device,
        queue: &mut Queue,
    ) {
        // Update pixels in screen buffer from ppu's screen
        for x in 0..self.texture.size.0 {
            for y in 0..self.texture.size.1 {
                self.texture.set_pixel(x, y, ppu.screen.get_pixel(x, y));
            }
        }

        self.texture.update(renderer, device, queue);
    }
}

struct StateWindow {
    textures: [UiTexture; 2],
}

impl StateWindow {
    fn new(renderer: &mut Renderer, device: &Device) -> Self {
        StateWindow {
            textures: [
                UiTexture::new(renderer, device, (128, 128)),
                UiTexture::new(renderer, device, (128, 128)),
            ],
        }
    }

    fn draw_ui(&mut self, ui: &imgui::Ui, emulator: &mut nes::Emulator) {
        // Window with CPU state
        let window = imgui::Window::new(im_str!("State"));
        window
            .size([425.0, 600.0], Condition::Once)
            .position([1170.0, 5.0], Condition::Once)
            .build(&ui, || {
                ui.text(format!("FPS: {:.0}", emulator.frame_time.fps));
                ui.text(format!("Total Clocks: {}", emulator.clock));
                ui.text(format!("PC: {:#X}", emulator.cpu.PC));

                for i in 0..2 {
                    Image::new(self.textures[i].texture_id, self.textures[i].get_size(1.5))
                        .build(&ui);
                    ui.same_line(0.0);
                }
            });
    }

    fn update(
        &mut self,
        ppu: &mut nes::ppu::Ppu,
        renderer: &mut Renderer,
        device: &Device,
        queue: &mut Queue,
    ) {
        let palette = 0;

        // Get and draw CHR ROM
        for i in 0..2 as usize {
            for tile_y in 0..16 {
                for tile_x in 0..16 {
                    let offset = tile_y * 256 + tile_x * 16;
                    for row in 0..8 {
                        let mut tile_lsb =
                            ppu.ppu_read((i as u16) * 0x1000 + offset + row + 0x0000);
                        let mut tile_msb =
                            ppu.ppu_read((i as u16) * 0x1000 + offset + row + 0x0008);

                        for col in 0..8 {
                            let pixel = (tile_msb & 0x01) << 1 | (tile_lsb & 0x01);

                            tile_lsb >>= 1;
                            tile_msb >>= 1;

                            self.textures[i].set_pixel(
                                (tile_x * 8 + (7 - col)) as usize,
                                (tile_y * 8 + row) as usize,
                                ppu.get_color_from_pal(palette, pixel),
                            );
                        }
                    }
                }
            }
            self.textures[i].update(renderer, device, queue);
        }
    }
}

pub struct NESApp {
    rom_files: Vec<PathBuf>,
    emulator: nes::Emulator,
}

impl NESApp {
    pub fn new() -> Self {
        let roms = find_roms().map(|res| res.unwrap()).collect();

        NESApp {
            rom_files: roms,
            emulator: nes::Emulator::new(),
        }
    }

    fn draw_ui(&mut self, ui: &imgui::Ui) {
        // Window with list of ROMs
        let window = imgui::Window::new(im_str!("ROMs"));
        window
            .size([350.0, 600.0], Condition::Once)
            .position([5.0, 5.0], Condition::Once)
            .build(&ui, || {
                for rom_file in &self.rom_files {
                    let filename = ImString::new(rom_file.file_name().unwrap().to_str().unwrap());
                    if ui.button(&filename, [0 as f32, 0 as f32]) {
                        self.emulator.load_rom(rom_file);
                    }
                }
            });

        // Help Window
        let window = imgui::Window::new(im_str!("Help"));
        window
            .size([350.0, 160.0], Condition::Once)
            .position([5.0, 660.0], Condition::Once)
            .build(&ui, || {
                ui.text(im_str!(
                    "Select ROM file, to control use keys:\nA,S,Z,X,\nArrow Keys\n\nHave fun!"
                ));
            });

        // Test Logger
        let logger = self.emulator.logger.borrow();
        let log_txt = String::from_utf8_lossy(&logger.bytes);

        let window = imgui::Window::new(im_str!("Test Logger"));
        window
            .size([425.0, 160.0], Condition::Once)
            .position([1170.0, 660.0], Condition::Once)
            .build(&ui, || {
                ui.text(format!("{:?}", logger.bytes[0]));
                ui.text(log_txt.clone());
            });
    }

    fn set_key_state(&mut self, code: VirtualKeyCode, state: bool) {
        let b = match code {
            VirtualKeyCode::X => 0x80,
            VirtualKeyCode::Z => 0x40,
            VirtualKeyCode::A => 0x20,
            VirtualKeyCode::S => 0x10,
            VirtualKeyCode::Up => 0x08,
            VirtualKeyCode::Down => 0x04,
            VirtualKeyCode::Left => 0x02,
            VirtualKeyCode::Right => 0x01,
            _ => return,
        };

        let mut controller = self.emulator.controllers[0].borrow_mut();
        if state {
            controller.input |= b;
        } else {
            controller.input &= !b;
        }
    }

    pub fn run(mut self: Rc<Self>) {
        // Set up window and GPU
        let event_loop = EventLoop::new();
        let mut hidpi_factor = 1.0;
        let (window, mut size, surface) = {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(LogicalSize {
                width: 1600.0,
                height: 900.0,
            });
            window.set_title("nes-rust");
            let size = window.inner_size();

            let surface = wgpu::Surface::create(&window);

            (window, size, surface)
        };

        let adapter = block_on(wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ))
        .unwrap();

        let (mut device, mut queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        }));

        // Set up swap chain
        let mut sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Set up dear imgui
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        // Restyle a bit
        let style = imgui.style_mut();
        style.window_rounding = 8.0;
        style.scrollbar_rounding = 8.0;
        style.frame_rounding = 8.0;
        style[imgui::StyleColor::TitleBg] = to_rgb01([110, 110, 100, 62]);
        style[imgui::StyleColor::TitleBgCollapsed] = to_rgb01([110, 110, 100, 52]);
        style[imgui::StyleColor::TitleBgActive] = to_rgb01([110, 110, 100, 87]);
        style[imgui::StyleColor::Header] = to_rgb01([110, 110, 110, 52]);
        style[imgui::StyleColor::HeaderHovered] = to_rgb01([110, 110, 110, 92]);
        style[imgui::StyleColor::HeaderActive] = to_rgb01([110, 110, 110, 72]);
        style[imgui::StyleColor::ScrollbarBg] = to_rgb01([110, 110, 110, 12]);
        style[imgui::StyleColor::ScrollbarGrab] = to_rgb01([110, 110, 110, 52]);
        style[imgui::StyleColor::ScrollbarGrabHovered] = to_rgb01([110, 110, 110, 92]);
        style[imgui::StyleColor::ScrollbarGrabActive] = to_rgb01([110, 110, 110, 72]);
        style[imgui::StyleColor::SliderGrab] = to_rgb01([110, 110, 110, 52]);
        style[imgui::StyleColor::SliderGrabActive] = to_rgb01([110, 110, 110, 72]);
        style[imgui::StyleColor::Button] = to_rgb01([182, 182, 182, 60]);
        style[imgui::StyleColor::ButtonHovered] = to_rgb01([182, 182, 182, 200]);
        style[imgui::StyleColor::ButtonActive] = to_rgb01([182, 182, 182, 140]);
        style[imgui::StyleColor::PopupBg] = to_rgb01([0, 0, 0, 230]);
        style[imgui::StyleColor::TextSelectedBg] = to_rgb01([10, 23, 18, 180]);
        style[imgui::StyleColor::FrameBg] = to_rgb01([70, 70, 70, 30]);
        style[imgui::StyleColor::FrameBgHovered] = to_rgb01([70, 70, 70, 70]);
        style[imgui::StyleColor::FrameBgActive] = to_rgb01([70, 70, 70, 50]);
        style[imgui::StyleColor::MenuBarBg] = to_rgb01([70, 70, 70, 30]);

        // Setup dear imgui wgpu renderer
        let clear_color = wgpu::Color {
            r: 0.03,
            g: 0.03,
            b: 0.03,
            a: 1.0,
        };
        let mut renderer = Renderer::new(
            &mut imgui,
            &device,
            &mut queue,
            sc_desc.format,
            Some(clear_color),
        );

        let mut last_frame = Instant::now();

        let mut screen = ScreenBuffer::new(&mut renderer, &device);
        let mut state_win = StateWindow::new(&mut renderer, &device);

        let mut last_cursor = None;

        // Event loop
        event_loop.run(move |event, _, control_flow| {
            let self_mut = Rc::get_mut(&mut self).unwrap();

            *control_flow = if cfg!(feature = "metal-auto-capture") {
                ControlFlow::Exit
            } else {
                ControlFlow::Poll
            };
            match event {
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                    ..
                } => {
                    hidpi_factor = scale_factor;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    size = window.inner_size();

                    sc_desc = wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8Unorm,
                        width: size.width as u32,
                        height: size.height as u32,
                        present_mode: wgpu::PresentMode::Mailbox,
                    };

                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        },
                    ..
                }
                | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(virtual_keycode),
                                    state,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    self_mut.set_key_state(virtual_keycode, state == ElementState::Pressed);
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawEventsCleared => {
                    last_frame = imgui.io_mut().update_delta_time(last_frame);

                    let frame = match swap_chain.get_next_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("dropped frame: {:?}", e);
                            return;
                        }
                    };
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    let ui = imgui.frame();

                    // Run emulator update
                    self_mut.emulator.update(ui.io().delta_time);

                    {
                        // Read and update screen buffer if changed:
                        let mut ppu = self_mut.emulator.ppu.borrow_mut();
                        if ppu.screen.complete {
                            ppu.screen.complete = false;
                            screen.update(&ppu, &mut renderer, &device, &mut queue);
                        }

                        // Read and update state textures
                        state_win.update(&mut ppu, &mut renderer, &device, &mut queue);
                    }

                    // Draw actual app UI
                    self_mut.draw_ui(&ui);
                    // Draw screen window
                    screen.draw_ui(&ui);
                    // Draw state window
                    state_win.draw_ui(&ui, &mut self_mut.emulator);

                    let mut encoder: wgpu::CommandEncoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        platform.prepare_render(&ui, &window);
                    }
                    renderer
                        .render(ui.render(), &mut device, &mut encoder, &frame.view)
                        .expect("Rendering failed");

                    queue.submit(&[encoder.finish()]);
                }
                _ => (),
            }

            platform.handle_event(imgui.io_mut(), &window, &event);
        });
    }
}
