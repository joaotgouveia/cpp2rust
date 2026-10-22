#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <utility>

void f214(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f215(std::atomic<char> &a0) { return a0.notify_one(); }

void f216(std::atomic<char> &a0) { return a0.notify_all(); }

using t67 = std::atomic<long double>;

std::atomic<long double> f217() { return {}; }

std::atomic<long double> f218(long double a0) { return {a0}; }

bool f219(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f220(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f221(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f222(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f223(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f224(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f225(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f226(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f227(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f228(const std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f229(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f230(const std::atomic<long double> &a0) { return a0.notify_all(); }

void f231(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f232(std::atomic<int> &a0) { return a0.notify_one(); }

void f233(std::atomic<int> &a0) { return a0.notify_all(); }

void f234(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f235(std::atomic<long long> &a0) { return a0.notify_one(); }

void f236(std::atomic<long long> &a0) { return a0.notify_all(); }

void f237(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f238(std::atomic<long> &a0) { return a0.notify_one(); }

void f239(std::atomic<long> &a0) { return a0.notify_all(); }

void f240(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f241(std::atomic<short> &a0) { return a0.notify_one(); }

void f242(std::atomic<short> &a0) { return a0.notify_all(); }

void f243(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f244(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f245(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

void f246(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f247(std::atomic<bool> &a0) { return a0.notify_one(); }

void f248(std::atomic<bool> &a0) { return a0.notify_all(); }

void f249(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f250(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f251(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

using t68 = std::atomic<double>;

std::atomic<double> f252() { return {}; }

std::atomic<double> f253(double a0) { return {a0}; }

bool f254(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f255(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f256(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f257(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f258(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f259(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f260(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f261(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f262(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f263(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f264(const std::atomic<double> &a0) { return a0.notify_one(); }

void f265(const std::atomic<double> &a0) { return a0.notify_all(); }

using t69 = std::atomic<float>;

std::atomic<float> f266() { return {}; }

std::atomic<float> f267(float a0) { return {a0}; }

bool f268(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f269(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f270(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f271(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f272(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f273(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f274(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f275(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f276(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f277(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f278(const std::atomic<float> &a0) { return a0.notify_one(); }

void f279(const std::atomic<float> &a0) { return a0.notify_all(); }

void f280(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f281(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f282(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

void f283(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f284(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f285(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

void f286(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f287(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f288(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

void f289(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f290(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f291(std::atomic<char16_t> &a0) { return a0.notify_all(); }

void f292(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f293(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f294(std::atomic<signed char> &a0) { return a0.notify_all(); }

void f295(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f296(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f297(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

using t70 = std::atomic<char8_t>;

std::atomic<char8_t> f298() { return {}; }

std::atomic<char8_t> f299(char8_t a0) { return {a0}; }

bool f300(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f301(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f302(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f303(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f304(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f305(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f306(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f307(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f308(std::atomic<char8_t> &a0) { return a0.notify_all(); }

bool f309() { return std::atomic<char8_t>::is_always_lock_free; }

void f310(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f311(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f312(std::atomic<char32_t> &a0) { return a0.notify_all(); }

template <typename T1>
void f313(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f314(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f315(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1> using t71 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t72 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f316(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f317() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f318(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f319(std::shared_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::shared_ptr<T1> f320(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f321(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f322(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f323(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f324(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f325(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f326(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f327(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f328(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f329(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f330(const std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f331(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f332(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1>
using t73 = typename std::atomic<std::shared_ptr<T1>>::value_type::element_type;

template <typename T1>
using t74 = typename std::atomic<std::shared_ptr<T1>>::value_type::weak_type;

template <typename T1> bool f333() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t75 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t76 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f334(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f335() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f336(std::weak_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::weak_ptr<T1> f337(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f338(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f339(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f340(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::weak_ptr<T1> f341(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f342(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f343(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f344(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f345(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f346(const std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f347(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f348(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1>
std::shared_ptr<T1>
f349(const typename std::atomic<std::weak_ptr<T1>>::value_type &a0) {
  return a0.lock();
}

template <typename T1> bool f350() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1>
void f351(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f352(std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f353(std::atomic<T1> &a0) {
  return a0.notify_all();
}

using t77 = std::atomic_char8_t;

void f354(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f355(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f356(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f357(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f358(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f359(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f360(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f361(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

template <typename T1> void f362(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f363(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

using t78 = std::atomic_signed_lock_free;

using t79 = std::atomic_unsigned_lock_free;

template <typename T1>
void f364(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f365(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
