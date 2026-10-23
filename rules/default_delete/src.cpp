#include <condition_variable>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::default_delete<T1[]>;

template <typename T1> std::default_delete<T1[]> f1() {
  return std::default_delete<T1[]>();
}

template <typename T1> using t2 = std::default_delete<T1>;

template <typename T1> std::default_delete<T1> f2() {
  return std::default_delete<T1>();
}

template <typename T1> void f3(const std::default_delete<T1> &a0, T1 *a1) {
  return a0.operator()(a1);
}
