#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <new>
#include <utility>

using t6 = std::destroying_delete_t;

std::destroying_delete_t f29() { return std::destroying_delete_t(); }

std::destroying_delete_t f30(const std::destroying_delete_t &a0) {
  return std::destroying_delete_t(a0);
}

std::destroying_delete_t f31(std::destroying_delete_t &&a0) {
  return std::destroying_delete_t(std::move(a0));
}

std::destroying_delete_t &f32(std::destroying_delete_t &a0,
                              const std::destroying_delete_t &a1) {
  return a0.operator=(a1);
}

std::destroying_delete_t &f33(std::destroying_delete_t &a0,
                              std::destroying_delete_t &&a1) {
  return a0.operator=(std::move(a1));
}
