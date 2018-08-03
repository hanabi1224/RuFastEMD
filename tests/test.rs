#[cfg(test)]
mod tests {
    extern crate ru_fastemd;
    use self::ru_fastemd::distance as fastemd_distance;
    use self::ru_fastemd::Signature;
    use self::ru_fastemd::Feature;

    //use std::vec::Vec;

    #[test]
    fn sanity_tests() {
        let sig1 = Signature{
            features: vec![
                Feature{
                    array: vec![100.0, 40.0, 22.0],
                },
                Feature{
                    array: vec![211.0, 20.0, 2.0],
                },
                Feature{
                    array: vec![32.0, 190.0, 150.0],
                },
                Feature{
                    array: vec![2.0, 100.0, 100.0],
                },
            ],
            weights: vec![0.4, 0.3, 0.2, 0.1],
        };

        let sig2 = Signature{
            features: vec![
                Feature{
                    array: vec![0.0, 0.0, 0.0],
                },
                Feature{
                    array: vec![50.0, 100.0, 80.0],
                },
                Feature{
                    array: vec![255.0, 255.0, 255.0],
                },
            ],
            weights: vec![0.5, 0.3, 0.2],
        };

        let dist = fastemd_distance(&sig1, &sig2);
        assert_eq!(dist, 160.5427069837236);
    }

    /*
    fn verify(left:&Vec<f64>, right:&Vec<f64>, expected:f64){
        unimplemented!()
    }
    */
}
