// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use object::{GenericObject, Upcast};
use translate::*;
use types;

pub type AppInfo = GenericObject<ffi::GAppInfo>;

impl types::StaticType for AppInfo {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::g_app_info_get_type()) }
    }
}

pub trait AppInfoExt {
}

impl<O: Upcast<AppInfo>> AppInfoExt for O {
}
