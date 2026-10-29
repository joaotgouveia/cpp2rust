#include <algorithm>
#include <new>
#include <utility>

void f1(void *a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator delete(a0, a1, a2);
}

void f2(void *a0, const std::nothrow_t &a1) { return operator delete(a0, a1); }

void f3(void *a0, void *a1) { return operator delete(a0, a1); }

void f4(void *a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator delete[](a0, a1, a2);
}

void f5(void *a0, const std::nothrow_t &a1) {
  return operator delete[](a0, a1);
}

void f6(void *a0, void *a1) { return operator delete[](a0, a1); }

void *f7(std::size_t a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator new(a0, a1, a2);
}

void *f8(std::size_t a0, const std::nothrow_t &a1) {
  return operator new(a0, a1);
}

void *f9(std::size_t a0, void *a1) { return operator new(a0, a1); }

void *f10(std::size_t a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator new[](a0, a1, a2);
}

void *f11(std::size_t a0, const std::nothrow_t &a1) {
  return operator new[](a0, a1);
}

void *f12(std::size_t a0, void *a1) { return operator new[](a0, a1); }

using t1 = std::align_val_t;

using t2 = std::bad_alloc;

std::bad_alloc f13() { return std::bad_alloc(); }

std::bad_alloc f14(const std::bad_alloc &a0) { return std::bad_alloc(a0); }

std::bad_alloc &f15(std::bad_alloc &a0, const std::bad_alloc &a1) {
  return a0.operator=(a1);
}

const char *f16(const std::bad_alloc &a0) { return a0.what(); }

using t3 = std::bad_array_new_length;

std::bad_array_new_length f17() { return std::bad_array_new_length(); }

const char *f18(const std::bad_array_new_length &a0) { return a0.what(); }

std::bad_array_new_length f19(const std::bad_array_new_length &a0) {
  return std::bad_array_new_length(a0);
}

std::bad_array_new_length &f20(std::bad_array_new_length &a0,
                               const std::bad_array_new_length &a1) {
  return a0.operator=(a1);
}

const std::destroying_delete_t &f21() { return std::destroying_delete; }

using t4 = std::destroying_delete_t;

std::destroying_delete_t f22() { return std::destroying_delete_t(); }

std::destroying_delete_t f23(const std::destroying_delete_t &a0) {
  return std::destroying_delete_t(a0);
}

std::destroying_delete_t f24(std::destroying_delete_t &&a0) {
  return std::destroying_delete_t(std::move(a0));
}

std::destroying_delete_t &f25(std::destroying_delete_t &a0,
                              const std::destroying_delete_t &a1) {
  return a0.operator=(a1);
}

std::destroying_delete_t &f26(std::destroying_delete_t &a0,
                              std::destroying_delete_t &&a1) {
  return a0.operator=(std::move(a1));
}

std::new_handler f27() { return std::get_new_handler(); }

std::size_t f28() { return std::hardware_constructive_interference_size; }

std::size_t f29() { return std::hardware_destructive_interference_size; }

template <typename T1> T1 *f30(T1 *a0) { return std::launder(a0); }

using t5 = std::new_handler;

const std::nothrow_t &f31() { return std::nothrow; }

using t6 = std::nothrow_t;

std::nothrow_t f32() { return std::nothrow_t(); }

std::nothrow_t f33(const std::nothrow_t &a0) { return std::nothrow_t(a0); }

std::nothrow_t f34(std::nothrow_t &&a0) {
  return std::nothrow_t(std::move(a0));
}

std::nothrow_t &f35(std::nothrow_t &a0, const std::nothrow_t &a1) {
  return a0.operator=(a1);
}

std::nothrow_t &f36(std::nothrow_t &a0, std::nothrow_t &&a1) {
  return a0.operator=(std::move(a1));
}

std::new_handler f37(std::new_handler a0) { return std::set_new_handler(a0); }
