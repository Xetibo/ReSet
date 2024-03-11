use std::cell::RefCell;
use std::sync::Arc;

use glib::clone;
use gtk::prelude::{ButtonExt, PopoverExt};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label, PasswordEntry, PasswordEntryBuffer, Popover};

use crate::components::input::source_box::SourceBox;

use super::error;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetError.ui")]
pub struct ReSetError {
    #[template_child]
    pub reset_error_label: TemplateChild<Label>,
    #[template_child]
    pub reset_error_button: TemplateChild<Button>,
}

unsafe impl Send for ReSetError {}
unsafe impl Sync for ReSetError {}

#[glib::object_subclass]
impl ObjectSubclass for ReSetError {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetError";
    type Type = error::ReSetError;
    type ParentType = Popover;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ReSetError {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ReSetError {}

impl WindowImpl for ReSetError {}

impl PopoverImpl for ReSetError {}

impl ApplicationWindowImpl for ReSetError {}

impl EditableImpl for ReSetError {}

pub fn show_error<T: ReSetErrorImpl + Send + Sync + 'static>(
    parent: Arc<T>,
    message: &'static str,
) {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let mut error = parent.error();
            let parent_ref = parent.clone();
            let imp = error.imp();
            imp.reset_error_label.set_text(message);
            imp.reset_error_button.connect_clicked(move |_| {
                parent_ref.error().popdown();
            });
            error.popup();
        });
    });
}

pub trait ReSetErrorImpl: Send + Sync {
    fn error(&self) -> &TemplateChild<error::ReSetError>;
}
