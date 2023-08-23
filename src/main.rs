/*
TODO: MAKE THIS SHIT CLEANER LOL

*/


mod chip8;

use std::thread::sleep;
use std::time::Duration;
use glow::{HasContext, NEAREST, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNPACK_ROW_LENGTH};
use imgui::{Condition, Context, TextureId};
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use rfd::{AsyncFileDialog};
use sdl2::{event::Event, video::{GLProfile, Window}};
use sdl2::keyboard::Keycode;
use crate::chip8::Chip8;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

#[tokio::main]
async fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
        .window("Chip-8 Emulator", 1280, 720)
        .allow_highdpi()
        .opengl()
        .resizable()
        .maximized()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl = glow_context(&window);

    let mut imgui = Context::create();

    // Debugging only
    //imgui.set_ini_filename(None);
    //imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();


    let texture = unsafe { renderer.gl_context().create_texture() }.unwrap();
    let id = TextureId::new(Into::<u32>::into((&texture).0) as usize);

    let mut chip8_emu = Chip8::new();

    unsafe {
        renderer.gl_context().bind_texture(TEXTURE_2D, Some(texture));
        renderer.gl_context().tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
        renderer.gl_context().tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
        renderer.gl_context().pixel_store_i32(UNPACK_ROW_LENGTH, 0);
        renderer.gl_context().tex_image_2d(TEXTURE_2D, 0, glow::RGB as i32, 64, 32, 0, glow::RGB, glow::UNSIGNED_BYTE, Some(&chip8_emu.display_screen));
        renderer.gl_context().bind_texture(TEXTURE_2D, None);
    }


    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::Quit { .. } => {
                    break 'main;
                }
                // TODO: Make a function for this and sort the order I do it in
                Event::KeyDown {keycode, ..} => {
                    if let Some(key) = keycode {
                        match key {
                            Keycode::Num1 => {
                                chip8_emu.keys[1] = true;
                            },
                            Keycode::Num2 => {
                                chip8_emu.keys[2] = true;
                            },
                            Keycode::Num3 => {
                                chip8_emu.keys[3] = true;
                            },
                            Keycode::Num4 => {
                                chip8_emu.keys[12] = true;
                            },
                            Keycode::Q => {
                                chip8_emu.keys[4] = true;
                            },
                            Keycode::W => {
                                chip8_emu.keys[5] = true;
                            },
                            Keycode::E => {
                                chip8_emu.keys[6] = true;
                            },
                            Keycode::R => {
                                chip8_emu.keys[13] = true;
                            },
                            Keycode::A => {
                                chip8_emu.keys[7] = true;
                            },
                            Keycode::S => {
                                chip8_emu.keys[8] = true;
                            },
                            Keycode::D => {
                                chip8_emu.keys[9] = true;
                            },
                            Keycode::F => {
                                chip8_emu.keys[14] = true;
                            },
                            Keycode::Z => {
                                chip8_emu.keys[10] = true;
                            },
                            Keycode::X => {
                                chip8_emu.keys[0] = true;
                            },
                            Keycode::C => {
                                chip8_emu.keys[11] = true;
                            },
                            Keycode::V => {
                                chip8_emu.keys[15] = true;
                            },
                            _ => {}
                        }
                    }
                }
                Event::KeyUp {keycode, ..} => {
                    if let Some(key) = keycode {
                        match key {
                            Keycode::Num1 => {
                                chip8_emu.keys[1] = false;
                            },
                            Keycode::Num2 => {
                                chip8_emu.keys[2] = false;
                            },
                            Keycode::Num3 => {
                                chip8_emu.keys[3] = false;
                            },
                            Keycode::Num4 => {
                                chip8_emu.keys[12] = false;
                            },
                            Keycode::Q => {
                                chip8_emu.keys[4] = false;
                            },
                            Keycode::W => {
                                chip8_emu.keys[5] = false;
                            },
                            Keycode::E => {
                                chip8_emu.keys[6] = false;
                            },
                            Keycode::R => {
                                chip8_emu.keys[13] = false;
                            },
                            Keycode::A => {
                                chip8_emu.keys[7] = false;
                            },
                            Keycode::S => {
                                chip8_emu.keys[8] = false;
                            },
                            Keycode::D => {
                                chip8_emu.keys[9] = false;
                            },
                            Keycode::F => {
                                chip8_emu.keys[14] = false;
                            },
                            Keycode::Z => {
                                chip8_emu.keys[10] = false;
                            },
                            Keycode::X => {
                                chip8_emu.keys[0] = false;
                            },
                            Keycode::C => {
                                chip8_emu.keys[11] = false;
                            },
                            Keycode::V => {
                                chip8_emu.keys[15] = false;
                            },

                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();

        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("File") {
                if ui.menu_item("Open ROM") {
                    let file = AsyncFileDialog::new()
                        .add_filter("text", &["ch8"])
                        .pick_file()
                        .await;
                    let data = file.unwrap().read().await;
                    chip8_emu.load(&data[..]);
                }
                if ui.menu_item("Close ROM") {
                    chip8_emu.unload();
                }
                if ui.menu_item("Quit") {
                    break 'main;
                }
                menu.end();
            }
            menu_bar.end();
        }

        chip8_emu.tick();


        ui.window("Main")
            .size([64.0 * 20.0, 32.0 * 20.0], Condition::FirstUseEver)
            .build(|| {
                let draw_list = ui.get_window_draw_list();
                let current_pos = ui.cursor_screen_pos();
                let ws = ui.content_region_avail();

                /*
                Border, I may add this later
                draw_list.add_rect(
                    [current_pos[0] + ws[0] - 10.0, current_pos[1] + 9.0],
                    [current_pos[0] + 10.0, current_pos[1] + ws[1] - 9.0],
                    [0.0, 1.0, 0.0]
                ).build();
                */

                if chip8_emu.graphics_update {
                    for yl in 0..32 {
                        for xl in 0..64 {
                            let pixel_index = (xl + (yl * 64)) as usize;
                            if chip8_emu.screen[pixel_index] != 0 {
                                for i in 0..3 {
                                    chip8_emu.display_screen[pixel_index * 3 + i] = 255;
                                }
                            } else {
                                for i in 0..3 {
                                    chip8_emu.display_screen[pixel_index * 3 + i] = 0;
                                }
                            }
                        }
                    }

                    unsafe {
                        renderer.gl_context().bind_texture(TEXTURE_2D, Some(texture));
                        renderer.gl_context().tex_image_2d(TEXTURE_2D, 0, glow::RGB as i32, 64, 32, 0, glow::RGB, glow::UNSIGNED_BYTE, Some(&chip8_emu.display_screen));
                        renderer.gl_context().bind_texture(TEXTURE_2D, None);
                    }
                }

                draw_list.add_image(id, [current_pos[0] + 10.0, current_pos[1] + 10.0], [current_pos[0] + ws[0] - 10.0, current_pos[1] + ws[1] - 10.0]).build();
            });


        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();

        // around 500hz
        sleep(Duration::new(0, 2_000_000u32));
    }
}
