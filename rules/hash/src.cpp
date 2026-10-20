#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <future>
#include <string>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t1 = std::hash<std::bitset<0>>;

std::size_t f1(const std::hash<std::bitset<0>> &a0, const std::bitset<0> &a1) {
  return a0.operator()(a1);
}

std::hash<std::bitset<0>> f2() { return std::hash<std::bitset<0>>(); }

std::hash<std::bitset<0>> f3(const std::hash<std::bitset<0>> &a0) {
  return std::hash<std::bitset<0>>(a0);
}

std::hash<std::bitset<0>> f4(std::hash<std::bitset<0>> &&a0) {
  return std::hash<std::bitset<0>>(std::move(a0));
}

std::hash<std::bitset<0>> &f5(std::hash<std::bitset<0>> &a0,
                              const std::hash<std::bitset<0>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::bitset<0>> &f6(std::hash<std::bitset<0>> &a0,
                              std::hash<std::bitset<0>> &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::hash<std::error_code>;

std::size_t f7(const std::hash<std::error_code> &a0,
               const std::error_code &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_code> f8() { return std::hash<std::error_code>(); }

std::hash<std::error_code> f9(const std::hash<std::error_code> &a0) {
  return std::hash<std::error_code>(a0);
}

std::hash<std::error_code> f10(std::hash<std::error_code> &&a0) {
  return std::hash<std::error_code>(std::move(a0));
}

std::hash<std::error_code> &f11(std::hash<std::error_code> &a0,
                                const std::hash<std::error_code> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_code> &f12(std::hash<std::error_code> &a0,
                                std::hash<std::error_code> &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::hash<std::thread::id>;

std::size_t f13(const std::hash<std::thread::id> &a0,
                const std::thread::id &a1) {
  return a0.operator()(a1);
}

std::hash<std::thread::id> f14() { return std::hash<std::thread::id>(); }

std::hash<std::thread::id> f15(const std::hash<std::thread::id> &a0) {
  return std::hash<std::thread::id>(a0);
}

std::hash<std::thread::id> f16(std::hash<std::thread::id> &&a0) {
  return std::hash<std::thread::id>(std::move(a0));
}

std::hash<std::thread::id> &f17(std::hash<std::thread::id> &a0,
                                const std::hash<std::thread::id> &a1) {
  return a0.operator=(a1);
}

std::hash<std::thread::id> &f18(std::hash<std::thread::id> &a0,
                                std::hash<std::thread::id> &&a1) {
  return a0.operator=(std::move(a1));
}

using t4 = std::hash<std::type_index>;

using t5 = std::hash<std::type_index>::result_type;

using t6 = std::hash<std::type_index>::argument_type;

std::size_t f19(const std::hash<std::type_index> &a0,
                const std::type_index &a1) {
  return a0.operator()(a1);
}

std::hash<std::type_index> f20() { return std::hash<std::type_index>(); }

std::hash<std::type_index> f21(const std::hash<std::type_index> &a0) {
  return std::hash<std::type_index>(a0);
}

std::hash<std::type_index> f22(std::hash<std::type_index> &&a0) {
  return std::hash<std::type_index>(std::move(a0));
}

std::hash<std::type_index> &f23(std::hash<std::type_index> &a0,
                                const std::hash<std::type_index> &a1) {
  return a0.operator=(a1);
}

std::hash<std::type_index> &f24(std::hash<std::type_index> &a0,
                                std::hash<std::type_index> &&a1) {
  return a0.operator=(std::move(a1));
}

using t7 = std::hash<char>;

std::size_t f25(const std::hash<char> &a0, char a1) {
  return a0.operator()(a1);
}

std::hash<char> f26() { return std::hash<char>(); }

std::hash<char> f27(const std::hash<char> &a0) { return std::hash<char>(a0); }

std::hash<char> f28(std::hash<char> &&a0) {
  return std::hash<char>(std::move(a0));
}

std::hash<char> &f29(std::hash<char> &a0, const std::hash<char> &a1) {
  return a0.operator=(a1);
}

std::hash<char> &f30(std::hash<char> &a0, std::hash<char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t8 = std::hash<long double>;

std::size_t f31(const std::hash<long double> &a0, long double a1) {
  return a0.operator()(a1);
}

std::hash<long double> f32() { return std::hash<long double>(); }

std::hash<long double> f33(const std::hash<long double> &a0) {
  return std::hash<long double>(a0);
}

std::hash<long double> f34(std::hash<long double> &&a0) {
  return std::hash<long double>(std::move(a0));
}

std::hash<long double> &f35(std::hash<long double> &a0,
                            const std::hash<long double> &a1) {
  return a0.operator=(a1);
}

std::hash<long double> &f36(std::hash<long double> &a0,
                            std::hash<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::hash<int>;

std::size_t f37(const std::hash<int> &a0, int a1) { return a0.operator()(a1); }

std::hash<int> f38() { return std::hash<int>(); }

std::hash<int> f39(const std::hash<int> &a0) { return std::hash<int>(a0); }

std::hash<int> f40(std::hash<int> &&a0) {
  return std::hash<int>(std::move(a0));
}

std::hash<int> &f41(std::hash<int> &a0, const std::hash<int> &a1) {
  return a0.operator=(a1);
}

std::hash<int> &f42(std::hash<int> &a0, std::hash<int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t10 = std::hash<long long>;

std::size_t f43(const std::hash<long long> &a0, long long a1) {
  return a0.operator()(a1);
}

std::hash<long long> f44() { return std::hash<long long>(); }

std::hash<long long> f45(const std::hash<long long> &a0) {
  return std::hash<long long>(a0);
}

std::hash<long long> f46(std::hash<long long> &&a0) {
  return std::hash<long long>(std::move(a0));
}

std::hash<long long> &f47(std::hash<long long> &a0,
                          const std::hash<long long> &a1) {
  return a0.operator=(a1);
}

std::hash<long long> &f48(std::hash<long long> &a0, std::hash<long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t11 = std::hash<long>;

std::size_t f49(const std::hash<long> &a0, long a1) {
  return a0.operator()(a1);
}

std::hash<long> f50() { return std::hash<long>(); }

std::hash<long> f51(const std::hash<long> &a0) { return std::hash<long>(a0); }

std::hash<long> f52(std::hash<long> &&a0) {
  return std::hash<long>(std::move(a0));
}

std::hash<long> &f53(std::hash<long> &a0, const std::hash<long> &a1) {
  return a0.operator=(a1);
}

std::hash<long> &f54(std::hash<long> &a0, std::hash<long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t12 = std::hash<short>;

std::size_t f55(const std::hash<short> &a0, short a1) {
  return a0.operator()(a1);
}

std::hash<short> f56() { return std::hash<short>(); }

std::hash<short> f57(const std::hash<short> &a0) {
  return std::hash<short>(a0);
}

std::hash<short> f58(std::hash<short> &&a0) {
  return std::hash<short>(std::move(a0));
}

std::hash<short> &f59(std::hash<short> &a0, const std::hash<short> &a1) {
  return a0.operator=(a1);
}

std::hash<short> &f60(std::hash<short> &a0, std::hash<short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t13 = std::hash<wchar_t>;

std::size_t f61(const std::hash<wchar_t> &a0, wchar_t a1) {
  return a0.operator()(a1);
}

std::hash<wchar_t> f62() { return std::hash<wchar_t>(); }

std::hash<wchar_t> f63(const std::hash<wchar_t> &a0) {
  return std::hash<wchar_t>(a0);
}

std::hash<wchar_t> f64(std::hash<wchar_t> &&a0) {
  return std::hash<wchar_t>(std::move(a0));
}

std::hash<wchar_t> &f65(std::hash<wchar_t> &a0, const std::hash<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::hash<wchar_t> &f66(std::hash<wchar_t> &a0, std::hash<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t14 = std::hash<bool>;

std::size_t f67(const std::hash<bool> &a0, bool a1) {
  return a0.operator()(a1);
}

std::hash<bool> f68() { return std::hash<bool>(); }

std::hash<bool> f69(const std::hash<bool> &a0) { return std::hash<bool>(a0); }

std::hash<bool> f70(std::hash<bool> &&a0) {
  return std::hash<bool>(std::move(a0));
}

std::hash<bool> &f71(std::hash<bool> &a0, const std::hash<bool> &a1) {
  return a0.operator=(a1);
}

std::hash<bool> &f72(std::hash<bool> &a0, std::hash<bool> &&a1) {
  return a0.operator=(std::move(a1));
}

using t15 = std::hash<unsigned char>;

std::size_t f73(const std::hash<unsigned char> &a0, unsigned char a1) {
  return a0.operator()(a1);
}

std::hash<unsigned char> f74() { return std::hash<unsigned char>(); }

std::hash<unsigned char> f75(const std::hash<unsigned char> &a0) {
  return std::hash<unsigned char>(a0);
}

std::hash<unsigned char> f76(std::hash<unsigned char> &&a0) {
  return std::hash<unsigned char>(std::move(a0));
}

std::hash<unsigned char> &f77(std::hash<unsigned char> &a0,
                              const std::hash<unsigned char> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned char> &f78(std::hash<unsigned char> &a0,
                              std::hash<unsigned char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t16 = std::hash<double>;

std::size_t f79(const std::hash<double> &a0, double a1) {
  return a0.operator()(a1);
}

std::hash<double> f80() { return std::hash<double>(); }

std::hash<double> f81(const std::hash<double> &a0) {
  return std::hash<double>(a0);
}

std::hash<double> f82(std::hash<double> &&a0) {
  return std::hash<double>(std::move(a0));
}

std::hash<double> &f83(std::hash<double> &a0, const std::hash<double> &a1) {
  return a0.operator=(a1);
}

std::hash<double> &f84(std::hash<double> &a0, std::hash<double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t17 = std::hash<float>;

std::size_t f85(const std::hash<float> &a0, float a1) {
  return a0.operator()(a1);
}

std::hash<float> f86() { return std::hash<float>(); }

std::hash<float> f87(const std::hash<float> &a0) {
  return std::hash<float>(a0);
}

std::hash<float> f88(std::hash<float> &&a0) {
  return std::hash<float>(std::move(a0));
}

std::hash<float> &f89(std::hash<float> &a0, const std::hash<float> &a1) {
  return a0.operator=(a1);
}

std::hash<float> &f90(std::hash<float> &a0, std::hash<float> &&a1) {
  return a0.operator=(std::move(a1));
}

using t18 = std::hash<unsigned int>;

std::size_t f91(const std::hash<unsigned int> &a0, unsigned int a1) {
  return a0.operator()(a1);
}

std::hash<unsigned int> f92() { return std::hash<unsigned int>(); }

std::hash<unsigned int> f93(const std::hash<unsigned int> &a0) {
  return std::hash<unsigned int>(a0);
}

std::hash<unsigned int> f94(std::hash<unsigned int> &&a0) {
  return std::hash<unsigned int>(std::move(a0));
}

std::hash<unsigned int> &f95(std::hash<unsigned int> &a0,
                             const std::hash<unsigned int> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned int> &f96(std::hash<unsigned int> &a0,
                             std::hash<unsigned int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t19 = std::hash<unsigned long long>;

std::size_t f97(const std::hash<unsigned long long> &a0,
                unsigned long long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long long> f98() { return std::hash<unsigned long long>(); }

std::hash<unsigned long long> f99(const std::hash<unsigned long long> &a0) {
  return std::hash<unsigned long long>(a0);
}

std::hash<unsigned long long> f100(std::hash<unsigned long long> &&a0) {
  return std::hash<unsigned long long>(std::move(a0));
}

std::hash<unsigned long long> &f101(std::hash<unsigned long long> &a0,
                                    const std::hash<unsigned long long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long long> &f102(std::hash<unsigned long long> &a0,
                                    std::hash<unsigned long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t20 = std::hash<unsigned long>;

std::size_t f103(const std::hash<unsigned long> &a0, unsigned long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long> f104() { return std::hash<unsigned long>(); }

std::hash<unsigned long> f105(const std::hash<unsigned long> &a0) {
  return std::hash<unsigned long>(a0);
}

std::hash<unsigned long> f106(std::hash<unsigned long> &&a0) {
  return std::hash<unsigned long>(std::move(a0));
}

std::hash<unsigned long> &f107(std::hash<unsigned long> &a0,
                               const std::hash<unsigned long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long> &f108(std::hash<unsigned long> &a0,
                               std::hash<unsigned long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t21 = std::hash<char16_t>;

std::size_t f109(const std::hash<char16_t> &a0, char16_t a1) {
  return a0.operator()(a1);
}

std::hash<char16_t> f110() { return std::hash<char16_t>(); }

std::hash<char16_t> f111(const std::hash<char16_t> &a0) {
  return std::hash<char16_t>(a0);
}

std::hash<char16_t> f112(std::hash<char16_t> &&a0) {
  return std::hash<char16_t>(std::move(a0));
}

std::hash<char16_t> &f113(std::hash<char16_t> &a0,
                          const std::hash<char16_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char16_t> &f114(std::hash<char16_t> &a0, std::hash<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t22 = std::hash<signed char>;

std::size_t f115(const std::hash<signed char> &a0, signed char a1) {
  return a0.operator()(a1);
}

std::hash<signed char> f116() { return std::hash<signed char>(); }

std::hash<signed char> f117(const std::hash<signed char> &a0) {
  return std::hash<signed char>(a0);
}

std::hash<signed char> f118(std::hash<signed char> &&a0) {
  return std::hash<signed char>(std::move(a0));
}

std::hash<signed char> &f119(std::hash<signed char> &a0,
                             const std::hash<signed char> &a1) {
  return a0.operator=(a1);
}

std::hash<signed char> &f120(std::hash<signed char> &a0,
                             std::hash<signed char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t23 = std::hash<unsigned short>;

std::size_t f121(const std::hash<unsigned short> &a0, unsigned short a1) {
  return a0.operator()(a1);
}

std::hash<unsigned short> f122() { return std::hash<unsigned short>(); }

std::hash<unsigned short> f123(const std::hash<unsigned short> &a0) {
  return std::hash<unsigned short>(a0);
}

std::hash<unsigned short> f124(std::hash<unsigned short> &&a0) {
  return std::hash<unsigned short>(std::move(a0));
}

std::hash<unsigned short> &f125(std::hash<unsigned short> &a0,
                                const std::hash<unsigned short> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned short> &f126(std::hash<unsigned short> &a0,
                                std::hash<unsigned short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t24 = std::hash<char32_t>;

std::size_t f127(const std::hash<char32_t> &a0, char32_t a1) {
  return a0.operator()(a1);
}

std::hash<char32_t> f128() { return std::hash<char32_t>(); }

std::hash<char32_t> f129(const std::hash<char32_t> &a0) {
  return std::hash<char32_t>(a0);
}

std::hash<char32_t> f130(std::hash<char32_t> &&a0) {
  return std::hash<char32_t>(std::move(a0));
}

std::hash<char32_t> &f131(std::hash<char32_t> &a0,
                          const std::hash<char32_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char32_t> &f132(std::hash<char32_t> &a0, std::hash<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <std::size_t T1> using t25 = std::hash<std::bitset<T1>>;

template <typename T1> using t26 = std::hash<T1 *>;

template <typename T1> std::size_t f133(const std::hash<T1 *> &a0, T1 *a1) {
  return a0.operator()(a1);
}

template <typename T1> using t27 = std::hash<std::shared_ptr<T1>>;

template <typename T1>
std::size_t f134(const std::hash<std::shared_ptr<T1>> &a0,
                 const std::shared_ptr<T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t28 = std::hash<std::vector<bool, T1>>;

template <typename T1>
std::size_t f135(const std::hash<std::vector<bool, T1>> &a0,
                 const std::vector<bool, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t29 = std::hash<T1>;
