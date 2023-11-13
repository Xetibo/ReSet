use crate::components::output::audioSourceImpl;
use adw::glib;
use adw::glib::Object;
use glib::subclass::types::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct AudioSourceEntry(ObjectSubclass<audioSourceImpl::AudioSourceEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AudioSourceEntry {
    pub fn new(name: String, volume: Vec<u32>, muted: bool, index: u32) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator 
        // TODO map the slider to volume
        // TODO properly use volume fraction
        // TODO map mute to callback
        // TODO map dropdown
        {
            let imp = obj.imp();
            imp.associatedIndex.set(index);
            imp.isMuted.set(muted);
            let mut volume_borrow = imp.volume.borrow_mut();
            *volume_borrow = volume;
            imp.resetSourceName.set_text(name.as_str());
            let fraction = (volume_borrow.first().unwrap_or_else(|| &(0 as u32)) / 100) as f64;
            imp.resetVolumeMeter.set_fraction(fraction);
            let percentage = (fraction * 100 as f64).to_string();
            imp.resetVolumePercentage.set_text(&percentage);
        }
        obj
    }
}
