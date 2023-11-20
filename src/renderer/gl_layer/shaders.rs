use std::ffi::{CStr, CString};

pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    pub fn to_gl_shader_type(&self) -> u32 {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    gl_shader: u32,
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: &str) -> Self {
        let gl_shader = unsafe { gl::CreateShader(shader_type.to_gl_shader_type()) };
        let source = CString::new(source.as_bytes()).unwrap();

        unsafe {
            gl::ShaderSource(gl_shader, 1, &(source.as_ptr()) as _, 0 as _);
            gl::CompileShader(gl_shader);

            let mut success = 0;
            gl::GetShaderiv(gl_shader, gl::COMPILE_STATUS, &mut success as _);
            let success = success != 0;

            if !success {
                let mut info_log: [i8; 256] = [0; 256];
                gl::GetShaderInfoLog(gl_shader, 256, 0 as _, &mut info_log as _);
                let info_log = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap();

                panic!("{}", info_log);
            }
        }

        Shader {
            gl_shader
        }
    }

    pub fn destroy(&self) {
        unsafe {
            gl::DeleteShader(self.gl_shader);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.destroy()
    }
}

pub struct ShaderPipeline {
    gl_program: u32,
}

impl ShaderPipeline {
    pub fn new(shaders: &[Shader]) -> Self {
        let gl_program = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(gl_program, shader.gl_shader);
            }
        }

        unsafe {
            gl::LinkProgram(gl_program);

            let mut success = 0;
            gl::GetProgramiv(gl_program, gl::LINK_STATUS, &mut success as _);
            let success = success != 0;

            if !success {
                let mut info_log: [i8; 256] = [0; 256];
                gl::GetProgramInfoLog(gl_program, 256, 0 as _, &mut info_log as _);
                let info_log = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap();

                panic!("{}", info_log);
            }
        }

        ShaderPipeline {
            gl_program
        }
    }

    pub fn destroy(&self) {
        unsafe {
            gl::DeleteProgram(self.gl_program);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.gl_program);
        }
    }
}

impl Drop for ShaderPipeline {
    fn drop(&mut self) {
        self.destroy()
    }
}
