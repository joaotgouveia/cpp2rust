#include <algorithm>
#include <bitset>
#include <utility>

template <std::size_t T1> using t1 = std::bitset<T1>;

template <std::size_t T1> using t2 = typename std::bitset<T1>::reference;

template <std::size_t T1> std::bitset<T1> f1() { return std::bitset<T1>(); }

template <std::size_t T1> std::bitset<T1> f2(unsigned long long a0) {
  return std::bitset<T1>(a0);
}

template <std::size_t T1>
std::bitset<T1> &f3(std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return a0.operator&=(a1);
}

template <std::size_t T1>
std::bitset<T1> &f4(std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return a0.operator|=(a1);
}

template <std::size_t T1>
std::bitset<T1> &f5(std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return a0.operator^=(a1);
}

template <std::size_t T1>
std::bitset<T1> &f6(std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator<<=(a1);
}

template <std::size_t T1>
std::bitset<T1> &f7(std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator>>=(a1);
}

template <std::size_t T1> std::bitset<T1> &f8(std::bitset<T1> &a0) {
  return a0.set();
}

template <std::size_t T1>
std::bitset<T1> &f9(std::bitset<T1> &a0, std::size_t a1, bool a2) {
  return a0.set(a1, a2);
}

template <std::size_t T1> std::bitset<T1> &f10(std::bitset<T1> &a0) {
  return a0.reset();
}

template <std::size_t T1>
std::bitset<T1> &f11(std::bitset<T1> &a0, std::size_t a1) {
  return a0.reset(a1);
}

template <std::size_t T1> std::bitset<T1> &f12(std::bitset<T1> &a0) {
  return a0.flip();
}

template <std::size_t T1>
std::bitset<T1> &f13(std::bitset<T1> &a0, std::size_t a1) {
  return a0.flip(a1);
}

template <std::size_t T1> std::bitset<T1> f14(const std::bitset<T1> &a0) {
  return a0.operator~();
}

template <std::size_t T1>
typename std::bitset<T1>::reference f15(std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <std::size_t T1> bool f16(const std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <std::size_t T1> unsigned long f17(const std::bitset<T1> &a0) {
  return a0.to_ulong();
}

template <std::size_t T1> unsigned long long f18(const std::bitset<T1> &a0) {
  return a0.to_ullong();
}

template <std::size_t T1>
std::basic_string<char, std::char_traits<char>, std::allocator<char>>
f19(const std::bitset<T1> &a0, char a1, char a2) {
  return a0.to_string(a1, a2);
}

template <std::size_t T1> std::size_t f20(const std::bitset<T1> &a0) {
  return a0.count();
}

template <std::size_t T1> std::size_t f21(const std::bitset<T1> &a0) {
  return a0.size();
}

template <std::size_t T1>
bool f22(const std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return a0.operator==(a1);
}

template <std::size_t T1> bool f23(const std::bitset<T1> &a0, std::size_t a1) {
  return a0.test(a1);
}

template <std::size_t T1> bool f24(const std::bitset<T1> &a0) {
  return a0.all();
}

template <std::size_t T1> bool f25(const std::bitset<T1> &a0) {
  return a0.any();
}

template <std::size_t T1> bool f26(const std::bitset<T1> &a0) {
  return a0.none();
}

template <std::size_t T1>
std::bitset<T1> f27(const std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator<<(a1);
}

template <std::size_t T1>
std::bitset<T1> f28(const std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator>>(a1);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::basic_string<T2, T3, T4> f29(const std::bitset<T1> &a0, T2 a1, T2 a2) {
  return a0.template to_string<T2, T3, T4>(std::move(a1), std::move(a2));
}

template <std::size_t T1, typename T2, typename T3>
std::basic_string<T2, T3, std::allocator<T2>> f30(const std::bitset<T1> &a0,
                                                  T2 a1, T2 a2) {
  return a0.template to_string<T2, T3>(std::move(a1), std::move(a2));
}

template <std::size_t T1>
std::bitset<T1> f31(const std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return std::operator& <T1>(a0, a1);
}

template <std::size_t T1>
std::bitset<T1> f32(const std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return std::operator^ <T1>(a0, a1);
}

template <std::size_t T1>
std::bitset<T1> f33(const std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return std::operator| <T1>(a0, a1);
}
