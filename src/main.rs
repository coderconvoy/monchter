extern crate piston_window as Pwind;
extern crate lazyf;

mod board;

use lazyf::get::SGetter;
use lazyf::cfg;



fn main() {
    let cg = cfg::Cfg::load("test_data/defs.lz");
    let bw = cg.get_t_def(("-bw","board.width"),8 );
    let bh = cg.get_t_def(("-bh","board.height"),8 );
    println!("Board : {},{}",bw,bh);
    let mut window : Pwind::PistonWindow =
        Pwind::WindowSettings::new("Monchters",[(board::T_SIZE*bw)as u32,(board::T_SIZE*bh)as u32]).exit_on_esc(true)
        .build().unwrap();

    let board = board::Board::new(bw,bh);
    while let Some(e) = window.next(){
        match e{
            Pwind::Event::Loop(_l) => {
            }
            _ => {
            }
        }
        window.draw_2d(&e,|c,mut g| {
            Pwind::clear([1.0,0.5,0.5,1.0],g);
            board.draw(c,&mut g)

        });
    }
}
