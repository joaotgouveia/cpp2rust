#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename... T2,
          std::enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::unique_ptr<T1> f69(T2 &&...a0) {
  return std::make_unique<T1, T2...>(std::forward<T2>(a0)...);
}
