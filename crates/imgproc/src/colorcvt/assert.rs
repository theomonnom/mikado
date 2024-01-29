#[inline]
pub fn valid_420(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    _width: u32,
    height: u32,
) {
    let chroma_height = (height + 1) / 2;
    assert!(src_y.len() >= (src_stride_y * height) as usize);
    assert!(src_u.len() >= (src_stride_u * chroma_height) as usize);
    assert!(src_v.len() >= (src_stride_v * chroma_height) as usize);
}

/*
#[inline]
pub fn valid_420a(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    src_a: &[u8],
    src_stride_a: u32,
    width: u32,
    height: u32,
) {
    valid_420(src_y, src_stride_y, src_u, src_stride_u, src_v, src_stride_v, width, height);
    assert!(src_a.len() >= (src_stride_a * height) as usize);
}
*/

#[inline]
pub fn valid_422(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    _width: u32,
    height: u32,
) {
    assert!(src_y.len() >= (src_stride_y * height) as usize);
    assert!(src_u.len() >= (src_stride_u * height) as usize);
    assert!(src_v.len() >= (src_stride_v * height) as usize);
}

#[inline]
pub fn valid_444(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    _width: u32,
    height: u32,
) {
    assert!(src_y.len() >= (src_stride_y * height) as usize);
    assert!(src_u.len() >= (src_stride_u * height) as usize);
    assert!(src_v.len() >= (src_stride_v * height) as usize);
}

#[inline]
pub fn valid_010(
    src_y: &[u16],
    src_stride_y: u32,
    src_u: &[u16],
    src_stride_u: u32,
    src_v: &[u16],
    src_stride_v: u32,
    _width: u32,
    height: u32,
) {
    let chroma_height = (height + 1) / 2;
    assert!(src_y.len() >= (src_stride_y * height) as usize);
    assert!(src_u.len() >= (src_stride_u * chroma_height) as usize);
    assert!(src_v.len() >= (src_stride_v * chroma_height) as usize);
}

#[inline]
pub fn valid_nv12(
    src_y: &[u8],
    src_stride_y: u32,
    src_uv: &[u8],
    src_stride_uv: u32,
    _width: u32,
    height: u32,
) {
    let chroma_height = (height + 1) / 2;
    assert!(src_y.len() >= (src_stride_y * height) as usize);
    assert!(src_uv.len() >= (src_stride_uv * chroma_height) as usize);
}

#[inline]
pub fn valid_rgba(src_argb: &[u8], src_stride_argb: u32, _width: u32, height: u32) {
    assert!(src_argb.len() >= (src_stride_argb * height) as usize);
}

#[inline]
pub fn valid_rgb(src_rgb: &[u8], src_stride_rgb: u32, _width: u32, height: u32) {
    assert!(src_rgb.len() >= (src_stride_rgb * height) as usize);
}
