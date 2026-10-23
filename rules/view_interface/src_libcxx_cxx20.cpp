#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2 = T1>
bool f19(std::ranges::view_interface<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = T1>
bool f20(const std::ranges::view_interface<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = T1>
bool f21(std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = T1>
bool f22(const std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = T1>
auto f23(std::ranges::view_interface<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = T1>
auto f24(const std::ranges::view_interface<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = T1>
auto f25(std::ranges::view_interface<T1> &a0) {
  return a0.size();
}

template <typename T1, typename T2 = T1>
auto f26(const std::ranges::view_interface<T1> &a0) {
  return a0.size();
}

template <typename T1, typename T2 = T1>
decltype(auto) f27(std::ranges::view_interface<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = T1>
decltype(auto) f28(const std::ranges::view_interface<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = T1>
decltype(auto) f29(std::ranges::view_interface<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = T1>
decltype(auto) f30(const std::ranges::view_interface<T1> &a0) {
  return a0.back();
}
