use image::io::Reader as ImageReader;


pub struct Texture {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_file(path: &str) -> Texture {
        println!("Loading texture {}", path);
        let img = ImageReader::open(path).unwrap().decode().unwrap().flipv().fliph().into_rgb();
        let width = img.width();
        let height = img.height();

        Texture {
            buffer: img.into_raw(),
            width,
            height,
        }
    }
}

pub struct Material {
    diffuse_map: Option<Texture>,
    diffuse: glm::Vec3,
    emission: glm::Vec3,
}

impl Material {
    pub fn new(diffuse_map: Option<Texture>, diffuse: glm::Vec3, emission: glm::Vec3) -> Material {
        Material {
            diffuse_map,
            diffuse,
            emission,
        }
    }

    pub fn emission(&self) -> &glm::Vec3 { &self.emission }

    pub fn sample_diffuse(&self, uv: &glm::Vec2) -> glm::Vector3<f32> {
        // let diffuse_map = self.diffuse_map.as_ref().expect("Cannot sample a Material without diffuse map");
        match &self.diffuse_map {
            Some(t) => {
                let pixelx = (uv.x * (t.width as f32)) as usize;
                let pixely = (uv.y * (t.height as f32)) as usize;
                let index = (pixely * t.width as usize * 3) + (pixelx*3);
                let r = t.buffer[index] as f32;
                let g = t.buffer[index + 1] as f32;
                let b = t.buffer[index + 2] as f32;

                glm::Vector3::new(r / 255.0, g / 255.0, b / 255.0)
            },
            None => {
                self.diffuse
            }
        }
    }
}