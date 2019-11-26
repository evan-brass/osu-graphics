#![allow(bad_style)]
#![allow(unused)]
use glutin::{
	dpi::{LogicalPosition, LogicalSize},
	event::*,
	event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
	window::WindowBuilder,
	Api::OpenGl,
	ContextBuilder, GlRequest, PossiblyCurrent, WindowedContext,
};

use cgmath::{Deg, Matrix4, PerspectiveFov, Point3, Vector3};

use std::{
	cell::RefCell,
	ffi::CStr,
	rc::Rc,
	thread,
	time::{Duration, Instant},
};

use crate::gl;
use crate::gl::types::*;
use crate::light::*;
use crate::material::*;
use crate::mesh::*;

pub trait SceneItem {
	fn anim(&mut self, update: Duration);
	fn draw(&self);
}

pub struct ButtonStates {
	pub left: ElementState,
	pub middle: ElementState,
	pub right: ElementState,
}

pub struct Demo {
	pub wrapped_context: WindowedContext<PossiblyCurrent>,

	last_animate: Option<Instant>,
	pub button_states: ButtonStates,
	paused: bool,

	yrot: f32,
	xrot: f32,
	scale: f32,

	scene_items: Vec<Box<dyn SceneItem>>,
}

impl Demo {
	pub fn new(
		wrapped_context: WindowedContext<PossiblyCurrent>,
		scene_items: Vec<&mut dyn FnMut() -> Box<dyn SceneItem>>,
	) -> Demo {
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
			// 			gl::Enable( gl::NORMALIZE );
			// gl::Enable(gl::COLOR_MATERIAL);
		}

		let scene_items = scene_items.into_iter().map(|item| item()).collect();

		Demo {
			wrapped_context,
			last_animate: None,
			yrot: 0.0,
			xrot: 0.0,
			scale: 0.5,
			// fog_on: false
			button_states: ButtonStates {
				left: ElementState::Released,
				middle: ElementState::Released,
				right: ElementState::Released,
			},
			scene_items,
			paused: false,
		}
	}
	pub fn draw(&mut self) {
		unsafe {
			gl::DrawBuffer(gl::BACK);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			gl::MatrixMode(gl::PROJECTION);

			// This is essentially a gluPerspective call
			let projection: Matrix4<f32> = PerspectiveFov {
				fovy: Deg(90.0).into(),
				aspect: 1.0,
				near: 0.1,
				far: 1000.0,
			}
			.into();
			let arr: &[f32; 16] = projection.as_ref();
			gl::LoadMatrixf(arr.as_ptr());

			gl::MatrixMode(gl::MODELVIEW);

			// This is essentially a gluLookAt call
			let mat: Matrix4<f32> = Matrix4::look_at(
				Point3::new(-1.0, 2.0, 3.0),
				Point3::new(0.0, 0.0, 0.0),
				Vector3::new(0.0, 1.0, 0.0),
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
	pub fn animate(&mut self) {
		let now = Instant::now();
		if !self.paused {
			match self.last_animate {
				None => {
					// We haven't animated yet.
				}
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
	pub fn resized(&mut self, logical_size: &LogicalSize) {
		let dpi_factor = self.wrapped_context.window().hidpi_factor();
		self.wrapped_context
			.resize(logical_size.to_physical(dpi_factor));
	}
	pub fn mouse_move(&mut self, diff_x: f32, diff_y: f32) {
		if self.button_states.left == ElementState::Pressed {
			self.yrot += diff_x;
			self.xrot += diff_y;
		}
	}
	pub fn toggle_paused(&mut self) {
		self.paused = !self.paused;
	}
	pub fn scroll_delta(&mut self, diff_y: f32) {
		self.scale -= diff_y / 80.0;
		if self.scale < 0.05 {
			self.scale = 0.05;
		}
	}
	pub fn handle_event(
		&mut self,
		event: Event<()>,
		_window_target: &EventLoopWindowTarget<()>,
		control_flow: &mut ControlFlow,
	) {
		match event {
			// Event::LoopDestroyed => {},
			Event::WindowEvent {
				ref event,
				window_id,
			} => {
				match event {
					WindowEvent::Resized(logical_size) => {
						self.resized(logical_size);
					}
					WindowEvent::RedrawRequested => {
						self.draw();
					}
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
					WindowEvent::ReceivedCharacter(ch) => match ch {
						// Handle Magic Keys
						// 'c' | 'C' => self.toggle_cockpit(),
						'f' | 'F' => self.toggle_paused(),
						// 'm' | 'M' => self.ocean.toggle_morph(),
						// 't' | 'T' => self.ocean.toggle_texture(),
						_ => {}
					},
					_ => {}
				}
			}
			Event::DeviceEvent { ref event, .. } => match event {
				// Pass mouse movement to the demo:
				DeviceEvent::MouseMotion { delta: (x, y) } => self.mouse_move(*x as f32, *y as f32),
				// Pass the mouse scroll to the demo:
				DeviceEvent::MouseWheel { delta } => match delta {
					MouseScrollDelta::LineDelta(_, forward) => {
						let y = -forward;
						self.scroll_delta(y);
					}
					MouseScrollDelta::PixelDelta(LogicalPosition { y, .. }) => {
						self.scroll_delta(*y as f32);
					}
				},
				// Keep the button states up to date:
				DeviceEvent::Button { button: 1, state } => {
					self.button_states.left = *state;

					// This was part of my attempt at a context menu:
					/* match state {
						ElementState::Pressed => {
							proxy.send_event(CustomEvents::Nothing).unwrap();
						},
						_ => ()
					};
					*/
				}
				DeviceEvent::Button { button: 2, state } => self.button_states.middle = *state,
				DeviceEvent::Button { button: 3, state } => {
					self.button_states.right = *state;
				}
				_ => {}
			},
			Event::NewEvents(StartCause::Poll) => {
				self.animate();
			}
			_ => {}
		}
	}
}
