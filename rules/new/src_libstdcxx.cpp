#include <algorithm>
#include <new>
#include <utility>

using t1 = std::bad_alloc;

std::bad_alloc f9() { return std::bad_alloc(); }

std::bad_alloc f10(const std::bad_alloc &a0) { return std::bad_alloc(a0); }

std::bad_alloc &f11(std::bad_alloc &a0, const std::bad_alloc &a1) {
  return a0.operator=(a1);
}

const char *f12(const std::bad_alloc &a0) { return a0.what(); }

using t2 = std::bad_array_new_length;

std::bad_array_new_length f13() { return std::bad_array_new_length(); }

const char *f14(const std::bad_array_new_length &a0) { return a0.what(); }

std::bad_array_new_length f15(const std::bad_array_new_length &a0) {
  return std::bad_array_new_length(a0);
}

std::bad_array_new_length &f16(std::bad_array_new_length &a0,
                               const std::bad_array_new_length &a1) {
  return a0.operator=(a1);
}

std::new_handler f17() { return std::get_new_handler(); }

using t3 = std::new_handler;

using t4 = std::nothrow_t;

std::nothrow_t f18() { return std::nothrow_t(); }

std::nothrow_t f19(const std::nothrow_t &a0) { return std::nothrow_t(a0); }

std::nothrow_t f20(std::nothrow_t &&a0) {
  return std::nothrow_t(std::move(a0));
}

std::nothrow_t &f21(std::nothrow_t &a0, const std::nothrow_t &a1) {
  return a0.operator=(a1);
}

std::nothrow_t &f22(std::nothrow_t &a0, std::nothrow_t &&a1) {
  return a0.operator=(std::move(a1));
}

std::new_handler f23(std::new_handler a0) { return std::set_new_handler(a0); }
