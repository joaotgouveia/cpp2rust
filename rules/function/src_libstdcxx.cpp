#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <exception>
#include <functional>
#include <string>
#include <utility>

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> &f14(std::function<T1(T2...)> &a0,
                              std::reference_wrapper<T3> a1) {
  return a0.template operator= <T3>(a1);
}
