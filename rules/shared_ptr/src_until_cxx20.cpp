#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1>
bool f40(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 != a1;
}

template <typename T1>
bool f41(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f42(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f43(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 < a1;
}

template <typename T1>
bool f44(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f45(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f46(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 <= a1;
}

template <typename T1>
bool f47(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f48(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f49(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
bool f50(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 > a1;
}

template <typename T1>
bool f51(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f52(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f53(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 >= a1;
}

template <typename T1>
bool f54(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f55(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 >= a1;
}
