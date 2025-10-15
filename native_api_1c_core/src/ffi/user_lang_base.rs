use crate::interface::AddInWrapper;

use widestring::U16CStr;

type This<T> = super::This<{ super::offset::USER_LANG }, T>;

#[repr(C)]
pub struct UserLanguageBaseVTable<T: AddInWrapper> {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    set_user_interface_language_code:
        unsafe extern "system" fn(&mut This<T>, *const u16),
}

impl<T: AddInWrapper> Default for UserLanguageBaseVTable<T> {
    fn default() -> Self {
        Self {
            dtor: 0,
            #[cfg(target_family = "unix")]
            dtor2: 0,
            set_user_interface_language_code,
        }
    }
}

unsafe extern "system" fn set_user_interface_language_code<T: AddInWrapper>(
    this: &mut This<T>,
    lang: *const u16,
) {
    let component = unsafe { this.get_component() };
    let lang = unsafe { U16CStr::from_ptr_str(lang) };
    component.addin.set_user_interface_language_code(lang)
}
