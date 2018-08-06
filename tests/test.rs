#[cfg(test)]
mod tests {
    extern crate ru_fastemd;
    use self::ru_fastemd::distance as fastemd_distance;
    use self::ru_fastemd::Signature;
    use self::ru_fastemd::Feature;

    use self::ru_fastemd::fastemd as fastemd_distance_ffi;
    use self::ru_fastemd::create_signature;
    use self::ru_fastemd::free_signature;


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

    #[test]
    fn c_ffi_sanity_tests(){
        let features1_ptr = Box::leak(Box::from([
            Box::leak(Box::from([100.0, 40.0, 22.0])).as_ptr(),
            Box::leak(Box::from([211.0, 20.0, 2.0])).as_ptr(),
            Box::leak(Box::from([32.0, 190.0, 150.0])).as_ptr(),
            Box::leak(Box::from([2.0, 100.0, 100.0])).as_ptr(),
        ])).as_ptr();
        let weights1_ptr = Box::leak(Box::from([0.4, 0.3, 0.2, 0.1])).as_ptr();

        let features2_ptr = Box::leak(Box::from([
            Box::leak(Box::from([0.0, 0.0, 0.0])).as_ptr(),
            Box::leak(Box::from([50.0, 100.0, 80.0])).as_ptr(),
            Box::leak(Box::from([255.0, 255.0, 255.0])).as_ptr(),
        ])).as_ptr();
        let weights2_ptr = Box::leak(Box::from([0.5, 0.3, 0.2])).as_ptr();

        let sig1= create_signature(features1_ptr, weights1_ptr, 4, 3);
        let sig2= create_signature(features2_ptr, weights2_ptr, 3, 3);

        let dist = fastemd_distance_ffi(sig1, sig2);
        assert_eq!(dist, 160.5427069837236);

        free_signature(sig1);
        free_signature(sig2);
    }
}
