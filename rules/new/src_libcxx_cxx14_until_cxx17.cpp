#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <new>
#include <utility>

void f42(void *a0, std::size_t a1, std::align_val_t a2) {
  return operator delete(a0, a1, a2);
}

void f43(void *a0, std::size_t a1, std::align_val_t a2) {
  return operator delete[](a0, a1, a2);
}
