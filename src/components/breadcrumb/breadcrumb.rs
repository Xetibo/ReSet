use adw::glib;
use adw::glib::Object;
use glib::{closure_local, ObjectExt, PropertyGet};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::Label;
use gtk::prelude::{BoxExt, WidgetExt};

use crate::components::breadcrumb::{breadcrumbImpl, CustomButton};
use crate::components::breadcrumb::breadcrumbItem::BreadcrumbItem;

glib::wrapper! {
    pub struct Breadcrumb(ObjectSubclass<breadcrumbImpl::Breadcrumb>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl Breadcrumb {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn resetAndSet(&self, name : &str) {
        let selfImp = self.imp();
        loop {
            let option = selfImp.resetBox.last_child();
            match option {
                None => break,
                Some(last) => selfImp.resetBox.remove(&last)
            }
        }
        *selfImp.items.borrow_mut() = 0;
        self.pushBreadcrumb(name);
    }

    pub fn pushBreadcrumb(&self, name: &str) {
        let selfImp = self.imp();

        let button = CustomButton::new();
        button.connect_closure("max-number-reached",
                               false,
                               closure_local!(move |_button: CustomButton, number: i32| {
            println!("The maximum number {} has been reached", number);
        }));


        let mut items = selfImp.items.borrow_mut();
        if *items != 0 {
            selfImp.resetBox.append(&Label::new(Some(">")));
        };
        *items += 1;

        selfImp.resetBox.append(&BreadcrumbItem::new(name));
        selfImp.resetBox.append(&button);
    }

    pub fn popBreadcrumb(&self) {
        let selfImp = self.imp();
        for _ in 0..2 {
            let option = selfImp.resetBox.last_child();
            match option {
                None => break,
                Some(last) => selfImp.resetBox.remove(&last)
            }
        }
    }
}
