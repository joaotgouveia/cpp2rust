#include <condition_variable>
#include <mutex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1>
std::scoped_lock<T1...> f4(std::adopt_lock_t a0, T1 &...a1) {
  return std::scoped_lock<T1...>(std::move(a0), a1...);
}
