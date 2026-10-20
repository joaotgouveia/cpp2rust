#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f56(const std::shared_ptr<T2> &a0) {
  return std::reinterpret_pointer_cast<T1, T2>(a0);
}

template <typename T1> using t3 = typename std::shared_ptr<T1>::weak_type;
