use std::io;
use std::fs::File;
use std::io::Write;

pub fn oliver_fill(pixels: &mut [u32], color: u32)
{
  pixels.fill(color);
}

pub fn oliver_draw_line(pixels: &mut [u32], pixel_width: usize, pixel_height: usize,
                        x1: i32, y1: i32, x2: i32, y2: i32, color: u32)
{
  /*
    Line equation:
    P1(x1, y1)   P2(x2, y2)
    y = mx + c

    y1 = mx1 + c
    y2 = mx2 + c
    c = y1 - mx1
    y2 = mx2 + y1 - mx1
    m = (y1 - y2) / (x1 - x2)
  */

  let dx = x2 - x1;
  let dy = y2 - y1;

  if dx != 0
  {
    let c= y1 - dy*x1/dx;

    for x in x1..=x2
    {
      if x >=0 && x < pixel_width as i32
      {
        let y = dy*x/dx + c;
        if 0 <= y && y < pixel_height as i32
        {
          pixels[(y * pixel_width as i32 + x) as usize] = color;
        }
      }
    }
  }
  else
  {
    // vertical line
    if x1 >= 0 && x1 < pixel_width as i32
    {
      for y in y1..=y2
      {
        if y >=0 && y < pixel_height as i32
        {
          pixels[(y * pixel_width as i32 + x1) as usize] = color;
        }
      }
    }
  }
}

pub fn oliver_draw_fill_rect(pixels: &mut [u32], pixel_width: usize, pixel_height: usize, 
                        x: i32, y: i32, w: i32, h: i32, color: u32)
{
  for dy in 0..h
  {
    let r = y + dy;
    if r >= 0 && r < pixel_height as i32
    {
      for dx in 0..w
      {
        let c = x + dx;
        if c >=0 && c < pixel_width as i32
        {
          pixels[(r * pixel_width as i32 + c) as usize] = color;
        }
      }
    }
  }
}

pub fn oliver_draw_fill_circle(pixels: &mut [u32], pixel_width: usize, pixel_height: usize,
                          x: i32, y: i32, radius: i32, color: u32)
{
  let cx = x + radius;
  let cy = y + radius;
  
  for dy in 0..2*radius
  {
    let r = y + dy;
    if 0 <= r && r < pixel_height as i32
    {
      for dx in 0..2*radius
      {
        let c = x + dx; 
        if 0 <= c && c < pixel_width as i32
        {
          if (r-cy)*(r-cy) + (c-cx)*(c-cx) <= radius * radius
          {
            pixels[(r * pixel_width as i32 + c) as usize] = color;
          }
        }
      }
    }
  }
}

pub fn oliver_save_to_ppm_file(pixels: &[u32], width: usize, height: usize, file_path: &'static str) -> io::Result<()>
{
  let mut file = File::create(file_path)?;

  file.write_fmt(format_args!("P6\n{} {}\n255\n", width, height))?;
  
  for i in pixels
  {
    // Color: 0xRRGGBBAA
    let buf: [u8; 3] = [
      ((i >> 24) & 0xFF) as u8,
      ((i >> 16) & 0xFF) as u8,
      ((i >> 8) & 0xFF) as u8
    ];

    file.write_all(&buf)?;
  }

  Ok(())
}
