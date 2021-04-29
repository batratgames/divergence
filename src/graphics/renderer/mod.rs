use std::ffi::{ CStr, CString };

pub mod vertex;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Program(gl::types::GLuint);

impl Program {
    pub fn from(shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()) };
        }

        unsafe { gl::LinkProgram(id) };

        let mut success: gl::types::GLint = 1;

        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) };

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe { gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar) };

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id()) };
        }

        Ok(Program(id))
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.0
    }

    pub fn activate(&self) {
        unsafe { gl::UseProgram(self.0) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) };
    }
}

pub enum Shader {
    Vertex(gl::types::GLuint),
    Fragment(gl::types::GLuint),
    Unimplemented(gl::types::GLuint),
}

impl Shader {
    fn from(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);

                return Err(error.to_string_lossy().into_owned());
            }
        }

        match kind {
            gl::VERTEX_SHADER => Ok(Shader::Vertex(id)),
            gl::FRAGMENT_SHADER => Ok(Shader::Fragment(id)),
            _ => Ok(Shader::Unimplemented(id))
        }
   }

    pub fn vertex(source: &CStr) -> Result<Shader, String> {
        Shader::from(source, gl::VERTEX_SHADER)
    }

    pub fn fragment(source: &CStr) -> Result<Shader, String> {
        Shader::from(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        match self {
            Shader::Vertex(x)        => *x,
            Shader::Fragment(x)      => *x,
            Shader::Unimplemented(x) => *x,
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(match self {
            Shader::Vertex(x)        => *x,
            Shader::Fragment(x)      => *x,
            Shader::Unimplemented(x) => *x,
        })}
    }
}
