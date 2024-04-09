const XRANGE: (i32, i32) = (288, 330);
const YRANGE: (i32, i32) = (-96, -50);
const MIN_VY: i32 = -96;
//
// x * (x + 1) = 288 * 2
// (x + 1/2) ^ 2 = 288 * 2 - 1/4
//
const MIN_VX: i32 = 24;
const MAX_VY: i32 = 95;
const MAX_VX: i32 = 330;
fn launch(v: (i32, i32)) -> bool {
    let (mut x, mut y) = (0, 0);
    let (mut vx, mut vy) = v;
    while x <= XRANGE.1 && y >= YRANGE.0 {
        if x >= XRANGE.0 && y <= YRANGE.1 {
            return true;
        }
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
    }
    false
}
pub fn run(day: usize) {
    let all_velo = {
        let mut v = Vec::new();
        for x in MIN_VX..MAX_VX + 1 {
            for y in MIN_VY..MAX_VY + 1 {
                if launch((x, y)) {
                    v.push((x, y));
                }
            }
        }
        v
    };
    println!("day17a: {}", {
        let vy = all_velo.iter().map(|x| x.1).max().unwrap();
        (vy * (vy + 1)) / 2
    });
    println!("day17b: {}", &all_velo.len());
}
