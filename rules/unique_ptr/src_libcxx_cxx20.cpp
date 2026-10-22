#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, std::enable_if_t<!std::is_array_v<T1>, int> T2 = 0>
std::unique_ptr<T1> f71() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1,
          std::enable_if_t<std::is_unbounded_array_v<T1>, int> T2 = 0>
std::unique_ptr<T1> f72(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}
