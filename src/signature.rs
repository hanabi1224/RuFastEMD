use feature::Feature;

pub struct Signature{
    pub features:Vec<Feature>,
    pub weights:Vec<f64>,
}

impl Signature{
    pub fn get_feature_dimension(&self)->usize{
        return self.features.len();
    }
}
