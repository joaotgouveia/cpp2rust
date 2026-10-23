#include <bit>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> T1 f1(const T2 &a0) {
  return std::bit_cast<T1, T2>(a0);
}

template <typename T1> T1 f2(T1 a0) { return std::bit_ceil<T1>(a0); }

template <typename T1> T1 f3(T1 a0) { return std::bit_floor<T1>(a0); }

template <typename T1> int f4(T1 a0) { return std::bit_width<T1>(a0); }

template <typename T1> int f5(T1 a0) { return std::countl_one<T1>(a0); }

template <typename T1> int f6(T1 a0) { return std::countl_zero<T1>(a0); }

template <typename T1> int f7(T1 a0) { return std::countr_one<T1>(a0); }

template <typename T1> int f8(T1 a0) { return std::countr_zero<T1>(a0); }

using t1 = std::endian;

std::endian f9() { return std::endian::little; }

std::endian f10() { return std::endian::big; }

std::endian f11() { return std::endian::native; }

template <typename T1> bool f12(T1 a0) { return std::has_single_bit<T1>(a0); }

template <typename T1> int f13(T1 a0) { return std::popcount<T1>(a0); }

template <typename T1> T1 f14(T1 a0, int a1) { return std::rotl<T1>(a0, a1); }

template <typename T1> T1 f15(T1 a0, int a1) { return std::rotr<T1>(a0, a1); }
