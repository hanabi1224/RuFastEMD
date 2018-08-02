mod feature;
mod edge;
mod signature;
mod min_cost_flow;

use feature::Feature;
pub use signature::Signature;

use std::vec::Vec;
use std::iter::Sum;


pub fn distance(signature1:Signature, signature2:Signature)->f32{
    unimplemented!()
}

fn emd_hat(p:Vec<f64>,q:Vec<f64>,c:Vec<Vec<f64>>,maxC:f64)->f64{
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
    let c_norm_factor = MULT_FACTOR / maxC;

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
    return dist;
}

fn emd_hat_impl_i64(pc:Vec<i64>, qc:Vec<i64>, c:Vec<Vec<i64>>, maxC:i64)->i64{
    unimplemented!()
}
