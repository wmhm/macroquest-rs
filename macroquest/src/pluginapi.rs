/// The Plugin trait implements the protocol that a MacroQuest plugin must
/// implement.
///
/// For each process, there is one global plugin instance, created using the
/// Plugin::new() function, and the MacroQuest plugin hooks will get dispatched
/// to the instance methods of that plugin instance.
///
/// All MacroQuest plugin hooks have a default, no-op implementation, allowing
/// a Plugin implementation to implement only the ones that they actually care
/// about, while leaving the no-op implementations to cover any other hook.
pub trait Plugin {
    fn new() -> Self;

    /// This is called once on plugin initialization and can be considered the
    /// startup routine for the plugin.
    fn initialize(&mut self) {}

    /// This is called once when the plugin has been asked to shutdown. The
    /// plugin has not actually shut down until this completes.
    fn shutdown(&mut self) {}

    /// This is called once just before the shutdown of the UI system and each
    /// time the game requests that the UI be cleaned. Most commonly this
    /// happens when a /loadskin command is issued, but it also occurs when
    /// reaching the character select screen and when first entering the game.
    ///
    /// One purpose of this function is to allow you to destroy any custom
    /// windows that you have created and cleanup any UI items that need to be
    /// removed.
    fn on_clean_ui(&mut self) {}

    /// This is called once just after the UI system is loaded. Most commonly
    /// this happens when a /loadskin command is issued, but it also occurs when
    /// first entering the game.
    ///
    /// One purpose of this function is to allow you to recreate any custom
    /// windows that you have setup.
    fn on_reload_ui(&mut self) {}
}

#[doc(hidden)]
pub struct PluginHandler<T: Plugin> {
    data: parking_lot::Mutex<Option<T>>,
}

impl<T: Plugin> PluginHandler<T> {
    pub const fn new() -> PluginHandler<T> {
        PluginHandler {
            data: parking_lot::Mutex::new(None),
        }
    }

    pub fn replace(&self, new: Option<T>) {
        let mut plugin = self.data.lock();
        *plugin = new;
    }

    pub fn hook_initialize(&self) {
        let mut lock = self.data.lock();
        let plugin: &mut T = lock.as_mut().expect("no plugin");

        plugin.initialize()
    }
}
