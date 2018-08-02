use std::vec::Vec;
use edge::{Edge,Edge0,Edge1,Edge2,Edge3};

pub fn compute(e:Vec<i64>, c:Vec<Edge>)->i64{
    unimplemented!()
}

fn compute_shortest_path(
    d:Vec<i64>,
    prev:Vec<i32>,
    from:i32,
    cost_forward:Vec<Edge1>,
    cost_backward:Vec<Edge2>,
    e:Vec<i64>,
    l:Vec<i32>)->(){
    unimplemented!()
}

fn heap_remove_first(q:Vec<Edge3>, nodes_to_q:Vec<i32>)->(){
    unimplemented!()
}

fn heap_decrease_key(q:Vec<Edge3>, nodes_to_q:Vec<i32>, v:i32, alt:i64)->(){
    unimplemented!()
}

fn swap_heap(q:Vec<Edge3>, nodes_to_q:Vec<i32>, i:i32, j:i32)->(){
    unimplemented!()
}

fn heapify(q:Vec<Edge3>, nodes_to_q:Vec<i32>, i:i32)->(){
    unimplemented!()
}

fn left(i:i32)->i32{
    return 2 * (i + 1) - 1;
}

fn right(i:i32)->i32{
    return 2 * (i + 1);
}

fn parent(i:i32)->i32{
    return (i - 1) / 2;
}
