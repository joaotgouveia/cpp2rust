#include <algorithm>
#include <cstddef>
#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void f1(void *a0, const std::nothrow_t &a1) { return operator delete(a0, a1); }

void f2(void *a0, void *a1) { return operator delete(a0, a1); }

void f3(void *a0, const std::nothrow_t &a1) {
  return operator delete[](a0, a1);
}

void f4(void *a0, void *a1) { return operator delete[](a0, a1); }

void *f5(std::size_t a0, const std::nothrow_t &a1) {
  return operator new(a0, a1);
}

void *f6(std::size_t a0, void *a1) { return operator new(a0, a1); }

void *f7(std::size_t a0, const std::nothrow_t &a1) {
  return operator new[](a0, a1);
}

void *f8(std::size_t a0, void *a1) { return operator new[](a0, a1); }
