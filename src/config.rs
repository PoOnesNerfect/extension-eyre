//! Configuration options for customizing the behavior of the provided panic
//! and error reporting hooks
use crate::extensions::Extensions;
use color_eyre::config::{
    EyreHook as EyreHookInner, HookBuilder as HookBuilderInner, PanicHook as PanicHookInner,
    PanicReport,
};
use color_eyre::section::PanicMessage;
use fmt::Display;
use std::fmt;

pub use color_eyre::config::{FilterCallback, Frame, Theme};

/// Builder for customizing the behavior of the global panic and error report hooks
pub struct HookBuilder {
    inner: HookBuilderInner,
}

impl HookBuilder {
    /// Construct a HookBuilder
    ///
    /// # Details
    ///
    /// By default this function calls `add_default_filters()` and
    /// `capture_span_trace_by_default(true)`. To get a `HookBuilder` with all
    /// features disabled by default call `HookBuilder::blank()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_eyre::config::HookBuilder;
    ///
    /// HookBuilder::new()
    ///     .install()
    ///     .unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            inner: HookBuilderInner::new(),
        }
    }

    /// Construct a HookBuilder with minimal features enabled
    pub fn blank() -> Self {
        HookBuilder {
            inner: HookBuilderInner::blank(),
        }
    }

    /// Set the global styles that `color_eyre` should use.
    ///
    /// **Tip:** You can test new styles by editing `examples/theme.rs` in the `color-eyre` repository.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.inner = self.inner.theme(theme);
        self
    }

    /// Add a custom section to the panic hook that will be printed
    /// in the panic message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// color_eyre::config::HookBuilder::default()
    ///     .panic_section("consider reporting the bug at https://github.com/yaahc/color-eyre")
    ///     .install()
    ///     .unwrap()
    /// ```
    pub fn panic_section<S: Display + Send + Sync + 'static>(mut self, section: S) -> Self {
        self.inner = self.inner.panic_section(section);
        self
    }

    /// Overrides the main error message printing section at the start of panic
    /// reports
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::{panic::Location, fmt};
    /// use color_eyre::section::PanicMessage;
    /// use color_eyre::owo_colors::OwoColorize;
    ///
    /// struct MyPanicMessage;
    ///
    /// color_eyre::config::HookBuilder::default()
    ///     .panic_message(MyPanicMessage)
    ///     .install()
    ///     .unwrap();
    ///
    /// impl PanicMessage for MyPanicMessage {
    ///     fn display(&self, pi: &std::panic::PanicInfo<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         writeln!(f, "{}", "The application panicked (crashed).".red())?;
    ///
    ///         // Print panic message.
    ///         let payload = pi
    ///             .payload()
    ///             .downcast_ref::<String>()
    ///             .map(String::as_str)
    ///             .or_else(|| pi.payload().downcast_ref::<&str>().cloned())
    ///             .unwrap_or("<non string panic payload>");
    ///
    ///         write!(f, "Message:  ")?;
    ///         writeln!(f, "{}", payload.cyan())?;
    ///
    ///         // If known, print panic location.
    ///         write!(f, "Location: ")?;
    ///         if let Some(loc) = pi.location() {
    ///             write!(f, "{}", loc.file().purple())?;
    ///             write!(f, ":")?;
    ///             write!(f, "{}", loc.line().purple())?;
    ///
    ///             write!(f, "\n\nConsider reporting the bug at {}", custom_url(loc, payload))?;
    ///         } else {
    ///             write!(f, "<unknown>")?;
    ///         }
    ///
    ///         Ok(())
    ///     }
    /// }
    ///
    /// fn custom_url(location: &Location<'_>, message: &str) -> impl fmt::Display {
    ///     "todo"
    /// }
    /// ```
    pub fn panic_message<S: PanicMessage>(mut self, section: S) -> Self {
        self.inner = self.inner.panic_message(section);
        self
    }

    /// Set an upstream github repo and enable issue reporting url generation
    ///
    /// # Details
    ///
    /// Once enabled, color-eyre will generate urls that will create customized
    /// issues pre-populated with information about the associated error report.
    ///
    /// Additional information can be added to the metadata table in the
    /// generated urls by calling `add_issue_metadata` when configuring the
    /// HookBuilder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// color_eyre::config::HookBuilder::default()
    ///     .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
    ///     .install()
    ///     .unwrap();
    /// ```
    #[cfg(feature = "issue-url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "issue-url")))]
    pub fn issue_url<S: ToString>(mut self, url: S) -> Self {
        self.inner = self.inner.issue_url(url);
        self
    }

    /// Add a new entry to the metadata table in generated github issue urls
    ///
    /// **Note**: this metadata will be ignored if no `issue_url` is set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// color_eyre::config::HookBuilder::default()
    ///     .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
    ///     .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
    ///     .install()
    ///     .unwrap();
    /// ```
    #[cfg(feature = "issue-url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "issue-url")))]
    pub fn add_issue_metadata<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Display,
        V: Display + Send + Sync + 'static,
    {
        self.inner = self.inner.add_issue_metadata(key, value);
        self
    }

    /// Configures a filter for disabling issue url generation for certain kinds of errors
    ///
    /// If the closure returns `true`, then the issue url will be generated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// color_eyre::config::HookBuilder::default()
    ///     .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
    ///     .issue_filter(|kind| match kind {
    ///         color_eyre::ErrorKind::NonRecoverable(payload) => {
    ///             let payload = payload
    ///                 .downcast_ref::<String>()
    ///                 .map(String::as_str)
    ///                 .or_else(|| payload.downcast_ref::<&str>().cloned())
    ///                 .unwrap_or("<non string panic payload>");
    ///
    ///             !payload.contains("my irrelevant error message")
    ///         },
    ///         color_eyre::ErrorKind::Recoverable(error) => !error.is::<std::fmt::Error>(),
    ///     })
    ///     .install()
    ///     .unwrap();
    ///
    #[cfg(feature = "issue-url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "issue-url")))]
    pub fn issue_filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(crate::ErrorKind<'_>) -> bool + Send + Sync + 'static,
    {
        self.inner = self.inner.issue_filter(predicate);
        self
    }

    /// Configures the default capture mode for `SpanTraces` in error reports and panics
    pub fn capture_span_trace_by_default(mut self, cond: bool) -> Self {
        self.inner = self.inner.capture_span_trace_by_default(cond);
        self
    }

    /// Configures the enviroment varible info section and whether or not it is displayed
    pub fn display_env_section(mut self, cond: bool) -> Self {
        self.inner = self.inner.display_env_section(cond);
        self
    }

    /// Configures the location info section and whether or not it is displayed.
    ///
    /// # Notes
    ///
    /// This will not disable the location section in a panic message.
    #[cfg(feature = "track-caller")]
    #[cfg_attr(docsrs, doc(cfg(feature = "track-caller")))]
    pub fn display_location_section(mut self, cond: bool) -> Self {
        self.inner = self.inner.display_location_section(cond);
        self
    }

    /// Add a custom filter to the set of frame filters
    ///
    /// # Examples
    ///
    /// ```rust
    /// color_eyre::config::HookBuilder::default()
    ///     .add_frame_filter(Box::new(|frames| {
    ///         let filters = &[
    ///             "uninteresting_function",
    ///         ];
    ///
    ///         frames.retain(|frame| {
    ///             !filters.iter().any(|f| {
    ///                 let name = if let Some(name) = frame.name.as_ref() {
    ///                     name.as_str()
    ///                 } else {
    ///                     return true;
    ///                 };
    ///
    ///                 name.starts_with(f)
    ///             })
    ///         });
    ///     }))
    ///     .install()
    ///     .unwrap();
    /// ```
    pub fn add_frame_filter(mut self, filter: Box<FilterCallback>) -> Self {
        self.inner = self.inner.add_frame_filter(filter);
        self
    }

    /// Install the given Hook as the global error report hook
    pub fn install(self) -> Result<(), crate::eyre::Report> {
        let (panic_hook, eyre_hook) = self.into_hooks();
        eyre_hook.install()?;
        panic_hook.install();
        Ok(())
    }

    /// Add the default set of filters to this `HookBuilder`'s configuration
    pub fn add_default_filters(mut self) -> Self {
        self.inner = self.inner.add_default_filters();
        self
    }

    /// Create a `PanicHook` and `EyreHook` from this `HookBuilder`.
    /// This can be used if you want to combine these handlers with other handlers.
    pub fn into_hooks(self) -> (PanicHook, EyreHook) {
        let (panic_hook, eyre_hook) = self.inner.into_hooks();

        let panic_hook = PanicHook { inner: panic_hook };

        let eyre_hook = EyreHook { inner: eyre_hook };

        (panic_hook, eyre_hook)
    }
}

