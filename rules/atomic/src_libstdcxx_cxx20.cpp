#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <utility>

void f345(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f346(std::atomic<char> &a0) { return a0.notify_one(); }

void f347(std::atomic<char> &a0) { return a0.notify_all(); }

using t67 = std::atomic<long double>;

std::atomic<long double> f348() { return {}; }

std::atomic<long double> f349(long double a0) { return {a0}; }

bool f350(const volatile std::atomic<long double> &a0) {
  return a0.is_lock_free();
}

bool f351(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f352(volatile std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

void f353(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f354(const volatile std::atomic<long double> &a0,
                 std::memory_order a1) {
  return a0.load(a1);
}

long double f355(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f356(const volatile std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f357(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f358(volatile std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

long double f359(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f360(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f361(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f362(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f363(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f364(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f365(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f366(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f367(volatile std::atomic<long double> &a0, long double &a1,
          long double a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f368(const std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f369(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f370(const std::atomic<long double> &a0) { return a0.notify_all(); }

void f371(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f372(std::atomic<int> &a0) { return a0.notify_one(); }

void f373(std::atomic<int> &a0) { return a0.notify_all(); }

void f374(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f375(std::atomic<long long> &a0) { return a0.notify_one(); }

void f376(std::atomic<long long> &a0) { return a0.notify_all(); }

void f377(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f378(std::atomic<long> &a0) { return a0.notify_one(); }

void f379(std::atomic<long> &a0) { return a0.notify_all(); }

void f380(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f381(std::atomic<short> &a0) { return a0.notify_one(); }

void f382(std::atomic<short> &a0) { return a0.notify_all(); }

void f383(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f384(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f385(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

void f386(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f387(std::atomic<bool> &a0) { return a0.notify_one(); }

void f388(std::atomic<bool> &a0) { return a0.notify_all(); }

void f389(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f390(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f391(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

using t68 = std::atomic<double>;

std::atomic<double> f392() { return {}; }

std::atomic<double> f393(double a0) { return {a0}; }

bool f394(const volatile std::atomic<double> &a0) { return a0.is_lock_free(); }

bool f395(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f396(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f397(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f398(const volatile std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f399(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f400(const volatile std::atomic<double> &a0) {
  return static_cast<double>(a0);
}

double f401(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f402(volatile std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

double f403(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f404(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f405(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f406(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f407(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f408(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f409(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f410(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f411(volatile std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f412(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f413(const std::atomic<double> &a0) { return a0.notify_one(); }

void f414(const std::atomic<double> &a0) { return a0.notify_all(); }

using t69 = std::atomic<float>;

std::atomic<float> f415() { return {}; }

std::atomic<float> f416(float a0) { return {a0}; }

bool f417(const volatile std::atomic<float> &a0) { return a0.is_lock_free(); }

bool f418(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f419(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f420(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f421(const volatile std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f422(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f423(const volatile std::atomic<float> &a0) {
  return static_cast<float>(a0);
}

float f424(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f425(volatile std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

float f426(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f427(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f428(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f429(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f430(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f431(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f432(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f433(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f434(volatile std::atomic<float> &a0, float &a1, float a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f435(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f436(const std::atomic<float> &a0) { return a0.notify_one(); }

void f437(const std::atomic<float> &a0) { return a0.notify_all(); }

void f438(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f439(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f440(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

void f441(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f442(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f443(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

void f444(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f445(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f446(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

void f447(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f448(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f449(std::atomic<char16_t> &a0) { return a0.notify_all(); }

void f450(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f451(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f452(std::atomic<signed char> &a0) { return a0.notify_all(); }

void f453(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f454(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f455(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

using t70 = std::atomic<char8_t>;

std::atomic<char8_t> f456() { return {}; }

std::atomic<char8_t> f457(char8_t a0) { return {a0}; }

bool f458(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

bool f459(const volatile std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f460(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

void f461(volatile std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f462(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f463(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f464(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f465(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f466(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f467(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f468(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

bool f469(volatile std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f470(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f471(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f472(std::atomic<char8_t> &a0) { return a0.notify_all(); }

bool f473() { return std::atomic<char8_t>::is_always_lock_free; }

void f474(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f475(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f476(std::atomic<char32_t> &a0) { return a0.notify_all(); }

template <typename T1> using t71 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t72 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f477(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f478() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f479(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f480(std::shared_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::shared_ptr<T1> f481(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f482(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f483(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f484(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f485(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f486(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f487(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f488(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f489(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f490(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f491(const std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f492(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f493(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f494() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t73 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t74 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f495(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f496() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f497(std::weak_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::weak_ptr<T1> f498(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f499(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f500(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f501(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::weak_ptr<T1> f502(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f503(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f504(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f505(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f506(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f507(const std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f508(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f509(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f510() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1>
std::atomic<T1> f511()
  requires(std::is_default_constructible_v<T1>)
{
  return {};
}

template <typename T1>
void f512(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

using t75 = std::atomic_char8_t;

void f513(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f514(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f515(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f516(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f517(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f518(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f519(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f520(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

using t76 = std::atomic_signed_lock_free;

using t77 = std::atomic_unsigned_lock_free;
