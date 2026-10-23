#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <future>
#include <string>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t17 = std::hash<std::bitset<0>>;

std::size_t f33(const std::hash<std::bitset<0>> &a0, const std::bitset<0> &a1) {
  return a0.operator()(a1);
}

std::hash<std::bitset<0>> f34() { return std::hash<std::bitset<0>>(); }

std::hash<std::bitset<0>> f35(const std::hash<std::bitset<0>> &a0) {
  return std::hash<std::bitset<0>>(a0);
}

std::hash<std::bitset<0>> f36(std::hash<std::bitset<0>> &&a0) {
  return std::hash<std::bitset<0>>(std::move(a0));
}

std::hash<std::bitset<0>> &f37(std::hash<std::bitset<0>> &a0,
                               const std::hash<std::bitset<0>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::bitset<0>> &f38(std::hash<std::bitset<0>> &a0,
                               std::hash<std::bitset<0>> &&a1) {
  return a0.operator=(std::move(a1));
}

using t18 = std::hash<std::thread::id>;

std::size_t f39(const std::hash<std::thread::id> &a0,
                const std::thread::id &a1) {
  return a0.operator()(a1);
}

std::hash<std::thread::id> f40() { return std::hash<std::thread::id>(); }

std::hash<std::thread::id> f41(const std::hash<std::thread::id> &a0) {
  return std::hash<std::thread::id>(a0);
}

std::hash<std::thread::id> f42(std::hash<std::thread::id> &&a0) {
  return std::hash<std::thread::id>(std::move(a0));
}

std::hash<std::thread::id> &f43(std::hash<std::thread::id> &a0,
                                const std::hash<std::thread::id> &a1) {
  return a0.operator=(a1);
}

std::hash<std::thread::id> &f44(std::hash<std::thread::id> &a0,
                                std::hash<std::thread::id> &&a1) {
  return a0.operator=(std::move(a1));
}

using t19 = std::hash<std::type_index>::result_type;

using t20 = std::hash<std::type_index>::argument_type;

std::size_t f45(const std::hash<std::type_index> &a0,
                const std::type_index &a1) {
  return a0.operator()(a1);
}

using t21 = std::hash<char>;

std::size_t f46(const std::hash<char> &a0, char a1) {
  return a0.operator()(a1);
}

std::hash<char> f47() { return std::hash<char>(); }

std::hash<char> f48(const std::hash<char> &a0) { return std::hash<char>(a0); }

std::hash<char> f49(std::hash<char> &&a0) {
  return std::hash<char>(std::move(a0));
}

std::hash<char> &f50(std::hash<char> &a0, const std::hash<char> &a1) {
  return a0.operator=(a1);
}

std::hash<char> &f51(std::hash<char> &a0, std::hash<char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t22 = std::hash<long double>;

std::size_t f52(const std::hash<long double> &a0, long double a1) {
  return a0.operator()(a1);
}

std::hash<long double> f53() { return std::hash<long double>(); }

std::hash<long double> f54(const std::hash<long double> &a0) {
  return std::hash<long double>(a0);
}

std::hash<long double> f55(std::hash<long double> &&a0) {
  return std::hash<long double>(std::move(a0));
}

std::hash<long double> &f56(std::hash<long double> &a0,
                            const std::hash<long double> &a1) {
  return a0.operator=(a1);
}

std::hash<long double> &f57(std::hash<long double> &a0,
                            std::hash<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t23 = std::hash<int>;

std::size_t f58(const std::hash<int> &a0, int a1) { return a0.operator()(a1); }

std::hash<int> f59() { return std::hash<int>(); }

std::hash<int> f60(const std::hash<int> &a0) { return std::hash<int>(a0); }

std::hash<int> f61(std::hash<int> &&a0) {
  return std::hash<int>(std::move(a0));
}

std::hash<int> &f62(std::hash<int> &a0, const std::hash<int> &a1) {
  return a0.operator=(a1);
}

std::hash<int> &f63(std::hash<int> &a0, std::hash<int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t24 = std::hash<long long>;

std::size_t f64(const std::hash<long long> &a0, long long a1) {
  return a0.operator()(a1);
}

std::hash<long long> f65() { return std::hash<long long>(); }

std::hash<long long> f66(const std::hash<long long> &a0) {
  return std::hash<long long>(a0);
}

std::hash<long long> f67(std::hash<long long> &&a0) {
  return std::hash<long long>(std::move(a0));
}

std::hash<long long> &f68(std::hash<long long> &a0,
                          const std::hash<long long> &a1) {
  return a0.operator=(a1);
}

std::hash<long long> &f69(std::hash<long long> &a0, std::hash<long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t25 = std::hash<long>;

std::size_t f70(const std::hash<long> &a0, long a1) {
  return a0.operator()(a1);
}

std::hash<long> f71() { return std::hash<long>(); }

std::hash<long> f72(const std::hash<long> &a0) { return std::hash<long>(a0); }

std::hash<long> f73(std::hash<long> &&a0) {
  return std::hash<long>(std::move(a0));
}

std::hash<long> &f74(std::hash<long> &a0, const std::hash<long> &a1) {
  return a0.operator=(a1);
}

std::hash<long> &f75(std::hash<long> &a0, std::hash<long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t26 = std::hash<short>;

std::size_t f76(const std::hash<short> &a0, short a1) {
  return a0.operator()(a1);
}

std::hash<short> f77() { return std::hash<short>(); }

std::hash<short> f78(const std::hash<short> &a0) {
  return std::hash<short>(a0);
}

std::hash<short> f79(std::hash<short> &&a0) {
  return std::hash<short>(std::move(a0));
}

std::hash<short> &f80(std::hash<short> &a0, const std::hash<short> &a1) {
  return a0.operator=(a1);
}

std::hash<short> &f81(std::hash<short> &a0, std::hash<short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t27 = std::hash<wchar_t>;

std::size_t f82(const std::hash<wchar_t> &a0, wchar_t a1) {
  return a0.operator()(a1);
}

std::hash<wchar_t> f83() { return std::hash<wchar_t>(); }

std::hash<wchar_t> f84(const std::hash<wchar_t> &a0) {
  return std::hash<wchar_t>(a0);
}

std::hash<wchar_t> f85(std::hash<wchar_t> &&a0) {
  return std::hash<wchar_t>(std::move(a0));
}

std::hash<wchar_t> &f86(std::hash<wchar_t> &a0, const std::hash<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::hash<wchar_t> &f87(std::hash<wchar_t> &a0, std::hash<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t28 = std::hash<bool>;

std::size_t f88(const std::hash<bool> &a0, bool a1) {
  return a0.operator()(a1);
}

std::hash<bool> f89() { return std::hash<bool>(); }

std::hash<bool> f90(const std::hash<bool> &a0) { return std::hash<bool>(a0); }

std::hash<bool> f91(std::hash<bool> &&a0) {
  return std::hash<bool>(std::move(a0));
}

std::hash<bool> &f92(std::hash<bool> &a0, const std::hash<bool> &a1) {
  return a0.operator=(a1);
}

std::hash<bool> &f93(std::hash<bool> &a0, std::hash<bool> &&a1) {
  return a0.operator=(std::move(a1));
}

using t29 = std::hash<unsigned char>;

std::size_t f94(const std::hash<unsigned char> &a0, unsigned char a1) {
  return a0.operator()(a1);
}

std::hash<unsigned char> f95() { return std::hash<unsigned char>(); }

std::hash<unsigned char> f96(const std::hash<unsigned char> &a0) {
  return std::hash<unsigned char>(a0);
}

std::hash<unsigned char> f97(std::hash<unsigned char> &&a0) {
  return std::hash<unsigned char>(std::move(a0));
}

std::hash<unsigned char> &f98(std::hash<unsigned char> &a0,
                              const std::hash<unsigned char> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned char> &f99(std::hash<unsigned char> &a0,
                              std::hash<unsigned char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t30 = std::hash<double>;

std::size_t f100(const std::hash<double> &a0, double a1) {
  return a0.operator()(a1);
}

std::hash<double> f101() { return std::hash<double>(); }

std::hash<double> f102(const std::hash<double> &a0) {
  return std::hash<double>(a0);
}

std::hash<double> f103(std::hash<double> &&a0) {
  return std::hash<double>(std::move(a0));
}

std::hash<double> &f104(std::hash<double> &a0, const std::hash<double> &a1) {
  return a0.operator=(a1);
}

std::hash<double> &f105(std::hash<double> &a0, std::hash<double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t31 = std::hash<float>;

std::size_t f106(const std::hash<float> &a0, float a1) {
  return a0.operator()(a1);
}

std::hash<float> f107() { return std::hash<float>(); }

std::hash<float> f108(const std::hash<float> &a0) {
  return std::hash<float>(a0);
}

std::hash<float> f109(std::hash<float> &&a0) {
  return std::hash<float>(std::move(a0));
}

std::hash<float> &f110(std::hash<float> &a0, const std::hash<float> &a1) {
  return a0.operator=(a1);
}

std::hash<float> &f111(std::hash<float> &a0, std::hash<float> &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::hash<unsigned int>;

std::size_t f112(const std::hash<unsigned int> &a0, unsigned int a1) {
  return a0.operator()(a1);
}

std::hash<unsigned int> f113() { return std::hash<unsigned int>(); }

std::hash<unsigned int> f114(const std::hash<unsigned int> &a0) {
  return std::hash<unsigned int>(a0);
}

std::hash<unsigned int> f115(std::hash<unsigned int> &&a0) {
  return std::hash<unsigned int>(std::move(a0));
}

std::hash<unsigned int> &f116(std::hash<unsigned int> &a0,
                              const std::hash<unsigned int> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned int> &f117(std::hash<unsigned int> &a0,
                              std::hash<unsigned int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t33 = std::hash<unsigned long long>;

std::size_t f118(const std::hash<unsigned long long> &a0,
                 unsigned long long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long long> f119() { return std::hash<unsigned long long>(); }

std::hash<unsigned long long> f120(const std::hash<unsigned long long> &a0) {
  return std::hash<unsigned long long>(a0);
}

std::hash<unsigned long long> f121(std::hash<unsigned long long> &&a0) {
  return std::hash<unsigned long long>(std::move(a0));
}

std::hash<unsigned long long> &f122(std::hash<unsigned long long> &a0,
                                    const std::hash<unsigned long long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long long> &f123(std::hash<unsigned long long> &a0,
                                    std::hash<unsigned long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t34 = std::hash<unsigned long>;

std::size_t f124(const std::hash<unsigned long> &a0, unsigned long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long> f125() { return std::hash<unsigned long>(); }

std::hash<unsigned long> f126(const std::hash<unsigned long> &a0) {
  return std::hash<unsigned long>(a0);
}

std::hash<unsigned long> f127(std::hash<unsigned long> &&a0) {
  return std::hash<unsigned long>(std::move(a0));
}

std::hash<unsigned long> &f128(std::hash<unsigned long> &a0,
                               const std::hash<unsigned long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long> &f129(std::hash<unsigned long> &a0,
                               std::hash<unsigned long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t35 = std::hash<char16_t>;

std::size_t f130(const std::hash<char16_t> &a0, char16_t a1) {
  return a0.operator()(a1);
}

std::hash<char16_t> f131() { return std::hash<char16_t>(); }

std::hash<char16_t> f132(const std::hash<char16_t> &a0) {
  return std::hash<char16_t>(a0);
}

std::hash<char16_t> f133(std::hash<char16_t> &&a0) {
  return std::hash<char16_t>(std::move(a0));
}

std::hash<char16_t> &f134(std::hash<char16_t> &a0,
                          const std::hash<char16_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char16_t> &f135(std::hash<char16_t> &a0, std::hash<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t36 = std::hash<signed char>;

std::size_t f136(const std::hash<signed char> &a0, signed char a1) {
  return a0.operator()(a1);
}

std::hash<signed char> f137() { return std::hash<signed char>(); }

std::hash<signed char> f138(const std::hash<signed char> &a0) {
  return std::hash<signed char>(a0);
}

std::hash<signed char> f139(std::hash<signed char> &&a0) {
  return std::hash<signed char>(std::move(a0));
}

std::hash<signed char> &f140(std::hash<signed char> &a0,
                             const std::hash<signed char> &a1) {
  return a0.operator=(a1);
}

std::hash<signed char> &f141(std::hash<signed char> &a0,
                             std::hash<signed char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t37 = std::hash<unsigned short>;

std::size_t f142(const std::hash<unsigned short> &a0, unsigned short a1) {
  return a0.operator()(a1);
}

std::hash<unsigned short> f143() { return std::hash<unsigned short>(); }

std::hash<unsigned short> f144(const std::hash<unsigned short> &a0) {
  return std::hash<unsigned short>(a0);
}

std::hash<unsigned short> f145(std::hash<unsigned short> &&a0) {
  return std::hash<unsigned short>(std::move(a0));
}

std::hash<unsigned short> &f146(std::hash<unsigned short> &a0,
                                const std::hash<unsigned short> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned short> &f147(std::hash<unsigned short> &a0,
                                std::hash<unsigned short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t38 = std::hash<char32_t>;

std::size_t f148(const std::hash<char32_t> &a0, char32_t a1) {
  return a0.operator()(a1);
}

std::hash<char32_t> f149() { return std::hash<char32_t>(); }

std::hash<char32_t> f150(const std::hash<char32_t> &a0) {
  return std::hash<char32_t>(a0);
}

std::hash<char32_t> f151(std::hash<char32_t> &&a0) {
  return std::hash<char32_t>(std::move(a0));
}

std::hash<char32_t> &f152(std::hash<char32_t> &a0,
                          const std::hash<char32_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char32_t> &f153(std::hash<char32_t> &a0, std::hash<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
using t39 = std::hash<std::unique_ptr<T1, T2>>;
