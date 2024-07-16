use crate::application::{vk_app::VulkanApp, vulkan::vk_buffer::BufferGPU};

use super::{material::Material, model::Model, triangle::Triangle};

#[derive(Default)]
pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
}

pub struct SceneBuffers {
    pub triangles_buffer: BufferGPU,
    pub models_buffer: BufferGPU,
    pub materials_buffer: BufferGPU,
}

impl Scene {
    pub fn init(&mut self) {
        // init the scene materials
        self.add_material(Material::default());

        // init the scene models
        // let teapot = String::from("/src/assets/models/teapot.obj");
        // self.add_model(teapot);
        let (model, triangles) = Model::triangle();
        self.models.push(model);
        self.triangles = triangles;
    }

    pub fn add_existing_model(&mut self, model_id: usize) {
        // TODO:
        todo!("need to implement {}", model_id)
    }

    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }

    pub fn add_model(&mut self, obj_path: String) {
        // TODO: fix this
        // read obj file
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let model_path = crate_path.to_owned() + &obj_path;

        // create a model
        let loaded_model = tobj::load_obj(model_path.clone(), &tobj::GPU_LOAD_OPTIONS);
        assert!(loaded_model.is_ok());

        let (models, materials) = loaded_model.unwrap_or_else(|err| {
            panic!("Failed to load OBJ file {}: {:?}", model_path.clone(), err)
        });

        // Materials might report a separate loading error if the MTL file wasn't found.
        // If you don't need the materials, you can generate a default here and use that
        // instead.
        let materials = materials.unwrap_or_else(|err| {
            panic!(
                "Failed to load MTL file for {}: {:?}",
                model_path.clone(),
                err
            )
        });

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            println!("model[{}].name = \'{}\'", i, m.name);
            println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

            println!(
                "Size of model[{}].face_arities: {}",
                i,
                mesh.face_arities.len()
            );

            let mut next_face = 0;
            for f in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                println!("    face[{}] = {:?}", f, face_indices);
                next_face = end;
            }
            // Normals and texture coordinates are also loaded, but not printed in this example
            println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
            assert!(mesh.positions.len() % 3 == 0);
            for _ in 0..mesh.positions.len() / 3 {}
        }

        for (i, m) in materials.iter().enumerate() {
            println!("material[{}].name = \'{}\'", i, m.name);
        }
    }

    fn upload_triangles(&self, application: &VulkanApp) -> BufferGPU {
        BufferGPU::upload_elements(&self.triangles, application)
    }

    fn upload_models(&self, application: &VulkanApp) -> BufferGPU {
        BufferGPU::upload_elements(&self.models, application)
    }

    fn upload_materials(&self, application: &VulkanApp) -> BufferGPU {
        BufferGPU::upload_elements(&self.materials, application)
    }

    pub fn upload_buffers(&self, application: &VulkanApp) -> SceneBuffers {
        SceneBuffers {
            triangles_buffer: self.upload_triangles(application),
            models_buffer: self.upload_models(application),
            materials_buffer: self.upload_materials(application),
        }
    }
}
