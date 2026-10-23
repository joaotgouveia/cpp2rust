#include <cstddef>
#include <new>
#include <utility>

void f34(void *a0, std::align_val_t a1) { return operator delete(a0, a1); }

void f35(void *a0, std::align_val_t a1) { return operator delete[](a0, a1); }

void *f36(std::size_t a0, std::align_val_t a1) { return operator new(a0, a1); }

void *f37(std::size_t a0, std::align_val_t a1) {
  return operator new[](a0, a1);
}
