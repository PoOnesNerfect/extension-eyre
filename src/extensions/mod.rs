//! Helpers for adding custom data to error reports
//!
//! [`ExtensionExt`] trait allows attaching custom data to error reports.
//!
//! [`Extension`] trait allows accessing custom data from error reports.

mod map;

use color_eyre::Report;
pub use map::Extensions;

use crate::private::Sealed;

/// Trait for accessing custom data from errors.
///
/// This trait is implemented for `eyre::Report`.
///
/// ### Example
///
/// ```rust
/// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
/// use std::process::Command;
/// use tracing::instrument;
///
/// pub struct Retry;
///
/// #[instrument]
/// fn app(path: &str) -> Result<String, Report> {
///     if let Err(err) = read_file("fake_file") {
///         if let Some(Retry) = err.extension_ref() {
///             // ...hanlde for retry
///         }
///     }
///
///     Ok(Default::default())
/// }
///
/// #[instrument]
/// fn read_file(path: &str) -> Result<String, Report> {
///     Command::new("cat").arg(path).output2()
/// }
///
/// trait Output {
///     fn output2(&mut self) -> Result<String, Report>;
/// }
///
/// impl Output for Command {
///     #[instrument]
///     fn output2(&mut self) -> Result<String, Report> {
///         let output = self.output()?;
///
///         let stdout = String::from_utf8_lossy(&output.stdout);
///
///         if !output.status.success() {
///             Err(eyre!("cmd exited with non-zero status code"))
///                 .extension(Retry)
///         } else {
///             Ok(stdout.into())
///         }
///     }
/// }
/// ```
pub trait Extension: Sealed {
    /// Method for accessing custom data from errors.
    ///
    /// This trait is implemented for `eyre::Report`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Retry;
    ///
    /// #[instrument]
    /// fn app(path: &str) -> Result<String, Report> {
    ///     if let Err(err) = read_file("fake_file") {
    ///         if let Some(Retry) = err.extension_ref() {
    ///             // ...hanlde for retry
    ///         }
    ///     }
    ///
    ///     Ok(Default::default())
    /// }
    ///
    /// #[instrument]
    /// fn read_file(path: &str) -> Result<String, Report> {
    ///     Command::new("cat").arg(path).output2()
    /// }
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code"))
    ///                 .extension(Retry)
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn extension_ref<T: Send + Sync + 'static>(&self) -> Option<&T>;

    /// Method for accessing mutable custom data from errors.
    ///
    /// This trait is implemented for `eyre::Report`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Counter(usize);
    ///
    /// #[instrument]
    /// fn app(path: &str) -> Result<String, Report> {
    ///     if let Err(mut err) = read_file("fake_file") {
    ///         if let Some(Counter(counter)) = err.extension_mut() {
    ///             *counter += 1;
    ///             return Err(err);
    ///         }
    ///     }
    ///
    ///     Ok(Default::default())
    /// }
    ///
    /// #[instrument]
    /// fn read_file(path: &str) -> Result<String, Report> {
    ///     Command::new("cat").arg(path).output2()
    /// }
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code"))
    ///                 .extension(Counter(0))
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn extension_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T>;

    /// Method for accessing [`Extensions`] typemap within the error report.
    ///
    /// This trait is implemented for `eyre::Report`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Retry;
    ///
    /// #[instrument]
    /// fn app(path: &str) -> Result<String, Report> {
    ///     if let Err(err) = read_file("fake_file") {
    ///         let extensions = err.extensions_ref().expect("map should exist");
    ///
    ///         if let Some(Retry) = extensions.get() {
    ///             // ...handle retry case
    ///         }
    ///     }
    ///
    ///     Ok(Default::default())
    /// }
    ///
    /// #[instrument]
    /// fn read_file(path: &str) -> Result<String, Report> {
    ///     Command::new("cat").arg(path).output2()
    /// }
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code"))
    ///                 .extension(Retry)
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn extensions_ref(&self) -> Option<&Extensions>;

    /// Method for accessing mutable [`Extensions`] typemap within the error report.
    ///
    /// This trait is implemented for `eyre::Report`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Retry;
    /// pub struct Counter(usize);
    ///
    /// #[instrument]
    /// fn app(path: &str) -> Result<String, Report> {
    ///     if let Err(mut err) = read_file("fake_file") {
    ///         let extensions = err.extensions_mut().expect("map should exist");
    ///
    ///         if let Some(Retry) = extensions.get() {
    ///             if let Some(Counter(counter)) = extensions.get_mut() {
    ///                 *counter += 1;
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(Default::default())
    /// }
    ///
    /// #[instrument]
    /// fn read_file(path: &str) -> Result<String, Report> {
    ///     Command::new("cat").arg(path).output2()
    /// }
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code"))
    ///                 .extension(Retry)
    ///                 .extension(Counter(0))
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn extensions_mut(&mut self) -> Option<&mut Extensions>;
}

impl Extension for Report {
    fn extension_ref<T: Send + Sync + 'static>(&self) -> Option<&T> {
        if let Some(handler) = self.handler().downcast_ref::<crate::Handler>() {
            return handler.extensions.get::<T>();
        }

