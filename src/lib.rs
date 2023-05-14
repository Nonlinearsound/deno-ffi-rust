use std::ffi::CStr;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

// C-ABI
#[no_mangle]
pub extern "C" fn add(x: f64, y: f64) -> f64 {
    x+y
}

#[no_mangle]
pub extern "C" fn print_string(str_ptr: *const i8) {
    let my_string: &str;

    unsafe {
       my_string = CStr::from_ptr(str_ptr)
            .to_str()
            .expect("The string could not be parsed")
    }

    println!("The string is: {}", my_string);
}

#[no_mangle]
pub extern "C" fn get_mouse_pos(coords_ptr: *mut [i32; 2]) -> bool {
    let mut p = POINT { x:0, y: 0 };
    
    unsafe {
        let success = GetCursorPos(&mut p).as_bool();

        if success {
            let view:  &mut [i32;2] = &mut *coords_ptr;
            view[0] = p.x;
            view[1] = p.y;
        }

        return success;
    }
}