#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <utility>

using t1 = std::atomic<char>;

std::atomic<char> f1() { return {}; }

std::atomic<char> f2(char a0) { return {a0}; }

bool f3(const std::atomic<char> &a0) { return a0.is_lock_free(); }

void f4(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f5(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
        std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f6(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f7(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
        std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f8(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t2 = std::atomic<int>;

std::atomic<int> f9() { return {}; }

std::atomic<int> f10(int a0) { return {a0}; }

bool f11(const std::atomic<int> &a0) { return a0.is_lock_free(); }

void f12(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f13(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f14(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f15(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f16(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t3 = std::atomic<long long>;

std::atomic<long long> f17() { return {}; }

std::atomic<long long> f18(long long a0) { return {a0}; }

bool f19(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

void f20(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f21(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f22(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f23(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f24(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t4 = std::atomic<long>;

std::atomic<long> f25() { return {}; }

std::atomic<long> f26(long a0) { return {a0}; }

bool f27(const std::atomic<long> &a0) { return a0.is_lock_free(); }

void f28(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f29(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f30(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f31(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f32(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t5 = std::atomic<short>;

std::atomic<short> f33() { return {}; }

std::atomic<short> f34(short a0) { return {a0}; }

bool f35(const std::atomic<short> &a0) { return a0.is_lock_free(); }

void f36(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f37(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f38(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f39(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f40(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t6 = std::atomic<wchar_t>;

std::atomic<wchar_t> f41() { return {}; }

std::atomic<wchar_t> f42(wchar_t a0) { return {a0}; }

bool f43(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f44(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f45(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f46(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f47(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f48(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t7 = std::atomic<bool>;

using t8 = std::atomic<bool>::value_type;

std::atomic<bool> f49() { return {}; }

std::atomic<bool> f50(bool a0) { return {a0}; }

bool f51(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f52(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f53(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f54(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f55(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f56(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f57(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f58(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f59(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f60(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t9 = std::atomic<unsigned char>;

std::atomic<unsigned char> f61() { return {}; }

std::atomic<unsigned char> f62(unsigned char a0) { return {a0}; }

bool f63(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

void f64(std::atomic<unsigned char> &a0, unsigned char a1,
         std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f65(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f66(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f67(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f68(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t10 = std::atomic<unsigned int>;

std::atomic<unsigned int> f69() { return {}; }

std::atomic<unsigned int> f70(unsigned int a0) { return {a0}; }

bool f71(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

void f72(std::atomic<unsigned int> &a0, unsigned int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f73(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f74(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f75(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f76(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t11 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f77() { return {}; }

std::atomic<unsigned long long> f78(unsigned long long a0) { return {a0}; }

bool f79(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f80(std::atomic<unsigned long long> &a0, unsigned long long a1,
         std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f81(std::atomic<unsigned long long> &a0, unsigned long long &a1,
         unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f82(std::atomic<unsigned long long> &a0, unsigned long long &a1,
         unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f83(std::atomic<unsigned long long> &a0, unsigned long long &a1,
         unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f84(std::atomic<unsigned long long> &a0, unsigned long long &a1,
         unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t12 = std::atomic<unsigned long>;

std::atomic<unsigned long> f85() { return {}; }

std::atomic<unsigned long> f86(unsigned long a0) { return {a0}; }

bool f87(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

void f88(std::atomic<unsigned long> &a0, unsigned long a1,
         std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f89(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f90(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f91(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f92(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t13 = std::atomic<char16_t>;

std::atomic<char16_t> f93() { return {}; }

std::atomic<char16_t> f94(char16_t a0) { return {a0}; }

bool f95(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

void f96(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f97(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f98(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f99(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f100(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t14 = std::atomic<signed char>;

std::atomic<signed char> f101() { return {}; }

std::atomic<signed char> f102(signed char a0) { return {a0}; }

bool f103(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

void f104(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f105(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f106(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f107(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f108(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t15 = std::atomic<unsigned short>;

std::atomic<unsigned short> f109() { return {}; }

std::atomic<unsigned short> f110(unsigned short a0) { return {a0}; }

bool f111(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

void f112(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f113(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f114(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f115(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f116(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t16 = std::atomic<char32_t>;

std::atomic<char32_t> f117() { return {}; }

std::atomic<char32_t> f118(char32_t a0) { return {a0}; }

bool f119(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

void f120(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f121(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f122(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f123(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f124(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1> using t17 = std::atomic<T1 *>;

template <typename T1> using t18 = typename std::atomic<T1 *>::value_type;

template <typename T1> using t19 = typename std::atomic<T1 *>::difference_type;

template <typename T1> std::atomic<T1 *> f125() { return {}; }

template <typename T1> std::atomic<T1 *> f126(T1 *a0) { return {a0}; }

template <typename T1> T1 *f127(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f128(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f129(std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f130(std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f131(std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f132(std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f133(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 *f134(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f135(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f136(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f137(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f138(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f139(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f140(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f141(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f142(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
T1 *f143(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f144(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> using t20 = std::atomic<T1>;

template <typename T1> using t21 = typename std::atomic<T1>::value_type;

template <typename T1> std::atomic<T1> f145() { return {}; }

template <typename T1> std::atomic<T1> f146(T1 a0) { return {a0}; }

template <typename T1> T1 f147(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f148(std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1> bool f149(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f150(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 f151(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f152(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f153(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f154(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f155(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f156(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t22 = std::atomic_bool;

using t23 = std::atomic_char;

using t24 = std::atomic_char16_t;

using t25 = std::atomic_char32_t;

template <typename T1> bool f157(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1> bool f158(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f159(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f160(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> bool f161(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1> bool f162(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f163(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f164(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> T1 f165(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1> T1 f166(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f167(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f168(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f169(volatile std::atomic<T1> *a0,
        typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(a0, a1);
}

template <typename T1>
T1 f170(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(a0, a1);
}

template <typename T1>
T1 f171(volatile std::atomic<T1> *a0,
        typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f172(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f173(volatile std::atomic<T1> *a0,
        typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(a0, a1);
}

template <typename T1>
T1 f174(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(a0, a1);
}

template <typename T1>
T1 f175(volatile std::atomic<T1> *a0,
        typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f176(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(a0, a1, a2);
}

void f177(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f178(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f179(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f180(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f181(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f182(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f183(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f184(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

template <typename T1> void f185(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1> void f186(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, a1);
}

using t26 = std::atomic_int;

using t27 = std::atomic_int16_t;

using t28 = std::atomic_int32_t;

using t29 = std::atomic_int64_t;

using t30 = std::atomic_int8_t;

using t31 = std::atomic_int_fast16_t;

using t32 = std::atomic_int_fast32_t;

using t33 = std::atomic_int_fast64_t;

using t34 = std::atomic_int_fast8_t;

using t35 = std::atomic_int_least16_t;

using t36 = std::atomic_int_least32_t;

using t37 = std::atomic_int_least64_t;

using t38 = std::atomic_int_least8_t;

using t39 = std::atomic_intmax_t;

using t40 = std::atomic_intptr_t;

template <typename T1> bool f187(const std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> bool f188(const volatile std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

using t41 = std::atomic_llong;

template <typename T1> T1 f189(const std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1> T1 f190(const volatile std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1>
T1 f191(const std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1>
T1 f192(const volatile std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

using t42 = std::atomic_long;

using t43 = std::atomic_ptrdiff_t;

using t44 = std::atomic_schar;

using t45 = std::atomic_short;

using t46 = std::atomic_size_t;

template <typename T1> void f193(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1> void f194(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f195(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f196(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

using t47 = std::atomic_uchar;

using t48 = std::atomic_uint;

using t49 = std::atomic_uint16_t;

using t50 = std::atomic_uint32_t;

using t51 = std::atomic_uint64_t;

using t52 = std::atomic_uint8_t;

using t53 = std::atomic_uint_fast16_t;

using t54 = std::atomic_uint_fast32_t;

using t55 = std::atomic_uint_fast64_t;

using t56 = std::atomic_uint_fast8_t;

using t57 = std::atomic_uint_least16_t;

using t58 = std::atomic_uint_least32_t;

using t59 = std::atomic_uint_least64_t;

using t60 = std::atomic_uint_least8_t;

using t61 = std::atomic_uintmax_t;

using t62 = std::atomic_uintptr_t;

using t63 = std::atomic_ullong;

using t64 = std::atomic_ulong;

using t65 = std::atomic_ushort;

using t66 = std::atomic_wchar_t;
