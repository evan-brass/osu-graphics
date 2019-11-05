#![allow(dead_code)]
#![allow(unused_imports)]
/**
 * This code is based largely off of two examples / tutorials:
 * https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/window.rs
 * https://wiki.alopex.li/LearningGfx
 * 
 * I've just been trimming it down to what I actually need to then recreate the sample 
 * project that Dr. Bailey has given us.
 */
use glutin::{
	event_loop::{ControlFlow, EventLoop},
	event::*,
	window::WindowBuilder,
	GlRequest, ContextBuilder, WindowedContext, PossiblyCurrent,
	dpi::{LogicalSize, LogicalPosition},
	Api::OpenGl
};

use cgmath::{Matrix4, Deg, PerspectiveFov, Point3, Vector3};

use std::{
	time::{
		Instant,
		Duration
	},
	ffi::CStr,
	thread, 
	sync::{Arc, Mutex}
};

mod gl;
use gl::types::*;

mod mesh;
use mesh::*;
mod light;
use light::*;
mod material;
use material::*;

trait SceneItem {
	fn init(&mut self);
	fn anim(&mut self, diff: Duration);
	fn draw(&mut self);
}


struct ButtonStates {
	left: ElementState,
	middle: ElementState,
	right: ElementState
}
const WATER_SIZE: usize = 20;
struct Demo {
	pub wrapped_context: WindowedContext<PossiblyCurrent>,

	last_animate: Option<Instant>,
	button_states: ButtonStates,
	paused: bool,

	yrot: f32,
	xrot: f32,
	scale: f32,

	scene_items: Vec<Box::<dyn SceneItem>>
}

#[derive(Debug)]
enum CustomEvents {
	Nothing,
}

impl Demo {
	fn new(wrapped_context: WindowedContext<PossiblyCurrent>, scene_items: Vec<Box::<dyn SceneItem>>) -> Demo {
		println!(
			"Pixel format of the window's GL context: {:?}",
			wrapped_context.get_pixel_format()
		);
		gl::load_with(|ptr| wrapped_context.context().get_proc_address(ptr) as *const _);

		
		let version = unsafe {
			let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
				.to_bytes()
				.to_vec();
			String::from_utf8(data).unwrap()
		};
		println!("OpenGL version {}", version);
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Enable(gl::DEPTH_TEST);
			// since we are using glScalef( ), be sure normals get unitized:
			gl::Enable( gl::NORMALIZE );
			// gl::Enable(gl::COLOR_MATERIAL);
		}

		for item in scene_items {
			item.init();
		}

		Demo { 
			wrapped_context,
			last_animate: None,
			yrot: 0.0, xrot: 0.0,
			scale: 0.5,
			// fog_on: false
			button_states: ButtonStates {
				left: ElementState::Released,
				middle: ElementState::Released,
				right: ElementState::Released
			},
			scene_items,
			paused: false
		}
	}
	fn draw(&mut self) {
		unsafe {
			gl::DrawBuffer( gl::BACK );
			gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );

			gl::MatrixMode( gl::PROJECTION );

			// This is essentially a gluPerspective call
			let projection: Matrix4<f32> = PerspectiveFov {
				fovy: Deg(90.0).into(), 
				aspect: 1.0, 
				near: 0.1, 
				far: 1000.0
			}.into();
			let arr: &[f32; 16] = projection.as_ref();
			gl::LoadMatrixf(arr.as_ptr());

			gl::MatrixMode( gl::MODELVIEW );

			// This is essentially a gluLookAt call
			let mat: Matrix4<f32> = Matrix4::look_at(
				Point3::new(-1.0, 2.0, 3.0), 
				Point3::new(0.0, 0.0, 0.0),
				Vector3::new(0.0, 1.0, 0.0)
			);
			let arr: &[f32; 16] = mat.as_ref();
			gl::LoadMatrixf(arr.as_ptr());

			// gl::ShadeModel(gl::SMOOTH);

			// Rotate the scene using the mouse
			gl::Rotatef(self.yrot, 0.0, 1.0, 0.0);
			gl::Rotatef(self.xrot, 1.0, 0.0, 0.0);
			gl::Scalef(self.scale, self.scale, self.scale);

			// Draw all the scene items:
			for item in &mut self.scene_items {
				item.draw();
			}

			gl::Flush();
        }
		self.wrapped_context.swap_buffers().unwrap();
	}
	fn animate(&mut self) {
		let now = Instant::now();
		if !self.paused {
			match self.last_animate {
				None => {
					// We haven't animated yet.
				},
				Some(last_inst) => {
					// Diff is the # of miliseconds since the last animate call.
					let diff = now.duration_since(last_inst);
					for item in &mut self.scene_items {
						item.anim(diff);
					}
				}
			}
		}
		self.last_animate = Some(now);
		self.wrapped_context.window().request_redraw();
	}
	fn resized(&mut self, logical_size: &LogicalSize) {
		let dpi_factor = self.wrapped_context.window().hidpi_factor();
		self.wrapped_context.resize(logical_size.to_physical(dpi_factor));
	}
	fn mouse_move(&mut self, diff_x: f32, diff_y: f32) {
		if self.button_states.left == ElementState::Pressed {
			self.yrot += diff_x;
			self.xrot += diff_y;
		}
	}
	fn toggle_paused(&mut self) {
		self.paused = !self.paused;
	}
	fn scroll_delta(&mut self, diff_y: f32) {
		self.scale -= diff_y / 80.0;
		if self.scale < 0.05 {
			self.scale = 0.05;
		}
	}
}

