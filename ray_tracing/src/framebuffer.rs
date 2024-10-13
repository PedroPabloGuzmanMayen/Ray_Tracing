use crate::color::Color;
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color
}

impl FrameBuffer {
    pub fn new (width: usize, height: usize) -> FrameBuffer {
        let default_color = Color::new(255,255,255);
        let buffer = vec![default_color; width*height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color: default_color,
            current_color: default_color
        }
    }

    pub fn clear(&mut self){
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x:usize, y:usize){
        self.buffer[self.width * y + x] =  self.current_color;
    }

    pub fn set_background_color(&mut self, color:Color){
        self.background_color = color;
    }

    pub fn get_color(&mut self, x:usize, y:usize) -> Color {
        self.buffer[self.width * y + x]
    }

    pub fn set_current_color(&mut self, color:Color){
        self.current_color = color;
    }
    pub fn cast_buffer(&self) -> Vec<u32> {
        let mut casted_vector: Vec<u32> = Vec::with_capacity(self.buffer.len());
        for color in &self.buffer {
            casted_vector.push(color.to_hex());
        }
        casted_vector
    }
}