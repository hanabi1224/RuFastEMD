#[derive(Debug, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: i64,
}

#[derive(Debug, Clone)]
pub struct Edge0 {
    pub to: usize,
    pub cost: i64,
    pub flow: i64,
}

#[derive(Debug, Clone)]
pub struct Edge1 {
    pub to: usize,
    pub reduced_cost: i64,
}

#[derive(Debug, Clone)]
pub struct Edge2 {
    pub to: usize,
    pub reduced_cost: i64,
    pub residual_capacity: i64,
}

#[derive(Debug, Clone, Default)]
pub struct Edge3 {
    pub to: usize,
    pub distance: i64,
}
