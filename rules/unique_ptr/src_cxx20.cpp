#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <utility>

template <typename T1> std::unique_ptr<T1> f61() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1> auto f62(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f63(std::basic_ostream<T1, T2> &a0,
                                const std::unique_ptr<T3, T4> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer>
f64(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer,
                                typename std::unique_ptr<T3, T4>::pointer>
f65(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 <=> a1;
}
