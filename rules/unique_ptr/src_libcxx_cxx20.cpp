#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> std::unique_ptr<T1> f76(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}
