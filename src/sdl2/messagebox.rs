use std::ffi::CString;
use std::ptr;

use video::Window;
use get_error;
use SdlResult;

use sys::messagebox as ll;

bitflags! {
    flags MessageBoxFlag: u32 {
        const MESSAGEBOX_ERROR = ll::SDL_MESSAGEBOX_ERROR,
        const MESSAGEBOX_WARNING = ll::SDL_MESSAGEBOX_WARNING,
        const MESSAGEBOX_INFORMATION = ll::SDL_MESSAGEBOX_INFORMATION
    }
}

pub fn show_simple_message_box(flags: MessageBoxFlag, title: &str, message: &str, window: Option<&Window>) -> SdlResult<()> {
    let result = unsafe {
        let title_cstr = CString::new(title).unwrap().as_ptr();
        let message_cstr = CString::new(message).unwrap().as_ptr();
        ll::SDL_ShowSimpleMessageBox(flags.bits(), 
                                     title_cstr, 
                                     message_cstr, 
                                     window.map_or(ptr::null(), |win| win.raw()))
    } == 0;

    if result {
        Ok(())
    } else {
        Err(get_error())
    }
}
