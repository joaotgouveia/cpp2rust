#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> std::unique_ptr<T1> f75(std::size_t a0) {
  return std::make_unique<T1>(a0);
}
