use yuv_sys;

#[inline]
fn assert_valid_420(
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
fn assert_valid_argb(src_argb: &[u8], src_stride_argb: u32, _width: u32, height: u32) {
    assert!(src_argb.len() >= (src_stride_argb * height) as usize);
}

macro_rules! x420_to_rgba {
    ($rust_fnc:ident, $yuv_sys_fnc:ident) => {
        pub fn $rust_fnc(
            src_y: &[u8],
            stride_y: u32,
            src_u: &[u8],
            stride_u: u32,
            src_v: &[u8],
            stride_v: u32,
            dst_rgba: &mut [u8],
            dst_stride_rgba: u32,
            width: u32,
            height: u32,
        ) -> bool {
            // Make sure it is safe to execute the sys function
            assert_valid_420(src_y, stride_y, src_u, stride_u, src_v, stride_v, width, height);
            assert_valid_argb(dst_rgba, dst_stride_rgba, width, height);

            unsafe {
                yuv_sys::$yuv_sys_fnc(
                    src_y.as_ptr(),
                    stride_y as i32,
                    src_u.as_ptr(),
                    stride_u as i32,
                    src_v.as_ptr(),
                    stride_v as i32,
                    dst_rgba.as_mut_ptr(),
                    dst_stride_rgba as i32,
                    width as i32,
                    height as i32,
                ) == 0
            }
        }
    };
}

x420_to_rgba!(i420_to_rgba, rs_I420ToRGBA);
x420_to_rgba!(i420_to_abgr, rs_I420ToABGR);
x420_to_rgba!(i420_to_bgra, rs_I420ToBGRA);
x420_to_rgba!(j420_to_argb, rs_J420ToARGB);
x420_to_rgba!(j420_to_abgr, rs_J420ToABGR);
x420_to_rgba!(h420_to_argb, rs_H420ToARGB);
x420_to_rgba!(h420_to_abgr, rs_H420ToABGR);
x420_to_rgba!(u420_to_argb, rs_U420ToARGB);
x420_to_rgba!(u420_to_abgr, rs_U420ToABGR);

macro_rules! rgba_to_rgba {
    ($rust_fnc:ident, $yuv_sys_fnc:ident) => {
        pub fn $rust_fnc(
            src_abgr: &[u8],
            src_stride_abgr: u32,
            dst_argb: &mut [u8],
            dst_stride_argb: u32,
            width: u32,
            height: u32,
        ) -> bool {
            assert_valid_argb(src_abgr, src_stride_abgr, width, height);
            assert_valid_argb(dst_argb, dst_stride_argb, width, height);

            unsafe {
                yuv_sys::$yuv_sys_fnc(
                    src_abgr.as_ptr(),
                    src_stride_abgr as i32,
                    dst_argb.as_mut_ptr(),
                    dst_stride_argb as i32,
                    width as i32,
                    height as i32,
                ) == 0
            }
        }
    };
}

rgba_to_rgba!(abgr_to_argb, rs_ABGRToARGB);
rgba_to_rgba!(argb_to_abgr, rs_ARGBToABGR);
rgba_to_rgba!(rgba_to_argb, rs_RGBAToARGB);
rgba_to_rgba!(bgra_to_argb, rs_BGRAToARGB);

macro_rules! rgba_to_420 {
    ($rust_fnc:ident, $yuv_sys_fnc:ident) => {
        pub fn $rust_fnc(
            src_rgba: &[u8],
            src_stride_rgba: u32,
            dst_y: &mut [u8],
            dst_stride_y: u32,
            dst_u: &mut [u8],
            dst_stride_u: u32,
            dst_v: &mut [u8],
            dst_stride_v: u32,
            width: u32,
            height: u32,
        ) -> bool {
            assert_valid_argb(src_rgba, src_stride_rgba, width, height);
            assert_valid_420(
                dst_y,
                dst_stride_y,
                dst_u,
                dst_stride_u,
                dst_v,
                dst_stride_v,
                width,
                height,
            );

            unsafe {
                yuv_sys::$yuv_sys_fnc(
                    src_rgba.as_ptr(),
                    src_stride_rgba as i32,
                    dst_y.as_mut_ptr(),
                    dst_stride_y as i32,
                    dst_u.as_mut_ptr(),
                    dst_stride_u as i32,
                    dst_v.as_mut_ptr(),
                    dst_stride_v as i32,
                    width as i32,
                    height as i32,
                ) == 0
            }
        }
    };
}

rgba_to_420!(rgba_to_i420, rs_RGBAToI420);
rgba_to_420!(bgra_to_i420, rs_BGRAToI420);
rgba_to_420!(argb_to_i420, rs_ARGBToI420);
rgba_to_420!(abgr_to_i420, rs_ABGRToI420);

pub fn raw_to_argb() {} // Should this be named rgb24 instead of RAW

pub fn argb_to_rgb24() {}
