use p_wind::*;
use p_wind::math::Matrix2d;
use p_wind::math::Scalar;

pub fn path<G:Graphics>( pts:&[(Scalar,Scalar)],g:&mut G,t:Matrix2d){
    if pts.len() < 2 {
        return
    }
    let mut prev = pts[0];
    for pt in pts[1..].iter(){
        let (px,py) = prev;
        let &(x,y) = pt;
        line([0.,0.,0.,255.],2.,[px,py,x,y],t,g);
        prev = *pt;
    }
}

pub fn stroke_rect<G:Graphics>((lx,ty):(Scalar,Scalar),(rx,by):(Scalar,Scalar),g:&mut G, t:Matrix2d){
    path(&[(lx,ty),(rx,ty),(rx,by),(lx,by),(lx,ty)],g,t);
}
