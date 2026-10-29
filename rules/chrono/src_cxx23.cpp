#include <chrono>
#include <compare>
#include <iosfwd>
#include <limits>
#include <string_view>
#include <type_traits>
#include <utility>

const std::chrono::month &f1() { return std::chrono::April; }

const std::chrono::month &f2() { return std::chrono::August; }

const std::chrono::month &f3() { return std::chrono::December; }

const std::chrono::month &f4() { return std::chrono::February; }

const std::chrono::weekday &f5() { return std::chrono::Friday; }

const std::chrono::month &f6() { return std::chrono::January; }

const std::chrono::month &f7() { return std::chrono::July; }

const std::chrono::month &f8() { return std::chrono::June; }

const std::chrono::month &f9() { return std::chrono::March; }

const std::chrono::month &f10() { return std::chrono::May; }

const std::chrono::weekday &f11() { return std::chrono::Monday; }

const std::chrono::month &f12() { return std::chrono::November; }

const std::chrono::month &f13() { return std::chrono::October; }

const std::chrono::weekday &f14() { return std::chrono::Saturday; }

const std::chrono::month &f15() { return std::chrono::September; }

const std::chrono::weekday &f16() { return std::chrono::Sunday; }

const std::chrono::weekday &f17() { return std::chrono::Thursday; }

const std::chrono::weekday &f18() { return std::chrono::Tuesday; }

const std::chrono::weekday &f19() { return std::chrono::Wednesday; }

template <typename T1, typename T2>
std::enable_if_t<std::numeric_limits<T1>::is_signed,
                 std::chrono::duration<T1, T2>>
f20(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(
      std::forward<std::chrono::duration<T1, T2>>(a0));
}

