pub struct Edge{
    pub to:usize,
    pub cost:i64,
}

pub struct Edge0{
    pub to:usize,
    pub cost:i64,
    pub flow:i64,
}

pub struct Edge1{
    pub to:usize,
    pub reduced_cost:i64,
}

pub struct Edge2{
    pub to:usize,
    pub reduced_cost:i64,
    pub residual_capacity: i64,
}

pub struct Edge3{
    pub to:usize,
    pub distance:i64,
}

impl Edge3{
    pub fn new()->Edge3{
        return Edge3{
            to:0,
            distance:0,
        }
    }
}