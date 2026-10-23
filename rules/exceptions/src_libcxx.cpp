#include <new>
#include <utility>

using t1 = std::bad_alloc;

std::bad_alloc f1() { return std::bad_alloc(); }

std::bad_alloc f2(const std::bad_alloc &a0) { return std::bad_alloc(a0); }

std::bad_alloc &f3(std::bad_alloc &a0, const std::bad_alloc &a1) {
  return a0.operator=(a1);
}

const char *f4(const std::bad_alloc &a0) { return a0.what(); }

using t2 = std::bad_array_new_length;

std::bad_array_new_length f5() { return std::bad_array_new_length(); }

std::bad_array_new_length f6(const std::bad_array_new_length &a0) {
  return std::bad_array_new_length(a0);
}

std::bad_array_new_length &f7(std::bad_array_new_length &a0,
                              const std::bad_array_new_length &a1) {
  return a0.operator=(a1);
}

const char *f8(const std::bad_array_new_length &a0) { return a0.what(); }
