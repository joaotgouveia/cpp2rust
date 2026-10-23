#include <chrono>
#include <compare>
#include <iosfwd>
#include <limits>
#include <string_view>
#include <type_traits>
#include <utility>

using t18 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f38(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time f39(std::chrono::ambiguous_local_time &&a0) {
  return std::chrono::ambiguous_local_time(std::move(a0));
}

std::chrono::ambiguous_local_time &
f40(std::chrono::ambiguous_local_time &a0,
    const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::ambiguous_local_time &f41(std::chrono::ambiguous_local_time &a0,
                                       std::chrono::ambiguous_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::ambiguous_local_time f42(const std::chrono::local_time<T1> &a0,
                                      const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f43(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

using t19 = std::chrono::choose;

std::chrono::choose f44() { return std::chrono::choose::earliest; }

std::chrono::choose f45() { return std::chrono::choose::latest; }

template <typename T1, typename T2, typename T3>
auto f46(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::clock_cast<T1, T2, T3>(a0);
}

const std::chrono::time_zone *f47() { return std::chrono::current_zone(); }

using t20 = std::chrono::day;

std::chrono::day f48() { return std::chrono::day(); }

std::chrono::day f49(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f50(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f51(std::chrono::day &a0, int a1) { return a0.operator++(a1); }

std::chrono::day &f52(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f53(std::chrono::day &a0, int a1) { return a0.operator--(a1); }

std::chrono::day &f54(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f55(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f56(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f57(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f58(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f59(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f60(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f61(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f62(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator-(a0, a1);
}

std::chrono::day f63(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::day f64(const std::chrono::days &a0, const std::chrono::day &a1) {
  return operator+(a0, a1);
}

std::chrono::day f65(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f66(const std::chrono::day &a0,
                         const std::chrono::day &a1) {
  return operator<=>(a0, a1);
}

bool f67(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator==(a0, a1);
}

using t21 = std::chrono::days;

using t22 = std::chrono::file_clock;

template <typename T1> using t23 = std::chrono::file_time<T1>;

template <typename T1>
std::chrono::leap_second_info f68(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t24 = std::chrono::gps_clock;

using t25 = std::chrono::gps_clock::rep;

using t26 = std::chrono::gps_clock::period;

using t27 = std::chrono::gps_clock::duration;

using t28 = std::chrono::gps_clock::time_point;

std::chrono::gps_clock::time_point f69() {
  return std::chrono::gps_clock::now();
}

std::chrono::gps_clock f70() { return std::chrono::gps_clock(); }

std::chrono::gps_clock f71(const std::chrono::gps_clock &a0) {
  return std::chrono::gps_clock(a0);
}

std::chrono::gps_clock f72(std::chrono::gps_clock &&a0) {
  return std::chrono::gps_clock(std::move(a0));
}

std::chrono::gps_clock &f73(std::chrono::gps_clock &a0,
                            const std::chrono::gps_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::gps_clock &f74(std::chrono::gps_clock &a0,
                            std::chrono::gps_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f75(const std::chrono::gps_time<T1> &a0) {
  return std::chrono::gps_clock::to_utc(a0);
}

template <typename T1>
std::chrono::gps_time<std::common_type_t<T1, std::chrono::seconds>>
f76(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::gps_clock::from_utc(a0);
}

bool f77() { return std::chrono::gps_clock::is_steady; }

using t29 = std::chrono::gps_seconds;

template <typename T1> using t30 = std::chrono::gps_time<T1>;

bool f78(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

bool f79(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

using t31 = std::chrono::last_spec;

std::chrono::last_spec f80() { return std::chrono::last_spec(); }

std::chrono::last_spec f81(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f82(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f83(std::chrono::last_spec &a0,
                            const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f84(std::chrono::last_spec &a0,
                            std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::chrono::leap_second;

std::chrono::leap_second f85(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f86(std::chrono::leap_second &a0,
                              const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f87(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f88(const std::chrono::leap_second &a0) {
  return a0.value();
}

std::strong_ordering f89(const std::chrono::leap_second &a0,
                         const std::chrono::leap_second &a1) {
  return operator<=>(a0, a1);
}

bool f90(const std::chrono::leap_second &a0,
         const std::chrono::leap_second &a1) {
  return operator==(a0, a1);
}

using t33 = std::chrono::leap_second_info;

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

using t34 = std::chrono::local_days;

using t35 = std::chrono::local_info;

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

using t36 = std::chrono::local_seconds;

using t37 = std::chrono::local_t;

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

template <typename T1> using t38 = std::chrono::local_time<T1>;

const std::chrono::time_zone *f109(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

std::chrono::hours f110(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f111(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}

using t39 = std::chrono::month;

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

using t40 = std::chrono::month_day;

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

using t41 = std::chrono::month_day_last;

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

using t42 = std::chrono::month_weekday;

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

using t43 = std::chrono::month_weekday_last;

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

using t44 = std::chrono::months;

using t45 = std::chrono::nonexistent_local_time;

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
auto f193(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f194(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f195(const std::chrono::zoned_time<T1, T2> &a0,
          const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

std::string f196() { return std::chrono::remote_version(); }

using t46 = std::chrono::sys_days;

using t47 = std::chrono::sys_info;

std::chrono::sys_info f197() { return std::chrono::sys_info(); }

std::chrono::sys_info f198(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f199(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f200(std::chrono::sys_info &a0,
                            const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f201(std::chrono::sys_info &a0,
                            std::chrono::sys_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t48 = std::chrono::sys_seconds;

template <typename T1> using t49 = std::chrono::sys_time<T1>;

using t50 = std::chrono::tai_clock;

using t51 = std::chrono::tai_clock::rep;

using t52 = std::chrono::tai_clock::period;

using t53 = std::chrono::tai_clock::duration;

using t54 = std::chrono::tai_clock::time_point;

std::chrono::tai_clock::time_point f202() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f203() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f204(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f205(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f206(std::chrono::tai_clock &a0,
                             const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f207(std::chrono::tai_clock &a0,
                             std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f208(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f209(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

bool f210() { return std::chrono::tai_clock::is_steady; }

using t55 = std::chrono::tai_seconds;

template <typename T1> using t56 = std::chrono::tai_time<T1>;

using t57 = std::chrono::time_zone;

std::string_view f211(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f212(const std::chrono::time_zone &a0,
                           const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f213(const std::chrono::time_zone &a0,
                             const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f214(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f215(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
     std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f216(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}

std::strong_ordering f217(const std::chrono::time_zone &a0,
                          const std::chrono::time_zone &a1) {
  return operator<=>(a0, a1);
}

bool f218(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return operator==(a0, a1);
}

using t58 = std::chrono::time_zone_link;

std::chrono::time_zone_link f219(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f220(std::chrono::time_zone_link &a0,
                                  std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f221(const std::chrono::time_zone_link &a0) {
  return a0.name();
}

std::string_view f222(const std::chrono::time_zone_link &a0) {
  return a0.target();
}

std::strong_ordering f223(const std::chrono::time_zone_link &a0,
                          const std::chrono::time_zone_link &a1) {
  return operator<=>(a0, a1);
}

bool f224(const std::chrono::time_zone_link &a0,
          const std::chrono::time_zone_link &a1) {
  return operator==(a0, a1);
}

using t59 = std::chrono::tzdb;

const std::chrono::time_zone *f225(const std::chrono::tzdb &a0,
                                   std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f226(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f227() { return std::chrono::tzdb(); }

std::chrono::tzdb f228(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f229(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}

using t60 = std::chrono::tzdb_list;

using t61 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f230(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f231(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f232(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f233(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f234(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f235(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t62 = std::chrono::utc_clock;

using t63 = std::chrono::utc_clock::rep;

using t64 = std::chrono::utc_clock::period;

using t65 = std::chrono::utc_clock::duration;

using t66 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f236() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f237() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f238(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f239(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f240(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f241(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f242(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f243(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

bool f244() { return std::chrono::utc_clock::is_steady; }

using t67 = std::chrono::utc_seconds;

template <typename T1> using t68 = std::chrono::utc_time<T1>;

using t69 = std::chrono::weekday;

std::chrono::weekday f245() { return std::chrono::weekday(); }

std::chrono::weekday f246(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f247(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f248(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f249(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f250(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f251(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f252(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f253(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f254(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f255(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f256(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f257(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f258(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f259(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f260(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f261(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f262(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f263(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f264(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f265(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f266(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::weekday f267(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator+(a0, a1);
}

bool f268(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return operator==(a0, a1);
}

using t70 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f269() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f270(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f271(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f272(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f273(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f274(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f275(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f276(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f277(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f278(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return operator==(a0, a1);
}

using t71 = std::chrono::weekday_last;

std::chrono::weekday_last f279(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f280(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f281(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f282(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f283(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f284(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f285(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f286(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return operator==(a0, a1);
}

using t72 = std::chrono::weeks;

using t73 = std::chrono::year;

std::chrono::year f287() { return std::chrono::year(); }

std::chrono::year f288(int a0) { return std::chrono::year(a0); }

std::chrono::year f289() { return std::chrono::year::min(); }

std::chrono::year f290() { return std::chrono::year::max(); }

std::chrono::year &f291(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f292(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f293(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f294(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f295(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f296(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f297(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f298(const std::chrono::year &a0) { return a0.operator-(); }

bool f299(const std::chrono::year &a0) { return a0.is_leap(); }

int f300(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f301(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f302(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f303(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f304(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f305(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f306(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return operator-(a0, a1);
}

std::chrono::year f307(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year f308(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return operator+(a0, a1);
}

std::chrono::year f309(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f310(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return operator<=>(a0, a1);
}

bool f311(const std::chrono::year &a0, const std::chrono::year &a1) {
  return operator==(a0, a1);
}

using t74 = std::chrono::year_month;

std::chrono::year_month f312() { return std::chrono::year_month(); }

std::chrono::year_month f313(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f314(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f315(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f316(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f317(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f318(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f319(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f320(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f321(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f322(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f323(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f324(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month f325(const std::chrono::year &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f326(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f327(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f328(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f329(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::months f330(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return operator-(a0, a1);
}

std::strong_ordering f331(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return operator<=>(a0, a1);
}

bool f332(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return operator==(a0, a1);
}

using t75 = std::chrono::year_month_day;

std::chrono::year_month_day f333() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f334(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f335(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f336(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f337(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f338(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f339(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f340(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f341(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f342(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f343(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f344(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f345(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f346(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f347(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f348(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f349(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f350(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f351(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day f352(const std::chrono::month_day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f353(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f354(int a0, const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f355(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f356(const std::chrono::year_month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f357(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f358(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f359(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f360(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f361(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return operator<=>(a0, a1);
}

bool f362(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return operator==(a0, a1);
}

using t76 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f363(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f364(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f365(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f366(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f367(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f368(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f369(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f370(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f371(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f372(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f373(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f374(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f375(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f376(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f377(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f378(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last f379(const std::chrono::month_day_last &a0,
                                      int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f380(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f381(int a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f382(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f383(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::year_month_day_last
f384(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f385(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f386(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f387(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f388(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return operator==(a0, a1);
}

using t77 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f389() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f390(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f391(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f392(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f393(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f394(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f395(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f396(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f397(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f398(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f399(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f400(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f401(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f402(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f403(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f404(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f405(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f406(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f407(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f408(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday f409(const std::chrono::month_weekday &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f410(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f411(int a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f412(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f413(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f414(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday
f415(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f416(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

bool f417(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return operator==(a0, a1);
}

using t78 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f418(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f419(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f420(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f421(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f422(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f423(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f424(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f425(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f426(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f427(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f428(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f429(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f430(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f431(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f432(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f433(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last
f434(const std::chrono::month_weekday_last &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f435(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f436(int a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f437(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last f438(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f439(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f440(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f441(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

bool f442(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t79 = std::chrono::years;

using t80 = std::chrono::zoned_seconds;
