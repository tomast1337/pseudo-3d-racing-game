mod assets;
mod directions;
mod graphics;
mod math;
mod player;
mod render;
mod road;

use assets::{Assets, Biome};
use directions::{MoveDirection, TurnDirection};
use glow::HasContext;
use graphics::Renderer;
use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use player::Player;
use winit::raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

struct GlState {
    surface: Surface<WindowSurface>,
    context: glutin::context::PossiblyCurrentContext,
    gl: glow::Context,
    renderer: Renderer,
    assets: Assets,
}

struct App {
    template: ConfigTemplateBuilder,
    display_builder: DisplayBuilder,
    gl_context: Option<NotCurrentContext>,
    gl_state: Option<GlState>,
    window: Option<Window>,
    player: Option<Player>,
    biome: Biome,
    road_scroll: f32,
    keys: InputState,
    last_frame: Instant,
    display_initialized: bool,
}

#[derive(Default)]
struct InputState {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

fn pick_gl_config(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);
            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .expect("no GL config")
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(raw_window_handle);
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let gl_display = gl_config.display();
    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .or_else(|_| gl_display.create_context(gl_config, &fallback_context_attributes))
            .or_else(|_| gl_display.create_context(gl_config, &legacy_context_attributes))
            .expect("failed to create GL context")
    }
}

impl App {
    fn new() -> Self {
        let template = ConfigTemplateBuilder::new().with_alpha_size(8);
        let window_attributes = Window::default_attributes()
            .with_title("Out Run Clone")
            .with_inner_size(winit::dpi::LogicalSize::new(
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            ));
        let display_builder =
            DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        Self {
            template,
            display_builder,
            gl_context: None,
            gl_state: None,
            window: None,
            player: None,
            biome: Biome::Tropical,
            road_scroll: 0.0,
            keys: InputState::default(),
            last_frame: Instant::now(),
            display_initialized: false,
        }
    }

    fn init_gl(&mut self, window: Window, gl_config: Config) {
        let gl_display = gl_config.display();
        let gl_context = self
            .gl_context
            .take()
            .unwrap_or_else(|| create_gl_context(&window, &gl_config));

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("surface attributes");
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("create surface")
        };

        let gl_context = gl_context.make_current(&gl_surface).unwrap();
        let _ = gl_surface.set_swap_interval(
            &gl_context,
            SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
        );

        let gl = unsafe {
            glow::Context::from_loader_function(|symbol| {
                let symbol = std::ffi::CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()) as *const _
            })
        };

        let size = window.inner_size();
        let width = size.width.max(1) as f32;
        let height = size.height.max(1) as f32;

        let renderer = unsafe { Renderer::new(&gl, width, height) };
        let assets = unsafe { Assets::load(&gl) };
        let player = Player::new(
            math::Vec2::new(width / 2.0, height / 1.3),
            200.0,
            360.0,
            240.0,
            width,
        );

        self.window = Some(window);
        self.gl_state = Some(GlState {
            surface: gl_surface,
            context: gl_context,
            gl,
            renderer,
            assets,
        });
        self.player = Some(player);
    }

    fn update_input(&mut self) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        player.movement = if self.keys.s {
            MoveDirection::Brake
        } else if self.keys.w {
            MoveDirection::Forward
        } else {
            MoveDirection::Coast
        };

        if player.speed > 0.0 {
            if self.keys.a && !self.keys.d {
                player.turn = TurnDirection::Left;
            } else if self.keys.d && !self.keys.a {
                player.turn = TurnDirection::Right;
            } else if !self.keys.a && !self.keys.d {
                player.turn = TurnDirection::None;
            }
        }
    }

    fn tick(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32().min(0.05);
        self.last_frame = now;

        self.update_input();
        if let Some(player) = self.player.as_mut() {
            player.update_player(dt);
            self.road_scroll += player.speed * dt * 0.02;
        }

        if let (Some(gl_state), Some(player)) = (&self.gl_state, &self.player) {
            unsafe {
                render::draw_scene(
                    &gl_state.gl,
                    &gl_state.renderer,
                    &gl_state.assets,
                    player,
                    self.biome,
                    self.road_scroll,
                );
            }
            let _ = gl_state.surface.swap_buffers(&gl_state.context);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gl_state.is_some() {
            return;
        }

        if !self.display_initialized {
            let (window, gl_config) = self
                .display_builder
                .clone()
                .build(event_loop, self.template.clone(), pick_gl_config)
                .expect("build display");
            self.display_initialized = true;
            self.gl_context = Some(create_gl_context(window.as_ref().unwrap(), &gl_config));
            let window = window.unwrap();
            self.init_gl(window, gl_config);
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                if let Some(gl_state) = &mut self.gl_state {
                    gl_state.renderer.resize(size.width as f32, size.height as f32);
                    gl_state.surface.resize(
                        &gl_state.context,
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );
                    unsafe {
                        gl_state
                            .gl
                            .viewport(0, 0, size.width as i32, size.height as i32);
                    }
                }
                if let Some(player) = &mut self.player {
                    player.set_screen_width(size.width as f32);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => self.keys.w = pressed,
                    PhysicalKey::Code(KeyCode::KeyA) => self.keys.a = pressed,
                    PhysicalKey::Code(KeyCode::KeyS) => self.keys.s = pressed,
                    PhysicalKey::Code(KeyCode::KeyD) => self.keys.d = pressed,
                    PhysicalKey::Code(KeyCode::KeyB) if pressed => {
                        self.biome = self.biome.next();
                    }
                    PhysicalKey::Code(KeyCode::KeyP) if pressed => {
                        if let Some(player) = &self.player {
                            println!("{player} biome={:?}", self.biome);
                        }
                    }
                    PhysicalKey::Code(KeyCode::Escape) if pressed => event_loop.exit(),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.gl_state.is_some() {
            self.tick();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    let mut app = App::new();
    event_loop.run_app(&mut app).expect("run app");
}
