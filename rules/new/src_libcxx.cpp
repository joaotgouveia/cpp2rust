#include <cstddef>
#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void f38(void *a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator delete(a0, a1, a2);
}

void f39(void *a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator delete[](a0, a1, a2);
}

void *f40(std::size_t a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator new(a0, a1, a2);
}

void *f41(std::size_t a0, std::align_val_t a1, const std::nothrow_t &a2) {
  return operator new[](a0, a1, a2);
}
