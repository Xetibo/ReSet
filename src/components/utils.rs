use std::time::Duration;

use adw::gdk::pango::EllipsizeMode;
use adw::prelude::ListModelExtManual;
use adw::ComboRow;
use dbus::blocking::{Connection, Proxy};
use glib::{Cast, Object};
use gtk::prelude::{GObjectPropertyExpressionExt, ListBoxRowExt, ListItemExt, WidgetExt};
use gtk::{Align, SignalListItemFactory, StringObject};

pub fn createDropdownLabelFactory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let label = gtk::Label::new(None);
        label.set_halign(Align::Start);
        item.property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", gtk::Widget::NONE);
        item.set_child(Some(&label));
    });
    factory
}

pub fn setComboRowEllipsis(element: ComboRow) {
    for (i, child) in element
        .child()
        .unwrap()
        .observe_children()
        .iter::<Object>()
        .enumerate()
    {
        if i == 2 {
            if let Ok(object) = child {
                if let Some(item) = object.downcast_ref::<gtk::Box>() {
                    if let Some(widget) = item.first_child() {
                        if let Some(label) = widget.downcast_ref::<gtk::Label>() {
                            label.set_ellipsize(EllipsizeMode::End);
                            label.set_max_width_chars(1);
                        }
                    }
                }
            }
        }
    }
}