        None
    }

    fn extension_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        if let Some(handler) = self.handler_mut().downcast_mut::<crate::Handler>() {
            return handler.extensions.get_mut::<T>();
        }

        None
    }

    fn extensions_ref(&self) -> Option<&Extensions> {
        if let Some(handler) = self.handler().downcast_ref::<crate::Handler>() {
            return Some(&handler.extensions);
        }

        None
    }

    fn extensions_mut(&mut self) -> Option<&mut Extensions> {
        if let Some(handler) = self.handler_mut().downcast_mut::<crate::Handler>() {
            return Some(&mut handler.extensions);
        }

        None
    }
}

/// Trait for attaching custom data to errors.
///
/// This trait is implemented for `eyre::Report` and `Result<T, E> where E: std::error::Error`.
///
/// ### Example
///
/// ```rust
/// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
/// use std::process::Command;
/// use tracing::instrument;
///
/// pub struct Retry;
///
/// trait Output {
///     fn output2(&mut self) -> Result<String, Report>;
/// }
///
/// impl Output for Command {
///     #[instrument]
///     fn output2(&mut self) -> Result<String, Report> {
///         let output = self.output()?;
///
///         let stdout = String::from_utf8_lossy(&output.stdout);
///
///         if !output.status.success() {
///             Err(eyre!("cmd exited with non-zero status code")).extension(Retry)
///         } else {
///             Ok(stdout.into())
///         }
///     }
/// }
/// ```
pub trait ExtensionExt: Sealed {
    #[allow(missing_docs)]
    type Return;

    /// Method for attaching custom data to errors.
    ///
    /// This trait is implemented for `eyre::Report` and `Result<T, E> where E: std::error::Error`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Retry;
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code")).extension(Retry)
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn extension<T: Send + Sync + 'static>(self, extension: T) -> Self::Return;

    /// Method for lazily attaching custom data to errors.
    ///
    /// This trait is implemented for `eyre::Report` and `Result<T, E> where E: std::error::Error`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Retry(String);
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code"))
    ///                 .with_extension(|| Retry("Some big string".to_owned()))
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn with_extension<T: Send + Sync + 'static, F: FnOnce() -> T>(self, f: F) -> Self::Return;

    /// Method for removing attached data from errors if exists.
    ///
    /// This trait is implemented for `eyre::Report` and `Result<T, E> where E: std::error::Error`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use extension_eyre::{eyre::eyre, ExtensionExt, Extension, eyre::Report};
    /// use std::process::Command;
    /// use tracing::instrument;
    ///
    /// pub struct Retry;
    ///
    /// #[instrument]
    /// fn read_file(path: &str) -> Result<String, Report> {
    ///     Command::new("cat")
    ///         .arg(path)
    ///         .output2()
    ///         .remove_extension::<Retry>()
    /// }
    ///
    /// trait Output {
    ///     fn output2(&mut self) -> Result<String, Report>;
    /// }
    ///
    /// impl Output for Command {
    ///     #[instrument]
    ///     fn output2(&mut self) -> Result<String, Report> {
    ///         let output = self.output()?;
    ///
    ///         let stdout = String::from_utf8_lossy(&output.stdout);
    ///
    ///         if !output.status.success() {
    ///             Err(eyre!("cmd exited with non-zero status code")).extension(Retry)
    ///         } else {
    ///             Ok(stdout.into())
    ///         }
    ///     }
    /// }
    /// ```
    fn remove_extension<T: Send + Sync + 'static>(self) -> Self::Return;
}

impl ExtensionExt for Report {
    type Return = Report;

    fn extension<T: Send + Sync + 'static>(mut self, extension: T) -> Self::Return {
        if let Some(handler) = self.handler_mut().downcast_mut::<crate::Handler>() {
            handler.extensions.insert::<T>(extension);
        }

        self
    }

    fn with_extension<T: Send + Sync + 'static, F: FnOnce() -> T>(mut self, f: F) -> Self::Return {
        if let Some(handler) = self.handler_mut().downcast_mut::<crate::Handler>() {
            handler.extensions.insert::<T>(f());
        }

        self
    }

    fn remove_extension<T: Send + Sync + 'static>(mut self) -> Self::Return {
        if let Some(handler) = self.handler_mut().downcast_mut::<crate::Handler>() {
            handler.extensions.remove::<T>();
        }

        self
    }
}

impl<T, E> ExtensionExt for Result<T, E>
where
    E: Into<Report>,
{
    type Return = Result<T, Report>;

    fn extension<Ext: Send + Sync + 'static>(self, extension: Ext) -> Self::Return {
        self.map_err(|error| error.into())
            .map_err(|report| report.extension::<Ext>(extension))
    }

    fn with_extension<Ext: Send + Sync + 'static, F: FnOnce() -> Ext>(self, f: F) -> Self::Return {
        self.map_err(|error| error.into())
            .map_err(|report| report.with_extension(f))
    }

    fn remove_extension<Ext: Send + Sync + 'static>(self) -> Self::Return {
        self.map_err(|error| error.into())
            .map_err(|report| report.remove_extension::<Ext>())
    }
}
