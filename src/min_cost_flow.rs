use super::edge::*;
use std::i64;
use std::vec::Vec;

pub fn compute(e: &mut Vec<i64>, c: &[Vec<Edge>]) -> i64 {
    let node_count = e.len();

    let mut x = vec![Vec::<Edge0>::new(); node_count];

    for from in 0..node_count {
        for edge in &c[from] {
            x[from].push(Edge0 {
                to: edge.to,
                cost: edge.cost,
                flow: 0,
            });
            x[edge.to].push(Edge0 {
                to: from,
                cost: -edge.cost,
                flow: 0,
            });
        }
    }

    // reduced costs for forward edges (c[i,j]-pi[i]+pi[j])
    // Note that for forward edges the residual capacity is infinity
    let mut r_cost_forward = vec![Vec::<Edge1>::new(); node_count];

    // reduced costs and capacity for backward edges
    // (c[j,i]-pi[j]+pi[i])
    // Since the flow at the beginning is 0, the residual capacity is
    // also zero
    let mut r_cost_cap_backward = vec![Vec::<Edge2>::new(); node_count];

    for from in 0..node_count {
        for edge in &c[from] {
            r_cost_forward[from].push(Edge1 {
                to: edge.to,
                reduced_cost: edge.cost,
            });
            r_cost_cap_backward[edge.to].push(Edge2 {
                to: from,
                reduced_cost: -edge.cost,
                residual_capacity: 0,
            });
        }
    }

    let mut d = vec![0; node_count];
    let mut prev = vec![0; node_count];

    let mut delta: i64;
    loop {
        let mut max_supply: i64 = 0;
        let mut k: usize = 0;
        for (i, &item) in e.iter().enumerate().take(node_count) {
            if item > 0 && max_supply < item {
                max_supply = item;
                k = i;
            }
        }

        if max_supply == 0 {
            break;
        }

        delta = max_supply;

        let mut l = vec![0; 1];

        compute_shortest_path(
            &mut d,
            &mut prev,
            k,
            &mut r_cost_forward,
            &mut r_cost_cap_backward,
            e,
            &mut l,
        );

        // find delta (minimum on the path from k to l)
        // delta= e[k];
        // if (-e[l]<delta) delta= e[k];
        let mut to = l[0];
        loop {
            let from = prev[to];
            // residual
            let mut itccb = 0;
            while itccb < r_cost_cap_backward[from].len()
                && r_cost_cap_backward[from][itccb].to != to
            {
                itccb += 1;
            }

            if itccb < r_cost_cap_backward[from].len()
                && r_cost_cap_backward[from][itccb].residual_capacity < delta
            {
                delta = r_cost_cap_backward[from][itccb].residual_capacity;
            }

            to = from;

            if to == k {
                break;
            }
        }

        // augment delta flow from k to l (backwards actually...)
        to = l[0];
        loop {
            let from = prev[to];
            // TODO - might do here O(n) can be done in O(1)
            let mut itx = 0;
            while x[from][itx].to != to {
                itx += 1;
            }

            x[from][itx].flow += delta;

            // update residual for backward edges
            let mut itccb = 0;
            while itccb < r_cost_cap_backward[to].len() && r_cost_cap_backward[to][itccb].to != from
            {
                itccb += 1;
            }

            if itccb < r_cost_cap_backward[to].len() {
                r_cost_cap_backward[to][itccb].residual_capacity += delta;
            }

            let mut itccb = 0;
            while itccb < r_cost_cap_backward[from].len()
                && r_cost_cap_backward[from][itccb].to != to
            {
                itccb += 1;
            }

            if itccb < r_cost_cap_backward[from].len() {
                r_cost_cap_backward[from][itccb].residual_capacity -= delta;
            }

            // update e
            e[to] += delta;
            e[from] -= delta;

            to = from;

            if to == k {
                break;
            }
        }
    }

    // compute distance from x
    let mut dist = 0;
    for d in &x[..node_count] {
        for edge in d {
            dist += edge.cost * edge.flow;
        }
    }

    dist
}

