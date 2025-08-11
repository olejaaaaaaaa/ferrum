

use std::fmt::Debug;
use std::path::Path;
use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;

use gltf::buffer::Source;
use gltf::Gltf;

pub fn open_gltf(path: &str) -> Result<Gltf, Box<dyn StdError>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    Ok(gltf)
}

pub fn load_mesh_data(gltf: &Gltf) -> Vec<(Vec<[f32; 3]>, Vec<u32>)> {

    let mut meshes = Vec::new();

    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            // Получаем reader с правильной обработкой источника буфера
            let reader = primitive.reader(|buffer| {
                let buffer = gltf.buffers().nth(buffer.index())?;
                match buffer.source() {
                    Source::Uri(uri) => {
                        // Загрузка из внешнего файла (реализуйте эту часть)
                        todo!("Implement external buffer loading")
                    }
                    Source::Bin => {
                        // Для встроенных бинарных данных (GLB)
                        gltf.blob.as_deref()
                    }
                }
            });
            
            // Получаем позиции вершин
            let positions: Vec<[f32; 3]> = reader.read_positions()
                .expect("Mesh has no positions")
                .collect();
            
            // Получаем индексы
            let indices = reader.read_indices()
                .map(|iter| iter.into_u32().collect())
                .unwrap_or_else(|| {
                    log::warn!("WARN NOT FOUND INDECES");
                    (0..positions.len() as u32).collect()
            });

            meshes.push((positions, indices));
        }
    }

    meshes
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

pub fn load_model<P: AsRef<Path> + Debug>(path: P) -> Result<(Vec<Vertex>, Vec<u32>), Box<dyn std::error::Error>> {

    let (models, materials) = tobj::load_obj(&path, &tobj::LoadOptions::default())?;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for model in models {
        let mesh = &model.mesh;

        for i in 0..mesh.positions.len() / 3 {
            let position = [
                mesh.positions[3 * i],
                mesh.positions[3 * i + 1],
                mesh.positions[3 * i + 2],
            ];

            vertices.push(Vertex {
                pos: position,
                color: [0.5, 0.2, 0.1]
            });
        }

        // Обрабатываем индексы
        indices.extend(&mesh.indices);
    }

    Ok((vertices, indices))
}