#[allow(unused_imports)]

use micromath::F32;
use super::gamma;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
#[repr(C)]
pub struct Color {
    r : u8,
    g : u8,
    b : u8,
}

#[repr(transparent)]
pub struct Image([Color; 64]);

const RED : Color = Color{r : 255, g : 0, b : 0};
const GREEN : Color = Color{r : 0, g : 255, b : 0};
const BLUE : Color = Color{r : 0, g : 0, b : 255};

fn limit(op : f32) -> u8 {
    if op > 255_f32 {
        255
    } else {
        op as u8
    }
}

impl Color {

    pub fn gamma_correct(&self) -> Self {
        Color{
            r : gamma::gamma_correct(self.r),
            g : gamma::gamma_correct(self.g),
            b : gamma::gamma_correct(self.b),
        }
    }

}

impl core::ops::Mul<f32> for Color {

    type Output = Color;

    fn mul(self, mult : f32) -> Self {
        Color{
            r : limit(self.r as f32 * mult),
            g : limit(self.g as f32 * mult),
            b : limit(self.b as f32 * mult),
        }
    }
}

impl core::ops::Div<f32> for Color {

    type Output = Color;

    fn div(self, div : f32) -> Self {
        self * (1_f32 / div)
    }
}

impl Image {

    pub fn new_solid(color : Color) -> Self {
        Image([color; 64])
    }

    pub fn row(&self, row : usize) -> &[Color] {
        &self.0[8 * row .. 8 * (row + 1)]
    }

    pub fn gradient(color : Color) -> Self {
        let mut grad : Image = Image::default();
        for row in 0..8 {
            for col in 0..8 {
                grad[(row, col)] = color / (1 + row * row + col) as f32
            }
        }
        grad
    }
}

impl Default for Image {
    
    fn default() -> Self {
        Image::new_solid(Default::default())
    }

}

impl core::ops::Index<(usize, usize)> for Image {

    type Output = Color;
    
    fn index(&self, index : (usize, usize)) -> &Color {
        let uindex : usize = (8 * index.0) + index.1;
        &self.0[uindex]
    }
}

impl core::ops::IndexMut<(usize, usize)> for Image {

    fn index_mut(&mut self, index : (usize, usize)) -> &mut Color{
        let uindex : usize = (8 * index.0) + index.1;
        &mut self.0[uindex]
    }

}

impl core::convert::AsRef<[u8; 192]> for Image {

    fn as_ref(&self) -> &[u8; 192] {
        unsafe {core::mem::transmute::<&Image, &[u8; 192]>(self)}
    }
}

impl core::convert::AsMut<[u8; 192]> for Image {

    fn as_mut(&mut self) -> &mut [u8; 192] {
        unsafe {core::mem::transmute::<&mut Image, &mut [u8; 192]>(self)}
    }
}