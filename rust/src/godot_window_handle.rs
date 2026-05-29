use godot::{classes::{DisplayServer, display_server::HandleType}, prelude::*};
use raw_window_handle::{
    DisplayHandle, WindowHandle, HasDisplayHandle, HasWindowHandle,
    RawDisplayHandle, RawWindowHandle, HandleError,
};

pub struct GodotWindowHandle;

impl GodotWindowHandle {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use std::num::NonZeroIsize;
    use raw_window_handle::{Win32WindowHandle, WindowsDisplayHandle};

    impl HasWindowHandle for GodotWindowHandle {
        fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
            let ds = DisplayServer::singleton();
            let hwnd = ds.window_get_native_handle(HandleType::WINDOW_HANDLE);
            let raw = RawWindowHandle::Win32(
                Win32WindowHandle::new(
                    NonZeroIsize::new(hwnd as isize).expect("HWND must be non-null")
                )
            );
            Ok(unsafe { WindowHandle::borrow_raw(raw) })
        }
    }

    impl HasDisplayHandle for GodotWindowHandle {
        fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
            let raw = RawDisplayHandle::Windows(WindowsDisplayHandle::new());
            Ok(unsafe { DisplayHandle::borrow_raw(raw) })
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use std::ptr::NonNull;
    use raw_window_handle::{XlibWindowHandle, XlibDisplayHandle, WaylandWindowHandle, WaylandDisplayHandle};

    impl HasWindowHandle for GodotWindowHandle {
        fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
            let ds = DisplayServer::singleton();
            let raw = if ds.get_name() == "Wayland" {
                let ptr = ds.window_get_native_handle(HandleType::WINDOW_HANDLE);
                RawWindowHandle::Wayland(
                    WaylandWindowHandle::new(
                        NonNull::new(ptr as usize as *mut std::ffi::c_void)
                            .expect("wl_surface must be non-null")
                    )
                )
            } else {
                let xid = ds.window_get_native_handle(HandleType::WINDOW_HANDLE);
                RawWindowHandle::Xlib(XlibWindowHandle::new(xid as u64))
            };
            Ok(unsafe { WindowHandle::borrow_raw(raw) })
        }
    }

    impl HasDisplayHandle for GodotWindowHandle {
        fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
            let ds = DisplayServer::singleton();
            let raw = if ds.get_name() == "Wayland" {
                let ptr = ds.window_get_native_handle(HandleType::DISPLAY_HANDLE);
                RawDisplayHandle::Wayland(
                    WaylandDisplayHandle::new(
                        NonNull::new(ptr as usize as *mut std::ffi::c_void)
                            .expect("wl_display must be non-null")
                    )
                )
            } else {
                let ptr = ds.window_get_native_handle(HandleType::DISPLAY_HANDLE);
                let display = NonNull::new(ptr as usize as *mut std::ffi::c_void);
                let mut handle = XlibDisplayHandle::new(display);
                handle.screen = 0;
                RawDisplayHandle::Xlib(handle)
            };
            Ok(unsafe { DisplayHandle::borrow_raw(raw) })
        }
    }
}

#[cfg(target_os = "android")]
mod platform {
    use super::*;
    use std::ptr::NonNull;
    use raw_window_handle::{AndroidNdkWindowHandle, AndroidNdkDisplayHandle};

    impl HasWindowHandle for GodotWindowHandle {
        fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
            let ds = DisplayServer::singleton();
            let ptr = ds.window_get_native_handle(HandleType::WINDOW_HANDLE);
            let raw = RawWindowHandle::AndroidNdk(
                AndroidNdkWindowHandle::new(
                    NonNull::new(ptr as usize as *mut std::ffi::c_void)
                        .expect("ANativeWindow must be non-null")
                )
            );
            Ok(unsafe { WindowHandle::borrow_raw(raw) })
        }
    }

    impl HasDisplayHandle for GodotWindowHandle {
        fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
            let raw = RawDisplayHandle::AndroidNdk(AndroidNdkDisplayHandle::new());
            Ok(unsafe { DisplayHandle::borrow_raw(raw) })
        }
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;
    use std::ptr::NonNull;
    use raw_window_handle::{AppKitWindowHandle, AppKitDisplayHandle};

    impl HasWindowHandle for GodotWindowHandle {
        fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
            let ds = DisplayServer::singleton();
            let ptr = ds.window_get_native_handle(HandleType::WINDOW_VIEW);
            let raw = RawWindowHandle::AppKit(
                AppKitWindowHandle::new(
                    NonNull::new(ptr as usize as *mut std::ffi::c_void)
                        .expect("NSView must be non-null")
                )
            );
            Ok(unsafe { WindowHandle::borrow_raw(raw) })
        }
    }

    impl HasDisplayHandle for GodotWindowHandle {
        fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
            let raw = RawDisplayHandle::AppKit(AppKitDisplayHandle::new());
            Ok(unsafe { DisplayHandle::borrow_raw(raw) })
        }
    }
}
