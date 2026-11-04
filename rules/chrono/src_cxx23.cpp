#include <chrono>
#include <compare>
#include <iosfwd>
#include <limits>
#include <string_view>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::enable_if_t<std::numeric_limits<T1>::is_signed,
                 std::chrono::duration<T1, T2>>
f20(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs(std::forward<std::chrono::duration<T1, T2>>(a0));
}

template <typename T1, typename T2, typename T3>
T1 f27(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::ceil<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f28(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::ceil<T1>(a0);
}

using t3 = std::chrono::day;

std::chrono::day f33() { return std::chrono::day(); }

std::chrono::day f34(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f35(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f36(std::chrono::day &a0, int a1) { return a0.operator++(a1); }

std::chrono::day &f37(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f38(std::chrono::day &a0, int a1) { return a0.operator--(a1); }

std::chrono::day &f39(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f40(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f41(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f42(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f43(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f44(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f45(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f46(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f47(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator-(a0, a1);
}

std::chrono::day f48(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::day f49(const std::chrono::days &a0, const std::chrono::day &a1) {
  return operator+(a0, a1);
}

std::chrono::day f50(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f51(const std::chrono::day &a0,
                         const std::chrono::day &a1) {
  return operator<=>(a0, a1);
}

bool f52(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator==(a0, a1);
}

using t4 = std::chrono::days;

template <typename T1, typename T2, typename T3>
T1 f53(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::duration_cast<T1>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f54(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::floor<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f55(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::floor<T1>(a0);
}

using t14 = std::chrono::high_resolution_clock;

using t15 = std::chrono::hours;

using t24 = std::chrono::microseconds;

using t25 = std::chrono::milliseconds;

using t26 = std::chrono::minutes;

using t33 = std::chrono::nanoseconds;

template <typename T1, typename T2, typename T3>
auto f193(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator%(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f194(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator%(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    typename std::enable_if<
        std::is_convertible<T3 const &,
                            typename std::common_type<T1, T3>::type>::value,
        typename std::common_type<T1, T3>::type>::type,
    T2>
f195(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator*(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    typename std::enable_if<
        std::is_convertible<T1 const &,
                            typename std::common_type<T2, T1>::type>::value,
        typename std::common_type<T2, T1>::type>::type,
    T3>
f196(const T1 &a0, const std::chrono::duration<T2, T3> &a1) {
  return std::chrono::operator*(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f197(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f198(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::time_point<T3, T4> &a1) {
  return std::chrono::operator+(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f199(const std::chrono::time_point<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f200(const std::chrono::time_point<T1, T2> &a0,
     const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f201(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f202(const std::chrono::time_point<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator-(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f203(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator/(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f204(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator/(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f205(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f206(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f207(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f208(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f209(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1)
  requires(std::three_way_comparable_with<T3, T2>)
{
  return std::chrono::operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f210(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1)
  requires(std::three_way_comparable<std::common_type_t<T1, T3>>)
{
  return std::chrono::operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f211(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f212(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f214(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f215(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f216(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f217(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3>
T1 f219(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::round<T1>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f220(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::round<T1>(a0);
}

using t35 = std::chrono::seconds;

using t36 = std::chrono::steady_clock;

using t37 = std::chrono::steady_clock::duration;

using t38 = std::chrono::steady_clock::rep;

using t39 = std::chrono::steady_clock::period;

using t40 = std::chrono::steady_clock::time_point;

std::chrono::steady_clock::time_point f221() {
  return std::chrono::steady_clock::now();
}

std::chrono::steady_clock f222() { return std::chrono::steady_clock(); }

std::chrono::steady_clock f223(const std::chrono::steady_clock &a0) {
  return std::chrono::steady_clock(a0);
}

std::chrono::steady_clock f224(std::chrono::steady_clock &&a0) {
  return std::chrono::steady_clock(std::move(a0));
}

std::chrono::steady_clock &f225(std::chrono::steady_clock &a0,
                                const std::chrono::steady_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::steady_clock &f226(std::chrono::steady_clock &a0,
                                std::chrono::steady_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f227() { return std::chrono::steady_clock::is_steady; }

using t41 = std::chrono::sys_days;

template <typename T1> using t44 = std::chrono::sys_time<T1>;

using t45 = std::chrono::system_clock;

using t46 = std::chrono::system_clock::duration;

using t47 = std::chrono::system_clock::rep;

using t48 = std::chrono::system_clock::period;

using t49 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f233() {
  return std::chrono::system_clock::now();
}

std::time_t f234(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f235(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f236() { return std::chrono::system_clock(); }

std::chrono::system_clock f237(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f238(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f239(std::chrono::system_clock &a0,
                                const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f240(std::chrono::system_clock &a0,
                                std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f241() { return std::chrono::system_clock::is_steady; }

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f251(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::time_point_cast<T1>(a0);
}

using t77 = std::chrono::weeks;

std::chrono::duration<long double> f514(long double a0) {
  return std::operator""s(a0);
}

template <char... T1> std::chrono::seconds f515() {
  return std::operator""s < T1...>();
}

std::chrono::day f525(unsigned long long a0) { return std::operator""d(a0); }

std::chrono::duration<long double, std::ratio<3600, 1>> f526(long double a0) {
  return std::operator""h(a0);
}

template <char... T1> std::chrono::hours f527() {
  return std::operator""h < T1...>();
}

std::chrono::duration<long double, std::ratio<60, 1>> f528(long double a0) {
  return std::operator""min(a0);
}

template <char... T1> std::chrono::minutes f529() {
  return std::operator""min < T1...>();
}

std::chrono::duration<long double, std::milli> f530(long double a0) {
  return std::operator""ms(a0);
}

template <char... T1> std::chrono::milliseconds f531() {
  return std::operator""ms < T1...>();
}

std::chrono::duration<long double, std::nano> f532(long double a0) {
  return std::operator""ns(a0);
}

template <char... T1> std::chrono::nanoseconds f533() {
  return std::operator""ns < T1...>();
}

std::chrono::duration<long double, std::micro> f534(long double a0) {
  return std::operator""us(a0);
}

template <char... T1> std::chrono::microseconds f535() {
  return std::operator""us < T1...>();
}
