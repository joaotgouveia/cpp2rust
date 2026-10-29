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

char f5(std::atomic<char> &a0, int a1) { return a0.operator++(a1); }

char f6(volatile std::atomic<char> &a0, int a1) { return a0.operator++(a1); }

char f7(std::atomic<char> &a0, int a1) { return a0.operator--(a1); }

char f8(volatile std::atomic<char> &a0, int a1) { return a0.operator--(a1); }

char f9(std::atomic<char> &a0) { return a0.operator++(); }

char f10(volatile std::atomic<char> &a0) { return a0.operator++(); }

char f11(std::atomic<char> &a0) { return a0.operator--(); }

char f12(volatile std::atomic<char> &a0) { return a0.operator--(); }

char f13(std::atomic<char> &a0, char a1) { return a0.operator+=(a1); }

char f14(volatile std::atomic<char> &a0, char a1) { return a0.operator+=(a1); }

char f15(std::atomic<char> &a0, char a1) { return a0.operator-=(a1); }

char f16(volatile std::atomic<char> &a0, char a1) { return a0.operator-=(a1); }

char f17(std::atomic<char> &a0, char a1) { return a0.operator&=(a1); }

char f18(volatile std::atomic<char> &a0, char a1) { return a0.operator&=(a1); }

char f19(std::atomic<char> &a0, char a1) { return a0.operator|=(a1); }

char f20(volatile std::atomic<char> &a0, char a1) { return a0.operator|=(a1); }

char f21(std::atomic<char> &a0, char a1) { return a0.operator^=(a1); }

char f22(volatile std::atomic<char> &a0, char a1) { return a0.operator^=(a1); }

bool f23(const std::atomic<char> &a0) { return a0.is_lock_free(); }

bool f24(const volatile std::atomic<char> &a0) { return a0.is_lock_free(); }

