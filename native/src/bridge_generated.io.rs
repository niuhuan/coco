use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_init(
    port_: i64,
    root: *mut wire_uint_8_list,
    downloads_to: *mut wire_uint_8_list,
) {
    wire_init_impl(port_, root, downloads_to)
}

#[no_mangle]
pub extern "C" fn wire_set_proxy(port_: i64, url: *mut wire_uint_8_list) {
    wire_set_proxy_impl(port_, url)
}

#[no_mangle]
pub extern "C" fn wire_desktop_root(port_: i64) {
    wire_desktop_root_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_http_get(port_: i64, url: *mut wire_uint_8_list) {
    wire_http_get_impl(port_, url)
}

#[no_mangle]
pub extern "C" fn wire_save_property(
    port_: i64,
    k: *mut wire_uint_8_list,
    v: *mut wire_uint_8_list,
) {
    wire_save_property_impl(port_, k, v)
}

#[no_mangle]
pub extern "C" fn wire_load_property(port_: i64, k: *mut wire_uint_8_list) {
    wire_load_property_impl(port_, k)
}

#[no_mangle]
pub extern "C" fn wire_tag_summary(port_: i64, host: *mut wire_uint_8_list) {
    wire_tag_summary_impl(port_, host)
}

#[no_mangle]
pub extern "C" fn wire_load_posts(
    port_: i64,
    host: *mut wire_uint_8_list,
    tags: *mut wire_uint_8_list,
    page: i64,
) {
    wire_load_posts_impl(port_, host, tags, page)
}

#[no_mangle]
pub extern "C" fn wire_load_cache_image(
    port_: i64,
    url: *mut wire_uint_8_list,
    useful: *mut wire_uint_8_list,
    extends_field_int_first: *mut i32,
    extends_field_int_second: *mut i32,
    extends_field_int_third: *mut i32,
) {
    wire_load_cache_image_impl(
        port_,
        url,
        useful,
        extends_field_int_first,
        extends_field_int_second,
        extends_field_int_third,
    )
}

#[no_mangle]
pub extern "C" fn wire_auto_clean(port_: i64, time: i64) {
    wire_auto_clean_impl(port_, time)
}

#[no_mangle]
pub extern "C" fn wire_clean_all_cache(port_: i64) {
    wire_clean_all_cache_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_add_download_post(
    port_: i64,
    host: *mut wire_uint_8_list,
    post: *mut wire_Post,
) {
    wire_add_download_post_impl(port_, host, post)
}

#[no_mangle]
pub extern "C" fn wire_all_downloads(port_: i64) {
    wire_all_downloads_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_load_dl_image(port_: i64, dl_key: *mut wire_uint_8_list) {
    wire_load_dl_image_impl(port_, dl_key)
}

#[no_mangle]
pub extern "C" fn wire_copy_image_to(
    port_: i64,
    src_path: *mut wire_uint_8_list,
    to_dir: *mut wire_uint_8_list,
) {
    wire_copy_image_to_impl(port_, src_path, to_dir)
}

#[no_mangle]
pub extern "C" fn wire_reset_failed_downloads(port_: i64) {
    wire_reset_failed_downloads_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_delete_dl_post(port_: i64, dl_key: *mut wire_uint_8_list) {
    wire_delete_dl_post_impl(port_, dl_key)
}

#[no_mangle]
pub extern "C" fn wire_downloads_to(port_: i64) {
    wire_downloads_to_impl(port_)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_post_0() -> *mut wire_Post {
    support::new_leak_box_ptr(wire_Post::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Post> for *mut wire_Post {
    fn wire2api(self) -> Post {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Post>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Post> for wire_Post {
    fn wire2api(self) -> Post {
        Post {
            id: self.id.wire2api(),
            tags: self.tags.wire2api(),
            created_at: self.created_at.wire2api(),
            creator_id: self.creator_id.wire2api(),
            author: self.author.wire2api(),
            change: self.change.wire2api(),
            source: self.source.wire2api(),
            score: self.score.wire2api(),
            md5: self.md5.wire2api(),
            file_size: self.file_size.wire2api(),
            file_url: self.file_url.wire2api(),
            is_shown_in_index: self.is_shown_in_index.wire2api(),
            preview_url: self.preview_url.wire2api(),
            preview_width: self.preview_width.wire2api(),
            preview_height: self.preview_height.wire2api(),
            actual_preview_width: self.actual_preview_width.wire2api(),
            actual_preview_height: self.actual_preview_height.wire2api(),
            sample_url: self.sample_url.wire2api(),
            sample_width: self.sample_width.wire2api(),
            sample_height: self.sample_height.wire2api(),
            sample_file_size: self.sample_file_size.wire2api(),
            jpeg_url: self.jpeg_url.wire2api(),
            jpeg_width: self.jpeg_width.wire2api(),
            jpeg_height: self.jpeg_height.wire2api(),
            jpeg_file_size: self.jpeg_file_size.wire2api(),
            rating: self.rating.wire2api(),
            has_children: self.has_children.wire2api(),
            parent_id: self.parent_id.wire2api(),
            status: self.status.wire2api(),
            width: self.width.wire2api(),
            height: self.height.wire2api(),
            is_held: self.is_held.wire2api(),
            frames_pending_string: self.frames_pending_string.wire2api(),
            frames_string: self.frames_string.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Post {
    id: i64,
    tags: *mut wire_uint_8_list,
    created_at: i64,
    creator_id: *mut i64,
    author: *mut wire_uint_8_list,
    change: i64,
    source: *mut wire_uint_8_list,
    score: i64,
    md5: *mut wire_uint_8_list,
    file_size: i64,
    file_url: *mut wire_uint_8_list,
    is_shown_in_index: bool,
    preview_url: *mut wire_uint_8_list,
    preview_width: i64,
    preview_height: i64,
    actual_preview_width: i64,
    actual_preview_height: i64,
    sample_url: *mut wire_uint_8_list,
    sample_width: i64,
    sample_height: i64,
    sample_file_size: i64,
    jpeg_url: *mut wire_uint_8_list,
    jpeg_width: i64,
    jpeg_height: i64,
    jpeg_file_size: i64,
    rating: *mut wire_uint_8_list,
    has_children: bool,
    parent_id: *mut i64,
    status: *mut wire_uint_8_list,
    width: i64,
    height: i64,
    is_held: bool,
    frames_pending_string: *mut wire_uint_8_list,
    frames_string: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Post {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            tags: core::ptr::null_mut(),
            created_at: Default::default(),
            creator_id: core::ptr::null_mut(),
            author: core::ptr::null_mut(),
            change: Default::default(),
            source: core::ptr::null_mut(),
            score: Default::default(),
            md5: core::ptr::null_mut(),
            file_size: Default::default(),
            file_url: core::ptr::null_mut(),
            is_shown_in_index: Default::default(),
            preview_url: core::ptr::null_mut(),
            preview_width: Default::default(),
            preview_height: Default::default(),
            actual_preview_width: Default::default(),
            actual_preview_height: Default::default(),
            sample_url: core::ptr::null_mut(),
            sample_width: Default::default(),
            sample_height: Default::default(),
            sample_file_size: Default::default(),
            jpeg_url: core::ptr::null_mut(),
            jpeg_width: Default::default(),
            jpeg_height: Default::default(),
            jpeg_file_size: Default::default(),
            rating: core::ptr::null_mut(),
            has_children: Default::default(),
            parent_id: core::ptr::null_mut(),
            status: core::ptr::null_mut(),
            width: Default::default(),
            height: Default::default(),
            is_held: Default::default(),
            frames_pending_string: core::ptr::null_mut(),
            frames_string: core::ptr::null_mut(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
