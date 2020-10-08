use macroquad::Texture2D;

pub fn get_noise_texture_solid(width: usize, height: usize, seed: u32) -> Texture2D {
    use noise::{OpenSimplex, Seedable, utils::*};
    use macroquad::{Image, load_texture_from_image};
    
    let open_simplex = OpenSimplex::new();
    let open_simplex = open_simplex.set_seed(seed);
    let plane = PlaneMapBuilder::new(&open_simplex);
    let plane = plane.set_size(width, height);
    let data = plane.build();
    let mut render = ImageRenderer::new();
    let render = render.render(&data);
    let mut img = Image {
        width: width as u16,
        height: height as u16,
        bytes: Vec::new(),

    };
    for y in 0..height {
        for x in 0..width {
            let color = render.get_value(x, y);
            img.bytes.push(color[0]);
            img.bytes.push(color[1]);
            img.bytes.push(color[2]);
            img.bytes.push(color[0]);
        }
    }

    let texture = load_texture_from_image(&img);
    texture
}



pub fn get_noise_texture_alpha(width: usize, height: usize, number_of_shades: u8, seed: u32) -> Texture2D {
    use noise::{OpenSimplex, Seedable, utils::*};
    use macroquad::{Image, load_texture_from_image};
    
    let open_simplex = OpenSimplex::new();
    let open_simplex = open_simplex.set_seed(seed);
    let plane = PlaneMapBuilder::new(&open_simplex);
    let plane = plane.set_size(width, height);
    let data = plane.build();
    let mut render = ImageRenderer::new();
    let render = render.render(&data);
    let mut img = Image {
        width: width as u16,
        height: height as u16,
        bytes: Vec::new(),

    };
    let color_div = 255 / number_of_shades;
    for y in 0..height {
        for x in 0..width {
            let color = render.get_value(x, y);
            //println!("color : {:?}", color);
            img.bytes.push((color[0] / color_div) * color_div);
            img.bytes.push((color[1] / color_div) * color_div);
            img.bytes.push((color[2] / color_div) * color_div);
            img.bytes.push((color[0] / color_div) * color_div);
        }
    }

    let texture = load_texture_from_image(&img);
    texture
}