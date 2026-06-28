mod oliver;

use oliver::{
  color::*, 
  core::*
};

// Constants
const WIDTH: usize = 200;
const HEIGHT: usize = 200;

const ROWS: i32 = 6;
const COLS: i32 = 8;
const COL_WIDTH: i32 = WIDTH as i32 / COLS;
const COL_HEIGHT: i32 = HEIGHT as i32 / ROWS;

const BACKGROUND_COLOR: u32 = RED;

// Functions
fn main()
{
  // Init pixels
  let mut pixels = vec![0u32; WIDTH*HEIGHT];

  // fill background
  oliver_fill(&mut pixels, BACKGROUND_COLOR);

  // checker_example(&mut pixels);
  // oliver_draw_fill_circle(&mut pixels, WIDTH, HEIGHT, WIDTH as i32 / 2 - 50, HEIGHT as i32 / 2  - 50, 50, GREEN);
  // oliver_draw_fill_rect(&mut pixels, WIDTH, HEIGHT, WIDTH as i32 / 2 - 50, HEIGHT as i32 / 2  - 50, 100, 100, BLUE);
  oliver_draw_line(&mut pixels, WIDTH, HEIGHT, 0, 0, WIDTH as i32, HEIGHT as i32, WHITE);
  oliver_draw_line(&mut pixels, WIDTH, HEIGHT, 0, HEIGHT as i32, WIDTH as i32, 0, BLACK);
  
  if let Err(e) = oliver_save_to_ppm_file(&pixels, WIDTH, HEIGHT, "output.ppm")
  {
    eprintln!("Failed to save to output.ppm file: {}", e);
  }
}

fn checker_example(pixels: &mut [u32])
{
  for y in 0..ROWS
  {
    for x in 0..COLS
    {
      let mut color: u32 = BACKGROUND_COLOR;
      if (x + y) & 1 == 0
      {
        color = BLACK;
      }

      oliver_draw_fill_rect(pixels, WIDTH, HEIGHT, x * COL_WIDTH, y * COL_HEIGHT, COL_WIDTH, COL_HEIGHT, color);
    }
  }
}
