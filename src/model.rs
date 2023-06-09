use math;
use gl;
use std::ffi::CString;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::fs::File;
use rand::Rng;
use crate::render_gl::{self, buffer, texture};
use sdl2::keyboard::Keycode;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex
{
	position: math::vector::Vector3,
	normal: math::vector::Vector3,
	texcoord: math::vector::Vector2,
	color: math::vector::Vector3
}

impl Vertex
{
	pub fn new(position: math::vector::Vector3, color: math::vector::Vector3, texcoord: math::vector::Vector2) -> Self
	{
		Self {
			position,
			normal: (0.0, 0.0, 0.0).into(),
			texcoord,
			color
		}
	}
}

pub struct Mesh
{
	vertices: Vec<Vertex>,
	indices: Vec<u32>,
	// textures: Vec<texture::Texture>,
	texture: texture::Texture,
	vao: buffer::VertexArray,
	vbo: buffer::ArrayBuffer,
	ebo: buffer::ElementArrayBuffer,
	program: render_gl::Program,
	model_mat: math::matrix::Matrix4
}

impl Mesh
{
	// basic constructor from raw data (could use it to re-create the triangla/square/cube objects)
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, program: render_gl::Program, tex_path: &str) -> Self
	{
		let vao = buffer::VertexArray::new();
		let ebo = buffer::ElementArrayBuffer::new();
		let vbo = buffer::ArrayBuffer::new();
		let texture = texture::Texture::new();
		texture.load(tex_path);

		let mesh = Mesh {
			vertices,
			indices,
			texture,
			vao,
			vbo,
			ebo,
			program,
			model_mat: math::matrix::Matrix4::new_identity()
		};

		mesh.setup_mesh();

		mesh
	}


	pub fn from_file<T>(filename: T, program: render_gl::Program, tex_path: &str) -> Self
	where T: AsRef<Path>
	{
		let vao = buffer::VertexArray::new();
		let ebo = buffer::ElementArrayBuffer::new();
		let vbo = buffer::ArrayBuffer::new();
		let texture = texture::Texture::new();
		texture.load(tex_path);
		texture.set_filtering(gl::REPEAT);
		texture.set_wrapping(gl::REPEAT);

		let mut temp_vertices = Vec::<math::vector::Vector3>::new();
		let mut temp_texcoords = Vec::<math::vector::Vector2>::new();
		let mut temp_normals = Vec::<math::vector::Vector3>::new();

		let mut indices = Vec::<u32>::new();

		if let Ok(lines) = read_lines(filename)
		{
			for line in lines
			{
				if let Ok(vertex) = line
				{
					let arr: Vec<&str> = vertex.split(" ").filter(|s| !s.is_empty()).collect();
					if arr.is_empty()
					{
						continue ;
					}
					// println!("{:?}", arr);
					if arr[0] == "v"
					{
						let tmp = math::vector::Vector3::new(arr[1].parse().unwrap(), arr[2].parse().unwrap(), arr[3].parse().unwrap());
						temp_vertices.push(tmp);
					}
					else if arr[0] == "vt"
					{
						let tmp = math::vector::Vector2::new(arr[1].parse().unwrap(), arr[2].parse().unwrap());
						temp_texcoords.push(tmp);
					}
					else if arr[0] == "vn"
					{
						let tmp = math::vector::Vector3::new(arr[1].parse().unwrap(), arr[2].parse().unwrap(), arr[3].parse().unwrap());
						temp_normals.push(tmp);
					}
					else if arr[0] == "f"
					{
						if arr[1].contains("/")
						{
							let mut temp_arr: Vec<&str> = Vec::<&str>::new();
							for (k, _) in arr.iter().enumerate()
							{
								if k != 0
								{
									let v: Vec<&str> = arr[k].split("/").collect();
									temp_arr.push(v[0]);
								}
								else
								{
									temp_arr.push(arr[k]);
								}
							};
							if temp_arr.len() == 5
							{
								let tmp1: u32 = temp_arr[1].parse().unwrap();
								let tmp2: u32 = temp_arr[2].parse().unwrap();
								let tmp3: u32 = temp_arr[3].parse().unwrap();
								let tmp4: u32 = temp_arr[4].parse().unwrap();
								indices.push(tmp1 - 1);
								indices.push(tmp2 - 1);
								indices.push(tmp3 - 1);
								indices.push(tmp1 - 1);
								indices.push(tmp3 - 1);
								indices.push(tmp4 - 1);
							}
							else
							{
								let tmp1: u32 = temp_arr[1].parse().unwrap();
								let tmp2: u32 = temp_arr[2].parse().unwrap();
								let tmp3: u32 = temp_arr[3].parse().unwrap();
								indices.push(tmp1 - 1);
								indices.push(tmp2 - 1);
								indices.push(tmp3 - 1);
							}
						}
						else
						{
							if arr.len() == 5
							{
								let tmp1: u32 = arr[1].parse().unwrap();
								let tmp2: u32 = arr[2].parse().unwrap();
								let tmp3: u32 = arr[3].parse().unwrap();
								let tmp4: u32 = arr[4].parse().unwrap();
								indices.push(tmp1 - 1);
								indices.push(tmp2 - 1);
								indices.push(tmp3 - 1);
								indices.push(tmp1 - 1);
								indices.push(tmp3 - 1);
								indices.push(tmp4 - 1);
							}
							else
							{
								let tmp1: u32 = arr[1].parse().unwrap();
								let tmp2: u32 = arr[2].parse().unwrap();
								let tmp3: u32 = arr[3].parse().unwrap();
								indices.push(tmp1 - 1);
								indices.push(tmp2 - 1);
								indices.push(tmp3 - 1);
							}
						}
					}
				}
			}
		}
		// println!("{:?}", temp_vertices);
		// println!("{:?}", temp_indices);
		let mut vertices = Vec::<Vertex>::new();
		let mut rng = rand::thread_rng();
		
		for i in temp_vertices
		{
			let random_num1: f32 = rng.gen_range(0.0..1.0);
			let random_num2: f32 = rng.gen_range(0.0..1.0);
			let random_num3: f32 = rng.gen_range(0.0..1.0);

			// let mut random_num1: f32 = (u32::MAX as f32) / (u32::MAX as f32 + 1.0);
			// let mut random_num2: f32 = (u32::MAX as f32) / (u32::MAX as f32 + 1.0);
			// let mut random_num3: f32 = (u32::MAX as f32) / (u32::MAX as f32 + 1.0);
			vertices.push(Vertex::new(i, (random_num1, random_num2, random_num3).into(), (i.x(), i.y()).into()));
		}

		// println!("{:?}", vertices);
		// println!("{:?}", indices);

		let mesh = Mesh {
			vertices,
			indices,
			texture,
			vao,
			vbo,
			ebo,
			program,
			model_mat: math::matrix::Matrix4::new_identity()
		};


		mesh.setup_mesh();

		mesh
	}

	fn setup_mesh(&self)
	{
		self.vao.bind();
		self.vbo.bind();

		self.vbo.static_draw_data(self.vertices.as_slice());

		self.ebo.bind();
		self.ebo.static_draw_data(self.indices.as_slice());

		unsafe
		{
			// vertex positions
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				0 as *const gl::types::GLvoid
			);

			// vertex normals
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				std::mem::size_of::<math::vector::Vector3>() as *const gl::types::GLvoid
			);

			// vertex texture coords
			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				(std::mem::size_of::<math::vector::Vector3>() * 2) as *const gl::types::GLvoid
			);

			// color
			gl::EnableVertexAttribArray(3);
			gl::VertexAttribPointer(
				3,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				(std::mem::size_of::<math::vector::Vector3>() * 2 + std::mem::size_of::<math::vector::Vector2>()) as *const gl::types::GLvoid
			)
		}
		self.vao.unbind();
	}

	pub fn update_pos(&mut self, event: &sdl2::event::Event)
	{
		match event
		{
			sdl2::event::Event::KeyDown {
				keycode: Some(key),
				..
			} => {
				match key
				{
					Keycode::A => {
						self.model_mat = math::rotate(&self.model_mat, 3_f32.to_radians(), &(0.0, -1.0, 0.0).into())
					},
					Keycode::D => {
						self.model_mat = math::rotate(&self.model_mat, 3_f32.to_radians(), &(0.0, 1.0, 0.0).into())
					},
					Keycode::W => {
						self.model_mat = math::rotate(&self.model_mat, 3_f32.to_radians(), &(-1.0, 0.0, 0.0).into())
					},
					Keycode::S => {
						self.model_mat = math::rotate(&self.model_mat, 3_f32.to_radians(), &(1.0, 0.0, 0.0).into())
					},
					_ => {}
				}
			},
			_ => {}
		}
	}

	pub fn render(&self, view: &math::matrix::Matrix4, projection: &math::matrix::Matrix4)
	{
		let model_location = unsafe {
			let string = CString::new("model").unwrap();
			gl::GetUniformLocation(self.program.id(), string.as_ptr())
		};

		let view_location = unsafe {
			let string = CString::new("view").unwrap();
			gl::GetUniformLocation(self.program.id(), string.as_ptr())
		};

		let projection_location = unsafe {
			let string = CString::new("projection").unwrap();
			gl::GetUniformLocation(self.program.id(), string.as_ptr())
		};

		self.program.set_used();

		self.texture.activate(gl::TEXTURE0);

		unsafe
		{
			// Need to transpose the matrix before passing to the shader, as opengl expects numbers in columns, and we save numbers in rows
			gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &self.model_mat.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(view_location, 1, gl::FALSE, &view.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, &projection.transposed() as *const math::matrix::Matrix4 as *const f32);
		}

		self.vao.bind();

		unsafe
		{
			gl::DrawElements(
				gl::TRIANGLES,
				self.indices.len() as gl::types::GLint,
				gl::UNSIGNED_INT,
				0 as *const gl::types::GLvoid
			);
		}

		self.vao.unbind();
	}
}

// Returns iterator over lines of a file
fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where T: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}