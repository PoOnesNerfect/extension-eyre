use crate::Handler;

impl eyre::EyreHandler for Handler {
    fn debug(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        eyre::EyreHandler::debug(self.inner.as_ref(), error, f)
    }

    #[cfg(feature = "track-caller")]
    fn track_caller(&mut self, location: &'static std::panic::Location<'static>) {
        self.inner.track_caller(location);
    }
}
