use std::fs;

#[derive(Debug)]
struct Hailstone{
    sx: i128,
    sy: i128,
    sz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
    a: i128,
    b: i128,
    c: i128
}

impl Hailstone{
    fn new(sx: i128,
            sy: i128,
            sz: i128,
            vx: i128,
            vy: i128,
            vz: i128) -> Self{
        
        Hailstone{
            sx: sx,
            sy: sy,
            sz: sz,
            vx: vx,
            vy: vy,
            vz: vz,
            a: vy,
            b: -vx,
            c: vy * sx - vx * sy
        }
    }
}

fn main() {

    // i had no idea how to solve this lol
    // i found a blogpost that claimed it was only necessary to build a systems of equations
    // of 9 variables. I used the 3 first vectorw and since I have no idea if there are crates for
    // solving systems of linear equations without having to read very cryptic docs with no comments on how
    // the machinery works lol
    // I used a sagemath code reference as posted here https://medium.com/@simontoth/daily-bit-e-of-c-advent-of-code-day-24-3faeef93c982
    //
    // x = var('x')
    // y = var('y')
    // z = var('z')
    // vx = var('vx')
    // vy = var('vy')
    // vz = var('vz')
    // t1 = var('t1')
    // t2 = var('t2')
    // t3 = var('t3')
    // eq1 = 368925240582247 == x + (vx-21)*t1
    // eq2 = 337542061908847 == y + (vy+126)*t1
    // eq3 = 298737178993847 == z + (vz+9)*t1
    // eq4 = 287668477092999 == x + (vx+21)*t2
    // eq5 = 306868689869154 == y + (vy+15)*t2
    // eq6 = 240173335647821 == z + (vz-29)*t2
    // eq7 = 172063062341522 == x + (vx+25)*t3
    // eq8 = 378381220662744 == y + (vy+38)*t3
    // eq9 = 223621999511007 == z + (vz+64)*t3
    // solutions = solve([eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8,eq9],x,y,z,vx,vy,vz,t1,t2,t3)
    // solutions[0][0]+solutions[0][1]+solutions[0][2]
    return 646810057104753
}
