#include <algorithm>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f2(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator!= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
bool f4(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}
