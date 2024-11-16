#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.54.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::entities::Post;
use crate::entities::PostPage;
use crate::entities::Tag;
use crate::entities::TagData;

// Section: wire functions

fn wire_init_impl(
    port_: MessagePort,
    root: impl Wire2Api<String> + UnwindSafe,
    downloads_to: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_root = root.wire2api();
            let api_downloads_to = downloads_to.wire2api();
            move |task_callback| Ok(init(api_root, api_downloads_to))
        },
    )
}
fn wire_set_proxy_impl(port_: MessagePort, url: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_proxy",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            move |task_callback| set_proxy(api_url)
        },
    )
}
fn wire_desktop_root_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "desktop_root",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| desktop_root(),
    )
}
fn wire_http_get_impl(port_: MessagePort, url: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "http_get",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            move |task_callback| http_get(api_url)
        },
    )
}
fn wire_save_property_impl(
    port_: MessagePort,
    k: impl Wire2Api<String> + UnwindSafe,
    v: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "save_property",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_k = k.wire2api();
            let api_v = v.wire2api();
            move |task_callback| save_property(api_k, api_v)
        },
    )
}
fn wire_load_property_impl(port_: MessagePort, k: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "load_property",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_k = k.wire2api();
            move |task_callback| load_property(api_k)
        },
    )
}
fn wire_tag_summary_impl(port_: MessagePort, host: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "tag_summary",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_host = host.wire2api();
            move |task_callback| tag_summary(api_host)
        },
    )
}
fn wire_load_posts_impl(
    port_: MessagePort,
    host: impl Wire2Api<String> + UnwindSafe,
    tags: impl Wire2Api<String> + UnwindSafe,
    page: impl Wire2Api<i64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "load_posts",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_host = host.wire2api();
            let api_tags = tags.wire2api();
            let api_page = page.wire2api();
            move |task_callback| load_posts(api_host, api_tags, api_page)
        },
    )
}
fn wire_load_cache_image_impl(
    port_: MessagePort,
    url: impl Wire2Api<String> + UnwindSafe,
    useful: impl Wire2Api<String> + UnwindSafe,
    extends_field_int_first: impl Wire2Api<Option<i32>> + UnwindSafe,
    extends_field_int_second: impl Wire2Api<Option<i32>> + UnwindSafe,
    extends_field_int_third: impl Wire2Api<Option<i32>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "load_cache_image",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            let api_useful = useful.wire2api();
            let api_extends_field_int_first = extends_field_int_first.wire2api();
            let api_extends_field_int_second = extends_field_int_second.wire2api();
            let api_extends_field_int_third = extends_field_int_third.wire2api();
            move |task_callback| {
                load_cache_image(
                    api_url,
                    api_useful,
                    api_extends_field_int_first,
                    api_extends_field_int_second,
                    api_extends_field_int_third,
                )
            }
        },
    )
}
fn wire_auto_clean_impl(port_: MessagePort, time: impl Wire2Api<i64> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "auto_clean",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_time = time.wire2api();
            move |task_callback| auto_clean(api_time)
        },
    )
}
fn wire_clean_all_cache_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "clean_all_cache",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| clean_all_cache(),
    )
}
fn wire_add_download_post_impl(
    port_: MessagePort,
    host: impl Wire2Api<String> + UnwindSafe,
    post: impl Wire2Api<Post> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_download_post",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_host = host.wire2api();
            let api_post = post.wire2api();
            move |task_callback| add_download_post(api_host, api_post)
        },
    )
}
fn wire_all_downloads_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "all_downloads",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| all_downloads(),
    )
}
fn wire_load_dl_image_impl(port_: MessagePort, dl_key: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "load_dl_image",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_dl_key = dl_key.wire2api();
            move |task_callback| load_dl_image(api_dl_key)
        },
    )
}
fn wire_copy_image_to_impl(
    port_: MessagePort,
    src_path: impl Wire2Api<String> + UnwindSafe,
    to_dir: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "copy_image_to",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_src_path = src_path.wire2api();
            let api_to_dir = to_dir.wire2api();
            move |task_callback| copy_image_to(api_src_path, api_to_dir)
        },
    )
}
fn wire_reset_failed_downloads_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "reset_failed_downloads",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| reset_failed_downloads(),
    )
}
fn wire_delete_dl_post_impl(port_: MessagePort, dl_key: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "delete_dl_post",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_dl_key = dl_key.wire2api();
            move |task_callback| delete_dl_post(api_dl_key)
        },
    )
}
fn wire_downloads_to_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "downloads_to",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| downloads_to(),
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for DlPost {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.host.into_dart(),
            self.id.into_dart(),
            self.tags.into_dart(),
            self.created_at.into_dart(),
            self.creator_id.into_dart(),
            self.author.into_dart(),
            self.change.into_dart(),
            self.source.into_dart(),
            self.score.into_dart(),
            self.md5.into_dart(),
            self.file_size.into_dart(),
            self.file_url.into_dart(),
            self.is_shown_in_index.into_dart(),
            self.preview_url.into_dart(),
            self.preview_width.into_dart(),
            self.preview_height.into_dart(),
            self.actual_preview_width.into_dart(),
            self.actual_preview_height.into_dart(),
            self.sample_url.into_dart(),
            self.sample_width.into_dart(),
            self.sample_height.into_dart(),
            self.sample_file_size.into_dart(),
            self.jpeg_url.into_dart(),
            self.jpeg_width.into_dart(),
            self.jpeg_height.into_dart(),
            self.jpeg_file_size.into_dart(),
            self.rating.into_dart(),
            self.has_children.into_dart(),
            self.parent_id.into_dart(),
            self.width.into_dart(),
            self.height.into_dart(),
            self.status.into_dart(),
            self.is_held.into_dart(),
            self.frames_pending_string.into_dart(),
            self.frames_string.into_dart(),
            self.dl_key.into_dart(),
            self.dl_created_time.into_dart(),
            self.dl_status.into_dart(),
            self.file_format.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for DlPost {}

impl support::IntoDart for LocalImage {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.abs_path.into_dart(),
            self.local_path.into_dart(),
            self.image_format.into_dart(),
            self.image_width.into_dart(),
            self.image_height.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LocalImage {}

impl support::IntoDart for Post {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.tags.into_dart(),
            self.created_at.into_dart(),
            self.creator_id.into_dart(),
            self.author.into_dart(),
            self.change.into_dart(),
            self.source.into_dart(),
            self.score.into_dart(),
            self.md5.into_dart(),
            self.file_size.into_dart(),
            self.file_url.into_dart(),
            self.is_shown_in_index.into_dart(),
            self.preview_url.into_dart(),
            self.preview_width.into_dart(),
            self.preview_height.into_dart(),
            self.actual_preview_width.into_dart(),
            self.actual_preview_height.into_dart(),
            self.sample_url.into_dart(),
            self.sample_width.into_dart(),
            self.sample_height.into_dart(),
            self.sample_file_size.into_dart(),
            self.jpeg_url.into_dart(),
            self.jpeg_width.into_dart(),
            self.jpeg_height.into_dart(),
            self.jpeg_file_size.into_dart(),
            self.rating.into_dart(),
            self.has_children.into_dart(),
            self.parent_id.into_dart(),
            self.status.into_dart(),
            self.width.into_dart(),
            self.height.into_dart(),
            self.is_held.into_dart(),
            self.frames_pending_string.into_dart(),
            self.frames_string.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Post {}

impl support::IntoDart for PostPage {
    fn into_dart(self) -> support::DartAbi {
        vec![self.posts.into_dart(), self.page_total.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PostPage {}

impl support::IntoDart for Tag {
    fn into_dart(self) -> support::DartAbi {
        vec![self.image_total.into_dart(), self.tag_names.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Tag {}

impl support::IntoDart for TagData {
    fn into_dart(self) -> support::DartAbi {
        vec![self.version.into_dart(), self.tags.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TagData {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;