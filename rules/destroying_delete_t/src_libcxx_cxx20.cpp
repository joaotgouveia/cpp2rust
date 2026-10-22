#include "../common/init.h"
#include "../common/traits.h"
#include <new>
#include <utility>

using t1 = std::destroying_delete_t;

std::destroying_delete_t f1() { return std::destroying_delete_t(); }

std::destroying_delete_t f2(const std::destroying_delete_t &a0) {
  return std::destroying_delete_t(a0);
}

std::destroying_delete_t f3(std::destroying_delete_t &&a0) {
  return std::destroying_delete_t(std::move(a0));
}

std::destroying_delete_t &f4(std::destroying_delete_t &a0,
                             const std::destroying_delete_t &a1) {
  return a0.operator=(a1);
}

std::destroying_delete_t &f5(std::destroying_delete_t &a0,
                             std::destroying_delete_t &&a1) {
  return a0.operator=(std::move(a1));
}
