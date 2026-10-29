#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <utility>

using t1 = std::atomic<char>;

std::atomic<char> f1() { return {}; }

std::atomic<char> f2(char a0) { return {a0}; }

char f3(const std::atomic<char> &a0) { return static_cast<char>(a0); }

char f4(const volatile std::atomic<char> &a0) { return static_cast<char>(a0); }

char f5(std::atomic<char> &a0, char a1) { return a0.operator=(a1); }

char f6(volatile std::atomic<char> &a0, char a1) { return a0.operator=(a1); }

char f7(std::atomic<char> &a0, int a1) { return a0.operator++(a1); }

char f8(volatile std::atomic<char> &a0, int a1) { return a0.operator++(a1); }

char f9(std::atomic<char> &a0, int a1) { return a0.operator--(a1); }

char f10(volatile std::atomic<char> &a0, int a1) { return a0.operator--(a1); }

char f11(std::atomic<char> &a0) { return a0.operator++(); }

char f12(volatile std::atomic<char> &a0) { return a0.operator++(); }

char f13(std::atomic<char> &a0) { return a0.operator--(); }

char f14(volatile std::atomic<char> &a0) { return a0.operator--(); }

char f15(std::atomic<char> &a0, char a1) { return a0.operator+=(a1); }

char f16(volatile std::atomic<char> &a0, char a1) { return a0.operator+=(a1); }

char f17(std::atomic<char> &a0, char a1) { return a0.operator-=(a1); }

char f18(volatile std::atomic<char> &a0, char a1) { return a0.operator-=(a1); }

char f19(std::atomic<char> &a0, char a1) { return a0.operator&=(a1); }

char f20(volatile std::atomic<char> &a0, char a1) { return a0.operator&=(a1); }

char f21(std::atomic<char> &a0, char a1) { return a0.operator|=(a1); }

char f22(volatile std::atomic<char> &a0, char a1) { return a0.operator|=(a1); }

char f23(std::atomic<char> &a0, char a1) { return a0.operator^=(a1); }

char f24(volatile std::atomic<char> &a0, char a1) { return a0.operator^=(a1); }

bool f25(const std::atomic<char> &a0) { return a0.is_lock_free(); }

bool f26(const volatile std::atomic<char> &a0) { return a0.is_lock_free(); }

