use smart_leds::RGB8;

pub(crate) fn draw_line(
    mut point0: (isize, isize),
    mut point1: (isize, isize),
    color: RGB8,
    render_func: fn((usize, usize), RGB8),
) {
    if point0.0 > crate::WIDTH as isize
        || (point0.1 < 0 || point0.1 > crate::HEIGHT as isize)
        || (point1.0 < 0 || point1.0 > crate::WIDTH as isize)
        || (point1.1 < 0 || point1.1 > crate::HEIGHT as isize)
    {
        return ();
    }
    let dxfst = point1.0 - point0.0;
    let dyfst = point1.1 - point0.1;
    if (dxfst < 0 && dyfst >= 0) || (dxfst < 0 && dyfst < 0) {
        let _temp = point0;
        point0 = point1;
        point1 = point0;
    }
    let dx = point1.0 - point0.0;
    let dy = point1.1 - point0.1;
    if dx > dy && dy >= 0 {
        moderate_climb(point0, point1, dx, dy, render_func, color);
    } else if dx > dy && dy < 0 {
        moderate_decent(point0, point1, dx, dy, render_func, color);
    } else if dx < dy && dy >= 0 {
        bigger_climb(point0, point1, render_func, color);
    } else {
        bigger_decent(point0, point1, render_func, color);
    }
}

fn moderate_climb(
    point1: (isize, isize),
    point2: (isize, isize),
    dx: isize,
    dy: isize,
    render_func: fn((usize, usize), RGB8),
    color: RGB8,
) {
    let mut x = point1.0;
    let mut y = point1.1;
    let mut eps = 0;
    while x <= point2.0 {
        render_func((x as usize, y as usize), color);
        eps += dy;
        if (eps * 2) >= dx {
            y += 1;
            eps -= dx;
        }
        x += 1;
    }
}

pub(crate) fn moderate_decent(
    point1: (isize, isize),
    point2: (isize, isize),
    dx: isize,
    dy: isize,
    render_func: fn((usize, usize), RGB8),
    color: RGB8,
) {
    let mut x = point1.0;
    let mut y = point1.1;
    let mut eps = 0;
    while x <= point2.0 {
        render_func((x as usize, y as usize), color);
        eps -= dy;
        if (eps * 2) >= dx {
            y -= 1;
            eps -= dx;
        }
        x += 1;
    }
}

pub(crate) fn bigger_climb(
    point1: (isize, isize),
    point2: (isize, isize),
    render_func: fn((usize, usize), RGB8),
    color: RGB8,
) {
    let point1rot = (point1.1, -point1.0);
    let point2rot = (point2.1, -point2.0);
    let dx = point2rot.0 - point1rot.0;
    let dy = point2rot.1 - point1rot.1;
    let mut x = point1rot.0;
    let mut y = point1rot.1;
    let mut eps = 0;
    while x <= point2rot.0 {
        render_func((-y as usize, x as usize), color);
        eps -= dy;
        if (eps * 2) >= dx {
            y -= 1;
            eps -= dx;
        }
        x += 1;
    }
}

pub(crate) fn bigger_decent(
    point1: (isize, isize),
    point2: (isize, isize),
    render_func: fn((usize, usize), RGB8),
    color: RGB8,
) {
    let point1rot = (point2.1, -point2.0);
    let point2rot = (point1.1, -point1.0);
    let dx = point2rot.0 - point1rot.0;
    let dy = point2rot.1 - point1rot.1;
    let mut x = point1rot.0;
    let mut y = point1rot.1;
    let mut eps = 0;
    while x <= point2rot.0 {
        render_func((-y as usize, x as usize), color);
        eps += dy;
        if (eps * 2) >= dx {
            y += 1;
            eps -= dx;
        }
        x += 1;
    }
}
