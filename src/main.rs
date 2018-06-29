extern crate piston_window as p_wind;
//extern crate gfx_graphics;
extern crate lazyf;
extern crate rand;

#[macro_use] extern crate custon_derive;
#[macro_use] extern crate enum_derive;


mod board;
mod creatures;
pub mod shapes;


use p_wind::*;//{PistonWindow,Event,Graphics};

//use gfx_graphics::back_end::GfxGraphics;

use lazyf::get::SGetter;
use lazyf::cfg;



fn main() {
    let cg = cfg::Cfg::load_first("-ffg",&["test_data/defs.lz"]);
    let bw = cg.get_t_def(("-bw","board.width"),8 );
    let bh = cg.get_t_def(("-bh","board.height"),8 );
    println!("Board : {},{}",bw,bh);
    let tsize = board::T_SIZE as usize;
    let mut window : PistonWindow =
        p_wind::WindowSettings::new("Monchters",[(tsize*bw)as u32,(tsize*bh)as u32]).exit_on_esc(true)
        .build().unwrap();

    let board = board::Board::new(bw,bh);
    while let Some(e) = window.next(){
        match e{
            Event::Loop(_l) => {
            }
            _ => {
            }
        }
        window.draw_2d(&e,|c,g|{
            p_wind::clear([1.0,0.5,0.5,1.0],g);
            board.draw(c,g)

        });
    }
}
