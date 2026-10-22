#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename... T2> std::unique_ptr<T1> f54(T2 &&...a0) {
  return std::make_unique<T1, T2...>(std::forward<T2>(a0)...);
}
