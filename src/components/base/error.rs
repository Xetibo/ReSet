use adw::glib::Object;
use glib::{clone, subclass::types::ObjectSubclassIsExt};
use gtk::{
    gdk,
    prelude::{ButtonExt, PopoverExt},
    Editable, Popover,
};

use super::error_impl;

glib::wrapper! {
    pub struct ReSetError(ObjectSubclass<error_impl::ReSetError>)
    @extends Popover, gtk::Widget,
    @implements Editable,gdk::Popup, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for ReSetError {}
unsafe impl Sync for ReSetError {}

impl ReSetError {
    pub fn new() -> Self {
        let error: ReSetError = Object::builder().build();
        error
            .imp()
            .reset_error_button
            .connect_clicked(clone!(@strong error => move |_| {
                println!("pingpangpung");
               error.popdown();
            }));
        error
    }
}

impl Default for ReSetError {
    fn default() -> Self {
        Self::new()
    }
}
