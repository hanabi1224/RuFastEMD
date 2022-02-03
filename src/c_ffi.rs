use super::*;
use libc::size_t;
use std::slice;

#[repr(C)]
pub struct Signature_FFI {
    pub features: Box<[Box<[f64]>]>,
    pub weights: Box<[f64]>,
}

ffi_fn! {
    fn fastemd(signature1_ptr: *const Signature_FFI, signature2_ptr: *const Signature_FFI) -> f64{
        let signature1_ffi = unsafe { &*signature1_ptr };
        let signature2_ffi = unsafe { &*signature2_ptr };

        let mut features1 = Vec::<Feature>::new();
        for item in signature1_ffi.features.iter(){
            features1.push(Feature{
                array: item.to_vec()
            });
        }

        let signature1 = Signature{
            features: features1,
            weights: Vec::from(signature1_ffi.weights.as_ref()),
        };

        let mut features2 = Vec::<Feature>::new();
        for item in signature2_ffi.features.iter(){
            features2.push(Feature{
                array: item.to_vec()
            });
        }

        let signature2 = Signature{
            features: features2,
            weights: Vec::from(signature2_ffi.weights.as_ref()),
        };

        distance(&signature1, &signature2)
    }
}

ffi_fn! {
    fn create_signature(features_ptr: *const *const f64, weights_ptr: *const f64, feature_count:size_t, feature_dim:size_t)->*const Signature_FFI{
        let features_slice = unsafe { slice::from_raw_parts(features_ptr, feature_count as usize) };
        let mut features = Vec::<Box<[f64]>>::with_capacity(feature_count as usize);
        for feature_ptr in features_slice{
            let feature_slice =  unsafe { slice::from_raw_parts(*feature_ptr, feature_dim as usize) };
            features.push(Vec::from(feature_slice).into_boxed_slice());
        }

        let weights_slice = unsafe { slice::from_raw_parts(weights_ptr, feature_count as usize) };
        let weights = Vec::from(weights_slice).into_boxed_slice();
        let signature = Signature_FFI{
            features: features.into_boxed_slice(),
            weights,
        };

        Box::into_raw(Box::new(signature))
    }
}

ffi_fn! {
    fn free_signature(signature_ptr:*const Signature_FFI){
        let _ = unsafe { Box::from_raw(signature_ptr as *mut Signature_FFI) };
    }
}
