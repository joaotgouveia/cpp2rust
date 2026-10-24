#include <algorithm>
#include <array>
#include <cstddef>
#include <iterator>
#include <span>
#include <utility>

template <typename T1, std::size_t T2> using t1 = std::span<T1, T2>;

template <typename T1, std::size_t T2>
using t2 = typename std::span<T1, T2>::element_type;

template <typename T1, std::size_t T2>
using t3 = typename std::span<T1, T2>::value_type;

template <typename T1, std::size_t T2>
using t4 = typename std::span<T1, T2>::size_type;

template <typename T1, std::size_t T2>
using t5 = typename std::span<T1, T2>::difference_type;

template <typename T1, std::size_t T2>
using t6 = typename std::span<T1, T2>::pointer;

template <typename T1, std::size_t T2>
using t7 = typename std::span<T1, T2>::const_pointer;

template <typename T1, std::size_t T2>
using t8 = typename std::span<T1, T2>::reference;

template <typename T1, std::size_t T2>
using t9 = typename std::span<T1, T2>::const_reference;

template <typename T1, std::size_t T2>
std::span<T1, T2> f1(const std::span<T1, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::span<T1, T2> &f2(std::span<T1, T2> &a0, const std::span<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::size_t T2>
std::size_t f3(const std::span<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, std::size_t T2>
std::size_t f4(const std::span<T1, T2> &a0) {
  return a0.size_bytes();
}

template <typename T1, std::size_t T2> bool f5(const std::span<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2> T1 &f6(const std::span<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2> T1 &f7(const std::span<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2>
T1 &f8(const std::span<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, std::size_t T2> T1 *f9(const std::span<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent> f10(const std::span<T1, T2> &a0,
                                       std::size_t a1) {
  return a0.first(a1);
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent> f11(const std::span<T1, T2> &a0,
                                       std::size_t a1) {
  return a0.last(a1);
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent> f12(const std::span<T1, T2> &a0,
                                       std::size_t a1, std::size_t a2) {
  return a0.subspan(a1, a2);
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T3> f13(const std::span<T1, T2> &a0) {
  return a0.template first<T3>();
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T3> f14(const std::span<T1, T2> &a0) {
  return a0.template last<T3>();
}

template <typename T1, std::size_t T2, std::size_t T3,
          std::size_t T4 = std::dynamic_extent>
auto f15(const std::span<T1, T2> &a0) {
  return a0.template subspan<T3>();
}

template <typename T1, std::size_t T2> std::size_t f16() {
  return std::span<T1, T2>::extent;
}