fn compute_shortest_path(
    d: &mut Vec<i64>,
    prev: &mut Vec<usize>,
    from: usize,
    cost_forward: &mut Vec<Vec<Edge1>>,
    cost_backward: &mut Vec<Vec<Edge2>>,
    e: &[i64],
    l: &mut Vec<usize>,
) {
    let node_count = e.len();
    let mut nodes_to_q = vec![0; node_count];
    let mut q = vec![Edge3::default(); node_count];

    q[0].to = from;

    let mut j = 1;
    // for i in 0..from {
    for (i, item) in nodes_to_q.iter_mut().enumerate().take(from) {
        q[j].to = i;
        q[j].distance = i64::MAX;
        *item = j;
        j += 1;
    }

    for (i, item) in nodes_to_q
        .iter_mut()
        .enumerate()
        .take(node_count)
        .skip(from + 1)
    {
        q[j].to = i;
        q[j].distance = i64::MAX;
        *item = j;
        j += 1;
    }

    let mut final_nodes_flag = vec![false; node_count];
    loop {
        let u = q[0].to;
        d[u] = q[0].distance;
        final_nodes_flag[u] = true;
        if e[u] < 0 {
            l[0] = u;
            break;
        }

        heap_remove_first(&mut q, &mut nodes_to_q);

        for edge in &(cost_forward[u]) {
            let alt = d[u] + edge.reduced_cost;
            let v = edge.to;
            if nodes_to_q[v] < q.len() && alt < q[nodes_to_q[v]].distance {
                heap_decrease_key(&mut q, &mut nodes_to_q, v, alt);
                prev[v] = u;
            }
        }

        for edge in &(cost_backward[u]) {
            if edge.residual_capacity > 0 {
                let alt = d[u] + edge.reduced_cost;
                let v = edge.to;
                if nodes_to_q[v] < q.len() && alt < q[nodes_to_q[v]].distance {
                    heap_decrease_key(&mut q, &mut nodes_to_q, v, alt);
                    prev[v] = u;
                }
            }
        }

        if q.is_empty() {
            break;
        }
    }

    for _from in 0..node_count {
        for edge in &mut (cost_forward[_from]) {
            if final_nodes_flag[_from] {
                edge.reduced_cost += d[_from] - d[l[0]];
            }

            if final_nodes_flag[edge.to] {
                edge.reduced_cost -= d[edge.to] - d[l[0]];
            }
        }

        for edge in &mut (cost_backward[_from]) {
            if final_nodes_flag[_from] {
                edge.reduced_cost += d[_from] - d[l[0]];
            }

            if final_nodes_flag[edge.to] {
                edge.reduced_cost -= d[edge.to] - d[l[0]];
            }
        }
    }
}

fn heap_remove_first(q: &mut Vec<Edge3>, nodes_to_q: &mut Vec<usize>) {
    let j = q.len() - 1;
    swap_heap(q, nodes_to_q, 0, j);
    q.remove(j);
    heapify(q, nodes_to_q, 0);
}

fn heap_decrease_key(q: &mut Vec<Edge3>, nodes_to_q: &mut Vec<usize>, v: usize, alt: i64) {
    let mut i = nodes_to_q[v];
    let mut parent_index = parent(i);
    q[i].distance = alt;
    while i > 0 && q[parent_index].distance > q[i].distance {
        swap_heap(q, nodes_to_q, i, parent_index);
        i = parent_index;
        parent_index = parent(i);
    }
}

fn swap_heap(q: &mut Vec<Edge3>, nodes_to_q: &mut Vec<usize>, i: usize, j: usize) {
    q.swap(i, j);

    nodes_to_q[q[j].to] = j;
    nodes_to_q[q[i].to] = i;
}

fn heapify(q: &mut Vec<Edge3>, nodes_to_q: &mut Vec<usize>, i: usize) {
    let mut _i = i;
    loop {
        let l = left(_i);
        let r = right(_i);
        let mut smallest;

        if l < q.len() && q[l].distance < q[_i].distance {
            smallest = l;
        } else {
            smallest = _i;
        }

        if r < q.len() && q[r].distance < q[smallest].distance {
            smallest = r;
        }

        if smallest == _i {
            return;
        }

        swap_heap(q, nodes_to_q, _i, smallest);
        _i = smallest;
    }
}

fn left(i: usize) -> usize {
    2 * (i + 1) - 1
}

fn right(i: usize) -> usize {
    2 * (i + 1)
}

fn parent(i: usize) -> usize {
    if i < 1 {
        return 0;
    }
    (i - 1) / 2
}
