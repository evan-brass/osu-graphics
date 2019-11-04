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
	time::Instant,
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

struct ButtonStates {
	left: ElementState,
	middle: ElementState,
	right: ElementState
}
const WATER_SIZE: usize = 20;
struct Demo {
	pub wrapped_context: WindowedContext<PossiblyCurrent>,
	last_animate: Option<Instant>,
	yrot: f32,
	xrot: f32,
	scale: f32,
	pub helicopter: Helicopter,
	pub propeller: Propeller,
	propeller_rot: f32,
	pub axis: Axis,
	pub ship: Ship,
	pub ocean: Ocean,
	pub cone: Cone,
	pub light_0: Light,
	cockpit: bool,
	paused: bool,
	// fog_on: bool
	button_states: ButtonStates
}

#[derive(Debug)]
enum CustomEvents {
	Nothing,
}

impl Demo {
	fn new(wrapped_context: WindowedContext<PossiblyCurrent>) -> Demo {
		println!(
			"Pixel format of the window's GL context: {:?}",
			wrapped_context.get_pixel_format()
		);
		gl::load_with(|ptr| wrapped_context.context().get_proc_address(ptr) as *const _);

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
			helicopter: Helicopter::new(),
			axis: Axis::new(),
			ship: Ship::new(),
			propeller: Propeller::new(),
			propeller_rot: 0.0,
			ocean: Ocean::new(),
			cone: Cone::new(),
			light_0: Light::new(0),
			cockpit: false,
			paused: false
		}
	}
	fn init(&mut self) {
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
		}

		// self.ocean.init();
		self.axis.init();
		self.cone.init();
		self.light_0.place(3.0, 5.0, 1.0, 1.0);
		self.light_0.diffuse(255.0, 0.0, 0.0);
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

			gl::ShadeModel(gl::SMOOTH);

			gl::Rotatef(self.yrot, 0.0, 1.0, 0.0);
			gl::Rotatef(self.xrot, 1.0, 0.0, 0.0);
			gl::Scalef(self.scale, self.scale, self.scale);

			self.light_0.call();

			// Draw the objects:
			self.axis.draw();
			// gl::Scalef(0.01, 0.01, 0.01);
			gl::Enable(gl::LIGHTING);
			self.cone.draw();
			gl::Disable(gl::LIGHTING);

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
					let _diff = now.duration_since(last_inst).as_millis();
					// self.yrot += diff as f32 / 50.0;
					// self.ocean.animate(diff as f32);
					// self.propeller_rot += diff as f32 / 2.0;
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
		if !self.cockpit && self.button_states.left == ElementState::Pressed {
			self.yrot += diff_x;
			self.xrot += diff_y;
		}
	}
	fn toggle_cockpit(&mut self) {
		self.cockpit = !self.cockpit;
	}
	fn toggle_paused(&mut self) {
		self.paused = !self.paused;
	}
	fn scroll_delta(&mut self, diff_y: f32) {
		if !self.cockpit {
			self.scale -= diff_y / 80.0;
			if self.scale < 0.05 {
				self.scale = 0.05;
			}
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
	
	let mut demo = Demo::new(unsafe { windowed_context.make_current().unwrap() });

	demo.init();
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
							'c' | 'C' => demo.toggle_cockpit(),
							'f' | 'F' => demo.toggle_paused(),
							'm' | 'M' => demo.ocean.toggle_morph(),
							't' | 'T' => demo.ocean.toggle_texture(),
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
