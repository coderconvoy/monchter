
use shapes;
use rand::{Rng,thread_rng};
use p_wind::{rectangle,Context,Graphics};

use p_wind::math::Matrix2d;
pub const T_SIZE : f64 = 50.0;

#[derive(Clone)]
pub enum Terrain{
    Land,
    Shallow,
    Deep,
    Hole,
    Block,
}

impl Terrain {
    pub fn rand()->Terrain{
        let n = thread_rng().gen_range(0,7);
        match n {
            0|1|2 => Terrain::Land,
            3|4 => Terrain::Shallow,
            5 => Terrain::Hole,
            _ => Terrain::Block,
        }
    }

    pub fn draw<G:Graphics>(&self,t:Matrix2d,g:&mut G){
        let rect = [0.0,0.0,T_SIZE,T_SIZE];
        match *self{
            Terrain::Land => {
                rectangle([0.7,0.9,0.3,1.0],rect,t,g)
            }
            Terrain::Shallow => {
                rectangle([0.0,1.1,1.0,1.0],rect,t,g)
            }
            Terrain::Deep => {
                rectangle([0.0,0.0,1.0,1.0],rect,t,g)
            }
            Terrain::Hole => {
                rectangle([0.5,0.5,0.5,1.0],rect,t,g)
            }
            _ => {
                rectangle([0.1,0.1,0.1,1.0],rect,t,g)
            }
        }
        shapes::stroke_rect((0.,0.),(T_SIZE,T_SIZE),g,t);

    }
}

#[derive(Clone)]
pub struct Tile {
    terrain: Terrain
}


pub struct Board{
    w:usize,
    h:usize,
    tiles:Vec<Tile>,
}

impl Board {
    pub fn new(w:usize,h:usize)->Board{
        let mut t:Vec<Tile> = Vec::new() ;
        for _ in 0 .. w*h{
            t.push(Tile{terrain:Terrain::rand()});
        }

        let mut res = Board{
            tiles:t,
            w:w,
            h:h,
        };
        let tlen = res.tiles.len();
        //rotate copy
        for k in 0 ..tlen/2 {
            res.tiles[(tlen-k) - 1] = res.tiles[k].clone();
        }
        //deep water doesnt touch land
        for k in 0 ..tlen{
            let mut fland = false;
            {
                let v = &res.tiles[k];
                if let Terrain::Shallow = v.terrain {} else{ continue }
                for i in 0..4{
                    let j = 2*i;
                    let r = res.step_dir(k ,j,1);
                    if let Result::Ok(n)= r {
                        if let Terrain::Land = res.tiles[n as usize].terrain { 
                            fland = true;
                            break;
                        }
                    }
                }
            }
            if !fland {
                res.tiles[k] = Tile{terrain:Terrain::Deep}
                //v.terrain = Terrain::Deep
            }
        }
        res
    }


    pub fn draw<G:Graphics>(&self,c:Context,g:&mut G){
        for (k,v) in self.tiles.iter().enumerate(){
            let x = (k  % self.w) as f64;
            let y = (k  / self.w) as f64;

            use p_wind::Transformed;
            let t = c.transform.trans(x*T_SIZE,y*T_SIZE);
            v.terrain.draw(t,g);
        }


    }


    pub fn step_dir(&self, pos:usize,dir:i32,dist:i32)->Result<usize,&str>{
        match dir % 8  {
            0 => self.step(pos,-dist,0),
            1 => self.step(pos,-dist,dist),
            2 => self.step(pos,0,dist),
            3 => self.step(pos,dist,dist),
            4 => self.step(pos,dist,0),
            5 => self.step(pos,dist,-dist),
            6 => self.step(pos,0,-dist),
            _ => self.step(pos,-dist,-dist),
        }
    }

    pub fn step(&self ,pos:usize,x:i32,y:i32)->Result<usize,&str>{
        //x first
        let pos = pos as i32;
        let w = self.w as i32;
        let h = self.h as i32;

        let ox = pos % w;
        let oy = pos / w;
        
        let x = ox + x;
        let y = oy + y;

        if (x < 0 )|( x >= w){
            return Result::Err("Out of X Bounds")
        }

        if (y <0) | (y >= h){
            return Result::Err("Out of Y Bounds")
        }

        Result::Ok((y * w + x)as usize)

    }
}


