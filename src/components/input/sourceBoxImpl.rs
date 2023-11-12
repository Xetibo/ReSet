use gtk::{CompositeTemplate, DropDown, TemplateChild, glib};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::components::base::listEntry::ListEntry;
use crate::components::input::inputStreamEntry::InputStreamEntry;
use crate::components::input::sourceBox;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetMicrophone.ui")]
pub struct SourceBox {
    #[template_child]
    pub resetSourceDropdown: TemplateChild<DropDown>,
    #[template_child]
    pub resetSourceRow: TemplateChild<ListEntry>,
    #[template_child]
    pub resetInputStreamButton: TemplateChild<ListEntry>,
}

#[glib::object_subclass]
impl ObjectSubclass for SourceBox {
    const NAME: &'static str = "resetMicrophone";
    type Type = sourceBox::SourceBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        InputStreamEntry::ensure_type();
        ListEntry::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for SourceBox {}

impl ObjectImpl for SourceBox {
    fn constructed(&self) {
        let obj = self.obj();
        obj.setupCallbacks();
    }
}

impl ListBoxRowImpl for SourceBox {}

impl WidgetImpl for SourceBox {}

impl WindowImpl for SourceBox {}

impl ApplicationWindowImpl for SourceBox {}