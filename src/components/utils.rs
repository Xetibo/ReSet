use adw::gdk::pango::EllipsizeMode;
use adw::prelude::ListModelExtManual;
use adw::{ActionRow, ComboRow};
use glib::{Object};
use glib::prelude::Cast;
use gtk::prelude::{GObjectPropertyExpressionExt, ListBoxRowExt, ListItemExt, WidgetExt};
use gtk::{Align, SignalListItemFactory, StringObject};

pub const DBUS_PATH: &str = "/org/Xetibo/ReSet/Daemon";
pub const WIRELESS: &str = "org.Xetibo.ReSet.Wireless";
pub const BLUETOOTH: &str = "org.Xetibo.ReSet.Bluetooth";
pub const AUDIO: &str = "org.Xetibo.ReSet.Audio";
pub const BASE: &str = "org.Xetibo.ReSet.Daemon";

pub fn create_dropdown_label_factory() -> SignalListItemFactory {
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

pub fn set_combo_row_ellipsis(element: ComboRow) {
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

pub fn set_action_row_ellipsis(element: ActionRow) {
    let option = element.first_child();
    if let Some(first_box) = option {
        for (i, child) in first_box.observe_children().iter::<Object>().enumerate() {
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
}
