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
    diffuse_map: Texture,
}

impl Material {
    pub fn new(diffuse_map: Texture) -> Material {
        Material {
            diffuse_map
        }
    }

    pub fn sample_diffuse(&self, u: f32, v: f32) -> glm::Vector3<f32> {
        let pixelx = (u * (self.diffuse_map.width as f32)) as usize;
        let pixely = (v * (self.diffuse_map.height as f32)) as usize;
        let index = (pixely * self.diffuse_map.width as usize * 3) + (pixelx*3);
        let r = self.diffuse_map.buffer[index] as f32;
        let g = self.diffuse_map.buffer[index + 1] as f32;
        let b = self.diffuse_map.buffer[index + 2] as f32;

        glm::Vector3::new(r / 255.0, g / 255.0, b / 255.0)
    }
}