#[allow(missing_docs)]
impl Default for HookBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A panic reporting hook
pub struct PanicHook {
    inner: PanicHookInner,
}

impl PanicHook {
    /// Install self as a global panic hook via `std::panic::set_hook`.
    pub fn install(self) {
        std::panic::set_hook(self.into_panic_hook());
    }

    /// Convert self into the type expected by `std::panic::set_hook`.
    pub fn into_panic_hook(
        self,
    ) -> Box<dyn Fn(&std::panic::PanicInfo<'_>) + Send + Sync + 'static> {
        Box::new(move |panic_info| {
            eprintln!("{}", self.panic_report(panic_info));
        })
    }

    /// Construct a panic reporter which prints it's panic report via the
    /// `Display` trait.
    pub fn panic_report<'a>(
        &'a self,
        panic_info: &'a std::panic::PanicInfo<'_>,
    ) -> PanicReport<'a> {
        self.inner.panic_report(panic_info)
    }
}

/// An eyre reporting hook used to construct `EyreHandler`s
pub struct EyreHook {
    inner: EyreHookInner,
}

impl EyreHook {
    /// Installs self as the global eyre handling hook via `eyre::set_hook`
    pub fn install(self) -> Result<(), crate::eyre::InstallError> {
        crate::eyre::set_hook(self.into_eyre_hook())
    }

    /// Convert the self into the boxed type expected by `eyre::set_hook`.
    pub fn into_eyre_hook(
        self,
    ) -> Box<
        dyn Fn(&(dyn std::error::Error + 'static)) -> Box<dyn eyre::EyreHandler>
            + Send
            + Sync
            + 'static,
    > {
        let f = self.inner.into_eyre_hook();
        Box::new(move |e| {
            Box::new(crate::Handler {
                inner: f(e),
                extensions: Extensions::new(),
            })
        })
    }
}

/// Callback for filtering issue url generation in error reports
#[cfg(feature = "issue-url")]
#[cfg_attr(docsrs, doc(cfg(feature = "issue-url")))]
pub type IssueFilterCallback = dyn Fn(crate::ErrorKind<'_>) -> bool + Send + Sync + 'static;
