#include <string>
#include <utility>

template <typename T1>
std::reference_wrapper<T1> f3(const std::reference_wrapper<T1> &a0) {
  return std::reference_wrapper<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> &f4(std::reference_wrapper<T1> &a0,
                               const std::reference_wrapper<T1> &a1) {
  return a0.operator=(a1);
}
