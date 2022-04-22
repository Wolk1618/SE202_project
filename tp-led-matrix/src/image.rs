//! This module contains different functions whose aim is to deal with images
//! The goal is to easily handle image creation in order to test matrix printing

#[allow(unused_imports)]

use micromath::F32;
use super::gamma;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
#[repr(C)]

/// This structure represents a color with its 3 RGB components.
pub struct Color {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}


#[repr(transparent)]

/// This structure is only a representation of an image (8x8 pixels)
pub struct Image([Color; 64]);

/// Utility function whose aim is to convert an f32 into an u8 by limiting the highest value.
fn limit(op : f32) -> u8 {
    if op > 255_f32 {
        255
    } else {
        op as u8
    }
}

impl Color {

    pub const RED : Color = Color{r : 255, g : 0, b : 0};
    pub const GREEN : Color = Color{r : 0, g : 255, b : 0};
    pub const BLUE : Color = Color{r : 0, g : 0, b : 255};

    /// Function that apply gamma correction to the given color. (Cf gamma module)
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

    /// Function that scales the value of the color by a multiplier (limiting to 255).
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

    /// Function that scales the value of the color by a divider.
    fn div(self, div : f32) -> Self {
        self * (1_f32 / div)
    }
}

impl Image {

    /// Function that creates an image full of the color given as argument.
    pub fn new_solid(color : Color) -> Self {
        Image([color; 64])
    }

    /// Functions that returns the colors (pixels) on a given row of the image.
    pub fn row(&self, row : usize) -> &[Color] {
        &self.0[8 * row .. 8 * (row + 1)]
    }

    /// Function that creates an image gradually decreasing intensity on the diagonal.
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
    
    /// This function returns an image filled with pixels of the default color.
    fn default() -> Self {
        Image::new_solid(Default::default())
    }

}

impl core::ops::Index<(usize, usize)> for Image {

    type Output = Color;
    
    /// This function returns the color located at the given position (x,y).
    /// This function is helpfull in order to easily access to a given pixel.
    fn index(&self, index : (usize, usize)) -> &Color {
        // Translating the (x,y) coordinates into an index
        let uindex : usize = (8 * index.0) + index.1;
        &self.0[uindex]
    }
}

impl core::ops::IndexMut<(usize, usize)> for Image {

    /// This function returns the color located at the given position (x,y) with a mutable reference.
    /// This function is helpfull in order to easily access to a given pixel.
    fn index_mut(&mut self, index : (usize, usize)) -> &mut Color{
        let uindex : usize = (8 * index.0) + index.1;
        &mut self.0[uindex]
    }

}

impl core::convert::AsRef<[u8; 192]> for Image {

    /// This function converts the image into a reference on an array.
    fn as_ref(&self) -> &[u8; 192] {
        unsafe {core::mem::transmute::<&Image, &[u8; 192]>(self)}
    }
}

impl core::convert::AsMut<[u8; 192]> for Image {

    /// This function converts the image into a reference on a mutable array.
    fn as_mut(&mut self) -> &mut [u8; 192] {
        unsafe {core::mem::transmute::<&mut Image, &mut [u8; 192]>(self)}
    }
}