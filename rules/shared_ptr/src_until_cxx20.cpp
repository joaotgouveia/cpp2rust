#include <algorithm>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
bool f22(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 != a1;
}

template <typename T1>
bool f23(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f24(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f25(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 < a1;
}

template <typename T1>
bool f26(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f27(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f28(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 <= a1;
}

template <typename T1>
bool f29(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f30(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f31(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
bool f32(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 > a1;
}

template <typename T1>
bool f33(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f34(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f35(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 >= a1;
}

template <typename T1>
bool f36(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f37(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0 >= a1;
}
