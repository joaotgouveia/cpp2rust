#include <algorithm>
#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void *f1(std::size_t a0, std::size_t a1, void *&a2, std::size_t &a3) {
  return std::align(a0, a1, a2, a3);
}
