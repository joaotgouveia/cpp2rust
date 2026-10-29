#include <bit>
#include <utility>

template <typename T1, typename T2>
T1 f1(const T2 &a0)
  requires((sizeof(T1) == sizeof(T2)) && std::is_trivially_copyable_v<T1> &&
           std::is_trivially_copyable_v<T2>)
{
  return std::bit_cast<T1, T2>(a0);
}

template <typename T1> T1 f2(T1 a0) { return std::bit_ceil<T1>(std::move(a0)); }

template <typename T1> T1 f3(T1 a0) {
  return std::bit_floor<T1>(std::move(a0));
}

template <typename T1> int f4(T1 a0) {
  return std::bit_width<T1>(std::move(a0));
}

template <typename T1>
T1 f5(T1 a0)
  requires(std::integral<T1>)
{
  return std::byteswap<T1>(std::move(a0));
}

template <typename T1> int f6(T1 a0) {
  return std::countl_one<T1>(std::move(a0));
}

template <typename T1> int f7(T1 a0) {
  return std::countl_zero<T1>(std::move(a0));
}

template <typename T1> int f8(T1 a0) {
  return std::countr_one<T1>(std::move(a0));
}

template <typename T1> int f9(T1 a0) {
  return std::countr_zero<T1>(std::move(a0));
}

using t1 = std::endian;

std::endian f10() { return std::endian::little; }

std::endian f11() { return std::endian::big; }

std::endian f12() { return std::endian::native; }

template <typename T1> bool f13(T1 a0) {
  return std::has_single_bit<T1>(std::move(a0));
}

template <typename T1> int f14(T1 a0) {
  return std::popcount<T1>(std::move(a0));
}

template <typename T1> T1 f15(T1 a0, int a1) {
  return std::rotl<T1>(std::move(a0), a1);
}

template <typename T1> T1 f16(T1 a0, int a1) {
  return std::rotr<T1>(std::move(a0), a1);
}