void f25(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f26(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char f27(const std::atomic<char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char f28(const volatile std::atomic<char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char f29(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char f30(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f31(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f32(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f33(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f34(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f35(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f36(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f37(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f38(volatile std::atomic<char> &a0, char &a1, char a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f39(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f40(std::atomic<char> &a0) { return a0.notify_one(); }

void f41(std::atomic<char> &a0) { return a0.notify_all(); }

char f42(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char f43(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char f44(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char f45(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char f46(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char f47(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char f48(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char f49(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char f50(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char f51(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f52() { return std::atomic<char>::is_always_lock_free; }

using t2 = std::atomic<long double>;

std::atomic<long double> f53() { return {}; }

std::atomic<long double> f54(long double a0) { return {a0}; }

bool f55(const volatile std::atomic<long double> &a0) {
  return a0.is_lock_free();
}

bool f56(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f57(volatile std::atomic<long double> &a0, long double a1,
         std::memory_order a2) {
  return a0.store(a1, a2);
}

void f58(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f59(const volatile std::atomic<long double> &a0,
                std::memory_order a1) {
  return a0.load(a1);
}

long double f60(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f61(const volatile std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f62(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f63(volatile std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long double f64(std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f65(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f66(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f67(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f68(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f69(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f70(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f71(std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f72(volatile std::atomic<long double> &a0, long double &a1, long double a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f73(const std::atomic<long double> &a0, long double a1,
         std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f74(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f75(const std::atomic<long double> &a0) { return a0.notify_all(); }

long double f76(std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long double f77(volatile std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long double f78(std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long double f79(volatile std::atomic<long double> &a0, long double a1,
                std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long double f80(std::atomic<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

long double f81(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

long double f82(std::atomic<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

long double f83(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

using t3 = std::atomic<int>;

std::atomic<int> f84() { return {}; }

std::atomic<int> f85(int a0) { return {a0}; }

int f86(const std::atomic<int> &a0) { return static_cast<int>(a0); }

int f87(const volatile std::atomic<int> &a0) { return static_cast<int>(a0); }

int f88(std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f89(volatile std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f90(std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f91(volatile std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f92(std::atomic<int> &a0) { return a0.operator++(); }

int f93(volatile std::atomic<int> &a0) { return a0.operator++(); }

int f94(std::atomic<int> &a0) { return a0.operator--(); }

int f95(volatile std::atomic<int> &a0) { return a0.operator--(); }

int f96(std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f97(volatile std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f98(std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f99(volatile std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f100(std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f101(volatile std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f102(std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f103(volatile std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f104(std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

int f105(volatile std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

bool f106(const std::atomic<int> &a0) { return a0.is_lock_free(); }

bool f107(const volatile std::atomic<int> &a0) { return a0.is_lock_free(); }

void f108(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f109(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

int f110(const std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f111(const volatile std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f112(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

int f113(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f114(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f115(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f116(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f117(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f118(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f119(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f120(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f121(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f122(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f123(std::atomic<int> &a0) { return a0.notify_one(); }

void f124(std::atomic<int> &a0) { return a0.notify_all(); }

int f125(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f126(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f127(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f128(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f129(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f130(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f131(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f132(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f133(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

int f134(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f135() { return std::atomic<int>::is_always_lock_free; }

using t4 = std::atomic<long long>;

std::atomic<long long> f136() { return {}; }

std::atomic<long long> f137(long long a0) { return {a0}; }

long long f138(const std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f139(const volatile std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f140(std::atomic<long long> &a0, int a1) { return a0.operator++(a1); }

long long f141(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator++(a1);
}

long long f142(std::atomic<long long> &a0, int a1) { return a0.operator--(a1); }

long long f143(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator--(a1);
}

long long f144(std::atomic<long long> &a0) { return a0.operator++(); }

long long f145(volatile std::atomic<long long> &a0) { return a0.operator++(); }

long long f146(std::atomic<long long> &a0) { return a0.operator--(); }

long long f147(volatile std::atomic<long long> &a0) { return a0.operator--(); }

long long f148(std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f149(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f150(std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f151(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f152(std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f153(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f154(std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f155(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f156(std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

long long f157(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

bool f158(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

bool f159(const volatile std::atomic<long long> &a0) {
  return a0.is_lock_free();
}

void f160(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f161(volatile std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

long long f162(const std::atomic<long long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long long f163(const volatile std::atomic<long long> &a0,
               std::memory_order a1) {
  return a0.load(a1);
}

long long f164(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long long f165(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f166(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f167(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f168(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f169(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f170(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f171(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f172(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f173(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f174(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f175(std::atomic<long long> &a0) { return a0.notify_one(); }

void f176(std::atomic<long long> &a0) { return a0.notify_all(); }

long long f177(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f178(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f179(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f180(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f181(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f182(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f183(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f184(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f185(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long long f186(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f187() { return std::atomic<long long>::is_always_lock_free; }

using t5 = std::atomic<long>;

std::atomic<long> f188() { return {}; }

std::atomic<long> f189(long a0) { return {a0}; }

long f190(const std::atomic<long> &a0) { return static_cast<long>(a0); }

long f191(const volatile std::atomic<long> &a0) {
  return static_cast<long>(a0);
}

long f192(std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f193(volatile std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f194(std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f195(volatile std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f196(std::atomic<long> &a0) { return a0.operator++(); }

long f197(volatile std::atomic<long> &a0) { return a0.operator++(); }

long f198(std::atomic<long> &a0) { return a0.operator--(); }

long f199(volatile std::atomic<long> &a0) { return a0.operator--(); }

long f200(std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f201(volatile std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f202(std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f203(volatile std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f204(std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f205(volatile std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f206(std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f207(volatile std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f208(std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

long f209(volatile std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

bool f210(const std::atomic<long> &a0) { return a0.is_lock_free(); }

bool f211(const volatile std::atomic<long> &a0) { return a0.is_lock_free(); }

void f212(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f213(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long f214(const std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f215(const volatile std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f216(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long f217(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f218(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f219(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f220(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f221(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f222(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f223(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f224(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f225(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f226(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f227(std::atomic<long> &a0) { return a0.notify_one(); }

void f228(std::atomic<long> &a0) { return a0.notify_all(); }

long f229(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f230(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f231(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f232(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f233(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f234(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f235(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f236(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f237(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long f238(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f239() { return std::atomic<long>::is_always_lock_free; }

using t6 = std::atomic<short>;

std::atomic<short> f240() { return {}; }

std::atomic<short> f241(short a0) { return {a0}; }

short f242(const std::atomic<short> &a0) { return static_cast<short>(a0); }

short f243(const volatile std::atomic<short> &a0) {
  return static_cast<short>(a0);
}

short f244(std::atomic<short> &a0, int a1) { return a0.operator++(a1); }

short f245(volatile std::atomic<short> &a0, int a1) {
  return a0.operator++(a1);
}

short f246(std::atomic<short> &a0, int a1) { return a0.operator--(a1); }

short f247(volatile std::atomic<short> &a0, int a1) {
  return a0.operator--(a1);
}

short f248(std::atomic<short> &a0) { return a0.operator++(); }

short f249(volatile std::atomic<short> &a0) { return a0.operator++(); }

short f250(std::atomic<short> &a0) { return a0.operator--(); }

short f251(volatile std::atomic<short> &a0) { return a0.operator--(); }

short f252(std::atomic<short> &a0, short a1) { return a0.operator+=(a1); }

short f253(volatile std::atomic<short> &a0, short a1) {
  return a0.operator+=(a1);
}

short f254(std::atomic<short> &a0, short a1) { return a0.operator-=(a1); }

short f255(volatile std::atomic<short> &a0, short a1) {
  return a0.operator-=(a1);
}

short f256(std::atomic<short> &a0, short a1) { return a0.operator&=(a1); }

short f257(volatile std::atomic<short> &a0, short a1) {
  return a0.operator&=(a1);
}

short f258(std::atomic<short> &a0, short a1) { return a0.operator|=(a1); }

short f259(volatile std::atomic<short> &a0, short a1) {
  return a0.operator|=(a1);
}

short f260(std::atomic<short> &a0, short a1) { return a0.operator^=(a1); }

short f261(volatile std::atomic<short> &a0, short a1) {
  return a0.operator^=(a1);
}

bool f262(const std::atomic<short> &a0) { return a0.is_lock_free(); }

bool f263(const volatile std::atomic<short> &a0) { return a0.is_lock_free(); }

void f264(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f265(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

short f266(const std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f267(const volatile std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f268(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

short f269(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f270(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f271(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f272(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f273(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f274(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f275(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f276(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f277(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f278(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f279(std::atomic<short> &a0) { return a0.notify_one(); }

void f280(std::atomic<short> &a0) { return a0.notify_all(); }

short f281(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f282(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f283(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f284(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f285(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f286(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f287(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f288(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f289(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

short f290(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f291() { return std::atomic<short>::is_always_lock_free; }

using t7 = std::atomic<wchar_t>;

std::atomic<wchar_t> f292() { return {}; }

std::atomic<wchar_t> f293(wchar_t a0) { return {a0}; }

wchar_t f294(const std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f295(const volatile std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f296(std::atomic<wchar_t> &a0, int a1) { return a0.operator++(a1); }

wchar_t f297(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator++(a1);
}

wchar_t f298(std::atomic<wchar_t> &a0, int a1) { return a0.operator--(a1); }

wchar_t f299(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator--(a1);
}

wchar_t f300(std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f301(volatile std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f302(std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f303(volatile std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f304(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator+=(a1); }

wchar_t f305(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator+=(a1);
}

wchar_t f306(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator-=(a1); }

wchar_t f307(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator-=(a1);
}

wchar_t f308(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator&=(a1); }

wchar_t f309(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator&=(a1);
}

wchar_t f310(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator|=(a1); }

wchar_t f311(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator|=(a1);
}

wchar_t f312(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator^=(a1); }

wchar_t f313(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator^=(a1);
}

bool f314(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

bool f315(const volatile std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f316(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f317(volatile std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

wchar_t f318(const std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f319(const volatile std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f320(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

wchar_t f321(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f322(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f323(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f324(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f325(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f326(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f327(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f328(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f329(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f330(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f331(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f332(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

wchar_t f333(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f334(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f335(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f336(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f337(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f338(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f339(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f340(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f341(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

wchar_t f342(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f343() { return std::atomic<wchar_t>::is_always_lock_free; }

using t8 = std::atomic<bool>;

using t9 = std::atomic<bool>::value_type;

std::atomic<bool> f344() { return {}; }

std::atomic<bool> f345(bool a0) { return {a0}; }

bool f346(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f347(volatile std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f348(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f349(const volatile std::atomic<bool> &a0) {
  return static_cast<bool>(a0);
}

bool f350(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

bool f351(const volatile std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f352(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f353(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f354(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f355(const volatile std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f356(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f357(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f358(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f359(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f360(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f361(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f362(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f363(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f364(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f365(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f366(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f367(std::atomic<bool> &a0) { return a0.notify_one(); }

void f368(std::atomic<bool> &a0) { return a0.notify_all(); }

bool f369() { return std::atomic<bool>::is_always_lock_free; }

using t10 = std::atomic<unsigned char>;

std::atomic<unsigned char> f370() { return {}; }

std::atomic<unsigned char> f371(unsigned char a0) { return {a0}; }

unsigned char f372(const std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f373(const volatile std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f374(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f375(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f376(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f377(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f378(std::atomic<unsigned char> &a0) { return a0.operator++(); }

unsigned char f379(volatile std::atomic<unsigned char> &a0) {
  return a0.operator++();
}

unsigned char f380(std::atomic<unsigned char> &a0) { return a0.operator--(); }

unsigned char f381(volatile std::atomic<unsigned char> &a0) {
  return a0.operator--();
}

unsigned char f382(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f383(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f384(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f385(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f386(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f387(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f388(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f389(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f390(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

unsigned char f391(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

bool f392(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

bool f393(const volatile std::atomic<unsigned char> &a0) {
  return a0.is_lock_free();
}

void f394(std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f395(volatile std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned char f396(const std::atomic<unsigned char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f397(const volatile std::atomic<unsigned char> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f398(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned char f399(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f400(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f401(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f402(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f403(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f404(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f405(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f406(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f407(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f408(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f409(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f410(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

unsigned char f411(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f412(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f413(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f414(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f415(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f416(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f417(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f418(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f419(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned char f420(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f421() { return std::atomic<unsigned char>::is_always_lock_free; }

using t11 = std::atomic<double>;

std::atomic<double> f422() { return {}; }

std::atomic<double> f423(double a0) { return {a0}; }

bool f424(const volatile std::atomic<double> &a0) { return a0.is_lock_free(); }

bool f425(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f426(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f427(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f428(const volatile std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f429(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f430(const volatile std::atomic<double> &a0) {
  return static_cast<double>(a0);
}

double f431(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f432(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

double f433(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f434(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f435(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f436(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f437(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f438(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f439(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f440(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f441(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f442(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f443(const std::atomic<double> &a0) { return a0.notify_one(); }

void f444(const std::atomic<double> &a0) { return a0.notify_all(); }

double f445(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f446(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f447(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f448(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f449(std::atomic<double> &a0, double a1) { return a0.operator+=(a1); }

double f450(volatile std::atomic<double> &a0, double a1) {
  return a0.operator+=(a1);
}

double f451(std::atomic<double> &a0, double a1) { return a0.operator-=(a1); }

double f452(volatile std::atomic<double> &a0, double a1) {
  return a0.operator-=(a1);
}

using t12 = std::atomic<float>;

std::atomic<float> f453() { return {}; }

std::atomic<float> f454(float a0) { return {a0}; }

bool f455(const volatile std::atomic<float> &a0) { return a0.is_lock_free(); }

bool f456(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f457(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f458(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f459(const volatile std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f460(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f461(const volatile std::atomic<float> &a0) {
  return static_cast<float>(a0);
}

float f462(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f463(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

float f464(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f465(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f466(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f467(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f468(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f469(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f470(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f471(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f472(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f473(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f474(const std::atomic<float> &a0) { return a0.notify_one(); }

void f475(const std::atomic<float> &a0) { return a0.notify_all(); }

float f476(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f477(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f478(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f479(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f480(std::atomic<float> &a0, float a1) { return a0.operator+=(a1); }

float f481(volatile std::atomic<float> &a0, float a1) {
  return a0.operator+=(a1);
}

float f482(std::atomic<float> &a0, float a1) { return a0.operator-=(a1); }

float f483(volatile std::atomic<float> &a0, float a1) {
  return a0.operator-=(a1);
}

using t13 = std::atomic<unsigned int>;

std::atomic<unsigned int> f484() { return {}; }

std::atomic<unsigned int> f485(unsigned int a0) { return {a0}; }

unsigned int f486(const std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f487(const volatile std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f488(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f489(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f490(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f491(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f492(std::atomic<unsigned int> &a0) { return a0.operator++(); }

unsigned int f493(volatile std::atomic<unsigned int> &a0) {
  return a0.operator++();
}

unsigned int f494(std::atomic<unsigned int> &a0) { return a0.operator--(); }

unsigned int f495(volatile std::atomic<unsigned int> &a0) {
  return a0.operator--();
}

unsigned int f496(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f497(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f498(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f499(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f500(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f501(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f502(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f503(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f504(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

unsigned int f505(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

bool f506(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

bool f507(const volatile std::atomic<unsigned int> &a0) {
  return a0.is_lock_free();
}

void f508(std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f509(volatile std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned int f510(const std::atomic<unsigned int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f511(const volatile std::atomic<unsigned int> &a0,
                  std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f512(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned int f513(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f514(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f515(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f516(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f517(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f518(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f519(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f520(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f521(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f522(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f523(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f524(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

unsigned int f525(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f526(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f527(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f528(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f529(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f530(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f531(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f532(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f533(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned int f534(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f535() { return std::atomic<unsigned int>::is_always_lock_free; }

using t14 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f536() { return {}; }

std::atomic<unsigned long long> f537(unsigned long long a0) { return {a0}; }

unsigned long long f538(const std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f539(const volatile std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f540(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f541(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f542(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f543(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f544(std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f545(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f546(std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f547(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f548(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f549(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f550(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f551(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f552(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f553(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f554(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f555(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f556(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

unsigned long long f557(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

bool f558(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

bool f559(const volatile std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f560(std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f561(volatile std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long long f562(const std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f563(const volatile std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f564(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long long f565(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f566(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f567(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f568(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f569(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f570(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f571(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f572(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f573(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f574(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f575(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f576(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

unsigned long long f577(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f578(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f579(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f580(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f581(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f582(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f583(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f584(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f585(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long long f586(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f587() { return std::atomic<unsigned long long>::is_always_lock_free; }

using t15 = std::atomic<unsigned long>;

std::atomic<unsigned long> f588() { return {}; }

std::atomic<unsigned long> f589(unsigned long a0) { return {a0}; }

unsigned long f590(const std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f591(const volatile std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f592(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f593(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f594(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f595(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f596(std::atomic<unsigned long> &a0) { return a0.operator++(); }

unsigned long f597(volatile std::atomic<unsigned long> &a0) {
  return a0.operator++();
}

unsigned long f598(std::atomic<unsigned long> &a0) { return a0.operator--(); }

unsigned long f599(volatile std::atomic<unsigned long> &a0) {
  return a0.operator--();
}

unsigned long f600(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f601(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f602(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f603(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f604(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f605(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f606(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f607(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f608(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

unsigned long f609(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

bool f610(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

bool f611(const volatile std::atomic<unsigned long> &a0) {
  return a0.is_lock_free();
}

void f612(std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f613(volatile std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long f614(const std::atomic<unsigned long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f615(const volatile std::atomic<unsigned long> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f616(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long f617(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f618(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f619(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f620(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f621(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f622(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f623(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f624(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f625(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f626(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f627(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f628(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

unsigned long f629(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f630(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f631(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f632(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f633(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f634(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f635(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f636(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f637(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long f638(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f639() { return std::atomic<unsigned long>::is_always_lock_free; }

using t16 = std::atomic<char16_t>;

std::atomic<char16_t> f640() { return {}; }

std::atomic<char16_t> f641(char16_t a0) { return {a0}; }

char16_t f642(const std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f643(const volatile std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f644(std::atomic<char16_t> &a0, int a1) { return a0.operator++(a1); }

char16_t f645(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator++(a1);
}

char16_t f646(std::atomic<char16_t> &a0, int a1) { return a0.operator--(a1); }

char16_t f647(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator--(a1);
}

char16_t f648(std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f649(volatile std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f650(std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f651(volatile std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f652(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f653(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f654(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f655(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f656(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f657(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f658(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f659(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f660(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

char16_t f661(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

bool f662(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

bool f663(const volatile std::atomic<char16_t> &a0) {
  return a0.is_lock_free();
}

void f664(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f665(volatile std::atomic<char16_t> &a0, char16_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char16_t f666(const std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f667(const volatile std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f668(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char16_t f669(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f670(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f671(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f672(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f673(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f674(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f675(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f676(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f677(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f678(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f679(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f680(std::atomic<char16_t> &a0) { return a0.notify_all(); }

char16_t f681(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f682(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f683(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f684(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f685(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f686(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f687(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f688(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f689(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char16_t f690(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f691() { return std::atomic<char16_t>::is_always_lock_free; }

using t17 = std::atomic<signed char>;

std::atomic<signed char> f692() { return {}; }

std::atomic<signed char> f693(signed char a0) { return {a0}; }

signed char f694(const std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f695(const volatile std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f696(std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f697(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f698(std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f699(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f700(std::atomic<signed char> &a0) { return a0.operator++(); }

signed char f701(volatile std::atomic<signed char> &a0) {
  return a0.operator++();
}

signed char f702(std::atomic<signed char> &a0) { return a0.operator--(); }

signed char f703(volatile std::atomic<signed char> &a0) {
  return a0.operator--();
}

signed char f704(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f705(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f706(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f707(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f708(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f709(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f710(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f711(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f712(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

signed char f713(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

bool f714(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

bool f715(const volatile std::atomic<signed char> &a0) {
  return a0.is_lock_free();
}

void f716(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f717(volatile std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

signed char f718(const std::atomic<signed char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

signed char f719(const volatile std::atomic<signed char> &a0,
                 std::memory_order a1) {
  return a0.load(a1);
}

signed char f720(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

signed char f721(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f722(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f723(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f724(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f725(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f726(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f727(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f728(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f729(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f730(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f731(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f732(std::atomic<signed char> &a0) { return a0.notify_all(); }

signed char f733(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f734(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f735(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f736(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f737(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f738(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f739(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f740(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f741(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

signed char f742(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f743() { return std::atomic<signed char>::is_always_lock_free; }

using t18 = std::atomic<unsigned short>;

std::atomic<unsigned short> f744() { return {}; }

std::atomic<unsigned short> f745(unsigned short a0) { return {a0}; }

unsigned short f746(const std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f747(const volatile std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f748(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f749(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f750(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f751(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f752(std::atomic<unsigned short> &a0) { return a0.operator++(); }

unsigned short f753(volatile std::atomic<unsigned short> &a0) {
  return a0.operator++();
}

unsigned short f754(std::atomic<unsigned short> &a0) { return a0.operator--(); }

unsigned short f755(volatile std::atomic<unsigned short> &a0) {
  return a0.operator--();
}

unsigned short f756(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f757(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f758(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f759(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f760(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f761(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f762(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f763(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f764(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator^=(a1);
}

unsigned short f765(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator^=(a1);
}

bool f766(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

bool f767(const volatile std::atomic<unsigned short> &a0) {
  return a0.is_lock_free();
}

void f768(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f769(volatile std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned short f770(const std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f771(const volatile std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f772(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned short f773(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f774(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f775(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f776(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f777(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f778(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f779(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f780(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f781(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f782(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f783(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f784(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

unsigned short f785(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f786(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f787(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f788(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f789(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f790(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f791(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f792(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f793(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned short f794(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f795() { return std::atomic<unsigned short>::is_always_lock_free; }

using t19 = std::atomic<char8_t>;

std::atomic<char8_t> f796() { return {}; }

std::atomic<char8_t> f797(char8_t a0) { return {a0}; }

char8_t f798(const std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f799(const volatile std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f800(std::atomic<char8_t> &a0, int a1) { return a0.operator++(a1); }

char8_t f801(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator++(a1);
}

char8_t f802(std::atomic<char8_t> &a0, int a1) { return a0.operator--(a1); }

char8_t f803(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator--(a1);
}

char8_t f804(std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f805(volatile std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f806(std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f807(volatile std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f808(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator+=(a1); }

char8_t f809(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator+=(a1);
}

char8_t f810(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator-=(a1); }

char8_t f811(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator-=(a1);
}

char8_t f812(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator&=(a1); }

char8_t f813(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator&=(a1);
}

char8_t f814(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator|=(a1); }

char8_t f815(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator|=(a1);
}

char8_t f816(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator^=(a1); }

char8_t f817(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator^=(a1);
}

bool f818(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

bool f819(const volatile std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f820(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f821(volatile std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char8_t f822(const std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f823(const volatile std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f824(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char8_t f825(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f826(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f827(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f828(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f829(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f830(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f831(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f832(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f833(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f834(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f835(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f836(std::atomic<char8_t> &a0) { return a0.notify_all(); }

char8_t f837(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f838(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f839(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f840(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f841(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f842(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f843(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f844(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f845(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char8_t f846(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f847() { return std::atomic<char8_t>::is_always_lock_free; }

using t20 = std::atomic<char32_t>;

std::atomic<char32_t> f848() { return {}; }

std::atomic<char32_t> f849(char32_t a0) { return {a0}; }

char32_t f850(const std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f851(const volatile std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f852(std::atomic<char32_t> &a0, int a1) { return a0.operator++(a1); }

char32_t f853(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator++(a1);
}

char32_t f854(std::atomic<char32_t> &a0, int a1) { return a0.operator--(a1); }

char32_t f855(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator--(a1);
}

char32_t f856(std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f857(volatile std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f858(std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f859(volatile std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f860(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f861(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f862(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f863(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f864(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f865(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f866(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f867(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f868(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

char32_t f869(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

bool f870(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

bool f871(const volatile std::atomic<char32_t> &a0) {
  return a0.is_lock_free();
}

void f872(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f873(volatile std::atomic<char32_t> &a0, char32_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char32_t f874(const std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f875(const volatile std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f876(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char32_t f877(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f878(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f879(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f880(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f881(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f882(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f883(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f884(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f885(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f886(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f887(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f888(std::atomic<char32_t> &a0) { return a0.notify_all(); }

char32_t f889(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f890(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f891(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f892(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f893(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f894(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f895(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f896(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f897(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char32_t f898(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f899() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> using t21 = std::atomic<T1 *>;

template <typename T1> using t22 = typename std::atomic<T1 *>::value_type;

template <typename T1> using t23 = typename std::atomic<T1 *>::difference_type;

template <typename T1> std::atomic<T1 *> f900() { return {}; }

template <typename T1> std::atomic<T1 *> f901(T1 *a0) { return {a0}; }

template <typename T1> T1 *f902(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f903(const volatile std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f904(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f905(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f906(std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f907(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f908(std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f909(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f910(std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f911(volatile std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f912(std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f913(volatile std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f914(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f915(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 *f916(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1>
T1 *f917(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f918(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f919(const volatile std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f920(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f921(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f922(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f923(const volatile std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f924(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 *f925(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f926(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f927(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f928(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f929(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f930(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f931(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f932(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f933(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f934(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f935(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f936(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 *f937(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f938(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
         std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f939(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 *f940(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
         std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> bool f941() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> using t24 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t25 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f942(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f943() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f944(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f945(std::shared_ptr<T1> a0) {
  return {std::forward<std::shared_ptr<T1>>(a0)};
}

template <typename T1>
std::shared_ptr<T1> f946(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f947(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f948(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
void f949(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(std::forward<std::shared_ptr<T1>>(a1));
}

template <typename T1>
void f950(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f951(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f952(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3, a4);
}

template <typename T1>
bool f953(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f954(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f955(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2),
                                  a3);
}

template <typename T1>
void f956(const std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1> void f957(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f958(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f959() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t26 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t27 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f960(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f961() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f962(std::weak_ptr<T1> a0) {
  return {std::forward<std::weak_ptr<T1>>(a0)};
}

template <typename T1>
std::weak_ptr<T1> f963(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f964(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f965(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
void f966(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(std::forward<std::weak_ptr<T1>>(a1));
}

template <typename T1>
std::weak_ptr<T1> f967(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f968(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                    a4);
}

template <typename T1>
bool f969(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f970(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f971(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3);
}

template <typename T1>
void f972(const std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1> void f973(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f974(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f975() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t28 = std::atomic<T1>;

template <typename T1> using t29 = typename std::atomic<T1>::value_type;

template <typename T1>
std::atomic<T1> f976()
  requires(std::is_default_constructible_v<T1>)
{
  return {};
}

template <typename T1> std::atomic<T1> f977(T1 a0) { return {std::move(a0)}; }

template <typename T1> T1 f978(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f979(const volatile std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f980(std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 f981(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f982(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f983(const volatile std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f984(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f985(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f986(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f987(const volatile std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f988(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f989(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f990(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f991(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f992(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f993(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f994(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f995(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f996(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f997(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f998(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(std::move(a1), a2);
}

template <typename T1> void f999(std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1000(std::atomic<T1> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f1001() {
  return std::atomic<T1>::is_always_lock_free;
}

using t30 = std::atomic_bool;

using t31 = std::atomic_char;

using t32 = std::atomic_char16_t;

using t33 = std::atomic_char32_t;

using t34 = std::atomic_char8_t;

template <typename T1> bool f1002(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, std::move(a2));
}

template <typename T1> bool f1003(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f1004(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, std::move(a2),
                                                          a3, a4);
}

template <typename T1>
bool f1005(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, std::move(a2),
                                                          a3, a4);
}

template <typename T1> bool f1006(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, std::move(a2));
}

template <typename T1> bool f1007(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f1008(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, std::move(a2),
                                                        a3, a4);
}

template <typename T1>
bool f1009(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, std::move(a2),
                                                        a3, a4);
}

template <typename T1> T1 f1010(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, std::move(a1));
}

template <typename T1> T1 f1011(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, std::move(a1));
}

template <typename T1>
T1 f1012(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1013(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1014(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1015(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1016(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1017(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1018(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1019(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1020(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1021(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

void f1022(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f1023(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f1024(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f1025(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f1026(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f1027(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f1028(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f1029(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f1030(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f1031(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f1032(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f1033(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f1034(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f1035(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f1036(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f1037(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

template <typename T1> void f1038(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, std::move(a1));
}

template <typename T1> void f1039(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, std::move(a1));
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

template <typename T1> bool f1040(const std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> bool f1041(const volatile std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

using t50 = std::atomic_llong;

template <typename T1> T1 f1042(const std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1> T1 f1043(const volatile std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1>
T1 f1044(const std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1>
T1 f1045(const volatile std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

using t51 = std::atomic_long;

template <typename T1> void f1046(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f1047(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

using t52 = std::atomic_ptrdiff_t;

using t53 = std::atomic_schar;

using t54 = std::atomic_short;

using t55 = std::atomic_signed_lock_free;

using t56 = std::atomic_size_t;

template <typename T1> void f1048(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, std::move(a1));
}

template <typename T1> void f1049(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, std::move(a1));
}

template <typename T1>
void f1050(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1>
void f1051(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, std::move(a1), a2);
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
void f1052(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1));
}

template <typename T1>
void f1053(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
           std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1), a2);
}

using t77 = std::atomic_wchar_t;
