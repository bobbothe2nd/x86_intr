use x86_intr::arch::{__m64, _m_from_int64, _m_to_int64, _mm_add_pi16};

const A_BYTES: [u8; 8] = [0, 10, 0, 5, 0, 20, 0, 15];
const B_BYTES: [u8; 8] = [0, 5, 0, 15, 0, 10, 0, 20];
const C_BYTES: [u8; 8] = [0, 15, 0, 20, 0, 30, 0, 35];

#[test]
fn mmx_vadd() {
    let a: __m64<0> = unsafe { _m_from_int64(i64::from_le_bytes(A_BYTES)) };
    let b: __m64<2> = unsafe { _m_from_int64(i64::from_le_bytes(B_BYTES)) };

    let c = unsafe { _mm_add_pi16(a, b) };

    let a_val = unsafe { _m_to_int64(a) };
    let c_val = unsafe { _m_to_int64(c) };

    assert_eq!(a_val, c_val);
    assert_eq!(c_val.to_le_bytes(), C_BYTES);
}
