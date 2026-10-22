#include "../common/init.h"
#include "../common/traits.h"
#include <mutex>
#include <utility>

template <typename... T1>
std::scoped_lock<T1...> f5(std::adopt_lock_t a0, T1 &...a1) {
  return {std::move(a0), a1...};
}
