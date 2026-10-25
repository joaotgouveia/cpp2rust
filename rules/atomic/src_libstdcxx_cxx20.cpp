#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <utility>

void f822(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f823(std::atomic<char> &a0) { return a0.notify_one(); }

void f824(std::atomic<char> &a0) { return a0.notify_all(); }

using t67 = std::atomic<long double>;

std::atomic<long double> f825() { return {}; }

std::atomic<long double> f826(long double a0) { return {a0}; }

bool f827(const volatile std::atomic<long double> &a0) {
  return a0.is_lock_free();
}

bool f828(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f829(volatile std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f830(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f831(const volatile std::atomic<long double> &a0,
                 std::memory_order a1) {
  return a0.load(a1);
}

long double f832(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f833(const volatile std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f834(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f835(volatile std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long double f836(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f837(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f838(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f839(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f840(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f841(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f842(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f843(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f844(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f845(const std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f846(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f847(const std::atomic<long double> &a0) { return a0.notify_all(); }

long double f848(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long double f849(volatile std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

long double f850(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long double f851(volatile std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

long double f852(std::atomic<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

long double f853(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

long double f854(std::atomic<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

long double f855(volatile std::atomic<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

void f856(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f857(std::atomic<int> &a0) { return a0.notify_one(); }

void f858(std::atomic<int> &a0) { return a0.notify_all(); }

void f859(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f860(std::atomic<long long> &a0) { return a0.notify_one(); }

void f861(std::atomic<long long> &a0) { return a0.notify_all(); }

void f862(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f863(std::atomic<long> &a0) { return a0.notify_one(); }

void f864(std::atomic<long> &a0) { return a0.notify_all(); }

void f865(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f866(std::atomic<short> &a0) { return a0.notify_one(); }

void f867(std::atomic<short> &a0) { return a0.notify_all(); }

void f868(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f869(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f870(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

void f871(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f872(std::atomic<bool> &a0) { return a0.notify_one(); }

void f873(std::atomic<bool> &a0) { return a0.notify_all(); }

void f874(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f875(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f876(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

using t68 = std::atomic<double>;

std::atomic<double> f877() { return {}; }

std::atomic<double> f878(double a0) { return {a0}; }

bool f879(const volatile std::atomic<double> &a0) { return a0.is_lock_free(); }

bool f880(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f881(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f882(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f883(const volatile std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f884(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f885(const volatile std::atomic<double> &a0) {
  return static_cast<double>(a0);
}

double f886(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f887(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

double f888(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f889(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f890(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f891(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f892(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f893(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f894(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f895(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f896(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f897(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f898(const std::atomic<double> &a0) { return a0.notify_one(); }

void f899(const std::atomic<double> &a0) { return a0.notify_all(); }

double f900(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f901(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

double f902(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f903(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

double f904(std::atomic<double> &a0, double a1) { return a0.operator+=(a1); }

double f905(volatile std::atomic<double> &a0, double a1) {
  return a0.operator+=(a1);
}

double f906(std::atomic<double> &a0, double a1) { return a0.operator-=(a1); }

double f907(volatile std::atomic<double> &a0, double a1) {
  return a0.operator-=(a1);
}

using t69 = std::atomic<float>;

std::atomic<float> f908() { return {}; }

std::atomic<float> f909(float a0) { return {a0}; }

bool f910(const volatile std::atomic<float> &a0) { return a0.is_lock_free(); }

bool f911(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f912(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f913(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f914(const volatile std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f915(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f916(const volatile std::atomic<float> &a0) {
  return static_cast<float>(a0);
}

float f917(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f918(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

float f919(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f920(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f921(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f922(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f923(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f924(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f925(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f926(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f927(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f928(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f929(const std::atomic<float> &a0) { return a0.notify_one(); }

void f930(const std::atomic<float> &a0) { return a0.notify_all(); }

float f931(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f932(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

float f933(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f934(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

float f935(std::atomic<float> &a0, float a1) { return a0.operator+=(a1); }

float f936(volatile std::atomic<float> &a0, float a1) {
  return a0.operator+=(a1);
}

float f937(std::atomic<float> &a0, float a1) { return a0.operator-=(a1); }

float f938(volatile std::atomic<float> &a0, float a1) {
  return a0.operator-=(a1);
}

void f939(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f940(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f941(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

void f942(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f943(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f944(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

void f945(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f946(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f947(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

void f948(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f949(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f950(std::atomic<char16_t> &a0) { return a0.notify_all(); }

void f951(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f952(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f953(std::atomic<signed char> &a0) { return a0.notify_all(); }

void f954(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f955(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f956(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

using t70 = std::atomic<char8_t>;

std::atomic<char8_t> f957() { return {}; }

std::atomic<char8_t> f958(char8_t a0) { return {a0}; }

char8_t f959(const std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f960(const volatile std::atomic<char8_t> &a0) {
  return static_cast<char8_t>(a0);
}

char8_t f961(std::atomic<char8_t> &a0, int a1) { return a0.operator++(a1); }

char8_t f962(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator++(a1);
}

char8_t f963(std::atomic<char8_t> &a0, int a1) { return a0.operator--(a1); }

char8_t f964(volatile std::atomic<char8_t> &a0, int a1) {
  return a0.operator--(a1);
}

char8_t f965(std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f966(volatile std::atomic<char8_t> &a0) { return a0.operator++(); }

char8_t f967(std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f968(volatile std::atomic<char8_t> &a0) { return a0.operator--(); }

char8_t f969(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator+=(a1); }

char8_t f970(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator+=(a1);
}

char8_t f971(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator-=(a1); }

char8_t f972(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator-=(a1);
}

char8_t f973(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator&=(a1); }

char8_t f974(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator&=(a1);
}

char8_t f975(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator|=(a1); }

char8_t f976(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator|=(a1);
}

char8_t f977(std::atomic<char8_t> &a0, char8_t a1) { return a0.operator^=(a1); }

char8_t f978(volatile std::atomic<char8_t> &a0, char8_t a1) {
  return a0.operator^=(a1);
}

bool f979(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

bool f980(const volatile std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f981(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f982(volatile std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

char8_t f983(const std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f984(const volatile std::atomic<char8_t> &a0, std::memory_order a1) {
  return a0.load(a1);
}

char8_t f985(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

char8_t f986(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f987(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f988(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f989(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f990(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f991(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f992(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f993(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f994(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f995(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f996(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f997(std::atomic<char8_t> &a0) { return a0.notify_all(); }

char8_t f998(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f999(volatile std::atomic<char8_t> &a0, char8_t a1,
             std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

char8_t f1000(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f1001(volatile std::atomic<char8_t> &a0, char8_t a1,
              std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

char8_t f1002(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f1003(volatile std::atomic<char8_t> &a0, char8_t a1,
              std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

char8_t f1004(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f1005(volatile std::atomic<char8_t> &a0, char8_t a1,
              std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

char8_t f1006(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

char8_t f1007(volatile std::atomic<char8_t> &a0, char8_t a1,
              std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

bool f1008() { return std::atomic<char8_t>::is_always_lock_free; }

void f1009(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f1010(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f1011(std::atomic<char32_t> &a0) { return a0.notify_all(); }

template <typename T1> using t71 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t72 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f1012(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f1013() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f1014(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f1015(std::shared_ptr<T1> a0) {
  return {std::forward<std::shared_ptr<T1>>(a0)};
}

template <typename T1>
std::shared_ptr<T1> f1016(const std::atomic<std::shared_ptr<T1>> &a0,
                          std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f1017(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f1018(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
           std::memory_order a2) {
  return a0.store(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
void f1019(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(std::forward<std::shared_ptr<T1>>(a1));
}

template <typename T1>
void f1020(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f1021(std::atomic<std::shared_ptr<T1>> &a0,
                          std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f1022(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
           std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3, a4);
}

template <typename T1>
bool f1023(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
           std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::shared_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f1024(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
           std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f1025(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
           std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::shared_ptr<T1>>(a2),
                                  a3);
}

template <typename T1>
void f1026(const std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
           std::memory_order a2) {
  return a0.wait(std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1> void f1027(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1028(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f1029() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t73 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t74 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f1030(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f1031() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f1032(std::weak_ptr<T1> a0) {
  return {std::forward<std::weak_ptr<T1>>(a0)};
}

template <typename T1>
std::weak_ptr<T1> f1033(const std::atomic<std::weak_ptr<T1>> &a0,
                        std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f1034(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f1035(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
           std::memory_order a2) {
  return a0.store(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
void f1036(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(std::forward<std::weak_ptr<T1>>(a1));
}

template <typename T1>
std::weak_ptr<T1> f1037(std::atomic<std::weak_ptr<T1>> &a0,
                        std::weak_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1>
bool f1038(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                    a4);
}

template <typename T1>
bool f1039(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::forward<std::weak_ptr<T1>>(a2),
                                    a3);
}

template <typename T1>
bool f1040(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3,
                                  a4);
}

template <typename T1>
bool f1041(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
           std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::forward<std::weak_ptr<T1>>(a2), a3);
}

template <typename T1>
void f1042(const std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
           std::memory_order a2) {
  return a0.wait(std::forward<std::weak_ptr<T1>>(a1), a2);
}

template <typename T1> void f1043(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1044(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f1045() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1>
std::atomic<T1> f1046()
  requires(std::is_default_constructible_v<T1>)
{
  return {};
}

using t75 = std::atomic_char8_t;

void f1047(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f1048(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f1049(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f1050(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f1051(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f1052(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f1053(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f1054(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

using t76 = std::atomic_signed_lock_free;

using t77 = std::atomic_unsigned_lock_free;
