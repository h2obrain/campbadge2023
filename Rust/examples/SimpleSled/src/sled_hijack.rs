extern crate libc;

mod bindings {
    extern crate libc;

    #[repr(C)]
    pub struct RGB {
        pub red: libc::c_uchar,
        pub green: libc::c_uchar,
        pub blue: libc::c_uchar,
        pub alpha: libc::c_uchar,
    }

    #[repr(C)]
    pub struct Module {
        pub name: *const libc::c_char,
        pub init: Option<unsafe extern "C" fn(libc::uint32_t) -> libc::c_int>,
        pub reset: Option<unsafe extern "C" fn(libc::uint32_t)>,
        pub draw: Option<unsafe extern "C" fn(libc::uint32_t) -> libc::c_int>,
        pub deinit: Option<unsafe extern "C" fn(libc::uint32_t)>,
    }
    
    #[link(name = "sled")]
    extern "C" {
        // Declare any C functions or types needed for accessing sled_modules
        pub static const sled_module_count: libc::uint32_t;
        pub static const sled_modules: [*const Module; 4];
    }    

    // #[link(name = "sled")]
    // extern "C" {
    //     pub fn matrix_init(outmodno: libc::c_int) -> libc::c_int;
    //     pub fn matrix_getx() -> libc::c_int;
    //     pub fn matrix_gety() -> libc::c_int;
    //     pub fn matrix_set(x: libc::c_int, y: libc::c_int, color: RGB);
    //     pub fn matrix_get(x: libc::c_int, y: libc::c_int) -> RGB;
    //     pub fn matrix_fill(start_x: libc::c_int, start_y: libc::c_int, end_x: libc::c_int, end_y: libc::c_int, color: RGB) -> libc::c_int;
    //     pub fn matrix_clear() -> libc::c_int;
    //     pub fn matrix_render() -> libc::c_int;
    //     pub fn matrix_deinit() -> libc::c_int;
    // }
}

// if not defined use the t+=1000 hack implemented as weak function :P
// #[no_mangle]
// pub extern "C" fn oscore_udate() -> libc::c_uint32_t {
//     // Implement the oscore_udate function's logic here
//     // You can create and return an instance of the OscoreTime struct
//     unimplemented!("Implement oscore_udate function")
// }

#[no_mangle]
pub extern "C" fn matrix_init(outmodno: libc::c_int) -> libc::c_int {
    sled_hijack::matrix_init(outmodno)
}

#[no_mangle]
pub extern "C" fn matrix_getx() -> libc::c_int {
    sled_hijack::matrix_getx()
}

#[no_mangle]
pub extern "C" fn matrix_gety() -> libc::c_int {
    sled_hijack::matrix_gety()
}

#[no_mangle]
pub extern "C" fn matrix_set(x: libc::c_int, y: libc::c_int, color: sled_hijack::bindings::RGB) {
    sled_hijack::matrix_set(x, y, color);
}

#[no_mangle]
pub extern "C" fn matrix_get(x: libc::c_int, y: libc::c_int) -> sled_hijack::bindings::RGB {
    sled_hijack::matrix_get(x, y)
}

#[no_mangle]
pub extern "C" fn matrix_fill(start_x: libc::c_int, start_y: libc::c_int, end_x: libc::c_int, end_y: libc::c_int, color: sled_hijack::bindings::RGB) -> libc::c_int {
    sled_hijack::matrix_fill(start_x, start_y, end_x, end_y, color)
}

#[no_mangle]
pub extern "C" fn matrix_clear() -> libc::c_int {
    sled_hijack::matrix_clear()
}

#[no_mangle]
pub extern "C" fn matrix_render() -> libc::c_int {
    sled_hijack::matrix_render()
}

#[no_mangle]
pub extern "C" fn matrix_deinit() -> libc::c_int {
    sled_hijack::matrix_deinit()
}