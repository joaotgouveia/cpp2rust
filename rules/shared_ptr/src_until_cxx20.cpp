#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
bool f32(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 != a1;
}

template <typename T1>
bool f33(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f34(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f35(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 < a1;
}

template <typename T1>
bool f36(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f37(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f38(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 <= a1;
}

template <typename T1>
bool f39(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f40(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f41(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
bool f42(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 > a1;
}

template <typename T1>
bool f43(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f44(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f45(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 >= a1;
}

template <typename T1>
bool f46(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f47(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 >= a1;
}
