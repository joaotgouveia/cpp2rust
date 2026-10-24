#include <chrono>
#include <compare>
#include <iosfwd>
#include <limits>
#include <string_view>
#include <type_traits>
#include <utility>

using t18 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f50(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time f51(std::chrono::ambiguous_local_time &&a0) {
  return std::chrono::ambiguous_local_time(std::move(a0));
}

std::chrono::ambiguous_local_time &
f52(std::chrono::ambiguous_local_time &a0,
    const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::ambiguous_local_time &f53(std::chrono::ambiguous_local_time &a0,
                                       std::chrono::ambiguous_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::ambiguous_local_time f54(const std::chrono::local_time<T1> &a0,
                                      const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f55(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

using t19 = std::chrono::choose;

std::chrono::choose f56() { return std::chrono::choose::earliest; }

std::chrono::choose f57() { return std::chrono::choose::latest; }

template <typename T1, typename T2, typename T3>
auto f58(const std::chrono::time_point<T2, T3> &a0)
  requires(std::chrono::__detail::__clock_convs<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_sys<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_utc<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_sys_utc<T1, T2, T3> ||
           std::chrono::__detail::__clock_convs_utc_sys<T1, T2, T3>)
{
  return std::chrono::clock_cast<T1, T2, T3>(a0);
}

const std::chrono::time_zone *f59() { return std::chrono::current_zone(); }

using t20 = std::chrono::day;

std::chrono::day f60() { return std::chrono::day(); }

std::chrono::day f61(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f62(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f63(std::chrono::day &a0, int a1) { return a0.operator++(a1); }

std::chrono::day &f64(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f65(std::chrono::day &a0, int a1) { return a0.operator--(a1); }

std::chrono::day &f66(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f67(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f68(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f69(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f70(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f71(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f72(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f73(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f74(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator-(a0, a1);
}

std::chrono::day f75(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::day f76(const std::chrono::days &a0, const std::chrono::day &a1) {
  return operator+(a0, a1);
}

std::chrono::day f77(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f78(const std::chrono::day &a0,
                         const std::chrono::day &a1) {
  return operator<=>(a0, a1);
}

bool f79(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator==(a0, a1);
}

using t21 = std::chrono::days;

using t22 = std::chrono::file_clock;

template <typename T1> using t23 = std::chrono::file_time<T1>;

template <typename T1>
std::chrono::leap_second_info f80(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t24 = std::chrono::gps_clock;

using t25 = std::chrono::gps_clock::rep;

using t26 = std::chrono::gps_clock::period;

using t27 = std::chrono::gps_clock::duration;

using t28 = std::chrono::gps_clock::time_point;

std::chrono::gps_clock::time_point f81() {
  return std::chrono::gps_clock::now();
}

std::chrono::gps_clock f82() { return std::chrono::gps_clock(); }

std::chrono::gps_clock f83(const std::chrono::gps_clock &a0) {
  return std::chrono::gps_clock(a0);
}

std::chrono::gps_clock f84(std::chrono::gps_clock &&a0) {
  return std::chrono::gps_clock(std::move(a0));
}

std::chrono::gps_clock &f85(std::chrono::gps_clock &a0,
                            const std::chrono::gps_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::gps_clock &f86(std::chrono::gps_clock &a0,
                            std::chrono::gps_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f87(const std::chrono::gps_time<T1> &a0) {
  return std::chrono::gps_clock::to_utc(a0);
}

template <typename T1>
std::chrono::gps_time<std::common_type_t<T1, std::chrono::seconds>>
f88(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::gps_clock::from_utc(a0);
}

bool f89() { return std::chrono::gps_clock::is_steady; }

using t29 = std::chrono::gps_seconds;

template <typename T1> using t30 = std::chrono::gps_time<T1>;

bool f90(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

bool f91(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

using t31 = std::chrono::last_spec;

std::chrono::last_spec f92() { return std::chrono::last_spec(); }

std::chrono::last_spec f93(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f94(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f95(std::chrono::last_spec &a0,
                            const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f96(std::chrono::last_spec &a0,
                            std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::chrono::leap_second;

std::chrono::leap_second f97(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f98(std::chrono::leap_second &a0,
                              const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f99(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f100(const std::chrono::leap_second &a0) {
  return a0.value();
}

template <typename T1>
auto f101(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1)
  requires(std::three_way_comparable_with<T1, std::chrono::seconds>)
{
  return operator<=>(a0, a1);
}

template <typename T1>
bool f102(const std::chrono::sys_time<T1> &a0,
          const std::chrono::leap_second &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f103(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f104(const std::chrono::sys_time<T1> &a0,
          const std::chrono::leap_second &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f105(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f106(const std::chrono::sys_time<T1> &a0,
          const std::chrono::leap_second &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f107(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f108(const std::chrono::sys_time<T1> &a0,
          const std::chrono::leap_second &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f109(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f110(const std::chrono::leap_second &a0,
          const std::chrono::sys_time<T1> &a1) {
  return operator==(a0, a1);
}

std::strong_ordering f111(const std::chrono::leap_second &a0,
                          const std::chrono::leap_second &a1) {
  return operator<=>(a0, a1);
}

bool f112(const std::chrono::leap_second &a0,
          const std::chrono::leap_second &a1) {
  return operator==(a0, a1);
}

using t33 = std::chrono::leap_second_info;

std::chrono::leap_second_info f113() { return std::chrono::leap_second_info(); }

std::chrono::leap_second_info f114(const std::chrono::leap_second_info &a0) {
  return std::chrono::leap_second_info(a0);
}

std::chrono::leap_second_info f115(std::chrono::leap_second_info &&a0) {
  return std::chrono::leap_second_info(std::move(a0));
}

std::chrono::leap_second_info &f116(std::chrono::leap_second_info &a0,
                                    const std::chrono::leap_second_info &a1) {
  return a0.operator=(a1);
}

std::chrono::leap_second_info &f117(std::chrono::leap_second_info &a0,
                                    std::chrono::leap_second_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t34 = std::chrono::local_days;

using t35 = std::chrono::local_info;

std::chrono::local_info f118() { return std::chrono::local_info(); }

std::chrono::local_info f119(const std::chrono::local_info &a0) {
  return std::chrono::local_info(a0);
}

std::chrono::local_info f120(std::chrono::local_info &&a0) {
  return std::chrono::local_info(std::move(a0));
}

std::chrono::local_info &f121(std::chrono::local_info &a0,
                              const std::chrono::local_info &a1) {
  return a0.operator=(a1);
}

std::chrono::local_info &f122(std::chrono::local_info &a0,
                              std::chrono::local_info &&a1) {
  return a0.operator=(std::move(a1));
}

int f123() { return std::chrono::local_info::unique; }

int f124() { return std::chrono::local_info::nonexistent; }

int f125() { return std::chrono::local_info::ambiguous; }

using t36 = std::chrono::local_seconds;

using t37 = std::chrono::local_t;

std::chrono::local_t f126() { return std::chrono::local_t(); }

std::chrono::local_t f127(const std::chrono::local_t &a0) {
  return std::chrono::local_t(a0);
}

std::chrono::local_t f128(std::chrono::local_t &&a0) {
  return std::chrono::local_t(std::move(a0));
}

std::chrono::local_t &f129(std::chrono::local_t &a0,
                           const std::chrono::local_t &a1) {
  return a0.operator=(a1);
}

std::chrono::local_t &f130(std::chrono::local_t &a0,
                           std::chrono::local_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t38 = std::chrono::local_time<T1>;

const std::chrono::time_zone *f131(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

std::chrono::hours f132(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f133(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}

using t39 = std::chrono::month;

std::chrono::month f134() { return std::chrono::month(); }

std::chrono::month f135(unsigned int a0) { return std::chrono::month(a0); }

std::chrono::month &f136(std::chrono::month &a0) { return a0.operator++(); }

std::chrono::month f137(std::chrono::month &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::month &f138(std::chrono::month &a0) { return a0.operator--(); }

std::chrono::month f139(std::chrono::month &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::month &f140(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::month &f141(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

unsigned int f142(const std::chrono::month &a0) {
  return static_cast<unsigned int>(a0);
}

bool f143(const std::chrono::month &a0) { return a0.ok(); }

std::chrono::month f144(const std::chrono::month &a0) {
  return std::chrono::month(a0);
}

std::chrono::month f145(std::chrono::month &&a0) {
  return std::chrono::month(std::move(a0));
}

std::chrono::month &f146(std::chrono::month &a0, const std::chrono::month &a1) {
  return a0.operator=(a1);
}

std::chrono::month &f147(std::chrono::month &a0, std::chrono::month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::months f148(const std::chrono::month &a0,
                         const std::chrono::month &a1) {
  return operator-(a0, a1);
}

std::chrono::month f149(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::month f150(const std::chrono::months &a0,
                        const std::chrono::month &a1) {
  return operator+(a0, a1);
}

std::chrono::month f151(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f152(const std::chrono::month &a0,
                          const std::chrono::month &a1) {
  return operator<=>(a0, a1);
}

bool f153(const std::chrono::month &a0, const std::chrono::month &a1) {
  return operator==(a0, a1);
}

using t40 = std::chrono::month_day;

std::chrono::month_day f154() { return std::chrono::month_day(); }

std::chrono::month_day f155(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return std::chrono::month_day(a0, a1);
}

std::chrono::month f156(const std::chrono::month_day &a0) { return a0.month(); }

std::chrono::day f157(const std::chrono::month_day &a0) { return a0.day(); }

bool f158(const std::chrono::month_day &a0) { return a0.ok(); }

std::chrono::month_day f159(const std::chrono::month_day &a0) {
  return std::chrono::month_day(a0);
}

std::chrono::month_day f160(std::chrono::month_day &&a0) {
  return std::chrono::month_day(std::move(a0));
}

std::chrono::month_day &f161(std::chrono::month_day &a0,
                             const std::chrono::month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day &f162(std::chrono::month_day &a0,
                             std::chrono::month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day f163(const std::chrono::day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f164(const std::chrono::day &a0,
                            const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f165(int a0, const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f166(const std::chrono::month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f167(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::strong_ordering f168(const std::chrono::month_day &a0,
                          const std::chrono::month_day &a1) {
  return operator<=>(a0, a1);
}

bool f169(const std::chrono::month_day &a0, const std::chrono::month_day &a1) {
  return operator==(a0, a1);
}

using t41 = std::chrono::month_day_last;

std::chrono::month_day_last f170(const std::chrono::month &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month f171(const std::chrono::month_day_last &a0) {
  return a0.month();
}

bool f172(const std::chrono::month_day_last &a0) { return a0.ok(); }

std::chrono::month_day_last f173(const std::chrono::month_day_last &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month_day_last f174(std::chrono::month_day_last &&a0) {
  return std::chrono::month_day_last(std::move(a0));
}

std::chrono::month_day_last &f175(std::chrono::month_day_last &a0,
                                  const std::chrono::month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day_last &f176(std::chrono::month_day_last &a0,
                                  std::chrono::month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day_last f177(std::chrono::last_spec a0, int a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f178(std::chrono::last_spec a0,
                                 const std::chrono::month &a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f179(int a0, std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::month_day_last f180(const std::chrono::month &a0,
                                 std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::strong_ordering f181(const std::chrono::month_day_last &a0,
                          const std::chrono::month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f182(const std::chrono::month_day_last &a0,
          const std::chrono::month_day_last &a1) {
  return operator==(a0, a1);
}

using t42 = std::chrono::month_weekday;

std::chrono::month_weekday f183(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return std::chrono::month_weekday(a0, a1);
}

std::chrono::month f184(const std::chrono::month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday_indexed f185(const std::chrono::month_weekday &a0) {
  return a0.weekday_indexed();
}

bool f186(const std::chrono::month_weekday &a0) { return a0.ok(); }

std::chrono::month_weekday f187(const std::chrono::month_weekday &a0) {
  return std::chrono::month_weekday(a0);
}

std::chrono::month_weekday f188(std::chrono::month_weekday &&a0) {
  return std::chrono::month_weekday(std::move(a0));
}

std::chrono::month_weekday &f189(std::chrono::month_weekday &a0,
                                 const std::chrono::month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday &f190(std::chrono::month_weekday &a0,
                                 std::chrono::month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday f191(const std::chrono::weekday_indexed &a0,
                                int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f192(const std::chrono::weekday_indexed &a0,
                                const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f193(int a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f194(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

bool f195(const std::chrono::month_weekday &a0,
          const std::chrono::month_weekday &a1) {
  return operator==(a0, a1);
}

using t43 = std::chrono::month_weekday_last;

std::chrono::month_weekday_last f196(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return std::chrono::month_weekday_last(a0, a1);
}

std::chrono::month f197(const std::chrono::month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday_last f198(const std::chrono::month_weekday_last &a0) {
  return a0.weekday_last();
}

bool f199(const std::chrono::month_weekday_last &a0) { return a0.ok(); }

std::chrono::month_weekday_last
f200(const std::chrono::month_weekday_last &a0) {
  return std::chrono::month_weekday_last(a0);
}

std::chrono::month_weekday_last f201(std::chrono::month_weekday_last &&a0) {
  return std::chrono::month_weekday_last(std::move(a0));
}

std::chrono::month_weekday_last &
f202(std::chrono::month_weekday_last &a0,
     const std::chrono::month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday_last &f203(std::chrono::month_weekday_last &a0,
                                      std::chrono::month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday_last f204(const std::chrono::weekday_last &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f205(const std::chrono::weekday_last &a0,
                                     const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f206(int a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f207(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

bool f208(const std::chrono::month_weekday_last &a0,
          const std::chrono::month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t44 = std::chrono::months;

using t45 = std::chrono::nonexistent_local_time;

std::chrono::nonexistent_local_time
f209(const std::chrono::nonexistent_local_time &a0) {
  return std::chrono::nonexistent_local_time(a0);
}

std::chrono::nonexistent_local_time
f210(std::chrono::nonexistent_local_time &&a0) {
  return std::chrono::nonexistent_local_time(std::move(a0));
}

std::chrono::nonexistent_local_time &
f211(std::chrono::nonexistent_local_time &a0,
     const std::chrono::nonexistent_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::nonexistent_local_time &
f212(std::chrono::nonexistent_local_time &a0,
     std::chrono::nonexistent_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::nonexistent_local_time f213(const std::chrono::local_time<T1> &a0,
                                         const std::chrono::local_info &a1) {
  return std::chrono::nonexistent_local_time(a0, a1);
}

const char *f214(const std::chrono::nonexistent_local_time &a0) {
  return a0.what();
}

template <typename T1, typename T2, typename T3>
auto f215(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1)
  requires(std::three_way_comparable_with<T3, T2>)
{
  return std::chrono::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f216(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1)
  requires(std::three_way_comparable<std::common_type_t<T1, T3>>)
{
  return std::chrono::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f217(const std::chrono::zoned_time<T1, T2> &a0,
          const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

std::string f218() { return std::chrono::remote_version(); }

using t46 = std::chrono::sys_days;

using t47 = std::chrono::sys_info;

std::chrono::sys_info f219() { return std::chrono::sys_info(); }

std::chrono::sys_info f220(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f221(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f222(std::chrono::sys_info &a0,
                            const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f223(std::chrono::sys_info &a0,
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

std::chrono::tai_clock::time_point f224() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f225() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f226(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f227(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f228(std::chrono::tai_clock &a0,
                             const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f229(std::chrono::tai_clock &a0,
                             std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f230(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f231(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

bool f232() { return std::chrono::tai_clock::is_steady; }

using t55 = std::chrono::tai_seconds;

template <typename T1> using t56 = std::chrono::tai_time<T1>;

using t57 = std::chrono::time_zone;

std::string_view f233(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f234(const std::chrono::time_zone &a0,
                           const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f235(const std::chrono::time_zone &a0,
                             const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f236(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f237(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
     std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f238(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}

std::strong_ordering f239(const std::chrono::time_zone &a0,
                          const std::chrono::time_zone &a1) {
  return operator<=>(a0, a1);
}

bool f240(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return operator==(a0, a1);
}

using t58 = std::chrono::time_zone_link;

std::chrono::time_zone_link f241(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f242(std::chrono::time_zone_link &a0,
                                  std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f243(const std::chrono::time_zone_link &a0) {
  return a0.name();
}

std::string_view f244(const std::chrono::time_zone_link &a0) {
  return a0.target();
}

std::strong_ordering f245(const std::chrono::time_zone_link &a0,
                          const std::chrono::time_zone_link &a1) {
  return operator<=>(a0, a1);
}

bool f246(const std::chrono::time_zone_link &a0,
          const std::chrono::time_zone_link &a1) {
  return operator==(a0, a1);
}

using t59 = std::chrono::tzdb;

const std::chrono::time_zone *f247(const std::chrono::tzdb &a0,
                                   std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f248(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f249() { return std::chrono::tzdb(); }

std::chrono::tzdb f250(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f251(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}

using t60 = std::chrono::tzdb_list;

using t61 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f252(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f253(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f254(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f255(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f256(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f257(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t62 = std::chrono::utc_clock;

using t63 = std::chrono::utc_clock::rep;

using t64 = std::chrono::utc_clock::period;

using t65 = std::chrono::utc_clock::duration;

using t66 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f258() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f259() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f260(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f261(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f262(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f263(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f264(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f265(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

bool f266() { return std::chrono::utc_clock::is_steady; }

using t67 = std::chrono::utc_seconds;

template <typename T1> using t68 = std::chrono::utc_time<T1>;

using t69 = std::chrono::weekday;

std::chrono::weekday f267() { return std::chrono::weekday(); }

std::chrono::weekday f268(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f269(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f270(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f271(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f272(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f273(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f274(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f275(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f276(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f277(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f278(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f279(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f280(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f281(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f282(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f283(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f284(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f285(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f286(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f287(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f288(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::weekday f289(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator+(a0, a1);
}

bool f290(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return operator==(a0, a1);
}

using t70 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f291() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f292(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f293(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f294(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f295(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f296(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f297(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f298(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f299(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f300(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return operator==(a0, a1);
}

using t71 = std::chrono::weekday_last;

std::chrono::weekday_last f301(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f302(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f303(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f304(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f305(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f306(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f307(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f308(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return operator==(a0, a1);
}

using t72 = std::chrono::weeks;

using t73 = std::chrono::year;

std::chrono::year f309() { return std::chrono::year(); }

std::chrono::year f310(int a0) { return std::chrono::year(a0); }

std::chrono::year f311() { return std::chrono::year::min(); }

std::chrono::year f312() { return std::chrono::year::max(); }

std::chrono::year &f313(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f314(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f315(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f316(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f317(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f318(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f319(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f320(const std::chrono::year &a0) { return a0.operator-(); }

bool f321(const std::chrono::year &a0) { return a0.is_leap(); }

int f322(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f323(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f324(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f325(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f326(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f327(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f328(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return operator-(a0, a1);
}

std::chrono::year f329(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year f330(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return operator+(a0, a1);
}

std::chrono::year f331(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f332(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return operator<=>(a0, a1);
}

bool f333(const std::chrono::year &a0, const std::chrono::year &a1) {
  return operator==(a0, a1);
}

using t74 = std::chrono::year_month;

std::chrono::year_month f334() { return std::chrono::year_month(); }

std::chrono::year_month f335(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f336(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f337(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f338(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f339(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f340(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f341(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f342(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f343(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f344(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f345(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f346(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month f347(const std::chrono::year &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f348(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f349(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f350(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f351(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::months f352(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f353(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f354(const std::chrono::months &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f355(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f356(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return operator<=>(a0, a1);
}

bool f357(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return operator==(a0, a1);
}

using t75 = std::chrono::year_month_day;

std::chrono::year_month_day f358() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f359(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f360(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f361(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f362(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f363(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f364(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f365(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f366(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f367(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f368(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f369(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f370(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f371(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f372(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f373(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f374(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f375(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f376(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day f377(const std::chrono::month_day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f378(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f379(int a0, const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f380(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f381(const std::chrono::year_month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f382(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f383(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f384(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f385(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f386(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f387(const std::chrono::months &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f388(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f389(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return operator<=>(a0, a1);
}

bool f390(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return operator==(a0, a1);
}

using t76 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f391(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f392(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f393(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f394(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f395(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f396(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f397(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f398(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f399(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f400(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f401(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f402(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f403(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f404(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f405(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f406(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last f407(const std::chrono::month_day_last &a0,
                                      int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f408(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f409(int a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f410(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f411(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::year_month_day_last
f412(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f413(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f414(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f415(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f416(const std::chrono::months &a0,
     const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f417(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f418(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f419(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return operator==(a0, a1);
}

using t77 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f420() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f421(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f422(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f423(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f424(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f425(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f426(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f427(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f428(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f429(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f430(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f431(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f432(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f433(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f434(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f435(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f436(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f437(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f438(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f439(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday f440(const std::chrono::month_weekday &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f441(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f442(int a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f443(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f444(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f445(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday f446(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday
f447(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f448(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday
f449(const std::chrono::months &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday f450(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f451(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return operator==(a0, a1);
}

using t78 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f452(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f453(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f454(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f455(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f456(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f457(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f458(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f459(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f460(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f461(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f462(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f463(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f464(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f465(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f466(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f467(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last
f468(const std::chrono::month_weekday_last &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f469(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f470(int a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f471(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last f472(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f473(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f474(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f475(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f476(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f477(const std::chrono::months &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f478(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f479(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t79 = std::chrono::years;

using t80 = std::chrono::zoned_seconds;
