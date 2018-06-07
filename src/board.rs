extern crate piston_window as Pwind;
extern crate rand;
use board::rand::Rng;

pub const T_SIZE : i32 = 50;

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
        let n = rand::thread_rng().gen_range(0,6);
        match n {
            0|1 => Terrain::Land,
            2|3 => Terrain::Shallow,
            //3 => Terrain::Deep,
            4 => Terrain::Hole,
            _ => Terrain::Block,
        }

    }

    pub fn draw(&self,t:[[f64; 3]; 2],g:&mut Pwind::G2d){
        let rect = [0.0,0.0,T_SIZE as f64,T_SIZE as f64];
        match *self{
            Terrain::Land => {
                Pwind::rectangle([0.7,0.9,0.3,1.0],rect,t,g)
            }
            Terrain::Shallow => {
                Pwind::rectangle([0.0,1.1,1.0,1.0],rect,t,g)
            }
            Terrain::Deep => {
                Pwind::rectangle([0.0,0.0,1.0,1.0],rect,t,g)
            }
            Terrain::Hole => {
                Pwind::rectangle([0.5,0.5,0.5,1.0],rect,t,g)
            }
            _ => {
                Pwind::rectangle([0.1,0.1,0.1,1.0],rect,t,g)
            }
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    terrain: Terrain
}


pub struct Board{
    w:i32,
    h:i32,
    tiles:Vec<Tile>,
}

impl Board {
    pub fn new(w:i32,h:i32)->Board{
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
                    let r = res.step_dir(k as i32,j,1);
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


    pub fn draw(&self,c:Pwind::Context,g:&mut Pwind::G2d){
        let dark = [0.0,0.0,0.0,0.2];
        for (k,v) in self.tiles.iter().enumerate(){
            let x = k as i32 % self.w ;
            let y = k as i32 / self.w ;

            use Pwind::Transformed;
            let t = c.transform.trans((x*T_SIZE) as f64,(y*T_SIZE)as f64);
            v.terrain.draw(t,g);
            if (x+y) % 2 == 0 { 
                Pwind::rectangle(dark,[0.0,0.0,T_SIZE as f64,T_SIZE as f64],t,g);
            }
        }


    }


    pub fn step_dir(&self, pos:i32,dir:i32,dist:i32)->Result<i32,&str>{
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

    pub fn step(&self ,pos:i32,x:i32,y:i32)->Result<i32,&str>{
        //x first
        let ox = pos % self.w;
        let oy = pos / self.w;
        
        let x = ox + x;
        let y = oy + y;

        if (x < 0 )|( x >= self.w){
            return Result::Err("Out of X Bounds")
        }

        if (y <0) | (y >= self.h){
            return Result::Err("Out of Y Bounds")
        }

        Result::Ok(y * self.w + x)

    }
}


