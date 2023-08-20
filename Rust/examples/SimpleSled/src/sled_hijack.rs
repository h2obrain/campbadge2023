extern crate libc;
extern crate rand;

use crate::led_matrix::LedMatrix;
use core::slice;
use std::sync::{LazyLock, Mutex};
//use embedded_svc::utils::mutex::Mutex;
use smart_leds::RGB8;
use esp_idf_svc::systime::EspSystemTime;
use rand::{rngs::SmallRng, Rng, SeedableRng};
// use once_cell::sync::Lazy;
use log::*;

pub mod bindings {
    extern crate libc;
    use std::{ffi::CStr, borrow::Cow};

    #[repr(C)]
    #[derive(Debug)]
    pub struct RGB {
        pub red: libc::c_uchar,
        pub green: libc::c_uchar,
        pub blue: libc::c_uchar,
        pub alpha: libc::c_uchar,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Module {
        pub _name: u32, //*const u8, // *const libc::c_char,
        pub init: Option<unsafe extern "C" fn(u32) -> i32>,
        pub reset: Option<unsafe extern "C" fn(u32)>,
        pub draw: Option<unsafe extern "C" fn(u32) -> i32>,
        pub deinit: Option<unsafe extern "C" fn(u32)>,
    }
    impl Module {
        pub fn name<'l>(&self) -> Cow<'l, str> {
            unsafe { CStr::from_ptr(self._name as *const i8).to_string_lossy() }
        }
    }

    #[link(name = "sled")]
    extern "C" {
        // Declare any C functions or types needed for accessing sled_modules
        pub static sled_module_count: u32;
        pub static sled_modules: *const Module;
    }

    // #[link(name = "sled")]
    // extern "C" {
    //     pub fn matrix_init(outmodno: i32) -> i32;
    //     pub fn matrix_getx() -> i32;
    //     pub fn matrix_gety() -> i32;
    //     pub fn matrix_set(x: i32, y: i32, color: RGB);
    //     pub fn matrix_get(x: i32, y: i32) -> RGB;
    //     pub fn matrix_fill(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: RGB) -> i32;
    //     pub fn matrix_clear() -> i32;
    //     pub fn matrix_render() -> i32;
    //     pub fn matrix_deinit() -> i32;
    // }
}

impl Into<RGB8> for bindings::RGB {
    fn into(self) -> RGB8 {
        RGB8 { r: self.red, g: self.green, b: self.blue }
    }
}
impl From<RGB8> for bindings::RGB {
    fn from(col: RGB8) -> Self {
        bindings::RGB { red: col.r, green: col.g, blue: col.b, alpha: 255 }
    }
}

pub fn sled_modules<'l>() -> &'l [bindings::Module] {
    let mods;
    unsafe {
        mods = slice::from_raw_parts(
            bindings::sled_modules,
            usize::try_from(bindings::sled_module_count).unwrap(),
        );
    }
    mods
}

pub struct SledState<'l> {
    pub modules: &'l [bindings::Module],
    pub rng: Mutex<SmallRng>,
    pub leds: Mutex<LedMatrix>,
    current: Mutex<Option<&'l bindings::Module>>,
    // static mut CUR_MOD: u32 = 0;
    // static mut CUR_MOD_I: usize = 1;
    // // static RNG: LazyLock<SmallRng> = LazyLock::new(||{
    // //     SmallRng::seed_from_u64(170)
    // // });
    // static mut RNG: Option<SmallRng> = None;
}
impl<'l> SledState<'l> {
    pub fn new() -> Self {
        info!("new sled state");
        let s = SledState {
            rng: Mutex::new(SmallRng::seed_from_u64(170)),
            modules: sled_modules(),
            leds: Mutex::new(LedMatrix::new(1, 0, 5, 5)),
            current: Mutex::new(None),
        };
        info!("Available sled modules");
        for sm in sled_modules() {
            info!("- {}", sm.name());
        }

        s.next_mod();
        s
    }
    fn next_mod_i(&self) -> usize {
        let i = unsafe { self.rng.lock().unwrap().gen::<u32>() % bindings::sled_module_count };
        info!("next mod index is {}", i);
        i as usize // not soo nice :D
        // 0
    }
    fn next_mod(&self) {
        info!("next mod");
        let next_mod = self.next_mod_i();
        {
            let last = self.current.lock().unwrap();
            if !last.is_none() {
                info!("deinit last mod");
                unsafe { last.unwrap().deinit.unwrap()(0); }
            }
        }
        info!("get next mod");
        let current = &self.modules[next_mod];
        let next_mod = next_mod as u32;
        info!("init next mod ({}) {:?}", current.name(), current.init);
        unsafe { current.init.unwrap()(next_mod) };
        info!("update current mod");
        *self.current.lock().unwrap() = Some(&current);
    }
    pub fn sled_tick(&self) {
        let r = unsafe { self.current.lock().unwrap().unwrap().draw.unwrap()(0) };
        if r == 0 {
            self.next_mod();
        }
    }
}
pub static SLED_STATE: LazyLock<SledState> = LazyLock::new(|| {
    SledState::new()
});

// // pub static SLED_STATE.leds: Lazy<LedMatrix> = Lazy::new(|| {
// //     LedMatrix::new(1, 0, 5, 5)
// // });
// pub static SLED_STATE.leds: LazyLock<LedMatrix> = LazyLock::new(|| {
//     LedMatrix::new(1, 0, 5, 5)
// });
// // pub static mut SLED_STATE.leds: Option<LedMatrix> = None;


// if not defined use the t+=1000 hack implemented as weak function :P
#[no_mangle]
pub extern "C" fn oscore_udate() -> u64 {
    // Implement the oscore_udate function's logic here
    // You can create and return an instance of the OscoreTime struct
    EspSystemTime {}.now().as_millis().try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn matrix_init(_outmodno: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn matrix_getx() -> i32 {
    i32::try_from(SLED_STATE.leds.lock().unwrap().led_columns()).unwrap()
}

#[no_mangle]
pub extern "C" fn matrix_gety() -> i32 {
    i32::try_from(SLED_STATE.leds.lock().unwrap().led_rows()).unwrap()
}

#[no_mangle]
pub extern "C" fn matrix_set(x: i32, y: i32, color: bindings::RGB) {
    SLED_STATE.leds.lock().unwrap().set_pixel(
        x.try_into().unwrap(),
        y.try_into().unwrap(),
        color.into()
    )
}

#[no_mangle]
pub extern "C" fn matrix_get(x: i32, y: i32) -> bindings::RGB {
    SLED_STATE.leds.lock().unwrap().get_pixel(x.try_into().unwrap(), y.try_into().unwrap()).into()
}

#[no_mangle]
pub extern "C" fn matrix_fill(
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    color: bindings::RGB,
) -> i32 {
    SLED_STATE.leds.lock().unwrap().set_pixel_range(
        start_x.try_into().unwrap(),
        start_y.try_into().unwrap(),
        end_x.try_into().unwrap(),
        end_y.try_into().unwrap(),
        color.into()
    );
    0
}

#[no_mangle]
pub extern "C" fn matrix_clear() -> i32 {
    SLED_STATE.leds.lock().unwrap().set_all_pixel(RGB8::new(0, 0, 0));
    0
}

#[no_mangle]
pub extern "C" fn matrix_render() -> i32 {
    SLED_STATE.leds.lock().unwrap().write_pixels();
    0
}

#[no_mangle]
pub extern "C" fn matrix_deinit() -> i32 {
    0
}



// Main loop management

