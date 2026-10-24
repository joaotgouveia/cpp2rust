#include <chrono>
#include <compare>
#include <iosfwd>
#include <limits>
#include <string_view>
#include <type_traits>
#include <utility>

using t18 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f43(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time f44(std::chrono::ambiguous_local_time &&a0) {
  return std::chrono::ambiguous_local_time(std::move(a0));
}

std::chrono::ambiguous_local_time &
f45(std::chrono::ambiguous_local_time &a0,
    const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::ambiguous_local_time &f46(std::chrono::ambiguous_local_time &a0,
                                       std::chrono::ambiguous_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::ambiguous_local_time f47(const std::chrono::local_time<T1> &a0,
                                      const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f48(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

using t19 = std::chrono::choose;

std::chrono::choose f49() { return std::chrono::choose::earliest; }

std::chrono::choose f50() { return std::chrono::choose::latest; }

template <typename T1, typename T2, typename T3>
auto f51(const std::chrono::time_point<T2, T3> &a0)
  requires(std::chrono::__detail::__clock_convs<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_sys<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_utc<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_sys_utc<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_utc_sys<T1, T2, T3>)
{
  return std::chrono::clock_cast<T1, T2, T3>(a0);
}

const std::chrono::time_zone *f52() { return std::chrono::current_zone(); }

using t20 = std::chrono::day;

std::chrono::day f53() { return std::chrono::day(); }

std::chrono::day f54(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f55(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f56(std::chrono::day &a0, int a1) { return a0.operator++(a1); }

std::chrono::day &f57(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f58(std::chrono::day &a0, int a1) { return a0.operator--(a1); }

std::chrono::day &f59(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f60(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f61(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f62(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f63(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f64(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f65(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f66(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f67(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator-(a0, a1);
}

std::chrono::day f68(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::day f69(const std::chrono::days &a0, const std::chrono::day &a1) {
  return operator+(a0, a1);
}

std::chrono::day f70(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f71(const std::chrono::day &a0,
                         const std::chrono::day &a1) {
  return operator<=>(a0, a1);
}

bool f72(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator==(a0, a1);
}

using t21 = std::chrono::days;

using t22 = std::chrono::file_clock;

template <typename T1> using t23 = std::chrono::file_time<T1>;

template <typename T1>
std::chrono::leap_second_info f73(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t24 = std::chrono::gps_clock;

using t25 = std::chrono::gps_clock::rep;

using t26 = std::chrono::gps_clock::period;

using t27 = std::chrono::gps_clock::duration;

using t28 = std::chrono::gps_clock::time_point;

std::chrono::gps_clock::time_point f74() {
  return std::chrono::gps_clock::now();
}

std::chrono::gps_clock f75() { return std::chrono::gps_clock(); }

std::chrono::gps_clock f76(const std::chrono::gps_clock &a0) {
  return std::chrono::gps_clock(a0);
}

std::chrono::gps_clock f77(std::chrono::gps_clock &&a0) {
  return std::chrono::gps_clock(std::move(a0));
}

std::chrono::gps_clock &f78(std::chrono::gps_clock &a0,
                            const std::chrono::gps_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::gps_clock &f79(std::chrono::gps_clock &a0,
                            std::chrono::gps_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f80(const std::chrono::gps_time<T1> &a0) {
  return std::chrono::gps_clock::to_utc(a0);
}

template <typename T1>
std::chrono::gps_time<std::common_type_t<T1, std::chrono::seconds>>
f81(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::gps_clock::from_utc(a0);
}

bool f82() { return std::chrono::gps_clock::is_steady; }

using t29 = std::chrono::gps_seconds;

template <typename T1> using t30 = std::chrono::gps_time<T1>;

bool f83(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

bool f84(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

using t31 = std::chrono::last_spec;

std::chrono::last_spec f85() { return std::chrono::last_spec(); }

std::chrono::last_spec f86(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f87(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f88(std::chrono::last_spec &a0,
                            const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f89(std::chrono::last_spec &a0,
                            std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::chrono::leap_second;

std::chrono::leap_second f90(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f91(std::chrono::leap_second &a0,
                              const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f92(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f93(const std::chrono::leap_second &a0) {
  return a0.value();
}

template <typename T1>
auto f94(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1)
  requires(std::three_way_comparable_with<T1, std::chrono::seconds>)
{
  return operator<=>(a0, a1);
}

template <typename T1>
bool f95(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f96(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f97(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f98(const std::chrono::leap_second &a0,
         const std::chrono::sys_time<T1> &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f99(const std::chrono::sys_time<T1> &a0,
         const std::chrono::leap_second &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f100(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f101(const std::chrono::sys_time<T1> &a0,
          const std::chrono::leap_second &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f102(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f103(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator==(a0, a1);
}

std::strong_ordering f104(const std::chrono::leap_second &a0,
                          const std::chrono::leap_second &a1) {
  return operator<=>(a0, a1);
}

bool f105(const std::chrono::leap_second &a0,
          const std::chrono::leap_second &a1) {
  return operator==(a0, a1);
}

using t33 = std::chrono::leap_second_info;

std::chrono::leap_second_info f106() { return std::chrono::leap_second_info(); }

std::chrono::leap_second_info f107(const std::chrono::leap_second_info &a0) {
  return std::chrono::leap_second_info(a0);
}

std::chrono::leap_second_info f108(std::chrono::leap_second_info &&a0) {
  return std::chrono::leap_second_info(std::move(a0));
}

std::chrono::leap_second_info &f109(std::chrono::leap_second_info &a0,
                                    const std::chrono::leap_second_info &a1) {
  return a0.operator=(a1);
}

std::chrono::leap_second_info &f110(std::chrono::leap_second_info &a0,
                                    std::chrono::leap_second_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t34 = std::chrono::local_days;

using t35 = std::chrono::local_info;

std::chrono::local_info f111() { return std::chrono::local_info(); }

std::chrono::local_info f112(const std::chrono::local_info &a0) {
  return std::chrono::local_info(a0);
}

std::chrono::local_info f113(std::chrono::local_info &&a0) {
  return std::chrono::local_info(std::move(a0));
}

std::chrono::local_info &f114(std::chrono::local_info &a0,
                              const std::chrono::local_info &a1) {
  return a0.operator=(a1);
}

std::chrono::local_info &f115(std::chrono::local_info &a0,
                              std::chrono::local_info &&a1) {
  return a0.operator=(std::move(a1));
}

int f116() { return std::chrono::local_info::unique; }

int f117() { return std::chrono::local_info::nonexistent; }

int f118() { return std::chrono::local_info::ambiguous; }

using t36 = std::chrono::local_seconds;

using t37 = std::chrono::local_t;

std::chrono::local_t f119() { return std::chrono::local_t(); }

std::chrono::local_t f120(const std::chrono::local_t &a0) {
  return std::chrono::local_t(a0);
}

std::chrono::local_t f121(std::chrono::local_t &&a0) {
  return std::chrono::local_t(std::move(a0));
}

std::chrono::local_t &f122(std::chrono::local_t &a0,
                           const std::chrono::local_t &a1) {
  return a0.operator=(a1);
}

std::chrono::local_t &f123(std::chrono::local_t &a0,
                           std::chrono::local_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t38 = std::chrono::local_time<T1>;

const std::chrono::time_zone *f124(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

std::chrono::hours f125(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f126(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}

using t39 = std::chrono::month;

std::chrono::month f127() { return std::chrono::month(); }

std::chrono::month f128(unsigned int a0) { return std::chrono::month(a0); }

std::chrono::month &f129(std::chrono::month &a0) { return a0.operator++(); }

std::chrono::month f130(std::chrono::month &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::month &f131(std::chrono::month &a0) { return a0.operator--(); }

std::chrono::month f132(std::chrono::month &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::month &f133(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::month &f134(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

unsigned int f135(const std::chrono::month &a0) {
  return static_cast<unsigned int>(a0);
}

bool f136(const std::chrono::month &a0) { return a0.ok(); }

std::chrono::month f137(const std::chrono::month &a0) {
  return std::chrono::month(a0);
}

std::chrono::month f138(std::chrono::month &&a0) {
  return std::chrono::month(std::move(a0));
}

std::chrono::month &f139(std::chrono::month &a0, const std::chrono::month &a1) {
  return a0.operator=(a1);
}

std::chrono::month &f140(std::chrono::month &a0, std::chrono::month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::months f141(const std::chrono::month &a0,
                         const std::chrono::month &a1) {
  return operator-(a0, a1);
}

std::chrono::month f142(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::month f143(const std::chrono::months &a0,
                        const std::chrono::month &a1) {
  return operator+(a0, a1);
}

std::chrono::month f144(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f145(const std::chrono::month &a0,
                          const std::chrono::month &a1) {
  return operator<=>(a0, a1);
}

bool f146(const std::chrono::month &a0, const std::chrono::month &a1) {
  return operator==(a0, a1);
}

using t40 = std::chrono::month_day;

std::chrono::month_day f147() { return std::chrono::month_day(); }

std::chrono::month_day f148(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return std::chrono::month_day(a0, a1);
}

std::chrono::month f149(const std::chrono::month_day &a0) { return a0.month(); }

std::chrono::day f150(const std::chrono::month_day &a0) { return a0.day(); }

bool f151(const std::chrono::month_day &a0) { return a0.ok(); }

std::chrono::month_day f152(const std::chrono::month_day &a0) {
  return std::chrono::month_day(a0);
}

std::chrono::month_day f153(std::chrono::month_day &&a0) {
  return std::chrono::month_day(std::move(a0));
}

std::chrono::month_day &f154(std::chrono::month_day &a0,
                             const std::chrono::month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day &f155(std::chrono::month_day &a0,
                             std::chrono::month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day f156(const std::chrono::day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f157(const std::chrono::day &a0,
                            const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f158(int a0, const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f159(const std::chrono::month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f160(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::strong_ordering f161(const std::chrono::month_day &a0,
                          const std::chrono::month_day &a1) {
  return operator<=>(a0, a1);
}

bool f162(const std::chrono::month_day &a0, const std::chrono::month_day &a1) {
  return operator==(a0, a1);
}

using t41 = std::chrono::month_day_last;

std::chrono::month_day_last f163(const std::chrono::month &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month f164(const std::chrono::month_day_last &a0) {
  return a0.month();
}

bool f165(const std::chrono::month_day_last &a0) { return a0.ok(); }

std::chrono::month_day_last f166(const std::chrono::month_day_last &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month_day_last f167(std::chrono::month_day_last &&a0) {
  return std::chrono::month_day_last(std::move(a0));
}

std::chrono::month_day_last &f168(std::chrono::month_day_last &a0,
                                  const std::chrono::month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day_last &f169(std::chrono::month_day_last &a0,
                                  std::chrono::month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day_last f170(std::chrono::last_spec a0, int a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f171(std::chrono::last_spec a0,
                                 const std::chrono::month &a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f172(int a0, std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::month_day_last f173(const std::chrono::month &a0,
                                 std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::strong_ordering f174(const std::chrono::month_day_last &a0,
                          const std::chrono::month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f175(const std::chrono::month_day_last &a0,
          const std::chrono::month_day_last &a1) {
  return operator==(a0, a1);
}

using t42 = std::chrono::month_weekday;

std::chrono::month_weekday f176(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return std::chrono::month_weekday(a0, a1);
}

std::chrono::month f177(const std::chrono::month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday_indexed f178(const std::chrono::month_weekday &a0) {
  return a0.weekday_indexed();
}

bool f179(const std::chrono::month_weekday &a0) { return a0.ok(); }

std::chrono::month_weekday f180(const std::chrono::month_weekday &a0) {
  return std::chrono::month_weekday(a0);
}

std::chrono::month_weekday f181(std::chrono::month_weekday &&a0) {
  return std::chrono::month_weekday(std::move(a0));
}

std::chrono::month_weekday &f182(std::chrono::month_weekday &a0,
                                 const std::chrono::month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday &f183(std::chrono::month_weekday &a0,
                                 std::chrono::month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday f184(const std::chrono::weekday_indexed &a0,
                                int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f185(const std::chrono::weekday_indexed &a0,
                                const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f186(int a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f187(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

bool f188(const std::chrono::month_weekday &a0,
          const std::chrono::month_weekday &a1) {
  return operator==(a0, a1);
}

using t43 = std::chrono::month_weekday_last;

std::chrono::month_weekday_last f189(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return std::chrono::month_weekday_last(a0, a1);
}

std::chrono::month f190(const std::chrono::month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday_last f191(const std::chrono::month_weekday_last &a0) {
  return a0.weekday_last();
}

bool f192(const std::chrono::month_weekday_last &a0) { return a0.ok(); }

std::chrono::month_weekday_last
f193(const std::chrono::month_weekday_last &a0) {
  return std::chrono::month_weekday_last(a0);
}

std::chrono::month_weekday_last f194(std::chrono::month_weekday_last &&a0) {
  return std::chrono::month_weekday_last(std::move(a0));
}

std::chrono::month_weekday_last &
f195(std::chrono::month_weekday_last &a0,
     const std::chrono::month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday_last &f196(std::chrono::month_weekday_last &a0,
                                      std::chrono::month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday_last f197(const std::chrono::weekday_last &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f198(const std::chrono::weekday_last &a0,
                                     const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f199(int a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f200(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

bool f201(const std::chrono::month_weekday_last &a0,
          const std::chrono::month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t44 = std::chrono::months;

using t45 = std::chrono::nonexistent_local_time;

std::chrono::nonexistent_local_time
f202(const std::chrono::nonexistent_local_time &a0) {
  return std::chrono::nonexistent_local_time(a0);
}

std::chrono::nonexistent_local_time
f203(std::chrono::nonexistent_local_time &&a0) {
  return std::chrono::nonexistent_local_time(std::move(a0));
}

std::chrono::nonexistent_local_time &
f204(std::chrono::nonexistent_local_time &a0,
     const std::chrono::nonexistent_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::nonexistent_local_time &
f205(std::chrono::nonexistent_local_time &a0,
     std::chrono::nonexistent_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::nonexistent_local_time f206(const std::chrono::local_time<T1> &a0,
                                         const std::chrono::local_info &a1) {
  return std::chrono::nonexistent_local_time(a0, a1);
}

const char *f207(const std::chrono::nonexistent_local_time &a0) {
  return a0.what();
}

template <typename T1, typename T2, typename T3>
auto f208(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1)
  requires(std::three_way_comparable_with<T3, T2>)
{
  return std::chrono::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f209(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1)
  requires(std::three_way_comparable<std::common_type_t<T1, T3>>)
{
  return std::chrono::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f210(const std::chrono::zoned_time<T1, T2> &a0,
          const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

std::string f211() { return std::chrono::remote_version(); }

using t46 = std::chrono::sys_days;

using t47 = std::chrono::sys_info;

std::chrono::sys_info f212() { return std::chrono::sys_info(); }

std::chrono::sys_info f213(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f214(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f215(std::chrono::sys_info &a0,
                            const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f216(std::chrono::sys_info &a0,
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

std::chrono::tai_clock::time_point f217() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f218() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f219(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f220(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f221(std::chrono::tai_clock &a0,
                             const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f222(std::chrono::tai_clock &a0,
                             std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f223(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f224(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

bool f225() { return std::chrono::tai_clock::is_steady; }

using t55 = std::chrono::tai_seconds;

template <typename T1> using t56 = std::chrono::tai_time<T1>;

using t57 = std::chrono::time_zone;

std::string_view f226(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f227(const std::chrono::time_zone &a0,
                           const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f228(const std::chrono::time_zone &a0,
                             const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f229(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f230(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
     std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f231(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}

std::strong_ordering f232(const std::chrono::time_zone &a0,
                          const std::chrono::time_zone &a1) {
  return operator<=>(a0, a1);
}

bool f233(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return operator==(a0, a1);
}

using t58 = std::chrono::time_zone_link;

std::chrono::time_zone_link f234(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f235(std::chrono::time_zone_link &a0,
                                  std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f236(const std::chrono::time_zone_link &a0) {
  return a0.name();
}

std::string_view f237(const std::chrono::time_zone_link &a0) {
  return a0.target();
}

std::strong_ordering f238(const std::chrono::time_zone_link &a0,
                          const std::chrono::time_zone_link &a1) {
  return operator<=>(a0, a1);
}

bool f239(const std::chrono::time_zone_link &a0,
          const std::chrono::time_zone_link &a1) {
  return operator==(a0, a1);
}

using t59 = std::chrono::tzdb;

const std::chrono::time_zone *f240(const std::chrono::tzdb &a0,
                                   std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f241(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f242() { return std::chrono::tzdb(); }

std::chrono::tzdb f243(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f244(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}

using t60 = std::chrono::tzdb_list;

using t61 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f245(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f246(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f247(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f248(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f249(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f250(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t62 = std::chrono::utc_clock;

using t63 = std::chrono::utc_clock::rep;

using t64 = std::chrono::utc_clock::period;

using t65 = std::chrono::utc_clock::duration;

using t66 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f251() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f252() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f253(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f254(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f255(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f256(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f257(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f258(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

bool f259() { return std::chrono::utc_clock::is_steady; }

using t67 = std::chrono::utc_seconds;

template <typename T1> using t68 = std::chrono::utc_time<T1>;

using t69 = std::chrono::weekday;

std::chrono::weekday f260() { return std::chrono::weekday(); }

std::chrono::weekday f261(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f262(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f263(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f264(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f265(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f266(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f267(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f268(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f269(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f270(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f271(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f272(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f273(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f274(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f275(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f276(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f277(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f278(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f279(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f280(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f281(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::weekday f282(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator+(a0, a1);
}

bool f283(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return operator==(a0, a1);
}

using t70 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f284() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f285(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f286(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f287(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f288(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f289(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f290(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f291(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f292(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f293(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return operator==(a0, a1);
}

using t71 = std::chrono::weekday_last;

std::chrono::weekday_last f294(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f295(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f296(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f297(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f298(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f299(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f300(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f301(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return operator==(a0, a1);
}

using t72 = std::chrono::weeks;

using t73 = std::chrono::year;

std::chrono::year f302() { return std::chrono::year(); }

std::chrono::year f303(int a0) { return std::chrono::year(a0); }

std::chrono::year f304() { return std::chrono::year::min(); }

std::chrono::year f305() { return std::chrono::year::max(); }

std::chrono::year &f306(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f307(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f308(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f309(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f310(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f311(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f312(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f313(const std::chrono::year &a0) { return a0.operator-(); }

bool f314(const std::chrono::year &a0) { return a0.is_leap(); }

int f315(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f316(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f317(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f318(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f319(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f320(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f321(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return operator-(a0, a1);
}

std::chrono::year f322(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year f323(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return operator+(a0, a1);
}

std::chrono::year f324(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f325(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return operator<=>(a0, a1);
}

bool f326(const std::chrono::year &a0, const std::chrono::year &a1) {
  return operator==(a0, a1);
}

using t74 = std::chrono::year_month;

std::chrono::year_month f327() { return std::chrono::year_month(); }

std::chrono::year_month f328(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f329(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f330(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f331(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f332(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f333(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f334(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f335(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f336(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f337(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f338(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f339(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month f340(const std::chrono::year &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f341(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f342(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f343(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f344(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::months f345(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f346(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f347(const std::chrono::months &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f348(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f349(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return operator<=>(a0, a1);
}

bool f350(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return operator==(a0, a1);
}

using t75 = std::chrono::year_month_day;

std::chrono::year_month_day f351() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f352(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f353(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f354(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f355(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f356(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f357(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f358(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f359(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f360(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f361(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f362(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f363(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f364(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f365(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f366(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f367(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f368(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f369(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day f370(const std::chrono::month_day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f371(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f372(int a0, const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f373(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f374(const std::chrono::year_month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f375(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f376(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f377(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f378(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f379(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f380(const std::chrono::months &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f381(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f382(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return operator<=>(a0, a1);
}

bool f383(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return operator==(a0, a1);
}

using t76 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f384(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f385(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f386(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f387(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f388(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f389(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f390(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f391(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f392(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f393(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f394(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f395(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f396(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f397(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f398(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f399(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last f400(const std::chrono::month_day_last &a0,
                                      int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f401(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f402(int a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f403(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f404(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::year_month_day_last
f405(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f406(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f407(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f408(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f409(const std::chrono::months &a0,
     const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f410(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f411(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f412(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return operator==(a0, a1);
}

using t77 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f413() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f414(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f415(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f416(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f417(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f418(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f419(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f420(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f421(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f422(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f423(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f424(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f425(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f426(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f427(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f428(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f429(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f430(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f431(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f432(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday f433(const std::chrono::month_weekday &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f434(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f435(int a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f436(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f437(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f438(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday f439(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday
f440(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f441(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday
f442(const std::chrono::months &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday f443(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f444(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return operator==(a0, a1);
}

using t78 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f445(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f446(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f447(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f448(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f449(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f450(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f451(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f452(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f453(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f454(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f455(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f456(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f457(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f458(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f459(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f460(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last
f461(const std::chrono::month_weekday_last &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f462(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f463(int a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f464(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last f465(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f466(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f467(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f468(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f469(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f470(const std::chrono::months &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f471(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f472(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t79 = std::chrono::years;

using t80 = std::chrono::zoned_seconds;
