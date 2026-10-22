#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <compare>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> std::unique_ptr<T1> f55() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1, typename T2>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer>
f56(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer,
                                typename std::unique_ptr<T3, T4>::pointer>
f57(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 <=> a1;
}