using t1 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f21(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time f22(std::chrono::ambiguous_local_time &&a0) {
  return std::chrono::ambiguous_local_time(std::move(a0));
}

std::chrono::ambiguous_local_time &
f23(std::chrono::ambiguous_local_time &a0,
    const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::ambiguous_local_time &f24(std::chrono::ambiguous_local_time &a0,
                                       std::chrono::ambiguous_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::ambiguous_local_time f25(const std::chrono::local_time<T1> &a0,
                                      const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f26(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

template <typename T1, typename T2, typename T3>
T1 f27(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f28(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::ceil<T1, T2, T3>(a0);
}

using t2 = std::chrono::choose;

std::chrono::choose f29() { return std::chrono::choose::earliest; }

std::chrono::choose f30() { return std::chrono::choose::latest; }

template <typename T1, typename T2, typename T3>
auto f31(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::clock_cast<T1, T2, T3>(a0);
}

const std::chrono::time_zone *f32() { return std::chrono::current_zone(); }

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
  return std::chrono::duration_cast<T1, T2, T3>(a0);
}

using t5 = std::chrono::file_clock;

template <typename T1> using t6 = std::chrono::file_time<T1>;

template <typename T1, typename T2, typename T3>
T1 f54(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f55(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::floor<T1, T2, T3>(a0);
}

template <typename T1>
std::chrono::leap_second_info f56(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t7 = std::chrono::gps_clock;

using t8 = std::chrono::gps_clock::rep;

using t9 = std::chrono::gps_clock::period;

using t10 = std::chrono::gps_clock::duration;

using t11 = std::chrono::gps_clock::time_point;

std::chrono::gps_clock::time_point f57() {
  return std::chrono::gps_clock::now();
}

std::chrono::gps_clock f58() { return std::chrono::gps_clock(); }

std::chrono::gps_clock f59(const std::chrono::gps_clock &a0) {
  return std::chrono::gps_clock(a0);
}

std::chrono::gps_clock f60(std::chrono::gps_clock &&a0) {
  return std::chrono::gps_clock(std::move(a0));
}

std::chrono::gps_clock &f61(std::chrono::gps_clock &a0,
                            const std::chrono::gps_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::gps_clock &f62(std::chrono::gps_clock &a0,
                            std::chrono::gps_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f63(const std::chrono::gps_time<T1> &a0) {
  return std::chrono::gps_clock::to_utc(a0);
}

template <typename T1>
std::chrono::gps_time<std::common_type_t<T1, std::chrono::seconds>>
f64(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::gps_clock::from_utc(a0);
}

bool f65() { return std::chrono::gps_clock::is_steady; }

using t12 = std::chrono::gps_seconds;

template <typename T1> using t13 = std::chrono::gps_time<T1>;

using t14 = std::chrono::high_resolution_clock;

using t15 = std::chrono::hours;

bool f66(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

template <typename T1> bool f67() { return std::chrono::is_clock_v<T1>; }

bool f68(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

const std::chrono::last_spec &f69() { return std::chrono::last; }

using t16 = std::chrono::last_spec;

std::chrono::last_spec f70() { return std::chrono::last_spec(); }

std::chrono::last_spec f71(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f72(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f73(std::chrono::last_spec &a0,
                            const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f74(std::chrono::last_spec &a0,
                            std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t17 = std::chrono::leap_second;

std::chrono::leap_second f75(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f76(std::chrono::leap_second &a0,
                              const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f77(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f78(const std::chrono::leap_second &a0) {
  return a0.value();
}

template <typename T1>
auto f79(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1)
  requires(std::three_way_comparable_with<T1, std::chrono::seconds>)
{
  return operator<=>(a0, a1);
}

template <typename T1>
bool f80(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f81(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f82(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f83(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f84(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f85(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f86(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f87(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f88(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator==(a0, a1);
}

std::strong_ordering f89(const std::chrono::leap_second &a0,
                         const std::chrono::leap_second &a1) {
  return operator<=>(a0, a1);
}

bool f90(const std::chrono::leap_second &a0,
         const std::chrono::leap_second &a1) {
  return operator==(a0, a1);
}

using t18 = std::chrono::leap_second_info;

std::chrono::leap_second_info f91() { return std::chrono::leap_second_info(); }

std::chrono::leap_second_info f92(const std::chrono::leap_second_info &a0) {
  return std::chrono::leap_second_info(a0);
}

std::chrono::leap_second_info f93(std::chrono::leap_second_info &&a0) {
  return std::chrono::leap_second_info(std::move(a0));
}

std::chrono::leap_second_info &f94(std::chrono::leap_second_info &a0,
                                   const std::chrono::leap_second_info &a1) {
  return a0.operator=(a1);
}

std::chrono::leap_second_info &f95(std::chrono::leap_second_info &a0,
                                   std::chrono::leap_second_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t19 = std::chrono::local_days;

using t20 = std::chrono::local_info;

std::chrono::local_info f96() { return std::chrono::local_info(); }

std::chrono::local_info f97(const std::chrono::local_info &a0) {
  return std::chrono::local_info(a0);
}

std::chrono::local_info f98(std::chrono::local_info &&a0) {
  return std::chrono::local_info(std::move(a0));
}

std::chrono::local_info &f99(std::chrono::local_info &a0,
                             const std::chrono::local_info &a1) {
  return a0.operator=(a1);
}

std::chrono::local_info &f100(std::chrono::local_info &a0,
                              std::chrono::local_info &&a1) {
  return a0.operator=(std::move(a1));
}

int f101() { return std::chrono::local_info::unique; }

int f102() { return std::chrono::local_info::nonexistent; }

int f103() { return std::chrono::local_info::ambiguous; }

using t21 = std::chrono::local_seconds;

using t22 = std::chrono::local_t;

std::chrono::local_t f104() { return std::chrono::local_t(); }

std::chrono::local_t f105(const std::chrono::local_t &a0) {
  return std::chrono::local_t(a0);
}

std::chrono::local_t f106(std::chrono::local_t &&a0) {
  return std::chrono::local_t(std::move(a0));
}

std::chrono::local_t &f107(std::chrono::local_t &a0,
                           const std::chrono::local_t &a1) {
  return a0.operator=(a1);
}

std::chrono::local_t &f108(std::chrono::local_t &a0,
                           std::chrono::local_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t23 = std::chrono::local_time<T1>;

const std::chrono::time_zone *f109(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

std::chrono::hours f110(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f111(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}

using t24 = std::chrono::microseconds;

using t25 = std::chrono::milliseconds;

using t26 = std::chrono::minutes;

using t27 = std::chrono::month;

std::chrono::month f112() { return std::chrono::month(); }

std::chrono::month f113(unsigned int a0) { return std::chrono::month(a0); }

std::chrono::month &f114(std::chrono::month &a0) { return a0.operator++(); }

std::chrono::month f115(std::chrono::month &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::month &f116(std::chrono::month &a0) { return a0.operator--(); }

std::chrono::month f117(std::chrono::month &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::month &f118(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::month &f119(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

unsigned int f120(const std::chrono::month &a0) {
  return static_cast<unsigned int>(a0);
}

bool f121(const std::chrono::month &a0) { return a0.ok(); }

std::chrono::month f122(const std::chrono::month &a0) {
  return std::chrono::month(a0);
}

std::chrono::month f123(std::chrono::month &&a0) {
  return std::chrono::month(std::move(a0));
}

std::chrono::month &f124(std::chrono::month &a0, const std::chrono::month &a1) {
  return a0.operator=(a1);
}

std::chrono::month &f125(std::chrono::month &a0, std::chrono::month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::months f126(const std::chrono::month &a0,
                         const std::chrono::month &a1) {
  return operator-(a0, a1);
}

std::chrono::month f127(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::month f128(const std::chrono::months &a0,
                        const std::chrono::month &a1) {
  return operator+(a0, a1);
}

std::chrono::month f129(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f130(const std::chrono::month &a0,
                          const std::chrono::month &a1) {
  return operator<=>(a0, a1);
}

bool f131(const std::chrono::month &a0, const std::chrono::month &a1) {
  return operator==(a0, a1);
}

using t28 = std::chrono::month_day;

std::chrono::month_day f132() { return std::chrono::month_day(); }

std::chrono::month_day f133(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return std::chrono::month_day(a0, a1);
}

std::chrono::month f134(const std::chrono::month_day &a0) { return a0.month(); }

std::chrono::day f135(const std::chrono::month_day &a0) { return a0.day(); }

bool f136(const std::chrono::month_day &a0) { return a0.ok(); }

std::chrono::month_day f137(const std::chrono::month_day &a0) {
  return std::chrono::month_day(a0);
}

std::chrono::month_day f138(std::chrono::month_day &&a0) {
  return std::chrono::month_day(std::move(a0));
}

std::chrono::month_day &f139(std::chrono::month_day &a0,
                             const std::chrono::month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day &f140(std::chrono::month_day &a0,
                             std::chrono::month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day f141(const std::chrono::day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f142(const std::chrono::day &a0,
                            const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f143(int a0, const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f144(const std::chrono::month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f145(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::strong_ordering f146(const std::chrono::month_day &a0,
                          const std::chrono::month_day &a1) {
  return operator<=>(a0, a1);
}

bool f147(const std::chrono::month_day &a0, const std::chrono::month_day &a1) {
  return operator==(a0, a1);
}

using t29 = std::chrono::month_day_last;

std::chrono::month_day_last f148(const std::chrono::month &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month f149(const std::chrono::month_day_last &a0) {
  return a0.month();
}

bool f150(const std::chrono::month_day_last &a0) { return a0.ok(); }

std::chrono::month_day_last f151(const std::chrono::month_day_last &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month_day_last f152(std::chrono::month_day_last &&a0) {
  return std::chrono::month_day_last(std::move(a0));
}

std::chrono::month_day_last &f153(std::chrono::month_day_last &a0,
                                  const std::chrono::month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day_last &f154(std::chrono::month_day_last &a0,
                                  std::chrono::month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day_last f155(std::chrono::last_spec a0, int a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f156(std::chrono::last_spec a0,
                                 const std::chrono::month &a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f157(int a0, std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::month_day_last f158(const std::chrono::month &a0,
                                 std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::strong_ordering f159(const std::chrono::month_day_last &a0,
                          const std::chrono::month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f160(const std::chrono::month_day_last &a0,
          const std::chrono::month_day_last &a1) {
  return operator==(a0, a1);
}

using t30 = std::chrono::month_weekday;

std::chrono::month_weekday f161(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return std::chrono::month_weekday(a0, a1);
}

std::chrono::month f162(const std::chrono::month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday_indexed f163(const std::chrono::month_weekday &a0) {
  return a0.weekday_indexed();
}

bool f164(const std::chrono::month_weekday &a0) { return a0.ok(); }

std::chrono::month_weekday f165(const std::chrono::month_weekday &a0) {
  return std::chrono::month_weekday(a0);
}

std::chrono::month_weekday f166(std::chrono::month_weekday &&a0) {
  return std::chrono::month_weekday(std::move(a0));
}

std::chrono::month_weekday &f167(std::chrono::month_weekday &a0,
                                 const std::chrono::month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday &f168(std::chrono::month_weekday &a0,
                                 std::chrono::month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday f169(const std::chrono::weekday_indexed &a0,
                                int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f170(const std::chrono::weekday_indexed &a0,
                                const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f171(int a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f172(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

bool f173(const std::chrono::month_weekday &a0,
          const std::chrono::month_weekday &a1) {
  return operator==(a0, a1);
}

using t31 = std::chrono::month_weekday_last;

std::chrono::month_weekday_last f174(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return std::chrono::month_weekday_last(a0, a1);
}

std::chrono::month f175(const std::chrono::month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday_last f176(const std::chrono::month_weekday_last &a0) {
  return a0.weekday_last();
}

bool f177(const std::chrono::month_weekday_last &a0) { return a0.ok(); }

std::chrono::month_weekday_last
f178(const std::chrono::month_weekday_last &a0) {
  return std::chrono::month_weekday_last(a0);
}

std::chrono::month_weekday_last f179(std::chrono::month_weekday_last &&a0) {
  return std::chrono::month_weekday_last(std::move(a0));
}

std::chrono::month_weekday_last &
f180(std::chrono::month_weekday_last &a0,
     const std::chrono::month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday_last &f181(std::chrono::month_weekday_last &a0,
                                      std::chrono::month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday_last f182(const std::chrono::weekday_last &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f183(const std::chrono::weekday_last &a0,
                                     const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f184(int a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f185(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

bool f186(const std::chrono::month_weekday_last &a0,
          const std::chrono::month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t32 = std::chrono::months;

using t33 = std::chrono::nanoseconds;

using t34 = std::chrono::nonexistent_local_time;

std::chrono::nonexistent_local_time
f187(const std::chrono::nonexistent_local_time &a0) {
  return std::chrono::nonexistent_local_time(a0);
}

std::chrono::nonexistent_local_time
f188(std::chrono::nonexistent_local_time &&a0) {
  return std::chrono::nonexistent_local_time(std::move(a0));
}

std::chrono::nonexistent_local_time &
f189(std::chrono::nonexistent_local_time &a0,
     const std::chrono::nonexistent_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::nonexistent_local_time &
f190(std::chrono::nonexistent_local_time &a0,
     std::chrono::nonexistent_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::nonexistent_local_time f191(const std::chrono::local_time<T1> &a0,
                                         const std::chrono::local_info &a1) {
  return std::chrono::nonexistent_local_time(a0, a1);
}

const char *f192(const std::chrono::nonexistent_local_time &a0) {
  return a0.what();
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    std::chrono::__common_rep_t<T1, std::chrono::__disable_if_is_duration<T3>>,
    T2>
f193(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator% <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f194(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator% <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    typename std::enable_if<
        std::is_convertible<T3 const &,
                            typename std::common_type<T1, T3>::type>::value,
        typename std::common_type<T1, T3>::type>::type,
    T2>
f195(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator* <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    typename std::enable_if<
        std::is_convertible<T1 const &,
                            typename std::common_type<T2, T1>::type>::value,
        typename std::common_type<T2, T1>::type>::type,
    T3>
f196(const T1 &a0, const std::chrono::duration<T2, T3> &a1) {
  return std::chrono::operator* <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f197(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f198(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::time_point<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f199(const std::chrono::time_point<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f200(const std::chrono::time_point<T1, T2> &a0,
     const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator- <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f201(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f202(const std::chrono::time_point<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    std::chrono::__common_rep_t<T1, std::chrono::__disable_if_is_duration<T3>>,
    T2>
f203(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator/ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f204(const std::chrono::duration<T1, T2> &a0,
     const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator/ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f205(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f206(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f207(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f208(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f209(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1)
  requires(std::three_way_comparable_with<T3, T2>)
{
  return std::chrono::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f210(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1)
  requires(std::three_way_comparable<std::common_type_t<T1, T3>>)
{
  return std::chrono::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f211(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f212(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f213(const std::chrono::zoned_time<T1, T2> &a0,
          const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f214(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f215(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f216(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f217(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator>= <T1, T2, T3, T4>(a0, a1);
}

std::string f218() { return std::chrono::remote_version(); }

template <typename T1, typename T2, typename T3>
T1 f219(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f220(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::round<T1, T2, T3>(a0);
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

using t42 = std::chrono::sys_info;

std::chrono::sys_info f228() { return std::chrono::sys_info(); }

std::chrono::sys_info f229(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f230(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f231(std::chrono::sys_info &a0,
                            const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f232(std::chrono::sys_info &a0,
                            std::chrono::sys_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t43 = std::chrono::sys_seconds;

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

using t50 = std::chrono::tai_clock;

using t51 = std::chrono::tai_clock::rep;

using t52 = std::chrono::tai_clock::period;

using t53 = std::chrono::tai_clock::duration;

using t54 = std::chrono::tai_clock::time_point;

std::chrono::tai_clock::time_point f242() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f243() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f244(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f245(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f246(std::chrono::tai_clock &a0,
                             const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f247(std::chrono::tai_clock &a0,
                             std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f248(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f249(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

bool f250() { return std::chrono::tai_clock::is_steady; }

using t55 = std::chrono::tai_seconds;

template <typename T1> using t56 = std::chrono::tai_time<T1>;

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1>
f251(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::time_point_cast<T1, T2, T3>(a0);
}

using t57 = std::chrono::time_zone;

std::string_view f252(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f253(const std::chrono::time_zone &a0,
                           const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f254(const std::chrono::time_zone &a0,
                             const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f255(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f256(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
     std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f257(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}

std::strong_ordering f258(const std::chrono::time_zone &a0,
                          const std::chrono::time_zone &a1) {
  return operator<=>(a0, a1);
}

bool f259(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return operator==(a0, a1);
}

using t58 = std::chrono::time_zone_link;

std::chrono::time_zone_link f260(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f261(std::chrono::time_zone_link &a0,
                                  std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f262(const std::chrono::time_zone_link &a0) {
  return a0.name();
}

std::string_view f263(const std::chrono::time_zone_link &a0) {
  return a0.target();
}

std::strong_ordering f264(const std::chrono::time_zone_link &a0,
                          const std::chrono::time_zone_link &a1) {
  return operator<=>(a0, a1);
}

bool f265(const std::chrono::time_zone_link &a0,
          const std::chrono::time_zone_link &a1) {
  return operator==(a0, a1);
}

template <typename T1> bool f266() {
  return std::chrono::treat_as_floating_point_v<T1>;
}

using t59 = std::chrono::tzdb;

const std::chrono::time_zone *f267(const std::chrono::tzdb &a0,
                                   std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f268(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f269() { return std::chrono::tzdb(); }

std::chrono::tzdb f270(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f271(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}

using t60 = std::chrono::tzdb_list;

using t61 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f272(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f273(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f274(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f275(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f276(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f277(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t62 = std::chrono::tzdb_list::const_iterator::value_type;

using t63 = std::chrono::tzdb_list::const_iterator::reference;

using t64 = std::chrono::tzdb_list::const_iterator::pointer;

using t65 = std::chrono::tzdb_list::const_iterator::difference_type;

using t66 = std::chrono::tzdb_list::const_iterator::iterator_category;

std::chrono::tzdb_list::const_iterator f278() {
  return std::chrono::tzdb_list::const_iterator();
}

std::chrono::tzdb_list::const_iterator
f279(const std::chrono::tzdb_list::const_iterator &a0) {
  return std::chrono::tzdb_list::const_iterator(a0);
}

std::chrono::tzdb_list::const_iterator
f280(std::chrono::tzdb_list::const_iterator &&a0) {
  return std::chrono::tzdb_list::const_iterator(std::move(a0));
}

std::chrono::tzdb_list::const_iterator &
f281(std::chrono::tzdb_list::const_iterator &a0,
     const std::chrono::tzdb_list::const_iterator &a1) {
  return a0.operator=(a1);
}

std::chrono::tzdb_list::const_iterator &
f282(std::chrono::tzdb_list::const_iterator &a0,
     std::chrono::tzdb_list::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

const std::chrono::tzdb &
f283(const std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator*();
}

const std::chrono::tzdb *
f284(const std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator->();
}

std::chrono::tzdb_list::const_iterator &
f285(std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator++();
}

std::chrono::tzdb_list::const_iterator
f286(std::chrono::tzdb_list::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

bool f287(const std::chrono::tzdb_list::const_iterator &a0,
          const std::chrono::tzdb_list::const_iterator &a1) {
  return a0.operator==(a1);
}

const std::string &f288(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->version;
}

const std::vector<std::chrono::time_zone> &
f289(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->zones;
}

const std::vector<std::chrono::time_zone_link> &
f290(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->links;
}

const std::vector<std::chrono::leap_second> &
f291(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->leap_seconds;
}

using t67 = std::chrono::utc_clock;

using t68 = std::chrono::utc_clock::rep;

using t69 = std::chrono::utc_clock::period;

using t70 = std::chrono::utc_clock::duration;

using t71 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f292() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f293() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f294(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f295(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f296(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f297(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f298(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f299(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

bool f300() { return std::chrono::utc_clock::is_steady; }

using t72 = std::chrono::utc_seconds;

template <typename T1> using t73 = std::chrono::utc_time<T1>;

using t74 = std::chrono::weekday;

std::chrono::weekday f301() { return std::chrono::weekday(); }

std::chrono::weekday f302(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f303(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f304(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f305(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f306(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f307(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f308(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f309(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f310(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f311(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f312(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f313(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f314(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f315(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f316(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f317(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f318(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f319(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f320(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f321(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f322(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::weekday f323(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator+(a0, a1);
}

bool f324(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return operator==(a0, a1);
}

using t75 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f325() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f326(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f327(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f328(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f329(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f330(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f331(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f332(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f333(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f334(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return operator==(a0, a1);
}

using t76 = std::chrono::weekday_last;

std::chrono::weekday_last f335(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f336(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f337(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f338(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f339(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f340(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f341(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f342(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return operator==(a0, a1);
}

using t77 = std::chrono::weeks;

using t78 = std::chrono::year;

std::chrono::year f343() { return std::chrono::year(); }

std::chrono::year f344(int a0) { return std::chrono::year(a0); }

std::chrono::year f345() { return std::chrono::year::min(); }

std::chrono::year f346() { return std::chrono::year::max(); }

std::chrono::year &f347(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f348(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f349(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f350(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f351(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f352(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f353(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f354(const std::chrono::year &a0) { return a0.operator-(); }

bool f355(const std::chrono::year &a0) { return a0.is_leap(); }

int f356(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f357(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f358(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f359(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f360(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f361(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f362(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return operator-(a0, a1);
}

std::chrono::year f363(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year f364(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return operator+(a0, a1);
}

std::chrono::year f365(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f366(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return operator<=>(a0, a1);
}

bool f367(const std::chrono::year &a0, const std::chrono::year &a1) {
  return operator==(a0, a1);
}

using t79 = std::chrono::year_month;

std::chrono::year_month f368() { return std::chrono::year_month(); }

std::chrono::year_month f369(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f370(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f371(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f372(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f373(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f374(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f375(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f376(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f377(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f378(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month &f379(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f380(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month f381(const std::chrono::year &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f382(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f383(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f384(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f385(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::months f386(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f387(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f388(const std::chrono::months &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f389(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f390(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return operator<=>(a0, a1);
}

bool f391(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return operator==(a0, a1);
}

using t80 = std::chrono::year_month_day;

std::chrono::year_month_day f392() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f393(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f394(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f395(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f396(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f397(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f398(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f399(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f400(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f401(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f402(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f403(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f404(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f405(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f406(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f407(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f408(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_day &f409(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f410(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day f411(const std::chrono::month_day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f412(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f413(int a0, const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f414(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f415(const std::chrono::year_month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f416(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f417(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f418(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f419(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f420(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f421(const std::chrono::months &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f422(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f423(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return operator<=>(a0, a1);
}

bool f424(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return operator==(a0, a1);
}

using t81 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f425(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f426(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f427(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f428(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f429(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f430(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f431(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f432(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f433(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f434(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f435(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f436(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f437(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f438(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_day_last &f439(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f440(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last f441(const std::chrono::month_day_last &a0,
                                      int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f442(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f443(int a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f444(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f445(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::year_month_day_last
f446(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f447(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f448(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f449(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f450(const std::chrono::months &a0,
     const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f451(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f452(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f453(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return operator==(a0, a1);
}

using t82 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f454() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f455(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f456(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f457(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f458(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f459(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f460(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f461(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f462(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f463(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f464(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f465(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f466(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f467(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f468(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f469(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f470(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f471(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_weekday &f472(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f473(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday f474(const std::chrono::month_weekday &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f475(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f476(int a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f477(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f478(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f479(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday f480(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday
f481(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f482(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday
f483(const std::chrono::months &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f484(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f485(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return operator==(a0, a1);
}

using t83 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f486(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f487(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f488(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f489(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f490(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f491(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f492(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f493(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f494(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f495(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f496(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f497(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f498(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f499(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_weekday_last &
f500(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f501(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last
f502(const std::chrono::month_weekday_last &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f503(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f504(int a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f505(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last f506(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f507(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f508(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f509(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f510(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f511(const std::chrono::months &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f512(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f513(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t84 = std::chrono::years;

using t85 = std::chrono::zoned_seconds;

std::chrono::duration<long double> f514(long double a0) {
  return std::operator""s(a0);
}

template <char... T1> std::chrono::seconds f515() {
  return std::operator""s < T1...>();
}

using t86 = std::filesystem::__file_clock;

using t87 = std::filesystem::__file_clock::duration;

using t88 = std::filesystem::__file_clock::rep;

using t89 = std::filesystem::__file_clock::period;

using t90 = std::filesystem::__file_clock::time_point;

std::filesystem::__file_clock::time_point f516() {
  return std::filesystem::__file_clock::now();
}

std::filesystem::__file_clock f517() { return std::filesystem::__file_clock(); }

std::filesystem::__file_clock f518(const std::filesystem::__file_clock &a0) {
  return std::filesystem::__file_clock(a0);
}

std::filesystem::__file_clock f519(std::filesystem::__file_clock &&a0) {
  return std::filesystem::__file_clock(std::move(a0));
}

std::filesystem::__file_clock &f520(std::filesystem::__file_clock &a0,
                                    const std::filesystem::__file_clock &a1) {
  return a0.operator=(a1);
}

std::filesystem::__file_clock &f521(std::filesystem::__file_clock &a0,
                                    std::filesystem::__file_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::file_time<std::common_type_t<T1, std::chrono::seconds>>
f522(const std::chrono::sys_time<T1> &a0) {
  return std::filesystem::__file_clock::from_sys(a0);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f523(const std::chrono::file_time<T1> &a0) {
  return std::filesystem::__file_clock::to_sys(a0);
}

bool f524() { return std::filesystem::__file_clock::is_steady; }

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

std::chrono::year f536(unsigned long long a0) { return std::operator""y(a0); }
