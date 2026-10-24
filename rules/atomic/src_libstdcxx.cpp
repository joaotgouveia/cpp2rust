#include <algorithm>
#include <atomic>
#include <utility>

using t4 = std::atomic<char>;

std::atomic<char> f81() { return {}; }

std::atomic<char> f82(char a0) { return {a0}; }

char f83(const std::atomic<char> &a0) { return static_cast<char>(a0); }

char f84(const volatile std::atomic<char> &a0) { return static_cast<char>(a0); }

char f85(std::atomic<char> &a0, int a1) { return a0.operator++(a1); }

char f86(volatile std::atomic<char> &a0, int a1) { return a0.operator++(a1); }

char f87(std::atomic<char> &a0, int a1) { return a0.operator--(a1); }

char f88(volatile std::atomic<char> &a0, int a1) { return a0.operator--(a1); }

char f89(std::atomic<char> &a0) { return a0.operator++(); }

char f90(volatile std::atomic<char> &a0) { return a0.operator++(); }

char f91(std::atomic<char> &a0) { return a0.operator--(); }

char f92(volatile std::atomic<char> &a0) { return a0.operator--(); }

char f93(std::atomic<char> &a0, char a1) { return a0.operator+=(a1); }

char f94(volatile std::atomic<char> &a0, char a1) { return a0.operator+=(a1); }

char f95(std::atomic<char> &a0, char a1) { return a0.operator-=(a1); }

char f96(volatile std::atomic<char> &a0, char a1) { return a0.operator-=(a1); }

char f97(std::atomic<char> &a0, char a1) { return a0.operator&=(a1); }

char f98(volatile std::atomic<char> &a0, char a1) { return a0.operator&=(a1); }

char f99(std::atomic<char> &a0, char a1) { return a0.operator|=(a1); }

char f100(volatile std::atomic<char> &a0, char a1) { return a0.operator|=(a1); }

char f101(std::atomic<char> &a0, char a1) { return a0.operator^=(a1); }

char f102(volatile std::atomic<char> &a0, char a1) { return a0.operator^=(a1); }

bool f103(const std::atomic<char> &a0) { return a0.is_lock_free(); }

bool f104(const volatile std::atomic<char> &a0) { return a0.is_lock_free(); }

