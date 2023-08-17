#![no_std]

use smart_leds::RGB8;

const WIDTH: usize = 12;
const HEIGHT: usize = 20;

mod bresenham;

pub type String<'a> = &'a [char];

struct State {
    pub(crate) fonts: &'static [&'static [&'static [bool]]; 29],
}

#[allow(non_upper_case_globals)]
static mut state: State = State {
    fonts: &[&[&[false; 7]]; 29],
};

#[inline]
fn mixpixel(position: (usize, usize), value: RGB8) {
    unsafe {
        display[position.0][position.1].r =
            display[position.0][position.1].r.saturating_add(value.r);
        display[position.0][position.1].g =
            display[position.0][position.1].g.saturating_add(value.g);
        display[position.0][position.1].b =
            display[position.0][position.1].b.saturating_add(value.b);
    }
}

#[inline]
fn overwritepixel(position: (usize, usize), value: RGB8) {
    unsafe {
        display[position.0][position.1] = value;
    }
}

#[allow(non_upper_case_globals)]
static mut display: [[RGB8; HEIGHT]; WIDTH] = [[RGB8 { r: 0, g: 0, b: 0 }; HEIGHT]; WIDTH];

pub type Vectormap<'a> = &'a [(RGB8, &'a [((usize, usize), (usize, usize))])];

pub fn render_vectormap(map: Vectormap) {
    for (color, array) in map {
        for points in *array {
            draw_line_internal(points.0, points.1, *color, overwritepixel);
        }
    }
}

pub fn buffer() -> &'static [[RGB8; HEIGHT]; WIDTH] {
    unsafe { &display }
}

pub fn draw_bitmap(map: &[&[RGB8]], position: (usize, usize)) {
    let mut xindex = position.0;
    let mut yindex = position.1;
    let mut xcounter = 0;
    let mut ycounter = 0;
    while xindex < WIDTH && xcounter < map.len() {
        while yindex < HEIGHT && ycounter < map[0].len() {
            unsafe {
                display[xindex][yindex] = map[xcounter][ycounter];
            }
            yindex += 1;
            ycounter += 1;
        }
        yindex = position.1;
        ycounter = 0;
        xindex += 1;
        xcounter += 1;
    }
}

pub fn draw_generic_bitmap(map: &[&[bool]], position: (usize, usize), color: RGB8) {
    let mut xindex = position.0;
    let mut yindex = position.1;
    let mut xcounter = 0;
    let mut ycounter = 0;
    while xindex < WIDTH && xcounter < map.len() {
        while yindex < HEIGHT && ycounter < map[0].len() {
            if map[xcounter][ycounter] {
                unsafe {
                    display[xindex][yindex] = color;
                }
            }
            yindex += 1;
            ycounter += 1;
        }
        yindex = 0;
        ycounter = 0;
        xindex += 1;
        xcounter += 1;
    }
}

pub fn draw_line(point1: (usize, usize), point2: (usize, usize), color: RGB8) {
    draw_line_internal(point1, point2, color, mixpixel);
}

fn draw_line_internal(
    point1: (usize, usize),
    point2: (usize, usize),
    color: RGB8,
    render_func: fn(position: (usize, usize), value: RGB8),
) {
    bresenham::draw_line(
        (point1.0 as isize, point1.1 as isize),
        (point2.0 as isize, point2.1 as isize),
        color,
        render_func,
    )
}

pub fn fonts(fonts: &'static [&'static [&'static [bool]]; 29]) {
    unsafe {
        state.fonts = fonts;
    }
}

pub fn initialize_text_buffer(buffer: &mut [&mut [RGB8]], text: String, color: RGB8) {
    let mut textindex = 0;
    let mut xindex = 0;
    let mut fontsindex = match text[textindex] as usize {
        32 => 26,
        c if c == 46 || c == 47 => c - 19,
        c => c - 65,
    }; //text[textindex] as usize - 65;
    let mut xcounter = 0;
    while xindex < buffer.len() {
        for i in 0..7 {
            buffer[xindex][i] = if unsafe { state.fonts[fontsindex][xcounter][i] } == true {
                color
            } else {
                buffer[xindex][i]
            };
        }
        xindex += 1;
        xcounter += 1;
        if xcounter == unsafe { state.fonts[fontsindex].len() } {
            xcounter = 0;
            textindex += 1;
            if textindex == text.len() {
                return ();
            }
            fontsindex = match text[textindex] as usize {
                32 => 26,
                c if c == 46 || c == 47 => c - 19,
                a => a - 65,
            };
            xindex += 1;
        }
    }
}

pub fn draw_text_buffer(buffer: &[&mut [RGB8]], ypos: usize) {
    let mut yindex = ypos;
    let mut xindex = 1;
    let mut xcounter = 0;
    let mut ycounter = 0;
    while xindex < WIDTH - 1 && xcounter < buffer.len() {
        while yindex < HEIGHT && ycounter < buffer[0].len() {
            unsafe {
                display[xindex][yindex] = buffer[xcounter][ycounter];
            }
            yindex += 1;
            ycounter += 1;
        }
        yindex = ypos;
        ycounter = 0;
        xindex += 1;
        xcounter += 1;
    }
}

pub fn reset_buffer() {
    let mut x: usize = 0;
    let mut y: usize = 0;
    while x < WIDTH {
        while y < HEIGHT {
            unsafe {
                display[x][y] = RGB8 { r: 0, g: 0, b: 0 };
                display[x][y]
            };
            y += 1;
        }
        y = 0;
        x += 1;
    }
}

pub fn shift_text_buffer(buffer: &mut [&mut [RGB8]]) {
    buffer.rotate_left(1);
}
