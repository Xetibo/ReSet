use std::cell::RefCell;
use std::sync::Arc;

use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, Label, PasswordEntry, PasswordEntryBuffer, Popover};

use super::popup;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetPopup.ui")]
pub struct Popup {
    #[template_child]
    pub reset_popup_label: TemplateChild<Label>,
    #[template_child]
    pub reset_popup_entry: TemplateChild<PasswordEntry>,
    #[template_child]
    pub reset_popup_button: TemplateChild<Button>,
    pub reset_popup_text: Arc<RefCell<PasswordEntryBuffer>>,
}

unsafe impl Send for Popup {}
unsafe impl Sync for Popup {}

#[glib::object_subclass]
impl ObjectSubclass for Popup {
    const ABSTRACT: bool = false;
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