void f105(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f106(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char f107(const std::atomic<char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char f108(const volatile std::atomic<char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char f109(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char f110(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f111(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f112(volatile std::atomic<char> &a0, char &a1, char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f113(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f114(volatile std::atomic<char> &a0, char &a1, char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f115(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f116(volatile std::atomic<char> &a0, char &a1, char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f117(std::atomic<char> &a0, char &a1, char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f118(volatile std::atomic<char> &a0, char &a1, char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

char f119(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char f120(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char f121(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char f122(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char f123(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char f124(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char f125(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char f126(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char f127(std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char f128(volatile std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t5 = std::atomic<int>;

std::atomic<int> f129() { return {}; }

std::atomic<int> f130(int a0) { return {a0}; }

int f131(const std::atomic<int> &a0) { return static_cast<int>(a0); }

int f132(const volatile std::atomic<int> &a0) { return static_cast<int>(a0); }

int f133(std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f134(volatile std::atomic<int> &a0, int a1) { return a0.operator++(a1); }

int f135(std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f136(volatile std::atomic<int> &a0, int a1) { return a0.operator--(a1); }

int f137(std::atomic<int> &a0) { return a0.operator++(); }

int f138(volatile std::atomic<int> &a0) { return a0.operator++(); }

int f139(std::atomic<int> &a0) { return a0.operator--(); }

int f140(volatile std::atomic<int> &a0) { return a0.operator--(); }

int f141(std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f142(volatile std::atomic<int> &a0, int a1) { return a0.operator+=(a1); }

int f143(std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f144(volatile std::atomic<int> &a0, int a1) { return a0.operator-=(a1); }

int f145(std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f146(volatile std::atomic<int> &a0, int a1) { return a0.operator&=(a1); }

int f147(std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f148(volatile std::atomic<int> &a0, int a1) { return a0.operator|=(a1); }

int f149(std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

int f150(volatile std::atomic<int> &a0, int a1) { return a0.operator^=(a1); }

bool f151(const std::atomic<int> &a0) { return a0.is_lock_free(); }

bool f152(const volatile std::atomic<int> &a0) { return a0.is_lock_free(); }

void f153(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f154(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

int f155(const std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f156(const volatile std::atomic<int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

int f157(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

int f158(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f159(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f160(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f161(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f162(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f163(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f164(volatile std::atomic<int> &a0, int &a1, int a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f165(std::atomic<int> &a0, int &a1, int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f166(volatile std::atomic<int> &a0, int &a1, int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

int f167(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f168(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

int f169(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f170(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

int f171(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f172(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

int f173(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f174(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

int f175(std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

int f176(volatile std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t6 = std::atomic<long long>;

std::atomic<long long> f177() { return {}; }

std::atomic<long long> f178(long long a0) { return {a0}; }

long long f179(const std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f180(const volatile std::atomic<long long> &a0) {
  return static_cast<long long>(a0);
}

long long f181(std::atomic<long long> &a0, int a1) { return a0.operator++(a1); }

long long f182(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator++(a1);
}

long long f183(std::atomic<long long> &a0, int a1) { return a0.operator--(a1); }

long long f184(volatile std::atomic<long long> &a0, int a1) {
  return a0.operator--(a1);
}

long long f185(std::atomic<long long> &a0) { return a0.operator++(); }

long long f186(volatile std::atomic<long long> &a0) { return a0.operator++(); }

long long f187(std::atomic<long long> &a0) { return a0.operator--(); }

long long f188(volatile std::atomic<long long> &a0) { return a0.operator--(); }

long long f189(std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f190(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator+=(a1);
}

long long f191(std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f192(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator-=(a1);
}

long long f193(std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f194(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator&=(a1);
}

long long f195(std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f196(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator|=(a1);
}

long long f197(std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

long long f198(volatile std::atomic<long long> &a0, long long a1) {
  return a0.operator^=(a1);
}

bool f199(const std::atomic<long long> &a0) { return a0.is_lock_free(); }

bool f200(const volatile std::atomic<long long> &a0) {
  return a0.is_lock_free();
}

void f201(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f202(volatile std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

long long f203(const std::atomic<long long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long long f204(const volatile std::atomic<long long> &a0,
               std::memory_order a1) {
  return a0.load(a1);
}

long long f205(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long long f206(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f207(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f208(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f209(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f210(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f211(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f212(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f213(std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f214(volatile std::atomic<long long> &a0, long long &a1, long long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

long long f215(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f216(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long long f217(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f218(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long long f219(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f220(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long long f221(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f222(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long long f223(std::atomic<long long> &a0, long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long long f224(volatile std::atomic<long long> &a0, long long a1,
               std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t7 = std::atomic<long>;

std::atomic<long> f225() { return {}; }

std::atomic<long> f226(long a0) { return {a0}; }

long f227(const std::atomic<long> &a0) { return static_cast<long>(a0); }

long f228(const volatile std::atomic<long> &a0) {
  return static_cast<long>(a0);
}

long f229(std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f230(volatile std::atomic<long> &a0, int a1) { return a0.operator++(a1); }

long f231(std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f232(volatile std::atomic<long> &a0, int a1) { return a0.operator--(a1); }

long f233(std::atomic<long> &a0) { return a0.operator++(); }

long f234(volatile std::atomic<long> &a0) { return a0.operator++(); }

long f235(std::atomic<long> &a0) { return a0.operator--(); }

long f236(volatile std::atomic<long> &a0) { return a0.operator--(); }

long f237(std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f238(volatile std::atomic<long> &a0, long a1) { return a0.operator+=(a1); }

long f239(std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f240(volatile std::atomic<long> &a0, long a1) { return a0.operator-=(a1); }

long f241(std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f242(volatile std::atomic<long> &a0, long a1) { return a0.operator&=(a1); }

long f243(std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f244(volatile std::atomic<long> &a0, long a1) { return a0.operator|=(a1); }

long f245(std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

long f246(volatile std::atomic<long> &a0, long a1) { return a0.operator^=(a1); }

bool f247(const std::atomic<long> &a0) { return a0.is_lock_free(); }

bool f248(const volatile std::atomic<long> &a0) { return a0.is_lock_free(); }

void f249(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f250(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long f251(const std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f252(const volatile std::atomic<long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long f253(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long f254(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f255(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f256(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f257(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f258(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f259(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f260(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f261(std::atomic<long> &a0, long &a1, long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f262(volatile std::atomic<long> &a0, long &a1, long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

long f263(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f264(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long f265(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f266(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long f267(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f268(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

long f269(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f270(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

long f271(std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

long f272(volatile std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t8 = std::atomic<short>;

std::atomic<short> f273() { return {}; }

std::atomic<short> f274(short a0) { return {a0}; }

short f275(const std::atomic<short> &a0) { return static_cast<short>(a0); }

short f276(const volatile std::atomic<short> &a0) {
  return static_cast<short>(a0);
}

short f277(std::atomic<short> &a0, int a1) { return a0.operator++(a1); }

short f278(volatile std::atomic<short> &a0, int a1) {
  return a0.operator++(a1);
}

short f279(std::atomic<short> &a0, int a1) { return a0.operator--(a1); }

short f280(volatile std::atomic<short> &a0, int a1) {
  return a0.operator--(a1);
}

short f281(std::atomic<short> &a0) { return a0.operator++(); }

short f282(volatile std::atomic<short> &a0) { return a0.operator++(); }

short f283(std::atomic<short> &a0) { return a0.operator--(); }

short f284(volatile std::atomic<short> &a0) { return a0.operator--(); }

short f285(std::atomic<short> &a0, short a1) { return a0.operator+=(a1); }

short f286(volatile std::atomic<short> &a0, short a1) {
  return a0.operator+=(a1);
}

short f287(std::atomic<short> &a0, short a1) { return a0.operator-=(a1); }

short f288(volatile std::atomic<short> &a0, short a1) {
  return a0.operator-=(a1);
}

short f289(std::atomic<short> &a0, short a1) { return a0.operator&=(a1); }

short f290(volatile std::atomic<short> &a0, short a1) {
  return a0.operator&=(a1);
}

short f291(std::atomic<short> &a0, short a1) { return a0.operator|=(a1); }

short f292(volatile std::atomic<short> &a0, short a1) {
  return a0.operator|=(a1);
}

short f293(std::atomic<short> &a0, short a1) { return a0.operator^=(a1); }

short f294(volatile std::atomic<short> &a0, short a1) {
  return a0.operator^=(a1);
}

bool f295(const std::atomic<short> &a0) { return a0.is_lock_free(); }

bool f296(const volatile std::atomic<short> &a0) { return a0.is_lock_free(); }

void f297(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f298(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

short f299(const std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f300(const volatile std::atomic<short> &a0, std::memory_order a1) {
  return a0.load(a1);
}

short f301(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

short f302(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f303(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f304(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f305(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f306(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f307(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f308(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f309(std::atomic<short> &a0, short &a1, short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f310(volatile std::atomic<short> &a0, short &a1, short a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

short f311(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f312(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

short f313(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f314(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

short f315(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f316(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

short f317(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f318(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

short f319(std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

short f320(volatile std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t9 = std::atomic<wchar_t>;

std::atomic<wchar_t> f321() { return {}; }

std::atomic<wchar_t> f322(wchar_t a0) { return {a0}; }

wchar_t f323(const std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f324(const volatile std::atomic<wchar_t> &a0) {
  return static_cast<wchar_t>(a0);
}

wchar_t f325(std::atomic<wchar_t> &a0, int a1) { return a0.operator++(a1); }

wchar_t f326(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator++(a1);
}

wchar_t f327(std::atomic<wchar_t> &a0, int a1) { return a0.operator--(a1); }

wchar_t f328(volatile std::atomic<wchar_t> &a0, int a1) {
  return a0.operator--(a1);
}

wchar_t f329(std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f330(volatile std::atomic<wchar_t> &a0) { return a0.operator++(); }

wchar_t f331(std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f332(volatile std::atomic<wchar_t> &a0) { return a0.operator--(); }

wchar_t f333(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator+=(a1); }

wchar_t f334(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator+=(a1);
}

wchar_t f335(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator-=(a1); }

wchar_t f336(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator-=(a1);
}

wchar_t f337(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator&=(a1); }

wchar_t f338(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator&=(a1);
}

wchar_t f339(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator|=(a1); }

wchar_t f340(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator|=(a1);
}

wchar_t f341(std::atomic<wchar_t> &a0, wchar_t a1) { return a0.operator^=(a1); }

wchar_t f342(volatile std::atomic<wchar_t> &a0, wchar_t a1) {
  return a0.operator^=(a1);
}

bool f343(const std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

bool f344(const volatile std::atomic<wchar_t> &a0) { return a0.is_lock_free(); }

void f345(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f346(volatile std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

wchar_t f347(const std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f348(const volatile std::atomic<wchar_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

wchar_t f349(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

wchar_t f350(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f351(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f352(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f353(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f354(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f355(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f356(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f357(std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f358(volatile std::atomic<wchar_t> &a0, wchar_t &a1, wchar_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

wchar_t f359(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f360(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

wchar_t f361(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f362(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

wchar_t f363(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f364(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

wchar_t f365(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f366(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

wchar_t f367(std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

wchar_t f368(volatile std::atomic<wchar_t> &a0, wchar_t a1,
             std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t10 = std::atomic<bool>;

using t11 = std::atomic<bool>::value_type;

std::atomic<bool> f369() { return {}; }

std::atomic<bool> f370(bool a0) { return {a0}; }

bool f371(std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f372(volatile std::atomic<bool> &a0, bool a1) { return a0.operator=(a1); }

bool f373(const std::atomic<bool> &a0) { return static_cast<bool>(a0); }

bool f374(const volatile std::atomic<bool> &a0) {
  return static_cast<bool>(a0);
}

bool f375(const std::atomic<bool> &a0) { return a0.is_lock_free(); }

bool f376(const volatile std::atomic<bool> &a0) { return a0.is_lock_free(); }

void f377(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f378(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f379(const std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f380(const volatile std::atomic<bool> &a0, std::memory_order a1) {
  return a0.load(a1);
}

bool f381(std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f382(volatile std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f383(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f384(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f385(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f386(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f387(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f388(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f389(std::atomic<bool> &a0, bool &a1, bool a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f390(volatile std::atomic<bool> &a0, bool &a1, bool a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

using t12 = std::atomic<unsigned char>;

std::atomic<unsigned char> f391() { return {}; }

std::atomic<unsigned char> f392(unsigned char a0) { return {a0}; }

unsigned char f393(const std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f394(const volatile std::atomic<unsigned char> &a0) {
  return static_cast<unsigned char>(a0);
}

unsigned char f395(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f396(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned char f397(std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f398(volatile std::atomic<unsigned char> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned char f399(std::atomic<unsigned char> &a0) { return a0.operator++(); }

unsigned char f400(volatile std::atomic<unsigned char> &a0) {
  return a0.operator++();
}

unsigned char f401(std::atomic<unsigned char> &a0) { return a0.operator--(); }

unsigned char f402(volatile std::atomic<unsigned char> &a0) {
  return a0.operator--();
}

unsigned char f403(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f404(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator+=(a1);
}

unsigned char f405(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f406(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator-=(a1);
}

unsigned char f407(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f408(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator&=(a1);
}

unsigned char f409(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f410(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator|=(a1);
}

unsigned char f411(std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

unsigned char f412(volatile std::atomic<unsigned char> &a0, unsigned char a1) {
  return a0.operator^=(a1);
}

bool f413(const std::atomic<unsigned char> &a0) { return a0.is_lock_free(); }

bool f414(const volatile std::atomic<unsigned char> &a0) {
  return a0.is_lock_free();
}

void f415(std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f416(volatile std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned char f417(const std::atomic<unsigned char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f418(const volatile std::atomic<unsigned char> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned char f419(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned char f420(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f421(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f422(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f423(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f424(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f425(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f426(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f427(std::atomic<unsigned char> &a0, unsigned char &a1, unsigned char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f428(volatile std::atomic<unsigned char> &a0, unsigned char &a1,
          unsigned char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

unsigned char f429(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f430(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned char f431(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f432(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned char f433(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f434(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned char f435(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f436(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned char f437(std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned char f438(volatile std::atomic<unsigned char> &a0, unsigned char a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t13 = std::atomic<unsigned int>;

std::atomic<unsigned int> f439() { return {}; }

std::atomic<unsigned int> f440(unsigned int a0) { return {a0}; }

unsigned int f441(const std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f442(const volatile std::atomic<unsigned int> &a0) {
  return static_cast<unsigned int>(a0);
}

unsigned int f443(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f444(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned int f445(std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f446(volatile std::atomic<unsigned int> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned int f447(std::atomic<unsigned int> &a0) { return a0.operator++(); }

unsigned int f448(volatile std::atomic<unsigned int> &a0) {
  return a0.operator++();
}

unsigned int f449(std::atomic<unsigned int> &a0) { return a0.operator--(); }

unsigned int f450(volatile std::atomic<unsigned int> &a0) {
  return a0.operator--();
}

unsigned int f451(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f452(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator+=(a1);
}

unsigned int f453(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f454(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator-=(a1);
}

unsigned int f455(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f456(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator&=(a1);
}

unsigned int f457(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f458(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator|=(a1);
}

unsigned int f459(std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

unsigned int f460(volatile std::atomic<unsigned int> &a0, unsigned int a1) {
  return a0.operator^=(a1);
}

bool f461(const std::atomic<unsigned int> &a0) { return a0.is_lock_free(); }

bool f462(const volatile std::atomic<unsigned int> &a0) {
  return a0.is_lock_free();
}

void f463(std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f464(volatile std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned int f465(const std::atomic<unsigned int> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f466(const volatile std::atomic<unsigned int> &a0,
                  std::memory_order a1) {
  return a0.load(a1);
}

unsigned int f467(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned int f468(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f469(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f470(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f471(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f472(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f473(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f474(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f475(std::atomic<unsigned int> &a0, unsigned int &a1, unsigned int a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f476(volatile std::atomic<unsigned int> &a0, unsigned int &a1,
          unsigned int a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

unsigned int f477(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f478(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned int f479(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f480(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned int f481(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f482(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned int f483(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f484(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned int f485(std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned int f486(volatile std::atomic<unsigned int> &a0, unsigned int a1,
                  std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t14 = std::atomic<unsigned long long>;

std::atomic<unsigned long long> f487() { return {}; }

std::atomic<unsigned long long> f488(unsigned long long a0) { return {a0}; }

unsigned long long f489(const std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f490(const volatile std::atomic<unsigned long long> &a0) {
  return static_cast<unsigned long long>(a0);
}

unsigned long long f491(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f492(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long long f493(std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f494(volatile std::atomic<unsigned long long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long long f495(std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f496(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator++();
}

unsigned long long f497(std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f498(volatile std::atomic<unsigned long long> &a0) {
  return a0.operator--();
}

unsigned long long f499(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f500(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator+=(a1);
}

unsigned long long f501(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f502(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator-=(a1);
}

unsigned long long f503(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f504(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator&=(a1);
}

unsigned long long f505(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f506(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator|=(a1);
}

unsigned long long f507(std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

unsigned long long f508(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1) {
  return a0.operator^=(a1);
}

bool f509(const std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

bool f510(const volatile std::atomic<unsigned long long> &a0) {
  return a0.is_lock_free();
}

void f511(std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f512(volatile std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long long f513(const std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f514(const volatile std::atomic<unsigned long long> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

unsigned long long f515(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long long f516(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f517(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f518(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f519(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f520(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f521(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f522(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f523(std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f524(volatile std::atomic<unsigned long long> &a0, unsigned long long &a1,
          unsigned long long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

unsigned long long f525(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f526(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long long f527(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f528(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long long f529(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f530(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long long f531(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f532(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long long f533(std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long long f534(volatile std::atomic<unsigned long long> &a0,
                        unsigned long long a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t15 = std::atomic<unsigned long>;

std::atomic<unsigned long> f535() { return {}; }

std::atomic<unsigned long> f536(unsigned long a0) { return {a0}; }

unsigned long f537(const std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f538(const volatile std::atomic<unsigned long> &a0) {
  return static_cast<unsigned long>(a0);
}

unsigned long f539(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f540(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned long f541(std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f542(volatile std::atomic<unsigned long> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned long f543(std::atomic<unsigned long> &a0) { return a0.operator++(); }

unsigned long f544(volatile std::atomic<unsigned long> &a0) {
  return a0.operator++();
}

unsigned long f545(std::atomic<unsigned long> &a0) { return a0.operator--(); }

unsigned long f546(volatile std::atomic<unsigned long> &a0) {
  return a0.operator--();
}

unsigned long f547(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f548(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator+=(a1);
}

unsigned long f549(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f550(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator-=(a1);
}

unsigned long f551(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f552(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator&=(a1);
}

unsigned long f553(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f554(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator|=(a1);
}

unsigned long f555(std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

unsigned long f556(volatile std::atomic<unsigned long> &a0, unsigned long a1) {
  return a0.operator^=(a1);
}

bool f557(const std::atomic<unsigned long> &a0) { return a0.is_lock_free(); }

bool f558(const volatile std::atomic<unsigned long> &a0) {
  return a0.is_lock_free();
}

void f559(std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f560(volatile std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned long f561(const std::atomic<unsigned long> &a0, std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f562(const volatile std::atomic<unsigned long> &a0,
                   std::memory_order a1) {
  return a0.load(a1);
}

unsigned long f563(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned long f564(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f565(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f566(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f567(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f568(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f569(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f570(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f571(std::atomic<unsigned long> &a0, unsigned long &a1, unsigned long a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f572(volatile std::atomic<unsigned long> &a0, unsigned long &a1,
          unsigned long a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

unsigned long f573(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f574(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned long f575(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f576(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned long f577(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f578(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned long f579(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f580(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned long f581(std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned long f582(volatile std::atomic<unsigned long> &a0, unsigned long a1,
                   std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t16 = std::atomic<char16_t>;

std::atomic<char16_t> f583() { return {}; }

std::atomic<char16_t> f584(char16_t a0) { return {a0}; }

char16_t f585(const std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f586(const volatile std::atomic<char16_t> &a0) {
  return static_cast<char16_t>(a0);
}

char16_t f587(std::atomic<char16_t> &a0, int a1) { return a0.operator++(a1); }

char16_t f588(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator++(a1);
}

char16_t f589(std::atomic<char16_t> &a0, int a1) { return a0.operator--(a1); }

char16_t f590(volatile std::atomic<char16_t> &a0, int a1) {
  return a0.operator--(a1);
}

char16_t f591(std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f592(volatile std::atomic<char16_t> &a0) { return a0.operator++(); }

char16_t f593(std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f594(volatile std::atomic<char16_t> &a0) { return a0.operator--(); }

char16_t f595(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f596(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator+=(a1);
}

char16_t f597(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f598(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator-=(a1);
}

char16_t f599(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f600(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator&=(a1);
}

char16_t f601(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f602(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator|=(a1);
}

char16_t f603(std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

char16_t f604(volatile std::atomic<char16_t> &a0, char16_t a1) {
  return a0.operator^=(a1);
}

bool f605(const std::atomic<char16_t> &a0) { return a0.is_lock_free(); }

bool f606(const volatile std::atomic<char16_t> &a0) {
  return a0.is_lock_free();
}

void f607(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f608(volatile std::atomic<char16_t> &a0, char16_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char16_t f609(const std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f610(const volatile std::atomic<char16_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char16_t f611(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char16_t f612(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f613(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f614(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f615(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f616(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f617(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f618(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f619(std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f620(volatile std::atomic<char16_t> &a0, char16_t &a1, char16_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

char16_t f621(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f622(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char16_t f623(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f624(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char16_t f625(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f626(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char16_t f627(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f628(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char16_t f629(std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char16_t f630(volatile std::atomic<char16_t> &a0, char16_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t17 = std::atomic<signed char>;

std::atomic<signed char> f631() { return {}; }

std::atomic<signed char> f632(signed char a0) { return {a0}; }

signed char f633(const std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f634(const volatile std::atomic<signed char> &a0) {
  return static_cast<signed char>(a0);
}

signed char f635(std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f636(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator++(a1);
}

signed char f637(std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f638(volatile std::atomic<signed char> &a0, int a1) {
  return a0.operator--(a1);
}

signed char f639(std::atomic<signed char> &a0) { return a0.operator++(); }

signed char f640(volatile std::atomic<signed char> &a0) {
  return a0.operator++();
}

signed char f641(std::atomic<signed char> &a0) { return a0.operator--(); }

signed char f642(volatile std::atomic<signed char> &a0) {
  return a0.operator--();
}

signed char f643(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f644(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator+=(a1);
}

signed char f645(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f646(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator-=(a1);
}

signed char f647(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f648(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator&=(a1);
}

signed char f649(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f650(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator|=(a1);
}

signed char f651(std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

signed char f652(volatile std::atomic<signed char> &a0, signed char a1) {
  return a0.operator^=(a1);
}

bool f653(const std::atomic<signed char> &a0) { return a0.is_lock_free(); }

bool f654(const volatile std::atomic<signed char> &a0) {
  return a0.is_lock_free();
}

void f655(std::atomic<signed char> &a0, signed char a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f656(volatile std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

signed char f657(const std::atomic<signed char> &a0, std::memory_order a1) {
  return a0.load(a1);
}

signed char f658(const volatile std::atomic<signed char> &a0,
                 std::memory_order a1) {
  return a0.load(a1);
}

signed char f659(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

signed char f660(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f661(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f662(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f663(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f664(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f665(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f666(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f667(std::atomic<signed char> &a0, signed char &a1, signed char a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f668(volatile std::atomic<signed char> &a0, signed char &a1,
          signed char a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

signed char f669(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f670(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

signed char f671(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f672(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

signed char f673(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f674(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

signed char f675(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f676(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

signed char f677(std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

signed char f678(volatile std::atomic<signed char> &a0, signed char a1,
                 std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t18 = std::atomic<unsigned short>;

std::atomic<unsigned short> f679() { return {}; }

std::atomic<unsigned short> f680(unsigned short a0) { return {a0}; }

unsigned short f681(const std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f682(const volatile std::atomic<unsigned short> &a0) {
  return static_cast<unsigned short>(a0);
}

unsigned short f683(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f684(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator++(a1);
}

unsigned short f685(std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f686(volatile std::atomic<unsigned short> &a0, int a1) {
  return a0.operator--(a1);
}

unsigned short f687(std::atomic<unsigned short> &a0) { return a0.operator++(); }

unsigned short f688(volatile std::atomic<unsigned short> &a0) {
  return a0.operator++();
}

unsigned short f689(std::atomic<unsigned short> &a0) { return a0.operator--(); }

unsigned short f690(volatile std::atomic<unsigned short> &a0) {
  return a0.operator--();
}

unsigned short f691(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f692(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator+=(a1);
}

unsigned short f693(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f694(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator-=(a1);
}

unsigned short f695(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f696(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator&=(a1);
}

unsigned short f697(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f698(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator|=(a1);
}

unsigned short f699(std::atomic<unsigned short> &a0, unsigned short a1) {
  return a0.operator^=(a1);
}

unsigned short f700(volatile std::atomic<unsigned short> &a0,
                    unsigned short a1) {
  return a0.operator^=(a1);
}

bool f701(const std::atomic<unsigned short> &a0) { return a0.is_lock_free(); }

bool f702(const volatile std::atomic<unsigned short> &a0) {
  return a0.is_lock_free();
}

void f703(std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f704(volatile std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

unsigned short f705(const std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f706(const volatile std::atomic<unsigned short> &a0,
                    std::memory_order a1) {
  return a0.load(a1);
}

unsigned short f707(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

unsigned short f708(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f709(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f710(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f711(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f712(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f713(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f714(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f715(std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f716(volatile std::atomic<unsigned short> &a0, unsigned short &a1,
          unsigned short a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

unsigned short f717(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f718(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

unsigned short f719(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f720(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

unsigned short f721(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f722(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

unsigned short f723(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f724(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

unsigned short f725(std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

unsigned short f726(volatile std::atomic<unsigned short> &a0, unsigned short a1,
                    std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

using t19 = std::atomic<char32_t>;

std::atomic<char32_t> f727() { return {}; }

std::atomic<char32_t> f728(char32_t a0) { return {a0}; }

char32_t f729(const std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f730(const volatile std::atomic<char32_t> &a0) {
  return static_cast<char32_t>(a0);
}

char32_t f731(std::atomic<char32_t> &a0, int a1) { return a0.operator++(a1); }

char32_t f732(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator++(a1);
}

char32_t f733(std::atomic<char32_t> &a0, int a1) { return a0.operator--(a1); }

char32_t f734(volatile std::atomic<char32_t> &a0, int a1) {
  return a0.operator--(a1);
}

char32_t f735(std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f736(volatile std::atomic<char32_t> &a0) { return a0.operator++(); }

char32_t f737(std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f738(volatile std::atomic<char32_t> &a0) { return a0.operator--(); }

char32_t f739(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f740(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator+=(a1);
}

char32_t f741(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f742(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator-=(a1);
}

char32_t f743(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f744(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator&=(a1);
}

char32_t f745(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f746(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator|=(a1);
}

char32_t f747(std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

char32_t f748(volatile std::atomic<char32_t> &a0, char32_t a1) {
  return a0.operator^=(a1);
}

bool f749(const std::atomic<char32_t> &a0) { return a0.is_lock_free(); }

bool f750(const volatile std::atomic<char32_t> &a0) {
  return a0.is_lock_free();
}

void f751(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f752(volatile std::atomic<char32_t> &a0, char32_t a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

char32_t f753(const std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f754(const volatile std::atomic<char32_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char32_t f755(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char32_t f756(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f757(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f758(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f759(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f760(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f761(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f762(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f763(std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f764(volatile std::atomic<char32_t> &a0, char32_t &a1, char32_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

char32_t f765(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f766(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char32_t f767(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f768(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char32_t f769(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f770(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char32_t f771(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f772(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char32_t f773(std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char32_t f774(volatile std::atomic<char32_t> &a0, char32_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1> using t20 = typename std::atomic<T1 *>::value_type;

template <typename T1> std::atomic<T1 *> f775() { return {}; }

template <typename T1> std::atomic<T1 *> f776(T1 *a0) { return {a0}; }

template <typename T1> using t21 = typename std::atomic<T1>::value_type;

template <typename T1> std::atomic<T1> f777(T1 a0) { return {a0}; }

using t22 = std::atomic_bool;

using t23 = std::atomic_char;

using t24 = std::atomic_char16_t;

using t25 = std::atomic_char32_t;

template <typename T1> bool f778(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1> bool f779(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f780(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f781(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> bool f782(volatile std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1> bool f783(std::atomic<T1> *a0, T1 *a1, T1 a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f784(volatile std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f785(std::atomic<T1> *a0, T1 *a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1> T1 f786(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1> T1 f787(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f788(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f789(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

void f790(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f791(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f792(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f793(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f794(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f795(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f796(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f797(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

template <typename T1> void f798(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1> void f799(std::atomic<T1> *a0, T1 a1) {
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

template <typename T1> void f800(volatile std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1> void f801(std::atomic<T1> *a0, T1 a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f802(volatile std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f803(std::atomic<T1> *a0, T1 a1, std::memory_order a2) {
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
