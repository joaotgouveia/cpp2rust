#include "../common/init.h"
#include "../common/traits.h"
#include <array>
#include <cstddef>
#include <mdspan>
#include <utility>

using t1 = std::layout_left;

std::layout_left f1() { return std::layout_left(); }

std::layout_left f2(const std::layout_left &a0) { return std::layout_left(a0); }

std::layout_left f3(std::layout_left &&a0) {
  return std::layout_left(std::move(a0));
}

std::layout_left &f4(std::layout_left &a0, const std::layout_left &a1) {
  return a0.operator=(a1);
}

std::layout_left &f5(std::layout_left &a0, std::layout_left &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::layout_right;

std::layout_right f6() { return std::layout_right(); }

std::layout_right f7(const std::layout_right &a0) {
  return std::layout_right(a0);
}

std::layout_right f8(std::layout_right &&a0) {
  return std::layout_right(std::move(a0));
}

std::layout_right &f9(std::layout_right &a0, const std::layout_right &a1) {
  return a0.operator=(a1);
}

std::layout_right &f10(std::layout_right &a0, std::layout_right &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::layout_stride;

std::layout_stride f11() { return std::layout_stride(); }

std::layout_stride f12(const std::layout_stride &a0) {
  return std::layout_stride(a0);
}

std::layout_stride f13(std::layout_stride &&a0) {
  return std::layout_stride(std::move(a0));
}

std::layout_stride &f14(std::layout_stride &a0, const std::layout_stride &a1) {
  return a0.operator=(a1);
}

std::layout_stride &f15(std::layout_stride &a0, std::layout_stride &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
using t4 = std::mdspan<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::mdspan<T1, T2, T3, T4>::extents_type;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::mdspan<T1, T2, T3, T4>::layout_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::mdspan<T1, T2, T3, T4>::accessor_type;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::mdspan<T1, T2, T3, T4>::mapping_type;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::mdspan<T1, T2, T3, T4>::element_type;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::mdspan<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::mdspan<T1, T2, T3, T4>::index_type;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::mdspan<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::mdspan<T1, T2, T3, T4>::rank_type;

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::mdspan<T1, T2, T3, T4>::data_handle_type;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::mdspan<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
typename std::mdspan<T1, T2, T3, T4>::rank_type f16() {
  return std::mdspan<T1, T2, T3, T4>::rank();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::mdspan<T1, T2, T3, T4>::rank_type f17() {
  return std::mdspan<T1, T2, T3, T4>::rank_dynamic();
}

template <typename T1, typename T2, typename T3, typename T4>
std::size_t f18(typename std::mdspan<T1, T2, T3, T4>::rank_type a0) {
  return std::mdspan<T1, T2, T3, T4>::static_extent(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::mdspan<T1, T2, T3, T4>::index_type
f19(const std::mdspan<T1, T2, T3, T4> &a0,
    typename std::mdspan<T1, T2, T3, T4>::rank_type a1) {
  return a0.extent(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4> f20() {
  return std::mdspan<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4> f21(const std::mdspan<T1, T2, T3, T4> &a0) {
  return std::mdspan<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4> f22(std::mdspan<T1, T2, T3, T4> &&a0) {
  return std::mdspan<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4>
f23(typename std::mdspan<T1, T2, T3, T4>::data_handle_type a0,
    const typename std::mdspan<T1, T2, T3, T4>::extents_type &a1) {
  return std::mdspan<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4>
f24(typename std::mdspan<T1, T2, T3, T4>::data_handle_type a0,
    const typename std::mdspan<T1, T2, T3, T4>::mapping_type &a1) {
  return std::mdspan<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4>
f25(typename std::mdspan<T1, T2, T3, T4>::data_handle_type a0,
    const typename std::mdspan<T1, T2, T3, T4>::mapping_type &a1,
    const typename std::mdspan<T1, T2, T3, T4>::accessor_type &a2) {
  return std::mdspan<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4> &f26(std::mdspan<T1, T2, T3, T4> &a0,
                                 const std::mdspan<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::mdspan<T1, T2, T3, T4> &f27(std::mdspan<T1, T2, T3, T4> &a0,
                                 std::mdspan<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::mdspan<T1, T2, T3, T4>::size_type
f28(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f29(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
const typename std::mdspan<T1, T2, T3, T4>::extents_type &
f30(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.extents();
}

template <typename T1, typename T2, typename T3, typename T4>
const typename std::mdspan<T1, T2, T3, T4>::data_handle_type &
f31(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.data_handle();
}

template <typename T1, typename T2, typename T3, typename T4>
const typename std::mdspan<T1, T2, T3, T4>::mapping_type &
f32(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.mapping();
}

template <typename T1, typename T2, typename T3, typename T4>
const typename std::mdspan<T1, T2, T3, T4>::accessor_type &
f33(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.accessor();
}

template <typename T1, typename T2, typename T3, typename T4> bool f34() {
  return std::mdspan<T1, T2, T3, T4>::is_always_unique();
}

template <typename T1, typename T2, typename T3, typename T4> bool f35() {
  return std::mdspan<T1, T2, T3, T4>::is_always_exhaustive();
}

template <typename T1, typename T2, typename T3, typename T4> bool f36() {
  return std::mdspan<T1, T2, T3, T4>::is_always_strided();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f37(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.is_unique();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f38(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.is_exhaustive();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f39(const std::mdspan<T1, T2, T3, T4> &a0) {
  return a0.is_strided();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::mdspan<T1, T2, T3, T4>::index_type
f40(const std::mdspan<T1, T2, T3, T4> &a0,
    typename std::mdspan<T1, T2, T3, T4>::rank_type a1) {
  return a0.stride(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::mdspan<T1, T2, T3, T4>
f41(typename std::mdspan<T1, T2, T3, T4>::data_handle_type a0, T5... a1) {
  return std::mdspan<T1, T2, T3, T4>(a0, a1...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::size_t T6>
std::mdspan<T1, T2, T3, T4>
f42(typename std::mdspan<T1, T2, T3, T4>::data_handle_type a0,
    const std::array<T5, T6> &a1) {
  return std::mdspan<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::size_t T6>
std::mdspan<T1, T2, T3, T4>
f43(typename std::mdspan<T1, T2, T3, T4>::data_handle_type a0,
    std::span<T5, T6> a1) {
  return std::mdspan<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7, typename T8>
std::mdspan<T1, T2, T3, T4> f44(const std::mdspan<T5, T6, T7, T8> &a0) {
  return std::mdspan<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::mdspan<T1, T2, T3, T4>::reference
f45(const std::mdspan<T1, T2, T3, T4> &a0, T5... a1) {
  return a0.template operator[]<T5...>(a1...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::mdspan<T1, T2, T3, T4>::reference
f46(const std::mdspan<T1, T2, T3, T4> &a0,
    const std::array<T5, std::mdspan<T1, T2, T3, T4>::rank()> &a1) {
  return a0.template operator[]<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::mdspan<T1, T2, T3, T4>::reference
f47(const std::mdspan<T1, T2, T3, T4> &a0,
    std::span<T5, std::mdspan<T1, T2, T3, T4>::rank()> a1) {
  return a0.template operator[]<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f48(std::mdspan<T1, T2, T3, T4> &a0, std::mdspan<T1, T2, T3, T4> &a1) {
  return swap(a0, a1);
}
