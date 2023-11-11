use std::cell::RefCell;
use std::sync::Arc;

use adw::subclass::prelude::{ActionRowImpl, PreferencesRowImpl};
use adw::subclass::window::AdwWindowImpl;
use gtk::gdk_pixbuf::subclass::prelude::{
    PixbufAnimationImpl, PixbufAnimationIterImpl, PixbufLoaderImpl,
};
use gtk::prelude::PopupExt;
use gtk::subclass::prelude::*;
use gtk::{
    gdk, glib, Button, CompositeTemplate, Entry, EntryBuffer, PasswordEntry, PasswordEntryBuffer, Popover,
};

use super::popup;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetPopup.ui")]
pub struct Popup {
    #[template_child]
    pub resetPopupEntry: TemplateChild<PasswordEntry>,
    #[template_child]
    pub resetPopupButton: TemplateChild<Button>,
    pub resetPopupText: Arc<RefCell<PasswordEntryBuffer>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Popup {
    const NAME: &'static str = "resetPopup";
    type Type = popup::Popup;
    type ParentType = Popover;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Popup {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for Popup {}

impl WindowImpl for Popup {}

impl PopoverImpl for Popup {}

impl ApplicationWindowImpl for Popup {}

impl EditableImpl for Popup {}
