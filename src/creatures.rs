use p_wind::{Texture,G2dTexture,TextureSettings};
use lazyf::{LzList,Lz};
use std::path::{PathBuf,Path};



custom_derive!{
    #[derive(Debug,EnumFromStr)]
    pub enum Elem{
        Fire,
        Water,
        Mind,
        Brawn,
        Earth,
        Wind,
        Plant,
        Electric,
    }
}

//#[derive(Clone)]
pub struct CreatureData{
    name:String,
    cost:u8,
    elem:Elem,
}


pub fn creature_data(fname:&str)->Vec<CreatureData>{ 
    let ls := Lz::load("assets/cards.lz").unwrap();
    let cd = ls.iter().map(CreatureDate::from_lz).collect();
}

impl CreatureData{
    pub from_lz(l:&Lz)->Self{
        CreatureData{
            name:l.Name,
            cost:l.get_t_def("Cost",0),
            elem:l.get_t_def("Type",Elem::Fire),
        }
    }

    pub texture<F:pwind::gfx_core::GfxFactory>(&self,f:F)->G2dTexture{
        let lcname = self.Name.to_lowercase().push_str(".png");
        let mut ppath = PathBuf::new("assets/cr_pics/").push(lcname);

        return Texture::from_path(f,ppath,FLIP::None,&TextureSettings::new()).unwrap();
    }

}



