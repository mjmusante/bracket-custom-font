use bracket_terminal::prelude::*;

struct State;

impl State {
    fn pet_write<T: ToString>(ctx: &mut BTerm, x: i32, y: i32, output: T) {
        let mut xpos = x;
        for c in output.to_string().chars() {
            match c {
                'a'..='z' => {
                    let val = (c as i32 - 'a' as i32 + 1) as u16;
                    ctx.set(xpos, y, RGB::named(YELLOW), RGB::named(BLACK), val);
                }
                ' ' => (),
                _ => ctx.set(
                    xpos,
                    y,
                    RGB::named(YELLOW),
                    RGB::named(BLACK),
                    to_cp437('?'),
                ),
            }
            xpos += 1;
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let green = RGB::named(GREEN);
        let black = RGB::named(BLACK);

        for i in 0..16 {
            for j in 0..16 {
                if i < 8 {
                    ctx.set(j + 1, i + 1, green, black, i * 16 + j);
                } else {
                    ctx.set(j + 1, i + 1, black, green, (i - 8) * 16 + j);
                }
            }
        }
        State::pet_write(ctx, 12, 43, "hello world");
        if let Some(VirtualKeyCode::Q) = ctx.key {
            std::process::exit(0);
        }
    }
}

bracket_terminal::embedded_resource!(PETSCII, "resources/petchars.png");

fn main() -> BError {
    bracket_terminal::link_resource!(PETSCII, "resources/petchars.png");

    let context = BTermBuilder::new()
        .with_simple_console(80, 50, "petchars.png")
        .with_title("DUNGEON")
        .with_tile_dimensions(16, 16)
        .with_font("petchars.png", 8, 8)
        .build()?;
    let gs = State {};

    main_loop(context, gs)
}
