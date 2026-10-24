#include <memory>
#include <utility>

template <typename T1> using t2 = typename std::weak_ptr<T1>::element_type;

template <typename T1> void f21(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::weak_ptr<T1> f22(const std::shared_ptr<T2> &a0) {
  return std::weak_ptr<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::weak_ptr<T1> &f23(std::weak_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
bool f24(const std::weak_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f25(const std::weak_ptr<T1> &a0, const std::weak_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}
