use x86_intr::{arch::{__m64, __tile_stored, __tile1024i, _m_from_int64, _m_to_int64, _mm_add_pi16, ace_scale_b}, is_cpuid_feature_detected};

const A_BYTES: [u8; 8] = [0, 10, 0, 5, 0, 20, 0, 15];
const B_BYTES: [u8; 8] = [0, 5, 0, 15, 0, 10, 0, 20];
const C_BYTES: [u8; 8] = [0, 15, 0, 20, 0, 30, 0, 35];

#[test]
fn mmx_vadd() {
    if is_cpuid_feature_detected!("mmx") {
        return;
    }

    let a: __m64<0> = unsafe { _m_from_int64(i64::from_le_bytes(A_BYTES)) };
    let b: __m64<2> = unsafe { _m_from_int64(i64::from_le_bytes(B_BYTES)) };

    let c = unsafe { _mm_add_pi16(a, b) };

    let a_val = unsafe { _m_to_int64(a) };
    let c_val = unsafe { _m_to_int64(c) };

    assert_eq!(a_val, c_val);
    assert_eq!(c_val.to_le_bytes(), C_BYTES);
}

#[test]
fn tile_zero() {
    if !is_cpuid_feature_detected!("amx-tile") {
        return;
    }

    let _ = ace_scale_b(0x3);

    unsafe {
        let tile = __tile1024i::<0>::zeroed();

        let mut out = [1_i32; 32];
        __tile_stored(out.as_mut_ptr().cast(), 64, tile);

        assert!(out.iter().all(|&x| x == 0));
    }
}
