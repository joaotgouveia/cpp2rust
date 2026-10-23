#include <atomic>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1>
std::remove_extent_t<T1> *f93(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1>
typename std::add_lvalue_reference<typename std::remove_extent<T1>::type>::type
f94(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::remove_extent_t<T1> *f95(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
typename std::add_lvalue_reference<typename std::remove_extent<T1>::type>::type
f96(const std::shared_ptr<T1> &a0, std::ptrdiff_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f97(const std::shared_ptr<T2> &a0,
                        std::remove_extent_t<T1> *a1) {
  return std::shared_ptr<T1>(a0, a1);
}
