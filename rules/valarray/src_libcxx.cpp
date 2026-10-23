#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

template <typename T1> void f113(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

using t3 = std::gslice;

std::gslice f114() { return std::gslice(); }

std::gslice f115(std::size_t a0, const std::valarray<std::size_t> &a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, a1, a2);
}

std::gslice f116(std::size_t a0, const std::valarray<std::size_t> &a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, a1, std::move(a2));
}

std::gslice f117(std::size_t a0, std::valarray<std::size_t> &&a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, std::move(a1), a2);
}

std::gslice f118(std::size_t a0, std::valarray<std::size_t> &&a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, std::move(a1), std::move(a2));
}

std::size_t f119(const std::gslice &a0) { return a0.start(); }

std::valarray<std::size_t> f120(const std::gslice &a0) { return a0.size(); }

std::valarray<std::size_t> f121(const std::gslice &a0) { return a0.stride(); }

std::gslice f122(const std::gslice &a0) { return std::gslice(a0); }

std::gslice f123(std::gslice &&a0) { return std::gslice(std::move(a0)); }

std::gslice &f124(std::gslice &a0, const std::gslice &a1) {
  return a0.operator=(a1);
}

std::gslice &f125(std::gslice &a0, std::gslice &&a1) {
  return a0.operator=(std::move(a1));
}

using t4 = std::slice;

std::slice f126() { return std::slice(); }

std::slice f127(std::size_t a0, std::size_t a1, std::size_t a2) {
  return std::slice(a0, a1, a2);
}

std::size_t f128(const std::slice &a0) { return a0.start(); }

std::size_t f129(const std::slice &a0) { return a0.size(); }

std::size_t f130(const std::slice &a0) { return a0.stride(); }

std::slice f131(const std::slice &a0) { return std::slice(a0); }

std::slice f132(std::slice &&a0) { return std::slice(std::move(a0)); }

std::slice &f133(std::slice &a0, const std::slice &a1) {
  return a0.operator=(a1);
}

std::slice &f134(std::slice &a0, std::slice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::valarray<T1> f135(const T1 *a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::gslice_array<T1> f136(std::valarray<T1> &a0, std::gslice &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::mask_array<T1> f137(std::valarray<T1> &a0, std::valarray<bool> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::indirect_array<T1> f138(std::valarray<T1> &a0,
                             std::valarray<std::size_t> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::valarray<T1> f139(const std::valarray<T1> &a0, T1(a1)(T1)) {
  return a0.apply(a1);
}

template <typename T1>
std::valarray<T1> f140(const std::valarray<T1> &a0, T1(a1)(const T1 &)) {
  return a0.apply(a1);
}
