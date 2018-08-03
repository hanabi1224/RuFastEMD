mod feature;
mod edge;
mod signature;
mod min_cost_flow;

pub use signature::Signature;

use std::vec::Vec;
use std::collections::HashSet;
use edge::{Edge};


pub fn distance(signature1:Signature, signature2:Signature)->f64{
    let total_feature_count = signature1.get_feature_dimension() + signature2.get_feature_dimension();
    let mut p = Vec::<f64>::with_capacity(total_feature_count);
    for i in 0..signature1.get_feature_dimension(){
        p[i] = signature1.weights[i];
    }

    let mut q = Vec::<f64>::with_capacity(total_feature_count);
    for i in 0..signature2.get_feature_dimension(){
        q[signature1.get_feature_dimension() + i] = signature2.weights[i];
    }

    let mut c = Vec::<Vec<f64>>::with_capacity(total_feature_count);
    for i in 0..total_feature_count{
        c[i] = Vec::<f64>::with_capacity(total_feature_count);
    }

    let mut max_dist:f64 = 0.0;
    for i in 0..signature1.get_feature_dimension(){
        for j in 0..signature2.get_feature_dimension(){
            let dist = signature1.features[i].ground_distance(&signature2.features[j]);
            c[i][j + signature1.get_feature_dimension()] = dist;
            c[j + signature1.get_feature_dimension()][i] = dist;
            if dist > max_dist{
                max_dist = dist;
            }
        }
    }

    if max_dist == 0.0{
        return 0.0;
    }

    return  emd_hat(p,q,c,max_dist);
}

fn emd_hat(p:Vec<f64>,q:Vec<f64>,c:Vec<Vec<f64>>,max_c:f64)->f64{
    const MULT_FACTOR:f64 = 1000000.0;
    let n = p.len();
    let mut ip:Vec<i64> = Vec::with_capacity(n);
    let mut iq:Vec<i64> = Vec::with_capacity(n);
    let mut ic:Vec<Vec<i64>> = Vec::with_capacity(n);
    for i in 0..n{
        ic[i] = Vec::<i64>::with_capacity(n);
    }

    let sum_p:f64 = p.iter().sum();
    let sum_q:f64 = q.iter().sum();

    let max_sum = sum_p.max(sum_q);
    let min_sum = sum_p.min(sum_q);

    let pq_norm_factor = MULT_FACTOR / max_sum;
    let c_norm_factor = MULT_FACTOR / max_c;

    let mut max_ic:i64=0;
    for i in 0..n{
        ip[i] = (p[i] * pq_norm_factor + 0.5).floor() as i64;
        iq[i] = (q[i] * pq_norm_factor + 0.5).floor() as i64;
        for j in 0..n{
            let value = (c[i][j] * c_norm_factor + 0.5).floor() as i64;
            ic[i][j] = value;
            if value > max_ic{
                max_ic = value;
            }
        }
    }

    // Computing distance without extra mass penalty
    let mut dist = emd_hat_impl_i64(ip, iq, ic, max_ic) as f64;
    dist = dist / pq_norm_factor / c_norm_factor;
    dist += (max_sum - min_sum) * max_c;
    return dist;
}

