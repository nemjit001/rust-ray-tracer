pub mod vertex_array;
pub mod shaders;
pub mod textures;

pub fn init(window: &mut glfw::Window) {
    gl::load_with(|name| window.get_proc_address(name));
}
