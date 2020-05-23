//use std::sync::Arc;
//use crate::shader::Shader;
//use crate::render_state::RenderState;

pub struct Material {
}

pub struct MaterialInstance {
}

/*

pub struct Material {
    profiles_manager: Arc<MaterialProfilesManager>,
}

pub struct MaterialInstance {
    profiles_manager: Arc<MaterialProfilesManager>,

    current_profile: Arc<MaterialProfile>,
}

struct MaterialProfilesManager {

}

pub struct MaterialProfile {
    render_state: RenderState,

    vertex_shader: Arc<Shader>,

    geometry_shader: Arc<Shader>,

    domain_shader: Arc<Shader>,

    tesselation_shader: Arc<Shader>,

    fragment_shader: Arc<Shader>,

    compute_shader: Arc<Shader>,
}

impl Material {
    pub fn create_instance(&self) -> Arc<MaterialInstance> {
        unimplemented!()
    }
}

impl MaterialInstance {
    pub fn current_profile(&self) -> &MaterialProfile {
        unimplemented!()
    }
}

impl MaterialProfilesManager {
    
}

*/
