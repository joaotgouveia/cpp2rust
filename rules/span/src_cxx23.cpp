#include <algorithm>
#include <array>
#include <cstddef>
#include <span>
#include <utility>

template <typename T1, std::size_t T2>
std::span<const std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f1(std::span<T1, T2> a0) {
  return std::as_bytes<T1, T2>(std::forward<std::span<T1, T2>>(a0));
}

template <typename T1, std::size_t T2>
std::span<std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f2(std::span<T1, T2> a0)
  requires((!std::is_const_v<T1>))
{
  return std::as_writable_bytes<T1, T2>(std::forward<std::span<T1, T2>>(a0));
}

std::size_t f3() { return std::dynamic_extent; }

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
using t10 = typename std::span<T1, T2>::iterator;

template <typename T1, std::size_t T2>
using t11 = typename std::span<T1, T2>::reverse_iterator;

template <typename T1, std::size_t T2>
using t12 = typename std::span<T1, T2>::const_iterator;

template <typename T1, std::size_t T2>
using t13 = typename std::span<T1, T2>::const_reverse_iterator;

template <typename T1, std::size_t T2>
std::span<T1, T2> f4()
  requires((T2 == std::dynamic_extent || T2 == 0))
{
  return std::span<T1, T2>();
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f5(const std::span<T1, T2> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::span<T1, T2> &f6(std::span<T1, T2> &a0, const std::span<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::size_t T2>
std::size_t f7(const std::span<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, std::size_t T2>
std::size_t f8(const std::span<T1, T2> &a0) {
  return a0.size_bytes();
}

template <typename T1, std::size_t T2> bool f9(const std::span<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2> T1 &f10(const std::span<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2> T1 &f11(const std::span<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2>
T1 &f12(const std::span<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, std::size_t T2> T1 *f13(const std::span<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::iterator f14(const std::span<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::iterator f15(const std::span<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::reverse_iterator f16(const std::span<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::reverse_iterator f17(const std::span<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_iterator f18(const std::span<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_iterator f19(const std::span<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_reverse_iterator
f20(const std::span<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::const_reverse_iterator
f21(const std::span<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent> f22(const std::span<T1, T2> &a0,
                                       std::size_t a1) {
  return a0.first(a1);
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent> f23(const std::span<T1, T2> &a0,
                                       std::size_t a1) {
  return a0.last(a1);
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent> f24(const std::span<T1, T2> &a0,
                                       std::size_t a1, std::size_t a2) {
  return a0.subspan(a1, a2);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f25(T3 a0, std::size_t a1)
  requires(std::contiguous_iterator<T3>)
{
  return std::span<T1, T2>(std::move(a0), a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
std::span<T1, T2> f26(T3 a0, T4 a1)
  requires(std::contiguous_iterator<T3>) && (std::sized_sentinel_for<T4, T3>)
{
  return std::span<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T2> f27(typename std::type_identity<T1>::type (&a0)[T3])
  requires((T2 == std::dynamic_extent || T3 == T2))
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f28(std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f29(const std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f30(T3 &&a0)
  requires(!(std::is_same<T3, const std::span<T1, T2> &>::value))
{
  return std::span<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f31(const std::span<T3, T4> &a0)
  requires((T2 == std::dynamic_extent || T4 == std::dynamic_extent ||
            T2 == T4) &&
           (!(std::is_same<T3, T1>::value && T4 == T2)))
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T3> f32(const std::span<T1, T2> &a0) {
  return a0.template first<T3>();
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T3> f33(const std::span<T1, T2> &a0) {
  return a0.template last<T3>();
}

template <typename T1, std::size_t T2, std::size_t T3,
          std::size_t T4 = std::dynamic_extent>
auto f34(const std::span<T1, T2> &a0) {
  return a0.template subspan<T3>();
}

template <typename T1, std::size_t T2> std::size_t f35() {
  return std::span<T1, T2>::extent;
}