fn main() {
    let event_loop = EventLoop::<CustomEvents>::with_user_event();
    let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

    let windowed_context =
        ContextBuilder::new()
		.with_gl(GlRequest::Specific(OpenGl, (2, 1)))
		.with_vsync(true)
		.build_windowed(wb, &event_loop).unwrap();
	
	struct LightingProject {
		torus: Torus,
		light_1: Light,
		mat_1: Material,
		light_motion: f32
	}
	imple LightingProject {
		fn new() -> Self {
			LightingProject {

			}
		}
	}
	impl SceneItem for LightingProject {
		fn init() {

		}
		fn anim(&mut self, diff: Duration) {
			self.light_motion += diff.as_secs_f32();
			self.light_1.place(0.0, 20.0 * self.light_motion.sin(), 0.0, 1.0);
		}
		fn draw(&mut self) {
			unsafe {
				self.light_1.call();
				gl::Enable(gl::LIGHTING);
				self.mat_1.call();
				gl::ShadeModel(gl::FLAT);
				self.torus.draw();
				gl::Disable(gl::LIGHTING);

			}
		}
	}
	let lighting_project = LightingProject {
		torus: {
			let mut torus = Torus::new();
			// torus.major_segments = 100;
			// torus.minor_segments = 50;
			torus
		},
		light_1: {
			let mut light = Light::new(0);
			light.attenuate(1.0, 0.0, 0.0);
			light.diffuse(244.0, 179.0, 147.0);
			light.specular(244.0, 179.0, 147.0);
			// Light::use_ambient(0.1, 0.1, 0.1);
			light
		},
		mat_1: {
			let mut material = Material::new(ActiveFace::Front);
			material.diffuse(122.0, 40.0, 203.0);
			material
		},
		light_motion: 0.0
	};

	let mut demo = Demo::new(
		unsafe { windowed_context.make_current().unwrap() },
		// All the scene items
		vec![
			Box::new(lighting_project)
		]
	);
	demo.animate(); // Initialize variables and

	// Proxy used for sending commands from the context menu to the main loop
	let proxy = event_loop.create_proxy();
    event_loop.run(move |event, _window_target, control_flow| {
		// println!("{:?}", event);
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, window_id } => {
				if window_id == demo.wrapped_context.window().id() {
					match event {
						WindowEvent::Resized(logical_size) => {
							demo.resized(logical_size);
						}
						WindowEvent::RedrawRequested => {
							demo.draw();
						}
						WindowEvent::CloseRequested => {
							*control_flow = ControlFlow::Exit
						},
						WindowEvent::ReceivedCharacter(ch) => match ch {
							// Handle Magic Keys
							// 'c' | 'C' => demo.toggle_cockpit(),
							'f' | 'F' => demo.toggle_paused(),
							// 'm' | 'M' => demo.ocean.toggle_morph(),
							// 't' | 'T' => demo.ocean.toggle_texture(),
							_ => ()
						},
						_ => (),
					}
				} else {

				}
            },
			Event::DeviceEvent{ ref event, .. } => match event {
				// Pass mouse movement to the demo:
				DeviceEvent::MouseMotion{ delta: (x, y) } => demo.mouse_move(*x as f32, *y as f32),
				// Pass the mouse scroll to the demo:
				DeviceEvent::MouseWheel{ delta } => match delta {
					MouseScrollDelta::LineDelta(_, forward) => {
						let y = -forward;
						demo.scroll_delta(y);
					},
					MouseScrollDelta::PixelDelta(LogicalPosition{ y, .. }) => {
						demo.scroll_delta(*y as f32);
					}
				},
				// Keep the button states up to date:
				DeviceEvent::Button{button: 1, state } => {
					demo.button_states.left = *state;
					match state {
						ElementState::Pressed => {
							proxy.send_event(CustomEvents::Nothing).unwrap();
						},
						_ => ()
					};
				},
				DeviceEvent::Button{button: 2, state } => demo.button_states.middle = *state,
				DeviceEvent::Button{button: 3, state } => {
					demo.button_states.right = *state;
				},
				_ => ()
			},
			Event::NewEvents(StartCause::Poll) => {
				demo.animate();
			},
            _ => (),
        }
    });
}
