#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <future>
#include <string>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t12 = std::hash<std::bitset<0>>;

std::size_t f28(const std::hash<std::bitset<0>> &a0, const std::bitset<0> &a1) {
  return a0.operator()(a1);
}

std::hash<std::bitset<0>> f29() { return std::hash<std::bitset<0>>(); }

std::hash<std::bitset<0>> f30(const std::hash<std::bitset<0>> &a0) {
  return std::hash<std::bitset<0>>(a0);
}

std::hash<std::bitset<0>> f31(std::hash<std::bitset<0>> &&a0) {
  return std::hash<std::bitset<0>>(std::move(a0));
}

std::hash<std::bitset<0>> &f32(std::hash<std::bitset<0>> &a0,
                               const std::hash<std::bitset<0>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::bitset<0>> &f33(std::hash<std::bitset<0>> &a0,
                               std::hash<std::bitset<0>> &&a1) {
  return a0.operator=(std::move(a1));
}

using t13 = std::hash<std::thread::id>;

std::size_t f34(const std::hash<std::thread::id> &a0,
                const std::thread::id &a1) {
  return a0.operator()(a1);
}

std::hash<std::thread::id> f35() { return std::hash<std::thread::id>(); }

std::hash<std::thread::id> f36(const std::hash<std::thread::id> &a0) {
  return std::hash<std::thread::id>(a0);
}

std::hash<std::thread::id> f37(std::hash<std::thread::id> &&a0) {
  return std::hash<std::thread::id>(std::move(a0));
}

std::hash<std::thread::id> &f38(std::hash<std::thread::id> &a0,
                                const std::hash<std::thread::id> &a1) {
  return a0.operator=(a1);
}

std::hash<std::thread::id> &f39(std::hash<std::thread::id> &a0,
                                std::hash<std::thread::id> &&a1) {
  return a0.operator=(std::move(a1));
}

using t14 = std::hash<std::type_index>::result_type;

using t15 = std::hash<std::type_index>::argument_type;

std::size_t f40(const std::hash<std::type_index> &a0,
                const std::type_index &a1) {
  return a0.operator()(a1);
}

using t16 = std::hash<char>;

std::size_t f41(const std::hash<char> &a0, char a1) {
  return a0.operator()(a1);
}

std::hash<char> f42() { return std::hash<char>(); }

std::hash<char> f43(const std::hash<char> &a0) { return std::hash<char>(a0); }

std::hash<char> f44(std::hash<char> &&a0) {
  return std::hash<char>(std::move(a0));
}

std::hash<char> &f45(std::hash<char> &a0, const std::hash<char> &a1) {
  return a0.operator=(a1);
}

std::hash<char> &f46(std::hash<char> &a0, std::hash<char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t17 = std::hash<long double>;

std::size_t f47(const std::hash<long double> &a0, long double a1) {
  return a0.operator()(a1);
}

std::hash<long double> f48() { return std::hash<long double>(); }

std::hash<long double> f49(const std::hash<long double> &a0) {
  return std::hash<long double>(a0);
}

std::hash<long double> f50(std::hash<long double> &&a0) {
  return std::hash<long double>(std::move(a0));
}

std::hash<long double> &f51(std::hash<long double> &a0,
                            const std::hash<long double> &a1) {
  return a0.operator=(a1);
}

std::hash<long double> &f52(std::hash<long double> &a0,
                            std::hash<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t18 = std::hash<int>;

std::size_t f53(const std::hash<int> &a0, int a1) { return a0.operator()(a1); }

std::hash<int> f54() { return std::hash<int>(); }

std::hash<int> f55(const std::hash<int> &a0) { return std::hash<int>(a0); }

std::hash<int> f56(std::hash<int> &&a0) {
  return std::hash<int>(std::move(a0));
}

std::hash<int> &f57(std::hash<int> &a0, const std::hash<int> &a1) {
  return a0.operator=(a1);
}

std::hash<int> &f58(std::hash<int> &a0, std::hash<int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t19 = std::hash<long long>;

std::size_t f59(const std::hash<long long> &a0, long long a1) {
  return a0.operator()(a1);
}

std::hash<long long> f60() { return std::hash<long long>(); }

std::hash<long long> f61(const std::hash<long long> &a0) {
  return std::hash<long long>(a0);
}

std::hash<long long> f62(std::hash<long long> &&a0) {
  return std::hash<long long>(std::move(a0));
}

std::hash<long long> &f63(std::hash<long long> &a0,
                          const std::hash<long long> &a1) {
  return a0.operator=(a1);
}

std::hash<long long> &f64(std::hash<long long> &a0, std::hash<long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t20 = std::hash<long>;

std::size_t f65(const std::hash<long> &a0, long a1) {
  return a0.operator()(a1);
}

std::hash<long> f66() { return std::hash<long>(); }

std::hash<long> f67(const std::hash<long> &a0) { return std::hash<long>(a0); }

std::hash<long> f68(std::hash<long> &&a0) {
  return std::hash<long>(std::move(a0));
}

std::hash<long> &f69(std::hash<long> &a0, const std::hash<long> &a1) {
  return a0.operator=(a1);
}

std::hash<long> &f70(std::hash<long> &a0, std::hash<long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t21 = std::hash<short>;

std::size_t f71(const std::hash<short> &a0, short a1) {
  return a0.operator()(a1);
}

std::hash<short> f72() { return std::hash<short>(); }

std::hash<short> f73(const std::hash<short> &a0) {
  return std::hash<short>(a0);
}

std::hash<short> f74(std::hash<short> &&a0) {
  return std::hash<short>(std::move(a0));
}

std::hash<short> &f75(std::hash<short> &a0, const std::hash<short> &a1) {
  return a0.operator=(a1);
}

std::hash<short> &f76(std::hash<short> &a0, std::hash<short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t22 = std::hash<wchar_t>;

std::size_t f77(const std::hash<wchar_t> &a0, wchar_t a1) {
  return a0.operator()(a1);
}

std::hash<wchar_t> f78() { return std::hash<wchar_t>(); }

std::hash<wchar_t> f79(const std::hash<wchar_t> &a0) {
  return std::hash<wchar_t>(a0);
}

std::hash<wchar_t> f80(std::hash<wchar_t> &&a0) {
  return std::hash<wchar_t>(std::move(a0));
}

std::hash<wchar_t> &f81(std::hash<wchar_t> &a0, const std::hash<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::hash<wchar_t> &f82(std::hash<wchar_t> &a0, std::hash<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t23 = std::hash<bool>;

std::size_t f83(const std::hash<bool> &a0, bool a1) {
  return a0.operator()(a1);
}

std::hash<bool> f84() { return std::hash<bool>(); }

std::hash<bool> f85(const std::hash<bool> &a0) { return std::hash<bool>(a0); }

std::hash<bool> f86(std::hash<bool> &&a0) {
  return std::hash<bool>(std::move(a0));
}

std::hash<bool> &f87(std::hash<bool> &a0, const std::hash<bool> &a1) {
  return a0.operator=(a1);
}

std::hash<bool> &f88(std::hash<bool> &a0, std::hash<bool> &&a1) {
  return a0.operator=(std::move(a1));
}

using t24 = std::hash<unsigned char>;

std::size_t f89(const std::hash<unsigned char> &a0, unsigned char a1) {
  return a0.operator()(a1);
}

std::hash<unsigned char> f90() { return std::hash<unsigned char>(); }

std::hash<unsigned char> f91(const std::hash<unsigned char> &a0) {
  return std::hash<unsigned char>(a0);
}

std::hash<unsigned char> f92(std::hash<unsigned char> &&a0) {
  return std::hash<unsigned char>(std::move(a0));
}

std::hash<unsigned char> &f93(std::hash<unsigned char> &a0,
                              const std::hash<unsigned char> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned char> &f94(std::hash<unsigned char> &a0,
                              std::hash<unsigned char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t25 = std::hash<double>;

std::size_t f95(const std::hash<double> &a0, double a1) {
  return a0.operator()(a1);
}

std::hash<double> f96() { return std::hash<double>(); }

std::hash<double> f97(const std::hash<double> &a0) {
  return std::hash<double>(a0);
}

std::hash<double> f98(std::hash<double> &&a0) {
  return std::hash<double>(std::move(a0));
}

std::hash<double> &f99(std::hash<double> &a0, const std::hash<double> &a1) {
  return a0.operator=(a1);
}

std::hash<double> &f100(std::hash<double> &a0, std::hash<double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t26 = std::hash<float>;

std::size_t f101(const std::hash<float> &a0, float a1) {
  return a0.operator()(a1);
}

std::hash<float> f102() { return std::hash<float>(); }

std::hash<float> f103(const std::hash<float> &a0) {
  return std::hash<float>(a0);
}

std::hash<float> f104(std::hash<float> &&a0) {
  return std::hash<float>(std::move(a0));
}

std::hash<float> &f105(std::hash<float> &a0, const std::hash<float> &a1) {
  return a0.operator=(a1);
}

std::hash<float> &f106(std::hash<float> &a0, std::hash<float> &&a1) {
  return a0.operator=(std::move(a1));
}

using t27 = std::hash<unsigned int>;

std::size_t f107(const std::hash<unsigned int> &a0, unsigned int a1) {
  return a0.operator()(a1);
}

std::hash<unsigned int> f108() { return std::hash<unsigned int>(); }

std::hash<unsigned int> f109(const std::hash<unsigned int> &a0) {
  return std::hash<unsigned int>(a0);
}

std::hash<unsigned int> f110(std::hash<unsigned int> &&a0) {
  return std::hash<unsigned int>(std::move(a0));
}

std::hash<unsigned int> &f111(std::hash<unsigned int> &a0,
                              const std::hash<unsigned int> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned int> &f112(std::hash<unsigned int> &a0,
                              std::hash<unsigned int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t28 = std::hash<unsigned long long>;

std::size_t f113(const std::hash<unsigned long long> &a0,
                 unsigned long long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long long> f114() { return std::hash<unsigned long long>(); }

std::hash<unsigned long long> f115(const std::hash<unsigned long long> &a0) {
  return std::hash<unsigned long long>(a0);
}

std::hash<unsigned long long> f116(std::hash<unsigned long long> &&a0) {
  return std::hash<unsigned long long>(std::move(a0));
}

std::hash<unsigned long long> &f117(std::hash<unsigned long long> &a0,
                                    const std::hash<unsigned long long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long long> &f118(std::hash<unsigned long long> &a0,
                                    std::hash<unsigned long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t29 = std::hash<unsigned long>;

std::size_t f119(const std::hash<unsigned long> &a0, unsigned long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long> f120() { return std::hash<unsigned long>(); }

std::hash<unsigned long> f121(const std::hash<unsigned long> &a0) {
  return std::hash<unsigned long>(a0);
}

std::hash<unsigned long> f122(std::hash<unsigned long> &&a0) {
  return std::hash<unsigned long>(std::move(a0));
}

std::hash<unsigned long> &f123(std::hash<unsigned long> &a0,
                               const std::hash<unsigned long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long> &f124(std::hash<unsigned long> &a0,
                               std::hash<unsigned long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t30 = std::hash<char16_t>;

std::size_t f125(const std::hash<char16_t> &a0, char16_t a1) {
  return a0.operator()(a1);
}

std::hash<char16_t> f126() { return std::hash<char16_t>(); }

std::hash<char16_t> f127(const std::hash<char16_t> &a0) {
  return std::hash<char16_t>(a0);
}

std::hash<char16_t> f128(std::hash<char16_t> &&a0) {
  return std::hash<char16_t>(std::move(a0));
}

std::hash<char16_t> &f129(std::hash<char16_t> &a0,
                          const std::hash<char16_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char16_t> &f130(std::hash<char16_t> &a0, std::hash<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t31 = std::hash<signed char>;

std::size_t f131(const std::hash<signed char> &a0, signed char a1) {
  return a0.operator()(a1);
}

std::hash<signed char> f132() { return std::hash<signed char>(); }

std::hash<signed char> f133(const std::hash<signed char> &a0) {
  return std::hash<signed char>(a0);
}

std::hash<signed char> f134(std::hash<signed char> &&a0) {
  return std::hash<signed char>(std::move(a0));
}

std::hash<signed char> &f135(std::hash<signed char> &a0,
                             const std::hash<signed char> &a1) {
  return a0.operator=(a1);
}

std::hash<signed char> &f136(std::hash<signed char> &a0,
                             std::hash<signed char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::hash<unsigned short>;

std::size_t f137(const std::hash<unsigned short> &a0, unsigned short a1) {
  return a0.operator()(a1);
}

std::hash<unsigned short> f138() { return std::hash<unsigned short>(); }

std::hash<unsigned short> f139(const std::hash<unsigned short> &a0) {
  return std::hash<unsigned short>(a0);
}

std::hash<unsigned short> f140(std::hash<unsigned short> &&a0) {
  return std::hash<unsigned short>(std::move(a0));
}

std::hash<unsigned short> &f141(std::hash<unsigned short> &a0,
                                const std::hash<unsigned short> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned short> &f142(std::hash<unsigned short> &a0,
                                std::hash<unsigned short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t33 = std::hash<char32_t>;

std::size_t f143(const std::hash<char32_t> &a0, char32_t a1) {
  return a0.operator()(a1);
}

std::hash<char32_t> f144() { return std::hash<char32_t>(); }

std::hash<char32_t> f145(const std::hash<char32_t> &a0) {
  return std::hash<char32_t>(a0);
}

std::hash<char32_t> f146(std::hash<char32_t> &&a0) {
  return std::hash<char32_t>(std::move(a0));
}

std::hash<char32_t> &f147(std::hash<char32_t> &a0,
                          const std::hash<char32_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char32_t> &f148(std::hash<char32_t> &a0, std::hash<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}
