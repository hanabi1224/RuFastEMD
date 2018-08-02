use feature::Feature;

pub struct Signature{
    pub features:Feature,
    pub weights:Vec<f64>,
}

impl Signature{
    fn get_feature_dimension(&self)->usize{
        return self.features.array.len();
    }
}
