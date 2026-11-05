#include <bit>
#include <utility>

template <typename T1, typename T2>
T1 f1(const T2 &a0)
  requires((sizeof(T1) == sizeof(T2)) && std::is_trivially_copyable_v<T1> &&
           std::is_trivially_copyable_v<T2>)
{
  return std::bit_cast<T1>(a0);
}

template <typename T1> T1 f2(T1 a0) { return std::bit_ceil(std::move(a0)); }

template <typename T1> int f6(T1 a0) { return std::countl_one(std::move(a0)); }

using t1 = std::endian;

std::endian f10() { return std::endian::little; }

std::endian f11() { return std::endian::big; }

std::endian f12() { return std::endian::native; }

template <typename T1> int f14(T1 a0) { return std::popcount(std::move(a0)); }

template <typename T1> T1 f16(T1 a0, int a1) {
  return std::rotr(std::move(a0), a1);
}
