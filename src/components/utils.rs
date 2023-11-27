use glib::Cast;
use gtk::{Align, SignalListItemFactory, StringObject};
use gtk::prelude::{GObjectPropertyExpressionExt, ListItemExt, WidgetExt};

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