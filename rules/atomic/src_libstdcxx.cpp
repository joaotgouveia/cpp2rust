#include <algorithm>
#include <atomic>
#include <utility>

using t4 = std::atomic<char>;

std::atomic<char> f45() { return {}; }

std::atomic<char> f46(char a0) { return {a0}; }

bool f47(const std::atomic<char> &a0) { return a0.is_lock_free(); }

void f48(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f49(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f50(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f51(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f52(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t5 = std::atomic<int>;

std::atomic<int> f53() { return {}; }

std::atomic<int> f54(int a0) { return {a0}; }

bool f55(const std::atomic<int> &a0) { return a0.is_lock_free(); }

void f56(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f57(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f58(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f59(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f60(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t6 = std::atomic<long long>;

std::atomic<long long> f61() { return {}; }

std::atomic<long long> f62(long long a0) { return {a0}; }

bool f63(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

void f64(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f65(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f66(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f67(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f68(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t7 = std::atomic<long>;

std::atomic<long> f69() { return {}; }

std::atomic<long> f70(long a0) { return {a0}; }

bool f71(const std::atomic<long> &a0) { return a0.is_lock_free(); }

void f72(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f73(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f74(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f75(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f76(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t8 = std::atomic<short>;

std::atomic<short> f77() { return {}; }

std::atomic<short> f78(short a0) { return {a0}; }

bool f79(const std::atomic<short> &a0) { return a0.is_lock_free(); }

void f80(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f81(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f82(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f83(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f84(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t9 = std::atomic<wchar_t>;

std::atomic<wchar_t> f85() { return {}; }

std::atomic<wchar_t> f86(wchar_t a0) { return {a0}; }

bool f87(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f88(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f89(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f90(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f91(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f92(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t10 = std::atomic<bool>;

using t11 = std::atomic<bool>::value_type;

std::atomic<bool> f93() { return {}; }

std::atomic<bool> f94(bool a0) { return {a0}; }

bool f95(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f96(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f97(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f98(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f99(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f100(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f101(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f102(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f103(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f104(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t12 = std::atomic<unsigned char>;

std::atomic<unsigned char> f105() { return {}; }

std::atomic<unsigned char> f106(unsigned char a0) { return {a0}; }

bool f107(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

void f108(std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f109(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f110(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f111(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f112(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t13 = std::atomic<unsigned int>;

std::atomic<unsigned int> f113() { return {}; }

std::atomic<unsigned int> f114(unsigned int a0) { return {a0}; }

bool f115(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

void f116(std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f117(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f118(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f119(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f120(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t14 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f121() { return {}; }

std::atomic<unsigned long long> f122(unsigned long long a0) { return {a0}; }

bool f123(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f124(std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f125(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f126(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f127(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f128(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t15 = std::atomic<unsigned long>;

std::atomic<unsigned long> f129() { return {}; }

std::atomic<unsigned long> f130(unsigned long a0) { return {a0}; }

bool f131(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

void f132(std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f133(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f134(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f135(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f136(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t16 = std::atomic<char16_t>;

std::atomic<char16_t> f137() { return {}; }

std::atomic<char16_t> f138(char16_t a0) { return {a0}; }

bool f139(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

void f140(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f141(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f142(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f143(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f144(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t17 = std::atomic<signed char>;

std::atomic<signed char> f145() { return {}; }

std::atomic<signed char> f146(signed char a0) { return {a0}; }

bool f147(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

void f148(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f149(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f150(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f151(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f152(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t18 = std::atomic<unsigned short>;

std::atomic<unsigned short> f153() { return {}; }

std::atomic<unsigned short> f154(unsigned short a0) { return {a0}; }

bool f155(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

void f156(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f157(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f158(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f159(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f160(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t19 = std::atomic<char32_t>;

std::atomic<char32_t> f161() { return {}; }

std::atomic<char32_t> f162(char32_t a0) { return {a0}; }

bool f163(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

void f164(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f165(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f166(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f167(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f168(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1> using t20 = typename std::atomic<T1 *>::value_type;

template <typename T1> std::atomic<T1 *> f169() { return {}; }

template <typename T1> std::atomic<T1 *> f170(T1 *a0) { return {a0}; }

template <typename T1> using t21 = typename std::atomic<T1>::value_type;

template <typename T1> std::atomic<T1> f171(T1 a0) { return {a0}; }

template <typename T1> T1 f172(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1>
void f173(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 f174(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f175(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f176(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f177(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f178(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f179(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t22 = std::atomic_bool;

using t23 = std::atomic_char;

using t24 = std::atomic_char16_t;

using t25 = std::atomic_char32_t;

template <typename T1> bool f180(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1> bool f181(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f182(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f183(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> bool f184(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1> bool f185(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f186(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f187(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> T1 f188(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1> T1 f189(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f190(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f191(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

void f192(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f193(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f194(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f195(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f196(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f197(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f198(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f199(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

template <typename T1> void f200(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1> void f201(std::atomic<T1> *a0, T1 a1) {
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

using t41 = std::atomic_llong;

using t42 = std::atomic_long;

using t43 = std::atomic_ptrdiff_t;

using t44 = std::atomic_schar;

using t45 = std::atomic_short;

using t46 = std::atomic_size_t;

template <typename T1> void f202(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1> void f203(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f204(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f205(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
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
