use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use glutin::dpi::LogicalSize;

use std::ffi::CStr;

pub mod bindings {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

struct Demo {
	wrapped_context: ContextWrapper<PossiblyCurrent, winit::window::Window>,
	gl: bindings::Gl
}

impl Demo {
	fn new(wrapped_context: ContextWrapper<PossiblyCurrent, winit::window::Window>) -> Demo {
		println!(
			"Pixel format of the window's GL context: {:?}",
			wrapped_context.get_pixel_format()
		);

		let gl = bindings::Gl::load_with(|ptr| wrapped_context.context().get_proc_address(ptr) as *const _);

		Demo { wrapped_context, gl }
	}
	fn init(&mut self) {
		let version = unsafe {
			let data = CStr::from_ptr(self.gl.GetString(bindings::VERSION) as *const _)
				.to_bytes()
				.to_vec();
			String::from_utf8(data).unwrap()
		};

		println!("OpenGL version {}", version);

		unsafe {
			let vs = self.gl.CreateShader(bindings::VERTEX_SHADER);
			let vs_src = include_bytes!("shaders/main.glslv");
			self.gl.ShaderSource(
				vs,
				1,
				// [include_bytes!("shaders/main.glslv").as_ptr() as *const _].as_ptr(),
				[vs_src.as_ptr() as *const _].as_ptr(),
				[vs_src.len() as i32].as_ptr(),
			);
			self.gl.CompileShader(vs);

			let fs = self.gl.CreateShader(bindings::FRAGMENT_SHADER);
			let fs_src = include_bytes!("shaders/main.glslf");
			self.gl.ShaderSource(
				fs,
				1,
				// [include_bytes!("shaders/main.glslf").as_ptr() as *const _].as_ptr(),
				[fs_src.as_ptr() as *const _].as_ptr(),
				[fs_src.len() as i32].as_ptr(),
			);
			self.gl.CompileShader(fs);

			let program = self.gl.CreateProgram();
			self.gl.AttachShader(program, vs);
			self.gl.AttachShader(program, fs);
			self.gl.LinkProgram(program);
			self.gl.UseProgram(program);
			
			#[rustfmt::skip]
			static VERTEX_DATA: [f32; 15] = [
				-0.5, -0.5,  1.0,  0.0,  0.0,
				0.0,  0.5,  0.0,  1.0,  0.0,
				0.5, -0.5,  0.0,  0.0,  1.0,
			];

			let mut vb = std::mem::zeroed();
			self.gl.GenBuffers(1, &mut vb);
			self.gl.BindBuffer(bindings::ARRAY_BUFFER, vb);
			self.gl.BufferData(
				bindings::ARRAY_BUFFER,
				(VERTEX_DATA.len() * std::mem::size_of::<f32>())
					as bindings::types::GLsizeiptr,
				VERTEX_DATA.as_ptr() as *const _,
				bindings::DYNAMIC_DRAW,
			);

			if self.gl.BindVertexArray.is_loaded() {
				let mut vao = std::mem::zeroed();
				self.gl.GenVertexArrays(1, &mut vao);
				self.gl.BindVertexArray(vao);
			}

			let pos_attrib =
				self.gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
			let color_attrib =
				self.gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
			self.gl.VertexAttribPointer(
				pos_attrib as bindings::types::GLuint,
				2,
				bindings::FLOAT,
				0,
				5 * std::mem::size_of::<f32>() as bindings::types::GLsizei,
				std::ptr::null(),
			);
			self.gl.VertexAttribPointer(
				color_attrib as bindings::types::GLuint,
				3,
				bindings::FLOAT,
				0,
				5 * std::mem::size_of::<f32>() as bindings::types::GLsizei,
				(2 * std::mem::size_of::<f32>()) as *const () as *const _,
			);
			self.gl.EnableVertexAttribArray(pos_attrib as bindings::types::GLuint);
			self.gl.EnableVertexAttribArray(color_attrib as bindings::types::GLuint);
		}
	}
	fn draw(&mut self) {
		println!("Drawing.");
		unsafe {
            self.gl.ClearColor(1.0, 0.5, 0.7, 1.0);
            self.gl.Clear(bindings::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(bindings::TRIANGLES, 0, 3);
        }
		self.wrapped_context.swap_buffers().unwrap();
	}
	fn resized(&mut self, logical_size: &LogicalSize) {
		let dpi_factor = self.wrapped_context.window().hidpi_factor();
		self.wrapped_context.resize(logical_size.to_physical(dpi_factor));
	}
}

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

    let windowed_context =
        ContextBuilder::new().build_windowed(wb, &event_loop).unwrap();
	
	let mut demo = Demo::new(unsafe { windowed_context.make_current().unwrap() });

	demo.init();

    event_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
					demo.resized(logical_size);
                }
                WindowEvent::RedrawRequested => {
					demo.draw();
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    });
}
