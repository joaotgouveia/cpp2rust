#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::reference_wrapper<T1> f1(const std::reference_wrapper<T1> &a0) {
  return std::reference_wrapper<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> &f2(std::reference_wrapper<T1> &a0,
                               const std::reference_wrapper<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 &f3(const std::reference_wrapper<T1> &a0) {
  return static_cast<T1 &>(a0);
}

template <typename T1> T1 &f4(const std::reference_wrapper<T1> &a0) {
  return a0.get();
}
