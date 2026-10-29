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

bool f84() { return std::atomic<long double>::is_always_lock_free; }

using t3 = std::atomic<int>;

std::atomic<int> f85() { return {}; }

std::atomic<int> f86(int a0) { return {a0}; }

int f87(const std::atomic<int> &a0) { return static_cast<int>(a0); }

int f88(const volatile std::atomic<int> &a0) { return static_cast<int>(a0); }

int f89(std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f90(volatile std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f91(std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f92(volatile std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f93(std::atomic<int> &a0) { return a0.operator++(); }

int f94(volatile std::atomic<int> &a0) { return a0.operator++(); }

int f95(std::atomic<int> &a0) { return a0.operator--(); }

int f96(volatile std::atomic<int> &a0) { return a0.operator--(); }

int f97(std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f98(volatile std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f99(std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f100(volatile std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f101(std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f102(volatile std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f103(std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f104(volatile std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f105(std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

int f106(volatile std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

bool f107(const std::atomic<int> &a0) { return a0.is_lock_free(); }

bool f108(const volatile std::atomic<int> &a0) { return a0.is_lock_free(); }

void f109(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f110(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

int f111(const std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f112(const volatile std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f113(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

int f114(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f115(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f116(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f117(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f118(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f119(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f120(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f121(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f122(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f123(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f124(std::atomic<int> &a0) { return a0.notify_one(); }

void f125(std::atomic<int> &a0) { return a0.notify_all(); }

int f126(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f127(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f128(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f129(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f130(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f131(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f132(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f133(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f134(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

int f135(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f136() { return std::atomic<int>::is_always_lock_free; }

using t4 = std::atomic<long long>;

std::atomic<long long> f137() { return {}; }

std::atomic<long long> f138(long long a0) { return {a0}; }

long long f139(const std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f140(const volatile std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f141(std::atomic<long long> &a0, int a1) { return a0.operator++(a1); }

long long f142(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator++(a1);
}

long long f143(std::atomic<long long> &a0, int a1) { return a0.operator--(a1); }

long long f144(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator--(a1);
}

long long f145(std::atomic<long long> &a0) { return a0.operator++(); }

long long f146(volatile std::atomic<long long> &a0) { return a0.operator++(); }

long long f147(std::atomic<long long> &a0) { return a0.operator--(); }

long long f148(volatile std::atomic<long long> &a0) { return a0.operator--(); }

long long f149(std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f150(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f151(std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f152(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f153(std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f154(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f155(std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f156(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f157(std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

long long f158(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

bool f159(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

bool f160(const volatile std::atomic<long long> &a0) {
  return a0.is_lock_free();
}

void f161(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f162(volatile std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

long long f163(const std::atomic<long long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long long f164(const volatile std::atomic<long long> &a0,
               std::memory_order a1) {
  return a0.load(a1);
}

long long f165(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long long f166(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f167(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f168(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f169(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f170(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f171(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f172(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f173(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f174(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f175(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f176(std::atomic<long long> &a0) { return a0.notify_one(); }

void f177(std::atomic<long long> &a0) { return a0.notify_all(); }

long long f178(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f179(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f180(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f181(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f182(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f183(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f184(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f185(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f186(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long long f187(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f188() { return std::atomic<long long>::is_always_lock_free; }

using t5 = std::atomic<long>;

std::atomic<long> f189() { return {}; }

std::atomic<long> f190(long a0) { return {a0}; }

long f191(const std::atomic<long> &a0) { return static_cast<long>(a0); }

long f192(const volatile std::atomic<long> &a0) {
  return static_cast<long>(a0);
}

long f193(std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f194(volatile std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f195(std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f196(volatile std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f197(std::atomic<long> &a0) { return a0.operator++(); }

long f198(volatile std::atomic<long> &a0) { return a0.operator++(); }

long f199(std::atomic<long> &a0) { return a0.operator--(); }

long f200(volatile std::atomic<long> &a0) { return a0.operator--(); }

long f201(std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f202(volatile std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f203(std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f204(volatile std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f205(std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f206(volatile std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f207(std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f208(volatile std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f209(std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

long f210(volatile std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

bool f211(const std::atomic<long> &a0) { return a0.is_lock_free(); }

bool f212(const volatile std::atomic<long> &a0) { return a0.is_lock_free(); }

void f213(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f214(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long f215(const std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f216(const volatile std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f217(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long f218(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f219(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f220(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f221(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f222(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f223(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f224(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f225(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f226(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f227(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f228(std::atomic<long> &a0) { return a0.notify_one(); }

void f229(std::atomic<long> &a0) { return a0.notify_all(); }

long f230(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f231(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f232(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f233(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f234(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f235(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f236(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f237(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f238(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long f239(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f240() { return std::atomic<long>::is_always_lock_free; }

using t6 = std::atomic<short>;

std::atomic<short> f241() { return {}; }

std::atomic<short> f242(short a0) { return {a0}; }

short f243(const std::atomic<short> &a0) { return static_cast<short>(a0); }

short f244(const volatile std::atomic<short> &a0) {
  return static_cast<short>(a0);
}

short f245(std::atomic<short> &a0, int a1) { return a0.operator++(a1); }

short f246(volatile std::atomic<short> &a0, int a1) {
  return a0.operator++(a1);
}

short f247(std::atomic<short> &a0, int a1) { return a0.operator--(a1); }

short f248(volatile std::atomic<short> &a0, int a1) {
  return a0.operator--(a1);
}

short f249(std::atomic<short> &a0) { return a0.operator++(); }

short f250(volatile std::atomic<short> &a0) { return a0.operator++(); }

short f251(std::atomic<short> &a0) { return a0.operator--(); }

short f252(volatile std::atomic<short> &a0) { return a0.operator--(); }

short f253(std::atomic<short> &a0, short a1) { return a0.operator+=(a1); }

short f254(volatile std::atomic<short> &a0, short a1) {
  return a0.operator+=(a1);
}

short f255(std::atomic<short> &a0, short a1) { return a0.operator-=(a1); }

short f256(volatile std::atomic<short> &a0, short a1) {
  return a0.operator-=(a1);
}

short f257(std::atomic<short> &a0, short a1) { return a0.operator&=(a1); }

short f258(volatile std::atomic<short> &a0, short a1) {
  return a0.operator&=(a1);
}

short f259(std::atomic<short> &a0, short a1) { return a0.operator|=(a1); }

short f260(volatile std::atomic<short> &a0, short a1) {
  return a0.operator|=(a1);
}

short f261(std::atomic<short> &a0, short a1) { return a0.operator^=(a1); }

short f262(volatile std::atomic<short> &a0, short a1) {
  return a0.operator^=(a1);
}

bool f263(const std::atomic<short> &a0) { return a0.is_lock_free(); }

bool f264(const volatile std::atomic<short> &a0) { return a0.is_lock_free(); }

void f265(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f266(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

short f267(const std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f268(const volatile std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f269(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

short f270(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f271(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f272(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f273(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f274(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f275(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f276(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f277(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f278(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f279(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f280(std::atomic<short> &a0) { return a0.notify_one(); }

void f281(std::atomic<short> &a0) { return a0.notify_all(); }

short f282(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f283(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f284(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f285(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f286(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f287(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f288(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f289(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f290(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

short f291(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f292() { return std::atomic<short>::is_always_lock_free; }

using t7 = std::atomic<wchar_t>;

std::atomic<wchar_t> f293() { return {}; }

std::atomic<wchar_t> f294(wchar_t a0) { return {a0}; }

wchar_t f295(const std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f296(const volatile std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f297(std::atomic<wchar_t> &a0, int a1) { return a0.operator++(a1); }

wchar_t f298(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator++(a1);
}

wchar_t f299(std::atomic<wchar_t> &a0, int a1) { return a0.operator--(a1); }

wchar_t f300(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator--(a1);
}

wchar_t f301(std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f302(volatile std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f303(std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f304(volatile std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f305(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator+=(a1); }

wchar_t f306(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator+=(a1);
}

wchar_t f307(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator-=(a1); }

wchar_t f308(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator-=(a1);
}

wchar_t f309(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator&=(a1); }

wchar_t f310(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator&=(a1);
}

wchar_t f311(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator|=(a1); }

wchar_t f312(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator|=(a1);
}

wchar_t f313(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator^=(a1); }

wchar_t f314(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator^=(a1);
}

bool f315(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

bool f316(const volatile std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f317(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f318(volatile std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

wchar_t f319(const std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f320(const volatile std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f321(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

wchar_t f322(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f323(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f324(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f325(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f326(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f327(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f328(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f329(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f330(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f331(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f332(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f333(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

wchar_t f334(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f335(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f336(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f337(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f338(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f339(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f340(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f341(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f342(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

wchar_t f343(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f344() { return std::atomic<wchar_t>::is_always_lock_free; }

using t8 = std::atomic<bool>;

using t9 = std::atomic<bool>::value_type;

std::atomic<bool> f345() { return {}; }

std::atomic<bool> f346(bool a0) { return {a0}; }

bool f347(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f348(volatile std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f349(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f350(const volatile std::atomic<bool> &a0) {
  return static_cast<bool>(a0);
}

bool f351(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

bool f352(const volatile std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f353(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f354(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f355(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f356(const volatile std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f357(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f358(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f359(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f360(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f361(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f362(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f363(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f364(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f365(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f366(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f367(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f368(std::atomic<bool> &a0) { return a0.notify_one(); }

void f369(std::atomic<bool> &a0) { return a0.notify_all(); }

bool f370() { return std::atomic<bool>::is_always_lock_free; }

using t10 = std::atomic<unsigned char>;

std::atomic<unsigned char> f371() { return {}; }

std::atomic<unsigned char> f372(unsigned char a0) { return {a0}; }

unsigned char f373(const std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f374(const volatile std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f375(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f376(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f377(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f378(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f379(std::atomic<unsigned char> &a0) { return a0.operator++(); }

unsigned char f380(volatile std::atomic<unsigned char> &a0) {
  return a0.operator++();
}

unsigned char f381(std::atomic<unsigned char> &a0) { return a0.operator--(); }

unsigned char f382(volatile std::atomic<unsigned char> &a0) {
  return a0.operator--();
}

unsigned char f383(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f384(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f385(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f386(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f387(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f388(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f389(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f390(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f391(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

unsigned char f392(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

bool f393(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

bool f394(const volatile std::atomic<unsigned char> &a0) {
  return a0.is_lock_free();
}

void f395(std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f396(volatile std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned char f397(const std::atomic<unsigned char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f398(const volatile std::atomic<unsigned char> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f399(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned char f400(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f401(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f402(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f403(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f404(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f405(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f406(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f407(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f408(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f409(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f410(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f411(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

unsigned char f412(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f413(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f414(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f415(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f416(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f417(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f418(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f419(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f420(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned char f421(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f422() { return std::atomic<unsigned char>::is_always_lock_free; }

using t11 = std::atomic<double>;

std::atomic<double> f423() { return {}; }

std::atomic<double> f424(double a0) { return {a0}; }

bool f425(const volatile std::atomic<double> &a0) { return a0.is_lock_free(); }

bool f426(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f427(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f428(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f429(const volatile std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f430(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f431(const volatile std::atomic<double> &a0) {
  return static_cast<double>(a0);
}

double f432(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f433(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

double f434(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f435(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f436(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f437(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f438(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f439(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f440(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f441(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f442(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f443(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f444(const std::atomic<double> &a0) { return a0.notify_one(); }

void f445(const std::atomic<double> &a0) { return a0.notify_all(); }

double f446(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f447(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f448(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f449(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f450(std::atomic<double> &a0, double a1) { return a0.operator+=(a1); }

double f451(volatile std::atomic<double> &a0, double a1) {
  return a0.operator+=(a1);
}

double f452(std::atomic<double> &a0, double a1) { return a0.operator-=(a1); }

double f453(volatile std::atomic<double> &a0, double a1) {
  return a0.operator-=(a1);
}

bool f454() { return std::atomic<double>::is_always_lock_free; }

using t12 = std::atomic<float>;

std::atomic<float> f455() { return {}; }

std::atomic<float> f456(float a0) { return {a0}; }

bool f457(const volatile std::atomic<float> &a0) { return a0.is_lock_free(); }

bool f458(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f459(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f460(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f461(const volatile std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f462(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f463(const volatile std::atomic<float> &a0) {
  return static_cast<float>(a0);
}

float f464(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f465(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

float f466(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f467(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f468(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f469(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f470(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f471(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f472(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f473(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f474(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f475(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f476(const std::atomic<float> &a0) { return a0.notify_one(); }

void f477(const std::atomic<float> &a0) { return a0.notify_all(); }

float f478(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f479(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f480(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f481(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f482(std::atomic<float> &a0, float a1) { return a0.operator+=(a1); }

float f483(volatile std::atomic<float> &a0, float a1) {
  return a0.operator+=(a1);
}

float f484(std::atomic<float> &a0, float a1) { return a0.operator-=(a1); }

float f485(volatile std::atomic<float> &a0, float a1) {
  return a0.operator-=(a1);
}

bool f486() { return std::atomic<float>::is_always_lock_free; }

using t13 = std::atomic<unsigned int>;

std::atomic<unsigned int> f487() { return {}; }

std::atomic<unsigned int> f488(unsigned int a0) { return {a0}; }

unsigned int f489(const std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f490(const volatile std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f491(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f492(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f493(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f494(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f495(std::atomic<unsigned int> &a0) { return a0.operator++(); }

unsigned int f496(volatile std::atomic<unsigned int> &a0) {
  return a0.operator++();
}

unsigned int f497(std::atomic<unsigned int> &a0) { return a0.operator--(); }

unsigned int f498(volatile std::atomic<unsigned int> &a0) {
  return a0.operator--();
}

unsigned int f499(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f500(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f501(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f502(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f503(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f504(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f505(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f506(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f507(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

unsigned int f508(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

bool f509(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

bool f510(const volatile std::atomic<unsigned int> &a0) {
  return a0.is_lock_free();
}

void f511(std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f512(volatile std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned int f513(const std::atomic<unsigned int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f514(const volatile std::atomic<unsigned int> &a0,
                  std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f515(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned int f516(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f517(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f518(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f519(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f520(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f521(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f522(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f523(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f524(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f525(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f526(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f527(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

unsigned int f528(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f529(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f530(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f531(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f532(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f533(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f534(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f535(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f536(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned int f537(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f538() { return std::atomic<unsigned int>::is_always_lock_free; }

using t14 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f539() { return {}; }

std::atomic<unsigned long long> f540(unsigned long long a0) { return {a0}; }

unsigned long long f541(const std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f542(const volatile std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f543(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f544(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f545(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f546(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f547(std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f548(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f549(std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f550(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f551(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f552(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f553(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f554(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f555(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f556(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f557(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f558(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f559(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

unsigned long long f560(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

bool f561(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

bool f562(const volatile std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f563(std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f564(volatile std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long long f565(const std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f566(const volatile std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f567(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long long f568(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f569(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f570(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f571(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f572(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f573(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f574(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f575(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f576(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f577(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f578(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f579(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

unsigned long long f580(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f581(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f582(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f583(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f584(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f585(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f586(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f587(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f588(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long long f589(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f590() { return std::atomic<unsigned long long>::is_always_lock_free; }

using t15 = std::atomic<unsigned long>;

std::atomic<unsigned long> f591() { return {}; }

std::atomic<unsigned long> f592(unsigned long a0) { return {a0}; }

unsigned long f593(const std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f594(const volatile std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f595(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f596(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f597(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f598(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f599(std::atomic<unsigned long> &a0) { return a0.operator++(); }

unsigned long f600(volatile std::atomic<unsigned long> &a0) {
  return a0.operator++();
}

unsigned long f601(std::atomic<unsigned long> &a0) { return a0.operator--(); }

unsigned long f602(volatile std::atomic<unsigned long> &a0) {
  return a0.operator--();
}

unsigned long f603(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f604(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f605(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f606(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f607(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f608(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f609(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f610(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f611(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

unsigned long f612(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

bool f613(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

bool f614(const volatile std::atomic<unsigned long> &a0) {
  return a0.is_lock_free();
}

void f615(std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f616(volatile std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long f617(const std::atomic<unsigned long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f618(const volatile std::atomic<unsigned long> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f619(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long f620(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f621(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f622(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f623(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f624(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f625(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f626(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f627(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f628(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f629(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f630(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f631(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

unsigned long f632(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f633(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f634(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f635(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f636(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f637(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f638(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f639(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f640(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long f641(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f642() { return std::atomic<unsigned long>::is_always_lock_free; }

using t16 = std::atomic<char16_t>;

std::atomic<char16_t> f643() { return {}; }

std::atomic<char16_t> f644(char16_t a0) { return {a0}; }

char16_t f645(const std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f646(const volatile std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f647(std::atomic<char16_t> &a0, int a1) { return a0.operator++(a1); }

char16_t f648(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator++(a1);
}

char16_t f649(std::atomic<char16_t> &a0, int a1) { return a0.operator--(a1); }

char16_t f650(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator--(a1);
}

char16_t f651(std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f652(volatile std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f653(std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f654(volatile std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f655(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f656(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f657(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f658(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f659(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f660(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f661(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f662(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f663(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

char16_t f664(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

bool f665(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

bool f666(const volatile std::atomic<char16_t> &a0) {
  return a0.is_lock_free();
}

void f667(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f668(volatile std::atomic<char16_t> &a0, char16_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char16_t f669(const std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f670(const volatile std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f671(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char16_t f672(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f673(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f674(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f675(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f676(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f677(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f678(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f679(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f680(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f681(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f682(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f683(std::atomic<char16_t> &a0) { return a0.notify_all(); }

char16_t f684(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f685(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f686(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f687(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f688(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f689(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f690(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f691(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f692(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char16_t f693(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f694() { return std::atomic<char16_t>::is_always_lock_free; }

using t17 = std::atomic<signed char>;

std::atomic<signed char> f695() { return {}; }

std::atomic<signed char> f696(signed char a0) { return {a0}; }

signed char f697(const std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f698(const volatile std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f699(std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f700(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f701(std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f702(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f703(std::atomic<signed char> &a0) { return a0.operator++(); }

signed char f704(volatile std::atomic<signed char> &a0) {
  return a0.operator++();
}

signed char f705(std::atomic<signed char> &a0) { return a0.operator--(); }

signed char f706(volatile std::atomic<signed char> &a0) {
  return a0.operator--();
}

signed char f707(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f708(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f709(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f710(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f711(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f712(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f713(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f714(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f715(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

signed char f716(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

bool f717(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

bool f718(const volatile std::atomic<signed char> &a0) {
  return a0.is_lock_free();
}

void f719(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f720(volatile std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

signed char f721(const std::atomic<signed char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

signed char f722(const volatile std::atomic<signed char> &a0,
                 std::memory_order a1) {
  return a0.load(a1);
}

signed char f723(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

signed char f724(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f725(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f726(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f727(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f728(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f729(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f730(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f731(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f732(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f733(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f734(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f735(std::atomic<signed char> &a0) { return a0.notify_all(); }

signed char f736(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f737(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f738(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f739(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f740(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f741(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f742(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f743(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f744(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

signed char f745(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f746() { return std::atomic<signed char>::is_always_lock_free; }

using t18 = std::atomic<unsigned short>;

std::atomic<unsigned short> f747() { return {}; }

std::atomic<unsigned short> f748(unsigned short a0) { return {a0}; }

unsigned short f749(const std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f750(const volatile std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f751(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f752(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f753(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f754(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f755(std::atomic<unsigned short> &a0) { return a0.operator++(); }

unsigned short f756(volatile std::atomic<unsigned short> &a0) {
  return a0.operator++();
}

unsigned short f757(std::atomic<unsigned short> &a0) { return a0.operator--(); }

unsigned short f758(volatile std::atomic<unsigned short> &a0) {
  return a0.operator--();
}

unsigned short f759(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f760(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f761(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f762(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f763(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f764(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f765(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f766(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f767(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator^=(a1);
}

unsigned short f768(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator^=(a1);
}

bool f769(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

bool f770(const volatile std::atomic<unsigned short> &a0) {
  return a0.is_lock_free();
}

void f771(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f772(volatile std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned short f773(const std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f774(const volatile std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f775(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned short f776(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f777(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f778(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f779(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f780(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f781(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f782(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f783(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f784(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f785(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f786(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f787(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

unsigned short f788(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f789(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f790(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f791(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f792(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f793(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f794(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f795(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f796(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned short f797(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f798() { return std::atomic<unsigned short>::is_always_lock_free; }

using t19 = std::atomic<char8_t>;

std::atomic<char8_t> f799() { return {}; }

std::atomic<char8_t> f800(char8_t a0) { return {a0}; }

char8_t f801(const std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f802(const volatile std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f803(std::atomic<char8_t> &a0, int a1) { return a0.operator++(a1); }

char8_t f804(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator++(a1);
}

char8_t f805(std::atomic<char8_t> &a0, int a1) { return a0.operator--(a1); }

char8_t f806(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator--(a1);
}

char8_t f807(std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f808(volatile std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f809(std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f810(volatile std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f811(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator+=(a1); }

char8_t f812(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator+=(a1);
}

char8_t f813(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator-=(a1); }

char8_t f814(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator-=(a1);
}

char8_t f815(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator&=(a1); }

char8_t f816(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator&=(a1);
}

char8_t f817(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator|=(a1); }

char8_t f818(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator|=(a1);
}

char8_t f819(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator^=(a1); }

char8_t f820(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator^=(a1);
}

bool f821(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

bool f822(const volatile std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f823(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f824(volatile std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char8_t f825(const std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f826(const volatile std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f827(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char8_t f828(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f829(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f830(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f831(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f832(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f833(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f834(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f835(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f836(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f837(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f838(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f839(std::atomic<char8_t> &a0) { return a0.notify_all(); }

char8_t f840(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f841(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f842(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f843(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f844(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f845(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f846(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f847(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f848(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char8_t f849(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f850() { return std::atomic<char8_t>::is_always_lock_free; }

using t20 = std::atomic<char32_t>;

std::atomic<char32_t> f851() { return {}; }

std::atomic<char32_t> f852(char32_t a0) { return {a0}; }

char32_t f853(const std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f854(const volatile std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f855(std::atomic<char32_t> &a0, int a1) { return a0.operator++(a1); }

char32_t f856(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator++(a1);
}

char32_t f857(std::atomic<char32_t> &a0, int a1) { return a0.operator--(a1); }

char32_t f858(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator--(a1);
}

char32_t f859(std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f860(volatile std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f861(std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f862(volatile std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f863(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f864(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f865(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f866(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f867(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f868(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f869(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f870(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f871(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

char32_t f872(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

bool f873(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

bool f874(const volatile std::atomic<char32_t> &a0) {
  return a0.is_lock_free();
}

void f875(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f876(volatile std::atomic<char32_t> &a0, char32_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char32_t f877(const std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f878(const volatile std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f879(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char32_t f880(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f881(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f882(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f883(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f884(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f885(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f886(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f887(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f888(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f889(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f890(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f891(std::atomic<char32_t> &a0) { return a0.notify_all(); }

char32_t f892(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f893(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f894(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f895(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f896(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f897(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f898(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f899(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f900(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char32_t f901(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f902() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> using t21 = std::atomic<T1 *>;

template <typename T1> using t22 = typename std::atomic<T1 *>::value_type;

template <typename T1> using t23 = typename std::atomic<T1 *>::difference_type;

template <typename T1> std::atomic<T1 *> f903() { return {}; }

template <typename T1> std::atomic<T1 *> f904(T1 *a0) { return {a0}; }

template <typename T1> T1 *f905(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f906(const volatile std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f907(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f908(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f909(std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f910(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f911(std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f912(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f913(std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f914(volatile std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f915(std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f916(volatile std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f917(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f918(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 *f919(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1>
T1 *f920(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f921(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f922(const volatile std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f923(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f924(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f925(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f926(const volatile std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f927(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 *f928(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f929(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f930(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f931(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f932(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f933(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f934(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f935(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f936(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f937(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f938(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f939(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 *f940(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f941(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
         std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f942(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 *f943(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
         std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> bool f944() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> using t24 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t25 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f945(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f946() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f947(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f948(std::shared_ptr<T1> a0) {
  return {std::forward<std::shared_ptr<T1>>(a0)};
}

template <typename T1>
std::shared_ptr<T1> f949(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f950(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f951(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
void f952(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(std::forward<std::shared_ptr<T1>>(a1));
}

template <typename T1>
void f953(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f954(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f955(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3, a4);
}

template <typename T1>
bool f956(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f957(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f958(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2),
                                  a3);
}

template <typename T1>
void f959(const std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1> void f960(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f961(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f962() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t26 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t27 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f963(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f964() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f965(std::weak_ptr<T1> a0) {
  return {std::forward<std::weak_ptr<T1>>(a0)};
}

template <typename T1>
std::weak_ptr<T1> f966(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f967(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f968(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
void f969(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(std::forward<std::weak_ptr<T1>>(a1));
}

template <typename T1>
std::weak_ptr<T1> f970(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f971(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                    a4);
}

template <typename T1>
bool f972(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f973(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f974(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3);
}

template <typename T1>
void f975(const std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1> void f976(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f977(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f978() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t28 = std::atomic<T1>;

template <typename T1> using t29 = typename std::atomic<T1>::value_type;

template <typename T1>
std::atomic<T1> f979()
  requires(std::is_default_constructible_v<T1>)
{
  return {};
}

template <typename T1> std::atomic<T1> f980(T1 a0) { return {std::move(a0)}; }

template <typename T1> T1 f981(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f982(const volatile std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f983(std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 f984(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f985(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f986(const volatile std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f987(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f988(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f989(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f990(const volatile std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f991(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f992(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f993(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f994(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f995(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f996(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f997(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f998(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f999(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f1000(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f1001(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(std::move(a1), a2);
}

template <typename T1> void f1002(std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1003(std::atomic<T1> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f1004() {
  return std::atomic<T1>::is_always_lock_free;
}

using t30 = std::atomic_bool;

using t31 = std::atomic_char;

using t32 = std::atomic_char16_t;

using t33 = std::atomic_char32_t;

using t34 = std::atomic_char8_t;

template <typename T1> bool f1005(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong(a0, a1, std::move(a2));
}

template <typename T1> bool f1006(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong(a0, a1, std::move(a2));
}

template <typename T1>
bool f1007(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit(a0, a1, std::move(a2), a3,
                                                      a4);
}

template <typename T1>
bool f1008(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit(a0, a1, std::move(a2), a3,
                                                      a4);
}

template <typename T1> bool f1009(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak(a0, a1, std::move(a2));
}

template <typename T1> bool f1010(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak(a0, a1, std::move(a2));
}

template <typename T1>
bool f1011(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit(a0, a1, std::move(a2), a3,
                                                    a4);
}

template <typename T1>
bool f1012(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit(a0, a1, std::move(a2), a3,
                                                    a4);
}

template <typename T1> T1 f1013(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange(a0, std::move(a1));
}

template <typename T1> T1 f1014(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange(a0, std::move(a1));
}

template <typename T1>
T1 f1015(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1016(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1017(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1018(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1019(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_add_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1020(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_add_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1021(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1022(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f1023(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_sub_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f1024(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_sub_explicit(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

void f1025(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f1026(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f1027(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f1028(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f1029(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f1030(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f1031(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f1032(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f1033(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f1034(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f1035(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f1036(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f1037(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f1038(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f1039(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f1040(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

template <typename T1> void f1041(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init(a0, std::move(a1));
}

template <typename T1> void f1042(std::atomic<T1> *a0, T1 a1) {
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

template <typename T1> bool f1043(const std::atomic<T1> *a0) {
  return std::atomic_is_lock_free(a0);
}

template <typename T1> bool f1044(const volatile std::atomic<T1> *a0) {
  return std::atomic_is_lock_free(a0);
}

using t50 = std::atomic_llong;

template <typename T1> T1 f1045(const std::atomic<T1> *a0) {
  return std::atomic_load(a0);
}

template <typename T1> T1 f1046(const volatile std::atomic<T1> *a0) {
  return std::atomic_load(a0);
}

template <typename T1>
T1 f1047(const std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit(a0, a1);
}

template <typename T1>
T1 f1048(const volatile std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit(a0, a1);
}

using t51 = std::atomic_long;

template <typename T1> void f1049(std::atomic<T1> *a0) {
  return std::atomic_notify_all(a0);
}

template <typename T1> void f1050(std::atomic<T1> *a0) {
  return std::atomic_notify_one(a0);
}

using t52 = std::atomic_ptrdiff_t;

using t53 = std::atomic_schar;

using t54 = std::atomic_short;

using t55 = std::atomic_signed_lock_free;

using t56 = std::atomic_size_t;

template <typename T1> void f1051(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store(a0, std::move(a1));
}

template <typename T1> void f1052(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store(a0, std::move(a1));
}

template <typename T1>
void f1053(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit(a0, std::move(a1), a2);
}

template <typename T1>
void f1054(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
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
void f1055(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1));
}

template <typename T1>
void f1056(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
           std::memory_order a2) {
  return std::atomic_wait_explicit(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1), a2);
}

using t77 = std::atomic_wchar_t;
