#include <algorithm>
#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void f24(void *a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator delete(a0, a1, a2);
}

void f25(void *a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator delete[](a0, a1, a2);
}

void *f26(std::size_t a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator new(a0, a1, a2);
}

void *f27(std::size_t a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator new[](a0, a1, a2);
}

using t5 = std::align_val_t;

template <typename T1> T1 *f28(T1 *a0) { return std::launder<T1>(a0); }