fn emd_hat_impl_i64(pc:Vec<i64>, qc:Vec<i64>, c:Vec<Vec<i64>>, max_c:i64)->i64{
    let n = pc.len();

    let sum_p:i64 = pc.iter().sum();
    let sum_q:i64 = qc.iter().sum();

    let abs_diff_sum_p_sum_q;
    let p:Vec<i64>;
    let q:Vec<i64>;

    if sum_q > sum_p{
        p = qc;
        q = pc;
        abs_diff_sum_p_sum_q = sum_q - sum_p;
    }
    else{
        p = pc;
        q = qc;
        abs_diff_sum_p_sum_q = sum_p - sum_q;
    }

    let mut b:Vec<i64> = Vec::with_capacity(2 * n + 2);
    let threshold_index = 2 * n;
    let artificial_index = threshold_index + 1;

    for i in 0..n{
        b[i] = p[i];
    }

    for i in n..2*n{
        b[i] = q[i-n];
    }

    b[threshold_index] = -abs_diff_sum_p_sum_q;
    b[artificial_index] = 0;

    let mut sources_that_flow_not_only_to_thresh = HashSet::<usize>::new();
    let mut sinks_that_get_flow_not_only_from_thresh = HashSet::<usize>::new();

    let mut pre_flow_cost:i64=0;
    let mut _c = Vec::<Vec<Edge>>::with_capacity(b.len());
    for i in 0.._c.len(){
        _c[i] = Vec::<Edge>::new();
    }

    for i in 0..n{
        if b[i] == 0{
            continue;
        }

        for k in 0..n{
            if b[k+n] == 0{
                continue;
            }

            let c_value = c[i][k];
            if c_value == max_c{
                continue;
            }

            _c[i].push(Edge{
                to: k + n,
                cost: c_value,
            });
            sources_that_flow_not_only_to_thresh.insert(i);
            sinks_that_get_flow_not_only_from_thresh.insert(k+n);
        }
    }

    // Converting all sinks to negative
    for i in n..2*n{
        b[i] = -b[i];
    }

    // Add edges from/to threshold node,
    // Note that costs are reversed to the paper (see also remark* above)
    // It is important that it will be this way because of remark* above.
    for i in 0..n{
        _c[i].push(Edge{
            to: threshold_index,
            cost: 0,
        });
        _c[threshold_index].push(Edge{
            to: i+n,
            cost:max_c,
        });
    }

    // artificial arcs - Note the restriction that only one edge i,j is
    // artificial so I ignore it...
    for i in 0..artificial_index{
        _c[i].push(Edge{
            to: artificial_index,
            cost: max_c + 1,
        });
        _c[artificial_index].push(Edge{
            to: i,
            cost: max_c + 1,
        });
    }

    // remove nodes with supply demand of 0
    // and vertexes that are connected only to the
    // threshold vertex
    // using None as a special flag !!!
    let mut nodes_new_indices = Vec::<Option<usize>>::with_capacity(b.len());
    let mut nodes_old_indices = Vec::<usize>::with_capacity(b.len());
    for i in 0..nodes_new_indices.len(){
        nodes_new_indices[i] = None;
        nodes_old_indices[i] = 0;
    }

    let mut current_index:usize = 0;
    for i in 0..2*n{
        if b[i] == 0{
            continue;
        }

        if sources_that_flow_not_only_to_thresh.contains(&i)
            || sinks_that_get_flow_not_only_from_thresh.contains(&i){
            nodes_new_indices[i] = Some(current_index);
            nodes_old_indices.push(i);
            current_index += 1;
        }
        else{
            if i>=n{
                pre_flow_cost -= b[i] * max_c;
            }

            b[threshold_index] += b[i];
        }
    }

    nodes_new_indices[threshold_index] = Some(current_index);
    nodes_old_indices.push(threshold_index);
    current_index += 1;
    nodes_new_indices[artificial_index] = Some(current_index);
    nodes_old_indices.push(artificial_index);
    current_index += 1;

    let mut bb = Vec::<i64>::with_capacity(current_index);
    let mut j:usize = 0;
    for i in 0..b.len(){
        match nodes_new_indices[i] {
            None => {continue;}
            Some(_x)=>{}
        }

        bb[j] = b[i];
        j += 1;
    }

    let mut cc = Vec::<Vec<Edge>>::with_capacity(bb.len());
    for i in 0..cc.len(){
        cc[i] = Vec::<Edge>::new();
    }

    for i in 0.._c.len(){
        match nodes_new_indices[i] {
            None => {continue;}
            Some(_x)=>{}
        }

        for edge in &_c[i]{
            match nodes_new_indices[edge.to] {
                Some(x)=>{
                    cc[nodes_new_indices[i].unwrap()].push(Edge{
                        to: x,
                        cost: edge.cost,
                    });
                }
                None =>{}
            }
        }
    }

    let mcf_dist = min_cost_flow::compute(&mut bb, &cc);
    return pre_flow_cost + mcf_dist;
}
