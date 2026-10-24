#include <algorithm>
#include <atomic>
#include <utility>

using t4 = std::atomic<char>;

std::atomic<char> f64() { return {}; }

std::atomic<char> f65(char a0) { return {a0}; }

bool f66(const std::atomic<char> &a0) { return a0.is_lock_free(); }

bool f67(const volatile std::atomic<char> &a0) { return a0.is_lock_free(); }

void f68(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f69(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f70(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f71(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f72(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f73(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f74(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f75(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f76(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f77(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t5 = std::atomic<int>;

std::atomic<int> f78() { return {}; }

std::atomic<int> f79(int a0) { return {a0}; }

bool f80(const std::atomic<int> &a0) { return a0.is_lock_free(); }

bool f81(const volatile std::atomic<int> &a0) { return a0.is_lock_free(); }

void f82(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f83(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f84(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f85(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f86(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f87(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f88(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f89(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f90(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f91(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t6 = std::atomic<long long>;

std::atomic<long long> f92() { return {}; }

std::atomic<long long> f93(long long a0) { return {a0}; }

bool f94(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

bool f95(const volatile std::atomic<long long> &a0) {
  return a0.is_lock_free();
}

void f96(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f97(volatile std::atomic<long long> &a0, long long a1,
         std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f98(std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f99(volatile std::atomic<long long> &a0, long long &a1, long long a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f100(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f101(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f102(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f103(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f104(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f105(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t7 = std::atomic<long>;

std::atomic<long> f106() { return {}; }

std::atomic<long> f107(long a0) { return {a0}; }

bool f108(const std::atomic<long> &a0) { return a0.is_lock_free(); }

bool f109(const volatile std::atomic<long> &a0) { return a0.is_lock_free(); }

void f110(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f111(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f112(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f113(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f114(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f115(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f116(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f117(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f118(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f119(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t8 = std::atomic<short>;

std::atomic<short> f120() { return {}; }

std::atomic<short> f121(short a0) { return {a0}; }

bool f122(const std::atomic<short> &a0) { return a0.is_lock_free(); }

bool f123(const volatile std::atomic<short> &a0) { return a0.is_lock_free(); }

void f124(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f125(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f126(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f127(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f128(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f129(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f130(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f131(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f132(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f133(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t9 = std::atomic<wchar_t>;

std::atomic<wchar_t> f134() { return {}; }

std::atomic<wchar_t> f135(wchar_t a0) { return {a0}; }

bool f136(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

bool f137(const volatile std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f138(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f139(volatile std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f140(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f141(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f142(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f143(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f144(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f145(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f146(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f147(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t10 = std::atomic<bool>;

using t11 = std::atomic<bool>::value_type;

std::atomic<bool> f148() { return {}; }

std::atomic<bool> f149(bool a0) { return {a0}; }

bool f150(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f151(volatile std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f152(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f153(const volatile std::atomic<bool> &a0) {
  return static_cast<bool>(a0);
}

bool f154(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

bool f155(const volatile std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f156(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f157(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f158(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f159(const volatile std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f160(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f161(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f162(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f163(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f164(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f165(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f166(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f167(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f168(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f169(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t12 = std::atomic<unsigned char>;

std::atomic<unsigned char> f170() { return {}; }

std::atomic<unsigned char> f171(unsigned char a0) { return {a0}; }

bool f172(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

bool f173(const volatile std::atomic<unsigned char> &a0) {
  return a0.is_lock_free();
}

void f174(std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f175(volatile std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f176(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f177(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f178(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f179(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f180(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f181(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f182(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f183(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t13 = std::atomic<unsigned int>;

std::atomic<unsigned int> f184() { return {}; }

std::atomic<unsigned int> f185(unsigned int a0) { return {a0}; }

bool f186(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

bool f187(const volatile std::atomic<unsigned int> &a0) {
  return a0.is_lock_free();
}

void f188(std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f189(volatile std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f190(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f191(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f192(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f193(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f194(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f195(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f196(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f197(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t14 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f198() { return {}; }

std::atomic<unsigned long long> f199(unsigned long long a0) { return {a0}; }

bool f200(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

bool f201(const volatile std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f202(std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f203(volatile std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f204(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f205(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f206(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f207(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f208(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f209(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f210(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f211(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t15 = std::atomic<unsigned long>;

std::atomic<unsigned long> f212() { return {}; }

std::atomic<unsigned long> f213(unsigned long a0) { return {a0}; }

bool f214(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

bool f215(const volatile std::atomic<unsigned long> &a0) {
  return a0.is_lock_free();
}

void f216(std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f217(volatile std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f218(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f219(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f220(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f221(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f222(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f223(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f224(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f225(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t16 = std::atomic<char16_t>;

std::atomic<char16_t> f226() { return {}; }

std::atomic<char16_t> f227(char16_t a0) { return {a0}; }

bool f228(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

bool f229(const volatile std::atomic<char16_t> &a0) {
  return a0.is_lock_free();
}

void f230(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f231(volatile std::atomic<char16_t> &a0, char16_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f232(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f233(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f234(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f235(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f236(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f237(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f238(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f239(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t17 = std::atomic<signed char>;

std::atomic<signed char> f240() { return {}; }

std::atomic<signed char> f241(signed char a0) { return {a0}; }

bool f242(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

bool f243(const volatile std::atomic<signed char> &a0) {
  return a0.is_lock_free();
}

void f244(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f245(volatile std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f246(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f247(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f248(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f249(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f250(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f251(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f252(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f253(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t18 = std::atomic<unsigned short>;

std::atomic<unsigned short> f254() { return {}; }

std::atomic<unsigned short> f255(unsigned short a0) { return {a0}; }

bool f256(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

bool f257(const volatile std::atomic<unsigned short> &a0) {
  return a0.is_lock_free();
}

void f258(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f259(volatile std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f260(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f261(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f262(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f263(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f264(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f265(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f266(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f267(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t19 = std::atomic<char32_t>;

std::atomic<char32_t> f268() { return {}; }

std::atomic<char32_t> f269(char32_t a0) { return {a0}; }

bool f270(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

bool f271(const volatile std::atomic<char32_t> &a0) {
  return a0.is_lock_free();
}

void f272(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f273(volatile std::atomic<char32_t> &a0, char32_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f274(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f275(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f276(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f277(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f278(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f279(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f280(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f281(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1> using t20 = typename std::atomic<T1 *>::value_type;

template <typename T1> std::atomic<T1 *> f282() { return {}; }

template <typename T1> std::atomic<T1 *> f283(T1 *a0) { return {a0}; }

template <typename T1> using t21 = typename std::atomic<T1>::value_type;

template <typename T1> std::atomic<T1> f284(T1 a0) { return {a0}; }

template <typename T1> T1 f285(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f286(const volatile std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1>
void f287(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f288(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 f289(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f290(const volatile std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f291(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 f292(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f293(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f294(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f295(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f296(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f297(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f298(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f299(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f300(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t22 = std::atomic_bool;

using t23 = std::atomic_char;

using t24 = std::atomic_char16_t;

using t25 = std::atomic_char32_t;

template <typename T1> bool f301(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1> bool f302(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f303(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f304(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> bool f305(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1> bool f306(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f307(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f308(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> T1 f309(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1> T1 f310(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f311(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f312(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

void f313(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f314(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f315(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f316(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f317(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f318(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f319(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f320(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

template <typename T1> void f321(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1> void f322(std::atomic<T1> *a0, T1 a1) {
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

template <typename T1> void f323(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1> void f324(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f325(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f326(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
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
