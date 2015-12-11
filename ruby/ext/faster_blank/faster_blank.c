#include <ruby.h>
#include <ruby/encoding.h>

typedef struct {
  void* data;
  size_t len;
} tr_buf_t;

static inline tr_buf_t
STR2BUF(VALUE str) {
  return (tr_buf_t) {
    .data = RSTRING_PTR(str),
    .len = RSTRING_LEN(str),
  };
}

int tr_str_is_blank(tr_buf_t);

static VALUE
rb_str_blank(VALUE self) {
  return tr_str_is_blank(STR2BUF(self)) ? Qtrue : Qfalse;
}

void Init_faster_blank( void )
{
  rb_define_method(rb_cString, "blank?", rb_str_blank, 0);
}
