pub use sdl2;
pub use gl;
pub use lazy_static;

pub mod audio;
pub mod core;
pub mod graphics;
pub mod input;
pub mod maths;

#[cfg(test)]
mod tests {
    #[test]
    fn empty() {
        
    }
}
