/*
TODO: MAKE THIS SHIT CLEANER LOL

*/


mod chip8;
mod menu_bar;
mod keypad;

use std::thread::sleep;
use std::time::Duration;
use glow::{HasContext, NEAREST, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNPACK_ROW_LENGTH};
use imgui::{Condition, Context, TextureId};
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
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
        renderer.gl_context().bind_texture(TEXTURE_2D, None);
    }


    let mut event_pump = sdl.event_pump().unwrap();
    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {keycode, ..} => {
                    if let Some(key) = keycode {
                        chip8_emu.keypad_binds(key, true);
                    }
                }
                Event::KeyUp {keycode, ..} => {
                    if let Some(key) = keycode {
                        chip8_emu.keypad_binds(key, false);
                    }
                }
                _ => {}
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();

        chip8_emu.draw_menu_bar(ui, &mut running).await;

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

                chip8_emu.draw(renderer.gl_context(), texture);

                draw_list.add_image(id, [current_pos[0] + 10.0, current_pos[1] + 10.0], [current_pos[0] + ws[0] - 10.0, current_pos[1] + ws[1] - 10.0]).build();
            });


        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();

        // around 500hz
        sleep(Duration::new(0, 500_000u32));
    }
}
