// src/compile_time.rs

/// Compile-time filtering for flash_error! macro
#[macro_export]
macro_rules! flash_error_ct {
    ($($args:tt)*) => {
        $crate::flash_error!($($args)*)
    };
}

/// Compile-time filtering for flash_warn! macro
#[macro_export]
macro_rules! flash_warn_ct {
    ($($args:tt)*) => {
        #[cfg(any(
            feature = "max-level-warn",
            feature = "max-level-info",
            feature = "max-level-debug",
            feature = "max-level-trace"
        ))]
        {
            $crate::flash_warn!($($args)*)
        }
    };
}

/// Compile-time filtering for flash_info! macro
#[macro_export]
macro_rules! flash_info_ct {
    ($($args:tt)*) => {
        #[cfg(any(
            feature = "max-level-info",
            feature = "max-level-debug",
            feature = "max-level-trace"
        ))]
        {
            $crate::flash_info!($($args)*)
        }
    };
}

/// Compile-time filtering for flash_debug! macro
#[macro_export]
macro_rules! flash_debug_ct {
    ($($args:tt)*) => {
        #[cfg(any(
            feature = "max-level-debug",
            feature = "max-level-trace"
        ))]
        {
            $crate::flash_debug!($($args)*)
        }
    };
}

/// Compile-time filtering for flash_trace! macro
#[macro_export]
macro_rules! flash_trace_ct {
    ($($args:tt)*) => {
        #[cfg(feature = "max-level-trace")]
        {
            $crate::flash_trace!($($args)*)
        }
    };
}