#![allow(bad_style)]
#![allow(unused)]
/**
 * This code is based largely off of two examples / tutorials:
 * https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/window.rs
 * https://wiki.alopex.li/LearningGfx
 *
 * I've just been trimming it down to what I actually need to then recreate the sample
 * project that Dr. Bailey has given us.
 */
use glutin::{
	dpi::LogicalPosition,
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
	Api::OpenGl,
	ContextBuilder, GlRequest,
};

use cgmath::{Deg, Matrix, Matrix4, PerspectiveFov, Point3, Vector3};

use std::{
	cell::RefCell, 
	rc::Rc, 
	time::Duration,
	mem::{
		zeroed,
		size_of,
		transmute
	},
	ffi::c_void
};

mod gl;

// mod mesh;
// use mesh::*;
// mod light;
// use light::*;
// mod material;
// use material::*;
mod demo;
use demo::*;

fn print_program_data(program: u32) {
	unsafe {
		let mut count = zeroed();
		gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut count);
		println!("Active Attributes: {}", count);
		for i in 0..count {
			const BUFFER_SIZE: usize = 16;
			#[derive(Debug)]
			struct AttributeInfo {
				name: [i8; BUFFER_SIZE],
				length: i32,
				size: i32,
				kind: u32
			}
			let mut attr = AttributeInfo {
				name: zeroed(),
				length: 0,
				size: 0,
				kind: 0
			};
			gl::GetActiveAttrib(
				program, i as u32, BUFFER_SIZE as i32, 
				&mut attr.length, 
				&mut attr.size, 
				&mut attr.kind, 
				&mut attr.name[0] as *mut i8
			);

			println!("Attribute {:?}", attr);
			println!("Attribute Name: {}", std::str::from_utf8(
				transmute::<&[i8], &[u8]>(&attr.name)
			).unwrap());
			println!("Attribute Type: {}", match(attr.kind) {
				gl::FLOAT => "float",
				gl::FLOAT_VEC3 => "vec3 float",
				_ => "Unknown type"
			});
		}
		let mut count = zeroed();
		gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut count);
		println!("Active Uniforms: {}", count);
		for i in 0..count {
			const BUFFER_SIZE: usize = 16;
			#[derive(Debug)]
			struct UniformInfo {
				name: [i8; BUFFER_SIZE],
				length: i32,
				size: i32,
				kind: u32
			}
			let mut attr = UniformInfo {
				name: zeroed(),
				length: 0,
				size: 0,
				kind: 0
			};
			gl::GetActiveUniform(
				program, i as u32, BUFFER_SIZE as i32, 
				&mut attr.length, 
				&mut attr.size, 
				&mut attr.kind, 
				&mut attr.name[0] as *mut i8
			);

			println!("Uniform {:?}", attr);
			println!("Uniform Name: {}", std::str::from_utf8(
				transmute::<&[i8], &[u8]>(&attr.name)
			).unwrap());
			println!("Uniform Type: {}", match(attr.kind) {
				gl::FLOAT => "float",
				gl::FLOAT_MAT4 => "mat4 float",
				gl::FLOAT_VEC3 => "vec3 float",
				_ => "Unknown type"
			});
		}
	}
}

fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

	let windowed_context = ContextBuilder::new()
		.with_gl(GlRequest::Latest)
		.with_vsync(true)
		.build_windowed(wb, &event_loop)
		.expect("Unable to build windowed Context");

	const CHUNK_WIDTH: f32 = 10.0;
	const CHUNK_HEIGHT: f32 = 10.0;
	const CHUNK_DEPTH: f32 = 10.0;
	const CHUNK_SIZE_WIDTH: usize = 5;
	const CHUNK_SIZE_HEIGHT: usize = 5;
	const CHUNK_SIZE_DEPTH: usize = 5;
	const NUM_ITEMS: usize = CHUNK_SIZE_WIDTH * CHUNK_SIZE_HEIGHT * CHUNK_SIZE_DEPTH;
	// #[repr(packed)] // TODO: Check if this helps...
	#[derive(Clone, Copy, Debug)]
	struct ChunkItem {
		pub size: f32,
		pub color: (f32, f32, f32)
	}
	#[derive(Clone)]
	struct Chunk {
		pub buffer_id: u32,
		pub x: f32,
		pub y: f32,
		pub z: f32
	}
	struct ChunkItemAccessor {
		pub index: usize,
		buffer_id: u32
	}
	impl ChunkItemAccessor {
		fn get(&self) -> ChunkItem {
			let mut ret: ChunkItem = unsafe { zeroed() };
			unsafe {
				gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
				gl::GetBufferSubData(
					gl::ARRAY_BUFFER,
					(self.index * size_of::<ChunkItem>()) as isize,
					size_of::<ChunkItem>() as isize,
					transmute::<&mut ChunkItem, &mut c_void>(&mut ret)
				);
				gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			}
			ret
		}
		fn set(&self, new_value: ChunkItem) {
			unsafe {
				gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
				gl::BufferSubData(
					gl::ARRAY_BUFFER,
					(self.index * size_of::<ChunkItem>()) as isize,
					size_of::<ChunkItem>() as isize,
					transmute::<&ChunkItem, &c_void>(&new_value)
				);
				gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			}
		}
	}

	// TODO: Implement array access for the chunk that fetches / updates GPU memory:
	//  - Update: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferSubData.xhtml
	//  - Fetch: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferSubData.xhtml
	impl Chunk {
		fn new(x: f32, y: f32, z: f32) -> Chunk {
			let mut ret = Chunk {
				x, y, z,
				buffer_id: unsafe { zeroed() }
			};
			unsafe {
				gl::GenBuffers(1, &mut ret.buffer_id);
				// Allocate the size of the buffer that we need without filling it with any data:
				gl::BindBuffer(gl::ARRAY_BUFFER, ret.buffer_id);
				gl::BufferData(
					gl::ARRAY_BUFFER, 
					(size_of::<ChunkItem>() * NUM_ITEMS) as isize, 
					0 as *const c_void, 
					gl::DYNAMIC_DRAW
				);
			}
			ret
		}
		fn access(&self, x: usize, y: usize, z: usize) -> ChunkItemAccessor {
			ChunkItemAccessor {
				index: z + CHUNK_SIZE_DEPTH * y + CHUNK_SIZE_DEPTH * CHUNK_SIZE_HEIGHT * x,
				buffer_id: self.buffer_id
			}
		}
		fn access_index(&self, index: usize) -> ChunkItemAccessor {
			ChunkItemAccessor {
				index,
				buffer_id: self.buffer_id
			}
		}

		fn draw(&self) {
			unsafe {
				gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
				gl::DrawArrays(gl::POINTS, 0, NUM_ITEMS as i32);
				gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			}
		}
		fn init() {
			unsafe fn add_shader(program: u32, stage: u32, source: &[u8]) {
				let id = gl::CreateShader(stage);
				gl::ShaderSource(
					id,
					1,
					[source.as_ptr() as *const _].as_ptr(),
					[source.len() as i32].as_ptr(),
				);
				gl::CompileShader(id);
				gl::AttachShader(program, id);
			}
			unsafe {
				// Create the voxel shader program:
				let program = gl::CreateProgram();

				// Vertex Shader:
				add_shader(program, gl::VERTEX_SHADER, include_bytes!("shaders/voxel.glslv"));
				// Geometry Shader:
				add_shader(program, gl::GEOMETRY_SHADER, include_bytes!("shaders/voxel.glslg"));
				// Fragment Shader:
				add_shader(program, gl::FRAGMENT_SHADER, include_bytes!("shaders/voxel.glslf"));

				gl::LinkProgram(program);
				gl::UseProgram(program);

				// DEBUG:
				print_program_data(program);
				
				if gl::BindVertexArray::is_loaded() {
					let mut vao = std::mem::zeroed();
					gl::GenVertexArrays(1, &mut vao);
					gl::BindVertexArray(vao);
				}

				// TODO: Set attribute locations
				let temp = ChunkItem { size: 1.0, color: (1.0, 1.0, 1.0) };
				
				let location = gl::GetAttribLocation(program, b"size\0".as_ptr() as *const _);
				let offset = &temp.size as *const _ as usize - &temp as *const _ as usize;
				let size = size_of::<ChunkItem>();
				if (location != -1) {
					gl::VertexAttribPointer(
						location as u32, // Specified  in vert shader
						1,
						gl::FLOAT,
						gl::FALSE,
						size as i32,
						// Measure the distance to the start of the struct field and use that as the offset:
						offset as *const c_void,
					);
					gl::EnableVertexAttribArray(location as u32);
				} else {
					println!("Attribute size didn't appear in the shader program.");
				}
				let location = gl::GetAttribLocation(program, "color\0".as_ptr() as *const _);
				let offset = &temp.color as *const _ as usize - &temp as *const _ as usize;
				let size = size_of::<ChunkItem>();
				if (location != -1) {
					gl::VertexAttribPointer(
						location as u32,
						3,
						gl::FLOAT,
						gl::FALSE,
						size as i32,
						// Measure the distance to the start of the struct field and use that as the offset:
						offset as *const c_void,
					);
					gl::EnableVertexAttribArray(location as u32);
				} else {
					println!("Attribute color didn't appear in the shader program.");
				}

				// Setup Uniform variables:
				let location = gl::GetUniformLocation(program, b"projection\0".as_ptr() as *const _);
				if (location != -1) {
					// This is essentially a gluPerspective call
					let projection: Matrix4<f32> = PerspectiveFov {
						fovy: Deg(90.0).into(),
						aspect: 1.0,
						near: 0.1,
						far: 1000.0,
					}
					.into();
					gl::UniformMatrix4fv(
						location,
						1,
						gl::FALSE,
						projection.as_ptr()
					);
				} else {
					println!("Uniform projection didn't appear in the shader program.");
				}
				let location = gl::GetUniformLocation(program, b"chunk_transform\0".as_ptr() as *const _);
				if (location != -1) {
					// This is essentially a gluLookAt call
					let look_at: Matrix4<f32> = Matrix4::look_at(
						Point3::new(-1.0, 2.0, 3.0), // Eye location
						Point3::new(0.0, 0.0, 0.0), // Center Point / Point of interest
						Vector3::new(0.0, 1.0, 0.0), // Up vector
					);
					gl::UniformMatrix4fv(
						location,
						1,
						gl::FALSE,
						look_at.as_ptr()
					);
				} else {
					println!("Uniform projection didn't appear in the shader program.");
				}
		
			}
		}
	}
	
	let mut demo = Demo::new(
		unsafe { windowed_context.make_current().unwrap() },
		// All the scene items
		vec![&mut || {
			struct VoxelProject {
				chunks: Vec<Chunk>
			}
			impl SceneItem for VoxelProject {
				fn anim(&mut self, update: Duration) {
					
				}
				fn draw(&self) {
					for chunk in self.chunks.iter() {
						chunk.draw();
					}
				}
			}

			// Compile / init shader pipeline
			Chunk::init();

			let mut chunk = Chunk::new(0.0, 0.0, 0.0);
			for i in 0..NUM_ITEMS {
				let accessor = chunk.access_index(i);
				// println!("Before: {:?}", accessor.get());
				accessor.set(ChunkItem {
					size: (i as f32 + 1.0) / NUM_ITEMS as f32,
					color: (1.0, 1.0, 1.0)
				});
				// println!("After: {:?}", accessor.get());
			}

			Box::new(VoxelProject {
				chunks: vec![chunk]
			})
		}],
	);
	demo.animate(); // Initialize variables and

	// Proxy used for sending commands from the context menu to the main loop
	event_loop.run(move |event, window_target, control_flow| {
		// println!("{:?}", event);
		*control_flow = ControlFlow::Poll;

		demo.handle_event(event, window_target, control_flow);
	});
}
