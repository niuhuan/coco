#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_Post {
  int64_t id;
  struct wire_uint_8_list *tags;
  int64_t created_at;
  int64_t *creator_id;
  struct wire_uint_8_list *author;
  int64_t change;
  struct wire_uint_8_list *source;
  int64_t score;
  struct wire_uint_8_list *md5;
  int64_t file_size;
  struct wire_uint_8_list *file_url;
  bool is_shown_in_index;
  struct wire_uint_8_list *preview_url;
  int64_t preview_width;
  int64_t preview_height;
  int64_t actual_preview_width;
  int64_t actual_preview_height;
  struct wire_uint_8_list *sample_url;
  int64_t sample_width;
  int64_t sample_height;
  int64_t sample_file_size;
  struct wire_uint_8_list *jpeg_url;
  int64_t jpeg_width;
  int64_t jpeg_height;
  int64_t jpeg_file_size;
  struct wire_uint_8_list *rating;
  bool has_children;
  int64_t *parent_id;
  struct wire_uint_8_list *status;
  int64_t width;
  int64_t height;
  bool is_held;
  struct wire_uint_8_list *frames_pending_string;
  struct wire_uint_8_list *frames_string;
} wire_Post;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_init(int64_t port_, struct wire_uint_8_list *root, struct wire_uint_8_list *downloads_to);

void wire_set_proxy(int64_t port_, struct wire_uint_8_list *url);

void wire_desktop_root(int64_t port_);

void wire_http_get(int64_t port_, struct wire_uint_8_list *url);

void wire_save_property(int64_t port_, struct wire_uint_8_list *k, struct wire_uint_8_list *v);

void wire_load_property(int64_t port_, struct wire_uint_8_list *k);

void wire_tag_summary(int64_t port_, struct wire_uint_8_list *host);

void wire_load_posts(int64_t port_,
                     struct wire_uint_8_list *host,
                     struct wire_uint_8_list *tags,
                     int64_t page);

void wire_load_cache_image(int64_t port_,
                           struct wire_uint_8_list *url,
                           struct wire_uint_8_list *useful,
                           int32_t *extends_field_int_first,
                           int32_t *extends_field_int_second,
                           int32_t *extends_field_int_third);

void wire_auto_clean(int64_t port_, int64_t time);

void wire_clean_all_cache(int64_t port_);

void wire_add_download_post(int64_t port_, struct wire_uint_8_list *host, struct wire_Post *post);

void wire_all_downloads(int64_t port_);

void wire_load_dl_image(int64_t port_, struct wire_uint_8_list *dl_key);

void wire_copy_image_to(int64_t port_,
                        struct wire_uint_8_list *src_path,
                        struct wire_uint_8_list *to_dir);

void wire_reset_failed_downloads(int64_t port_);

void wire_delete_dl_post(int64_t port_, struct wire_uint_8_list *dl_key);

void wire_downloads_to(int64_t port_);

int32_t *new_box_autoadd_i32_0(int32_t value);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct wire_Post *new_box_autoadd_post_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_init);
    dummy_var ^= ((int64_t) (void*) wire_set_proxy);
    dummy_var ^= ((int64_t) (void*) wire_desktop_root);
    dummy_var ^= ((int64_t) (void*) wire_http_get);
    dummy_var ^= ((int64_t) (void*) wire_save_property);
    dummy_var ^= ((int64_t) (void*) wire_load_property);
    dummy_var ^= ((int64_t) (void*) wire_tag_summary);
    dummy_var ^= ((int64_t) (void*) wire_load_posts);
    dummy_var ^= ((int64_t) (void*) wire_load_cache_image);
    dummy_var ^= ((int64_t) (void*) wire_auto_clean);
    dummy_var ^= ((int64_t) (void*) wire_clean_all_cache);
    dummy_var ^= ((int64_t) (void*) wire_add_download_post);
    dummy_var ^= ((int64_t) (void*) wire_all_downloads);
    dummy_var ^= ((int64_t) (void*) wire_load_dl_image);
    dummy_var ^= ((int64_t) (void*) wire_copy_image_to);
    dummy_var ^= ((int64_t) (void*) wire_reset_failed_downloads);
    dummy_var ^= ((int64_t) (void*) wire_delete_dl_post);
    dummy_var ^= ((int64_t) (void*) wire_downloads_to);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_post_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}