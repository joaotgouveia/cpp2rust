#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> bool f3(std::ranges::view_interface<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f4(const std::ranges::view_interface<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f5(std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f6(const std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> auto f7(std::ranges::view_interface<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f8(const std::ranges::view_interface<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f9(std::ranges::view_interface<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f10(const std::ranges::view_interface<T1> &a0) {
  return a0.size();
}

template <typename T1> decltype(auto) f11(std::ranges::view_interface<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f12(const std::ranges::view_interface<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f13(std::ranges::view_interface<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f14(const std::ranges::view_interface<T1> &a0) {
  return a0.back();
}
