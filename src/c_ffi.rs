use libc::size_t;
use std::slice;
use super::{Signature,Feature, distance};

ffi_fn!{
    fn fastemd(signature1_ptr: *const Signature, signature2_ptr: *const Signature) -> f64{
        let signature1:&Signature = unsafe { &*signature1_ptr };
        let signature2:&Signature = unsafe { &*signature2_ptr };

        return distance(signature1, signature2);
    }
}

ffi_fn!{
    fn create_signature(features_ptr: *const *const f64, weights_ptr: *const f64, feature_count:size_t, feature_dim:size_t)->*const Signature{
        let features_slice = unsafe { slice::from_raw_parts(features_ptr, feature_count as usize) };
        let mut features = Vec::<Feature>::with_capacity(feature_count as usize);
        for feature_ptr in features_slice{
            let feature_slice =  unsafe { slice::from_raw_parts(*feature_ptr, feature_dim as usize) };
            features.push( Feature{
                array: Vec::from(feature_slice),
            });
        }

        let weights_slice = unsafe { slice::from_raw_parts(weights_ptr, feature_count) };
        let weights = Vec::from(weights_slice);
        let signature = Signature{
            features: features,
            weights: weights,
        };

        return Box::into_raw(Box::new(signature));
    }
}

ffi_fn! {
    fn free_signature(signature_ptr:*const Signature){
        let _ = unsafe { Box::from_raw(signature_ptr as *mut Signature) };
    }
}
