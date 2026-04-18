
use easy_imgui::cgmath::Vector2;
use easy_imgui::{Color, DrawFlags, WindowFlags, lbl, lbl_id};
use easy_imgui_sdl3::{Application, sdl3};
use easy_imgui_sdl3::sdl3::video::{GLProfile, SwapInterval, WindowPos};

#[allow(dead_code)]
pub fn run() {
    let sdl = sdl3::init().unwrap();

    let sdl_video = sdl.video().unwrap();
    let sdl_event = sdl.event().unwrap();

    let gla = sdl_video.gl_attr();
    gla.set_context_version(3, 2);
    gla.set_context_profile(GLProfile::Core);
    gla.set_depth_size(0);
    let main_scale = sdl_video
        .get_primary_display()
        .unwrap()
        .get_content_scale()
        .unwrap();

    let mut window = sdl_video
        .window(
            "easy-imgui-sdl3 demo",
            (800.0 * main_scale) as u32,
            (600.0 * main_scale) as u32,
        )
        .opengl()
        .resizable()
        .hidden()
        .high_pixel_density()
        .build()
        .unwrap();
    let sdl_gl = window.gl_create_context().unwrap();
    window.gl_make_current(&sdl_gl).unwrap();
    let _ = sdl_video.gl_set_swap_interval(SwapInterval::VSync);
    window.set_position(WindowPos::Centered, WindowPos::Centered);
    window.show();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut app_handler = easy_imgui_sdl3::AppHandler::<App>::new(
        &easy_imgui::ContextBuilder::new(),
        &sdl_event,
        window,
        sdl_gl,
        (),
    );

    let io = app_handler.imgui_mut().io_mut();
    io.enable_docking(true);
    io.enable_viewports(true);
    app_handler.run(&mut event_pump);
}

struct App;

impl Application for App {
    type UserEvent = ();
    type Data = ();

    fn new(_args: easy_imgui_sdl3::Args<'_, Self>) -> Self {
        App
    }
}

impl easy_imgui::UiBuilder for App {
    fn do_ui(&mut self, ui: &easy_imgui::Ui<Self>) {
        ui.show_demo_window(None);

        ui.set_next_window_pos((10.0, 10.0).into(), easy_imgui::Cond::Always, (0.0, 0.0).into());
        ui.window_config(lbl("Instructions"))
            .flags(
                WindowFlags::NoMove
                    | WindowFlags::NoResize
                    | WindowFlags::NoNav
                    | WindowFlags::NoCollapse
                    | WindowFlags::NoDocking
            )
            .with(|| {
                ui.text("Use left-right arrow keys to change blue channel");
                ui.text("Use the mouse to change red and green channels");
                ui.text("Press ESC to show/hide the demo");
                if ui.button(lbl_id("Click Me to Crash", "crashy")) {
                    println!("JK");
                }
            });

        ui.set_next_window_pos((100., 10.0).into(), easy_imgui::Cond::Appearing, (0.0, 0.0).into());
        ui.window_config(lbl("Brick Breaker"))
            .flags(
                WindowFlags::NoResize
                    | WindowFlags::NoNav
                    | WindowFlags::NoCollapse
            )
            .with(|| {
                ui.invisible_button_config("test")
                    .size(Vector2::new(500., 500.))
                    .build();
                
                let bottom_right_pos = ui.get_cursor_screen_pos();
                let mouse_pos = ui.get_mouse_pos();

                let draw_list = ui.foreground_draw_list();

                draw_list.add_rect_filled(Vector2::new(mouse_pos.x, bottom_right_pos.y - 50.), Vector2::new(mouse_pos.x + 50., bottom_right_pos.y - 40.), Color::WHITE, 0., DrawFlags::None);
            });

    }
}