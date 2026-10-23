#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f47(std::nullptr_t a0, T2 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f48(std::nullptr_t a0, T2 a1, T3 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1> bool f49(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <typename T1>
typename std::shared_ptr<T1>::element_type &f50(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}