void f27(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f28(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char f29(const std::atomic<char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char f30(const volatile std::atomic<char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char f31(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char f32(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f33(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f34(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f35(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f36(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f37(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f38(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f39(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f40(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f41(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f42(std::atomic<char> &a0) { return a0.notify_one(); }

void f43(std::atomic<char> &a0) { return a0.notify_all(); }

char f44(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char f45(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char f46(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char f47(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char f48(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char f49(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char f50(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char f51(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char f52(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char f53(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f54() { return std::atomic<char>::is_always_lock_free; }

using t2 = std::atomic<long double>;

std::atomic<long double> f55() { return {}; }

std::atomic<long double> f56(long double a0) { return {a0}; }

long double f57(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator=(a1);
}

long double f58(std::atomic<long double> &a0, long double a1) {
  return a0.operator=(a1);
}

bool f59(const volatile std::atomic<long double> &a0) {
  return a0.is_lock_free();
}

bool f60(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f61(volatile std::atomic<long double> &a0, long double a1,
         std::memory_order a2) {
  return a0.store(a1, a2);
}

void f62(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f63(const volatile std::atomic<long double> &a0,
                std::memory_order a1) {
  return a0.load(a1);
}

long double f64(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f65(const volatile std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f66(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f67(volatile std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long double f68(std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f69(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f70(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f71(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f72(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f73(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f74(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f75(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f76(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f77(const std::atomic<long double> &a0, long double a1,
         std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f78(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f79(const std::atomic<long double> &a0) { return a0.notify_all(); }

long double f80(std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long double f81(volatile std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long double f82(std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long double f83(volatile std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long double f84(std::atomic<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

long double f85(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

long double f86(std::atomic<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

long double f87(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

bool f88() { return std::atomic<long double>::is_always_lock_free; }

using t3 = std::atomic<int>;

std::atomic<int> f89() { return {}; }

std::atomic<int> f90(int a0) { return {a0}; }

int f91(const std::atomic<int> &a0) { return static_cast<int>(a0); }

int f92(const volatile std::atomic<int> &a0) { return static_cast<int>(a0); }

int f93(std::atomic<int> &a0, int a1) { return a0.operator=(a1); }

int f94(volatile std::atomic<int> &a0, int a1) { return a0.operator=(a1); }

int f95(std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f96(volatile std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f97(std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f98(volatile std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f99(std::atomic<int> &a0) { return a0.operator++(); }

int f100(volatile std::atomic<int> &a0) { return a0.operator++(); }

int f101(std::atomic<int> &a0) { return a0.operator--(); }

int f102(volatile std::atomic<int> &a0) { return a0.operator--(); }

int f103(std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f104(volatile std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f105(std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f106(volatile std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f107(std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f108(volatile std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f109(std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f110(volatile std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f111(std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

int f112(volatile std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

bool f113(const std::atomic<int> &a0) { return a0.is_lock_free(); }

bool f114(const volatile std::atomic<int> &a0) { return a0.is_lock_free(); }

void f115(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f116(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

int f117(const std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f118(const volatile std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f119(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

int f120(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f121(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f122(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f123(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f124(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f125(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f126(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f127(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f128(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f129(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f130(std::atomic<int> &a0) { return a0.notify_one(); }

void f131(std::atomic<int> &a0) { return a0.notify_all(); }

int f132(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f133(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f134(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f135(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f136(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f137(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f138(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f139(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f140(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

int f141(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f142() { return std::atomic<int>::is_always_lock_free; }

using t4 = std::atomic<long long>;

std::atomic<long long> f143() { return {}; }

std::atomic<long long> f144(long long a0) { return {a0}; }

long long f145(const std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f146(const volatile std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f147(std::atomic<long long> &a0, long long a1) {
  return a0.operator=(a1);
}

long long f148(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator=(a1);
}

long long f149(std::atomic<long long> &a0, int a1) { return a0.operator++(a1); }

long long f150(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator++(a1);
}

long long f151(std::atomic<long long> &a0, int a1) { return a0.operator--(a1); }

long long f152(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator--(a1);
}

long long f153(std::atomic<long long> &a0) { return a0.operator++(); }

long long f154(volatile std::atomic<long long> &a0) { return a0.operator++(); }

long long f155(std::atomic<long long> &a0) { return a0.operator--(); }

long long f156(volatile std::atomic<long long> &a0) { return a0.operator--(); }

long long f157(std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f158(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f159(std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f160(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f161(std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f162(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f163(std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f164(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f165(std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

long long f166(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

bool f167(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

bool f168(const volatile std::atomic<long long> &a0) {
  return a0.is_lock_free();
}

void f169(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f170(volatile std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

long long f171(const std::atomic<long long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long long f172(const volatile std::atomic<long long> &a0,
               std::memory_order a1) {
  return a0.load(a1);
}

long long f173(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long long f174(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f175(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f176(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f177(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f178(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f179(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f180(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f181(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f182(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f183(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f184(std::atomic<long long> &a0) { return a0.notify_one(); }

void f185(std::atomic<long long> &a0) { return a0.notify_all(); }

long long f186(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f187(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f188(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f189(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f190(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f191(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f192(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f193(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f194(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long long f195(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f196() { return std::atomic<long long>::is_always_lock_free; }

using t5 = std::atomic<long>;

std::atomic<long> f197() { return {}; }

std::atomic<long> f198(long a0) { return {a0}; }

long f199(const std::atomic<long> &a0) { return static_cast<long>(a0); }

long f200(const volatile std::atomic<long> &a0) {
  return static_cast<long>(a0);
}

long f201(std::atomic<long> &a0, long a1) { return a0.operator=(a1); }

long f202(volatile std::atomic<long> &a0, long a1) { return a0.operator=(a1); }

long f203(std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f204(volatile std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f205(std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f206(volatile std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f207(std::atomic<long> &a0) { return a0.operator++(); }

long f208(volatile std::atomic<long> &a0) { return a0.operator++(); }

long f209(std::atomic<long> &a0) { return a0.operator--(); }

long f210(volatile std::atomic<long> &a0) { return a0.operator--(); }

long f211(std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f212(volatile std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f213(std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f214(volatile std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f215(std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f216(volatile std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f217(std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f218(volatile std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f219(std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

long f220(volatile std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

bool f221(const std::atomic<long> &a0) { return a0.is_lock_free(); }

bool f222(const volatile std::atomic<long> &a0) { return a0.is_lock_free(); }

void f223(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f224(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long f225(const std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f226(const volatile std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f227(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long f228(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f229(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f230(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f231(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f232(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f233(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f234(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f235(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f236(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f237(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f238(std::atomic<long> &a0) { return a0.notify_one(); }

void f239(std::atomic<long> &a0) { return a0.notify_all(); }

long f240(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f241(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f242(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f243(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f244(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f245(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f246(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f247(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f248(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long f249(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f250() { return std::atomic<long>::is_always_lock_free; }

using t6 = std::atomic<short>;

std::atomic<short> f251() { return {}; }

std::atomic<short> f252(short a0) { return {a0}; }

short f253(const std::atomic<short> &a0) { return static_cast<short>(a0); }

short f254(const volatile std::atomic<short> &a0) {
  return static_cast<short>(a0);
}

short f255(std::atomic<short> &a0, short a1) { return a0.operator=(a1); }

short f256(volatile std::atomic<short> &a0, short a1) {
  return a0.operator=(a1);
}

short f257(std::atomic<short> &a0, int a1) { return a0.operator++(a1); }

short f258(volatile std::atomic<short> &a0, int a1) {
  return a0.operator++(a1);
}

short f259(std::atomic<short> &a0, int a1) { return a0.operator--(a1); }

short f260(volatile std::atomic<short> &a0, int a1) {
  return a0.operator--(a1);
}

short f261(std::atomic<short> &a0) { return a0.operator++(); }

short f262(volatile std::atomic<short> &a0) { return a0.operator++(); }

short f263(std::atomic<short> &a0) { return a0.operator--(); }

short f264(volatile std::atomic<short> &a0) { return a0.operator--(); }

short f265(std::atomic<short> &a0, short a1) { return a0.operator+=(a1); }

short f266(volatile std::atomic<short> &a0, short a1) {
  return a0.operator+=(a1);
}

short f267(std::atomic<short> &a0, short a1) { return a0.operator-=(a1); }

short f268(volatile std::atomic<short> &a0, short a1) {
  return a0.operator-=(a1);
}

short f269(std::atomic<short> &a0, short a1) { return a0.operator&=(a1); }

short f270(volatile std::atomic<short> &a0, short a1) {
  return a0.operator&=(a1);
}

short f271(std::atomic<short> &a0, short a1) { return a0.operator|=(a1); }

short f272(volatile std::atomic<short> &a0, short a1) {
  return a0.operator|=(a1);
}

short f273(std::atomic<short> &a0, short a1) { return a0.operator^=(a1); }

short f274(volatile std::atomic<short> &a0, short a1) {
  return a0.operator^=(a1);
}

bool f275(const std::atomic<short> &a0) { return a0.is_lock_free(); }

bool f276(const volatile std::atomic<short> &a0) { return a0.is_lock_free(); }

void f277(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f278(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

short f279(const std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f280(const volatile std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f281(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

short f282(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f283(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f284(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f285(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f286(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f287(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f288(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f289(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f290(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f291(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f292(std::atomic<short> &a0) { return a0.notify_one(); }

void f293(std::atomic<short> &a0) { return a0.notify_all(); }

short f294(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f295(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f296(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f297(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f298(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f299(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f300(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f301(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f302(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

short f303(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f304() { return std::atomic<short>::is_always_lock_free; }

using t7 = std::atomic<wchar_t>;

std::atomic<wchar_t> f305() { return {}; }

std::atomic<wchar_t> f306(wchar_t a0) { return {a0}; }

wchar_t f307(const std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f308(const volatile std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f309(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator=(a1); }

wchar_t f310(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator=(a1);
}

wchar_t f311(std::atomic<wchar_t> &a0, int a1) { return a0.operator++(a1); }

wchar_t f312(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator++(a1);
}

wchar_t f313(std::atomic<wchar_t> &a0, int a1) { return a0.operator--(a1); }

wchar_t f314(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator--(a1);
}

wchar_t f315(std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f316(volatile std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f317(std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f318(volatile std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f319(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator+=(a1); }

wchar_t f320(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator+=(a1);
}

wchar_t f321(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator-=(a1); }

wchar_t f322(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator-=(a1);
}

wchar_t f323(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator&=(a1); }

wchar_t f324(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator&=(a1);
}

wchar_t f325(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator|=(a1); }

wchar_t f326(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator|=(a1);
}

wchar_t f327(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator^=(a1); }

wchar_t f328(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator^=(a1);
}

bool f329(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

bool f330(const volatile std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f331(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f332(volatile std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

wchar_t f333(const std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f334(const volatile std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f335(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

wchar_t f336(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f337(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f338(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f339(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f340(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f341(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f342(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f343(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f344(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f345(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f346(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f347(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

wchar_t f348(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f349(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f350(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f351(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f352(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f353(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f354(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f355(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f356(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

wchar_t f357(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f358() { return std::atomic<wchar_t>::is_always_lock_free; }

using t8 = std::atomic<bool>;

using t9 = std::atomic<bool>::value_type;

std::atomic<bool> f359() { return {}; }

std::atomic<bool> f360(bool a0) { return {a0}; }

bool f361(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f362(volatile std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f363(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f364(const volatile std::atomic<bool> &a0) {
  return static_cast<bool>(a0);
}

bool f365(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

bool f366(const volatile std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f367(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f368(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f369(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f370(const volatile std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f371(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f372(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f373(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f374(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f375(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f376(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f377(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f378(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f379(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f380(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f381(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f382(std::atomic<bool> &a0) { return a0.notify_one(); }

void f383(std::atomic<bool> &a0) { return a0.notify_all(); }

bool f384() { return std::atomic<bool>::is_always_lock_free; }

using t10 = std::atomic<unsigned char>;

std::atomic<unsigned char> f385() { return {}; }

std::atomic<unsigned char> f386(unsigned char a0) { return {a0}; }

unsigned char f387(const std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f388(const volatile std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f389(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator=(a1);
}

unsigned char f390(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator=(a1);
}

unsigned char f391(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f392(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f393(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f394(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f395(std::atomic<unsigned char> &a0) { return a0.operator++(); }

unsigned char f396(volatile std::atomic<unsigned char> &a0) {
  return a0.operator++();
}

unsigned char f397(std::atomic<unsigned char> &a0) { return a0.operator--(); }

unsigned char f398(volatile std::atomic<unsigned char> &a0) {
  return a0.operator--();
}

unsigned char f399(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f400(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f401(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f402(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f403(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f404(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f405(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f406(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f407(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

unsigned char f408(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

bool f409(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

bool f410(const volatile std::atomic<unsigned char> &a0) {
  return a0.is_lock_free();
}

void f411(std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f412(volatile std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned char f413(const std::atomic<unsigned char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f414(const volatile std::atomic<unsigned char> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f415(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned char f416(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f417(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f418(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f419(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f420(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f421(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f422(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f423(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f424(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f425(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f426(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f427(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

unsigned char f428(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f429(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f430(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f431(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f432(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f433(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f434(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f435(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f436(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned char f437(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f438() { return std::atomic<unsigned char>::is_always_lock_free; }

using t11 = std::atomic<double>;

std::atomic<double> f439() { return {}; }

std::atomic<double> f440(double a0) { return {a0}; }

double f441(volatile std::atomic<double> &a0, double a1) {
  return a0.operator=(a1);
}

double f442(std::atomic<double> &a0, double a1) { return a0.operator=(a1); }

bool f443(const volatile std::atomic<double> &a0) { return a0.is_lock_free(); }

bool f444(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f445(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f446(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f447(const volatile std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f448(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f449(const volatile std::atomic<double> &a0) {
  return static_cast<double>(a0);
}

double f450(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f451(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

double f452(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f453(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f454(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f455(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f456(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f457(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f458(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f459(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f460(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f461(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f462(const std::atomic<double> &a0) { return a0.notify_one(); }

void f463(const std::atomic<double> &a0) { return a0.notify_all(); }

double f464(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f465(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f466(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f467(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f468(std::atomic<double> &a0, double a1) { return a0.operator+=(a1); }

double f469(volatile std::atomic<double> &a0, double a1) {
  return a0.operator+=(a1);
}

double f470(std::atomic<double> &a0, double a1) { return a0.operator-=(a1); }

double f471(volatile std::atomic<double> &a0, double a1) {
  return a0.operator-=(a1);
}

bool f472() { return std::atomic<double>::is_always_lock_free; }

using t12 = std::atomic<float>;

std::atomic<float> f473() { return {}; }

std::atomic<float> f474(float a0) { return {a0}; }

float f475(volatile std::atomic<float> &a0, float a1) {
  return a0.operator=(a1);
}

float f476(std::atomic<float> &a0, float a1) { return a0.operator=(a1); }

bool f477(const volatile std::atomic<float> &a0) { return a0.is_lock_free(); }

bool f478(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f479(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f480(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f481(const volatile std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f482(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f483(const volatile std::atomic<float> &a0) {
  return static_cast<float>(a0);
}

float f484(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f485(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

float f486(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f487(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f488(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f489(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f490(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f491(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f492(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f493(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f494(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f495(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f496(const std::atomic<float> &a0) { return a0.notify_one(); }

void f497(const std::atomic<float> &a0) { return a0.notify_all(); }

float f498(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f499(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f500(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f501(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f502(std::atomic<float> &a0, float a1) { return a0.operator+=(a1); }

float f503(volatile std::atomic<float> &a0, float a1) {
  return a0.operator+=(a1);
}

float f504(std::atomic<float> &a0, float a1) { return a0.operator-=(a1); }

float f505(volatile std::atomic<float> &a0, float a1) {
  return a0.operator-=(a1);
}

bool f506() { return std::atomic<float>::is_always_lock_free; }

using t13 = std::atomic<unsigned int>;

std::atomic<unsigned int> f507() { return {}; }

std::atomic<unsigned int> f508(unsigned int a0) { return {a0}; }

unsigned int f509(const std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f510(const volatile std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f511(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator=(a1);
}

unsigned int f512(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator=(a1);
}

unsigned int f513(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f514(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f515(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f516(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f517(std::atomic<unsigned int> &a0) { return a0.operator++(); }

unsigned int f518(volatile std::atomic<unsigned int> &a0) {
  return a0.operator++();
}

unsigned int f519(std::atomic<unsigned int> &a0) { return a0.operator--(); }

unsigned int f520(volatile std::atomic<unsigned int> &a0) {
  return a0.operator--();
}

unsigned int f521(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f522(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f523(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f524(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f525(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f526(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f527(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f528(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f529(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

unsigned int f530(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

bool f531(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

bool f532(const volatile std::atomic<unsigned int> &a0) {
  return a0.is_lock_free();
}

void f533(std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f534(volatile std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned int f535(const std::atomic<unsigned int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f536(const volatile std::atomic<unsigned int> &a0,
                  std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f537(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned int f538(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f539(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f540(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f541(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f542(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f543(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f544(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f545(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f546(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f547(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f548(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f549(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

unsigned int f550(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f551(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f552(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f553(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f554(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f555(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f556(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f557(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f558(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned int f559(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f560() { return std::atomic<unsigned int>::is_always_lock_free; }

using t14 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f561() { return {}; }

std::atomic<unsigned long long> f562(unsigned long long a0) { return {a0}; }

unsigned long long f563(const std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f564(const volatile std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f565(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator=(a1);
}

unsigned long long f566(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator=(a1);
}

unsigned long long f567(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f568(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f569(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f570(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f571(std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f572(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f573(std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f574(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f575(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f576(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f577(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f578(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f579(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f580(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f581(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f582(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f583(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

unsigned long long f584(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

bool f585(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

bool f586(const volatile std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f587(std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f588(volatile std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long long f589(const std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f590(const volatile std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f591(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long long f592(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f593(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f594(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f595(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f596(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f597(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f598(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f599(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f600(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f601(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f602(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f603(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

unsigned long long f604(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f605(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f606(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f607(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f608(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f609(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f610(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f611(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f612(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long long f613(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f614() { return std::atomic<unsigned long long>::is_always_lock_free; }

using t15 = std::atomic<unsigned long>;

std::atomic<unsigned long> f615() { return {}; }

std::atomic<unsigned long> f616(unsigned long a0) { return {a0}; }

unsigned long f617(const std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f618(const volatile std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f619(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator=(a1);
}

unsigned long f620(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator=(a1);
}

unsigned long f621(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f622(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f623(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f624(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f625(std::atomic<unsigned long> &a0) { return a0.operator++(); }

unsigned long f626(volatile std::atomic<unsigned long> &a0) {
  return a0.operator++();
}

unsigned long f627(std::atomic<unsigned long> &a0) { return a0.operator--(); }

unsigned long f628(volatile std::atomic<unsigned long> &a0) {
  return a0.operator--();
}

unsigned long f629(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f630(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f631(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f632(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f633(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f634(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f635(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f636(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f637(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

unsigned long f638(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

bool f639(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

bool f640(const volatile std::atomic<unsigned long> &a0) {
  return a0.is_lock_free();
}

void f641(std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f642(volatile std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long f643(const std::atomic<unsigned long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f644(const volatile std::atomic<unsigned long> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f645(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long f646(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f647(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f648(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f649(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f650(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f651(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f652(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f653(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f654(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f655(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f656(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f657(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

unsigned long f658(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f659(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f660(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f661(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f662(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f663(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f664(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f665(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f666(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long f667(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f668() { return std::atomic<unsigned long>::is_always_lock_free; }

using t16 = std::atomic<char16_t>;

std::atomic<char16_t> f669() { return {}; }

std::atomic<char16_t> f670(char16_t a0) { return {a0}; }

char16_t f671(const std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f672(const volatile std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f673(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator=(a1);
}

char16_t f674(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator=(a1);
}

char16_t f675(std::atomic<char16_t> &a0, int a1) { return a0.operator++(a1); }

char16_t f676(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator++(a1);
}

char16_t f677(std::atomic<char16_t> &a0, int a1) { return a0.operator--(a1); }

char16_t f678(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator--(a1);
}

char16_t f679(std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f680(volatile std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f681(std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f682(volatile std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f683(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f684(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f685(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f686(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f687(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f688(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f689(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f690(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f691(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

char16_t f692(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

bool f693(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

bool f694(const volatile std::atomic<char16_t> &a0) {
  return a0.is_lock_free();
}

void f695(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f696(volatile std::atomic<char16_t> &a0, char16_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char16_t f697(const std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f698(const volatile std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f699(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char16_t f700(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f701(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f702(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f703(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f704(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f705(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f706(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f707(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f708(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f709(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f710(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f711(std::atomic<char16_t> &a0) { return a0.notify_all(); }

char16_t f712(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f713(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f714(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f715(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f716(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f717(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f718(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f719(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f720(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char16_t f721(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f722() { return std::atomic<char16_t>::is_always_lock_free; }

using t17 = std::atomic<signed char>;

std::atomic<signed char> f723() { return {}; }

std::atomic<signed char> f724(signed char a0) { return {a0}; }

signed char f725(const std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f726(const volatile std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f727(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator=(a1);
}

signed char f728(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator=(a1);
}

signed char f729(std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f730(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f731(std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f732(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f733(std::atomic<signed char> &a0) { return a0.operator++(); }

signed char f734(volatile std::atomic<signed char> &a0) {
  return a0.operator++();
}

signed char f735(std::atomic<signed char> &a0) { return a0.operator--(); }

signed char f736(volatile std::atomic<signed char> &a0) {
  return a0.operator--();
}

signed char f737(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f738(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f739(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f740(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f741(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f742(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f743(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f744(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f745(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

signed char f746(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

bool f747(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

bool f748(const volatile std::atomic<signed char> &a0) {
  return a0.is_lock_free();
}

void f749(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f750(volatile std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

signed char f751(const std::atomic<signed char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

signed char f752(const volatile std::atomic<signed char> &a0,
                 std::memory_order a1) {
  return a0.load(a1);
}

signed char f753(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

signed char f754(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f755(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f756(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f757(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f758(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f759(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f760(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f761(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f762(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f763(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f764(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f765(std::atomic<signed char> &a0) { return a0.notify_all(); }

signed char f766(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f767(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f768(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f769(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f770(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f771(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f772(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f773(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f774(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

signed char f775(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f776() { return std::atomic<signed char>::is_always_lock_free; }

using t18 = std::atomic<unsigned short>;

std::atomic<unsigned short> f777() { return {}; }

std::atomic<unsigned short> f778(unsigned short a0) { return {a0}; }

unsigned short f779(const std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f780(const volatile std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f781(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator=(a1);
}

unsigned short f782(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator=(a1);
}

unsigned short f783(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f784(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f785(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f786(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f787(std::atomic<unsigned short> &a0) { return a0.operator++(); }

unsigned short f788(volatile std::atomic<unsigned short> &a0) {
  return a0.operator++();
}

unsigned short f789(std::atomic<unsigned short> &a0) { return a0.operator--(); }

unsigned short f790(volatile std::atomic<unsigned short> &a0) {
  return a0.operator--();
}

unsigned short f791(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f792(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f793(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f794(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f795(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f796(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f797(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f798(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f799(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator^=(a1);
}

unsigned short f800(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator^=(a1);
}

bool f801(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

bool f802(const volatile std::atomic<unsigned short> &a0) {
  return a0.is_lock_free();
}

void f803(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f804(volatile std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned short f805(const std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f806(const volatile std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f807(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned short f808(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f809(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f810(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f811(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f812(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f813(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f814(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f815(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f816(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f817(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f818(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f819(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

unsigned short f820(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f821(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f822(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f823(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f824(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f825(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f826(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f827(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f828(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned short f829(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f830() { return std::atomic<unsigned short>::is_always_lock_free; }

using t19 = std::atomic<char8_t>;

std::atomic<char8_t> f831() { return {}; }

std::atomic<char8_t> f832(char8_t a0) { return {a0}; }

char8_t f833(const std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f834(const volatile std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f835(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator=(a1); }

char8_t f836(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator=(a1);
}

char8_t f837(std::atomic<char8_t> &a0, int a1) { return a0.operator++(a1); }

char8_t f838(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator++(a1);
}

char8_t f839(std::atomic<char8_t> &a0, int a1) { return a0.operator--(a1); }

char8_t f840(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator--(a1);
}

char8_t f841(std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f842(volatile std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f843(std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f844(volatile std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f845(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator+=(a1); }

char8_t f846(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator+=(a1);
}

char8_t f847(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator-=(a1); }

char8_t f848(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator-=(a1);
}

char8_t f849(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator&=(a1); }

char8_t f850(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator&=(a1);
}

char8_t f851(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator|=(a1); }

char8_t f852(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator|=(a1);
}

char8_t f853(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator^=(a1); }

char8_t f854(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator^=(a1);
}

bool f855(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

bool f856(const volatile std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f857(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f858(volatile std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char8_t f859(const std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f860(const volatile std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f861(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char8_t f862(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f863(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f864(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f865(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f866(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f867(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f868(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f869(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f870(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f871(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f872(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f873(std::atomic<char8_t> &a0) { return a0.notify_all(); }

char8_t f874(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f875(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f876(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f877(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f878(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f879(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f880(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f881(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f882(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char8_t f883(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f884() { return std::atomic<char8_t>::is_always_lock_free; }

using t20 = std::atomic<char32_t>;

std::atomic<char32_t> f885() { return {}; }

std::atomic<char32_t> f886(char32_t a0) { return {a0}; }

char32_t f887(const std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f888(const volatile std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f889(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator=(a1);
}

char32_t f890(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator=(a1);
}

char32_t f891(std::atomic<char32_t> &a0, int a1) { return a0.operator++(a1); }

char32_t f892(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator++(a1);
}

char32_t f893(std::atomic<char32_t> &a0, int a1) { return a0.operator--(a1); }

char32_t f894(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator--(a1);
}

char32_t f895(std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f896(volatile std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f897(std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f898(volatile std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f899(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f900(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f901(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f902(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f903(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f904(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f905(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f906(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f907(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

char32_t f908(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

bool f909(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

bool f910(const volatile std::atomic<char32_t> &a0) {
  return a0.is_lock_free();
}

void f911(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f912(volatile std::atomic<char32_t> &a0, char32_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char32_t f913(const std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f914(const volatile std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f915(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char32_t f916(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f917(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f918(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f919(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f920(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f921(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f922(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f923(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f924(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f925(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f926(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f927(std::atomic<char32_t> &a0) { return a0.notify_all(); }

char32_t f928(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f929(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f930(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f931(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f932(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f933(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f934(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f935(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f936(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char32_t f937(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f938() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> using t21 = std::atomic<T1 *>;

template <typename T1> using t22 = typename std::atomic<T1 *>::value_type;

template <typename T1> using t23 = typename std::atomic<T1 *>::difference_type;

template <typename T1> std::atomic<T1 *> f939() { return {}; }

template <typename T1> std::atomic<T1 *> f940(T1 *a0) { return {a0}; }

template <typename T1> T1 *f941(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f942(const volatile std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f943(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f944(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f945(std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f946(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f947(std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f948(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f949(std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f950(volatile std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f951(std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f952(volatile std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f953(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f954(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 *f955(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1>
T1 *f956(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f957(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f958(const volatile std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f959(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f960(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f961(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f962(const volatile std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f963(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 *f964(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f965(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f966(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f967(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f968(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f969(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f970(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f971(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f972(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f973(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f974(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f975(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 *f976(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f977(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
         std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f978(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 *f979(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
         std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> bool f980() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> using t24 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t25 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f981(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f982() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f983(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f984(std::shared_ptr<T1> a0) {
  return {std::forward<std::shared_ptr<T1>>(a0)};
}

template <typename T1>
std::shared_ptr<T1> f985(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f986(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f987(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
void f988(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(std::forward<std::shared_ptr<T1>>(a1));
}

template <typename T1>
void f989(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f990(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f991(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3, a4);
}

template <typename T1>
bool f992(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f993(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f994(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2),
                                  a3);
}

template <typename T1>
void f995(const std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1> void f996(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f997(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f998() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t26 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t27 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f999(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f1000() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f1001(std::weak_ptr<T1> a0) {
  return {std::forward<std::weak_ptr<T1>>(a0)};
}

template <typename T1>
std::weak_ptr<T1> f1002(const std::atomic<std::weak_ptr<T1>> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f1003(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f1004(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
           std::memory_order a2) {
  return a0.store(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
void f1005(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(std::forward<std::weak_ptr<T1>>(a1));
}

template <typename T1>
std::weak_ptr<T1> f1006(std::atomic<std::weak_ptr<T1>> &a0,
                        std::weak_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f1007(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                    a4);
}

template <typename T1>
bool f1008(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f1009(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f1010(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3);
}

template <typename T1>
void f1011(const std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
           std::memory_order a2) {
  return a0.wait(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1> void f1012(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1013(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f1014() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t28 = std::atomic<T1>;

template <typename T1> using t29 = typename std::atomic<T1>::value_type;

template <typename T1>
std::atomic<T1> f1015()
  requires(std::is_default_constructible_v<T1>)
{
  return {};
}

template <typename T1> std::atomic<T1> f1016(T1 a0) { return {std::move(a0)}; }

template <typename T1> T1 f1017(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f1018(const volatile std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f1019(std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 f1020(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f1021(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f1022(const volatile std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f1023(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f1024(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f1025(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f1026(const volatile std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f1027(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f1028(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f1029(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1030(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1031(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1032(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1033(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1034(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1035(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f1036(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f1037(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(std::move(a1), a2);
}

template <typename T1> void f1038(std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1039(std::atomic<T1> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f1040() {
  return std::atomic<T1>::is_always_lock_free;
}

using t30 = std::atomic_bool;

using t31 = std::atomic_char;

using t32 = std::atomic_char16_t;

using t33 = std::atomic_char32_t;

using t34 = std::atomic_char8_t;

template <typename T1> bool f1041(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong(a0, a1, std::move(a2));
}

template <typename T1> bool f1042(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong(a0, a1, std::move(a2));
}

template <typename T1>
bool f1043(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit(a0, a1, std::move(a2), a3,
                                                      a4);
}

template <typename T1>
bool f1044(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit(a0, a1, std::move(a2), a3,
                                                      a4);
}

template <typename T1> bool f1045(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak(a0, a1, std::move(a2));
}

template <typename T1> bool f1046(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak(a0, a1, std::move(a2));
}

template <typename T1>
bool f1047(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit(a0, a1, std::move(a2), a3,
                                                    a4);
}

template <typename T1>
bool f1048(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit(a0, a1, std::move(a2), a3,
                                                    a4);
}

template <typename T1> T1 f1049(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange(a0, std::move(a1));
}

template <typename T1> T1 f1050(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange(a0, std::move(a1));
}

template <typename T1>
T1 f1051(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1052(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1053(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1054(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1055(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_add_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1056(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_add_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1057(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1058(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1059(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_sub_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1060(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_sub_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

void f1061(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f1062(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f1063(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f1064(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f1065(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f1066(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f1067(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f1068(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f1069(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f1070(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f1071(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f1072(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f1073(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f1074(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f1075(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f1076(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

template <typename T1> void f1077(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init(a0, std::move(a1));
}

template <typename T1> void f1078(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init(a0, std::move(a1));
}

using t35 = std::atomic_int;

using t36 = std::atomic_int16_t;

using t37 = std::atomic_int32_t;

using t38 = std::atomic_int64_t;

using t39 = std::atomic_int8_t;

using t40 = std::atomic_int_fast16_t;

using t41 = std::atomic_int_fast32_t;

using t42 = std::atomic_int_fast64_t;

using t43 = std::atomic_int_fast8_t;

using t44 = std::atomic_int_least16_t;

using t45 = std::atomic_int_least32_t;

using t46 = std::atomic_int_least64_t;

using t47 = std::atomic_int_least8_t;

using t48 = std::atomic_intmax_t;

using t49 = std::atomic_intptr_t;

template <typename T1> bool f1079(const std::atomic<T1> *a0) {
  return std::atomic_is_lock_free(a0);
}

template <typename T1> bool f1080(const volatile std::atomic<T1> *a0) {
  return std::atomic_is_lock_free(a0);
}

using t50 = std::atomic_llong;

template <typename T1> T1 f1081(const std::atomic<T1> *a0) {
  return std::atomic_load(a0);
}

template <typename T1> T1 f1082(const volatile std::atomic<T1> *a0) {
  return std::atomic_load(a0);
}

template <typename T1>
T1 f1083(const std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit(a0, a1);
}

template <typename T1>
T1 f1084(const volatile std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit(a0, a1);
}

using t51 = std::atomic_long;

template <typename T1> void f1085(std::atomic<T1> *a0) {
  return std::atomic_notify_all(a0);
}

template <typename T1> void f1086(std::atomic<T1> *a0) {
  return std::atomic_notify_one(a0);
}

using t52 = std::atomic_ptrdiff_t;

using t53 = std::atomic_schar;

using t54 = std::atomic_short;

using t55 = std::atomic_signed_lock_free;

using t56 = std::atomic_size_t;

template <typename T1> void f1087(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store(a0, std::move(a1));
}

template <typename T1> void f1088(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store(a0, std::move(a1));
}

template <typename T1>
void f1089(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit(a0, std::move(a1), a2);
}

template <typename T1>
void f1090(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit(a0, std::move(a1), a2);
}

using t57 = std::atomic_uchar;

using t58 = std::atomic_uint;

using t59 = std::atomic_uint16_t;

using t60 = std::atomic_uint32_t;

using t61 = std::atomic_uint64_t;

using t62 = std::atomic_uint8_t;

using t63 = std::atomic_uint_fast16_t;

using t64 = std::atomic_uint_fast32_t;

using t65 = std::atomic_uint_fast64_t;

using t66 = std::atomic_uint_fast8_t;

using t67 = std::atomic_uint_least16_t;

using t68 = std::atomic_uint_least32_t;

using t69 = std::atomic_uint_least64_t;

using t70 = std::atomic_uint_least8_t;

using t71 = std::atomic_uintmax_t;

using t72 = std::atomic_uintptr_t;

using t73 = std::atomic_ullong;

using t74 = std::atomic_ulong;

using t75 = std::atomic_unsigned_lock_free;

using t76 = std::atomic_ushort;

template <typename T1>
void f1091(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1));
}

template <typename T1>
void f1092(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
           std::memory_order a2) {
  return std::atomic_wait_explicit(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1), a2);
}

using t77 = std::atomic_wchar_t;
