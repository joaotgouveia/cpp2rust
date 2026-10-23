#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <utility>

void f224(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f225(std::atomic<char> &a0) { return a0.notify_one(); }

void f226(std::atomic<char> &a0) { return a0.notify_all(); }

using t67 = std::atomic<long double>;

std::atomic<long double> f227() { return {}; }

std::atomic<long double> f228(long double a0) { return {a0}; }

bool f229(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f230(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f231(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f232(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f233(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f234(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f235(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f236(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f237(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f238(const std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f239(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f240(const std::atomic<long double> &a0) { return a0.notify_all(); }

void f241(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f242(std::atomic<int> &a0) { return a0.notify_one(); }

void f243(std::atomic<int> &a0) { return a0.notify_all(); }

void f244(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f245(std::atomic<long long> &a0) { return a0.notify_one(); }

void f246(std::atomic<long long> &a0) { return a0.notify_all(); }

void f247(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f248(std::atomic<long> &a0) { return a0.notify_one(); }

void f249(std::atomic<long> &a0) { return a0.notify_all(); }

void f250(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f251(std::atomic<short> &a0) { return a0.notify_one(); }

void f252(std::atomic<short> &a0) { return a0.notify_all(); }

void f253(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f254(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f255(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

void f256(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f257(std::atomic<bool> &a0) { return a0.notify_one(); }

void f258(std::atomic<bool> &a0) { return a0.notify_all(); }

void f259(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f260(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f261(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

using t68 = std::atomic<double>;

std::atomic<double> f262() { return {}; }

std::atomic<double> f263(double a0) { return {a0}; }

bool f264(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f265(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f266(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f267(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f268(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f269(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f270(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f271(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f272(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f273(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f274(const std::atomic<double> &a0) { return a0.notify_one(); }

void f275(const std::atomic<double> &a0) { return a0.notify_all(); }

using t69 = std::atomic<float>;

std::atomic<float> f276() { return {}; }

std::atomic<float> f277(float a0) { return {a0}; }

bool f278(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f279(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f280(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f281(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f282(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f283(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f284(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f285(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f286(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f287(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f288(const std::atomic<float> &a0) { return a0.notify_one(); }

void f289(const std::atomic<float> &a0) { return a0.notify_all(); }

void f290(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f291(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f292(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

void f293(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f294(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f295(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

void f296(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f297(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f298(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

void f299(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f300(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f301(std::atomic<char16_t> &a0) { return a0.notify_all(); }

void f302(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f303(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f304(std::atomic<signed char> &a0) { return a0.notify_all(); }

void f305(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f306(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f307(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

using t70 = std::atomic<char8_t>;

std::atomic<char8_t> f308() { return {}; }

std::atomic<char8_t> f309(char8_t a0) { return {a0}; }

bool f310(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f311(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f312(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f313(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f314(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f315(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f316(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f317(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f318(std::atomic<char8_t> &a0) { return a0.notify_all(); }

bool f319() { return std::atomic<char8_t>::is_always_lock_free; }

void f320(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f321(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f322(std::atomic<char32_t> &a0) { return a0.notify_all(); }

template <typename T1> using t71 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t72 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f323(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f324() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f325(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f326(std::shared_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::shared_ptr<T1> f327(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f328(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f329(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f330(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f331(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f332(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f333(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f334(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f335(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f336(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f337(const std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f338(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f339(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f340() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t73 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t74 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f341(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f342() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f343(std::weak_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::weak_ptr<T1> f344(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f345(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f346(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f347(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::weak_ptr<T1> f348(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f349(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f350(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f351(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f352(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f353(const std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f354(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f355(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1> bool f356() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1>
std::atomic<T1> f357()
  requires(std::is_default_constructible_v<T1>)
{
  return {};
}

using t75 = std::atomic_char8_t;

void f358(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f359(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f360(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f361(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f362(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f363(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f364(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f365(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

using t76 = std::atomic_signed_lock_free;

using t77 = std::atomic_unsigned_lock_free;
