#include <algorithm>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <ostream>
#include <utility>

template <typename T1>
bool f21(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f22(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f23(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1>
bool f24(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f25(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f26(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1>
bool f27(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f28(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f29(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1>
bool f30(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f31(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f32(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1>
bool f34(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f35(std::nullptr_t a0, const std::shared_ptr<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f36(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
