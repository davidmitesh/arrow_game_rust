use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

//Here we are designing the game frame
pub fn new_frame() -> Frame{//In this way, we are benefitted by using the type alias for Frame
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS{
        let mut col = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_ROWS{
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

//Everythign we want to see needs to be drawn in the frame

pub trait Drawable{
    fn draw(&self,frame:&mut Frame);
}