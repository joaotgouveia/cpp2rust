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

using t62 = std::chrono::tzdb_list::const_iterator::value_type;

using t63 = std::chrono::tzdb_list::const_iterator::reference;

using t64 = std::chrono::tzdb_list::const_iterator::pointer;

using t65 = std::chrono::tzdb_list::const_iterator::difference_type;

using t66 = std::chrono::tzdb_list::const_iterator::iterator_category;

std::chrono::tzdb_list::const_iterator f258() {
  return std::chrono::tzdb_list::const_iterator();
}

std::chrono::tzdb_list::const_iterator
f259(const std::chrono::tzdb_list::const_iterator &a0) {
  return std::chrono::tzdb_list::const_iterator(a0);
}

std::chrono::tzdb_list::const_iterator
f260(std::chrono::tzdb_list::const_iterator &&a0) {
  return std::chrono::tzdb_list::const_iterator(std::move(a0));
}

std::chrono::tzdb_list::const_iterator &
f261(std::chrono::tzdb_list::const_iterator &a0,
     const std::chrono::tzdb_list::const_iterator &a1) {
  return a0.operator=(a1);
}

std::chrono::tzdb_list::const_iterator &
f262(std::chrono::tzdb_list::const_iterator &a0,
     std::chrono::tzdb_list::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

const std::chrono::tzdb &
f263(const std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator*();
}

const std::chrono::tzdb *
f264(const std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator->();
}

std::chrono::tzdb_list::const_iterator &
f265(std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator++();
}

std::chrono::tzdb_list::const_iterator
f266(std::chrono::tzdb_list::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

bool f267(const std::chrono::tzdb_list::const_iterator &a0,
          const std::chrono::tzdb_list::const_iterator &a1) {
  return a0.operator==(a1);
}

const std::string &f268(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->version;
}

const std::vector<std::chrono::time_zone> &
f269(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->zones;
}

const std::vector<std::chrono::time_zone_link> &
f270(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->links;
}

const std::vector<std::chrono::leap_second> &
f271(std::chrono::tzdb_list::const_iterator &a0) {
  return a0->leap_seconds;
}

using t67 = std::chrono::utc_clock;

using t68 = std::chrono::utc_clock::rep;

using t69 = std::chrono::utc_clock::period;

using t70 = std::chrono::utc_clock::duration;

using t71 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f272() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f273() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f274(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f275(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f276(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f277(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f278(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f279(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

bool f280() { return std::chrono::utc_clock::is_steady; }

using t72 = std::chrono::utc_seconds;

template <typename T1> using t73 = std::chrono::utc_time<T1>;

using t74 = std::chrono::weekday;

std::chrono::weekday f281() { return std::chrono::weekday(); }

std::chrono::weekday f282(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f283(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f284(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f285(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f286(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f287(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f288(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f289(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f290(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f291(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f292(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f293(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f294(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f295(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f296(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f297(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f298(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f299(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f300(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f301(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f302(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::weekday f303(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator+(a0, a1);
}

bool f304(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return operator==(a0, a1);
}

using t75 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f305() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f306(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f307(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f308(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f309(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f310(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f311(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f312(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f313(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f314(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return operator==(a0, a1);
}

using t76 = std::chrono::weekday_last;

std::chrono::weekday_last f315(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f316(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f317(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f318(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f319(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f320(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f321(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f322(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return operator==(a0, a1);
}

using t77 = std::chrono::weeks;

using t78 = std::chrono::year;

std::chrono::year f323() { return std::chrono::year(); }

std::chrono::year f324(int a0) { return std::chrono::year(a0); }

std::chrono::year f325() { return std::chrono::year::min(); }

std::chrono::year f326() { return std::chrono::year::max(); }

std::chrono::year &f327(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f328(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f329(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f330(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f331(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f332(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f333(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f334(const std::chrono::year &a0) { return a0.operator-(); }

bool f335(const std::chrono::year &a0) { return a0.is_leap(); }

int f336(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f337(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f338(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f339(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f340(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f341(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f342(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return operator-(a0, a1);
}

std::chrono::year f343(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year f344(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return operator+(a0, a1);
}

std::chrono::year f345(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f346(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return operator<=>(a0, a1);
}

bool f347(const std::chrono::year &a0, const std::chrono::year &a1) {
  return operator==(a0, a1);
}

using t79 = std::chrono::year_month;

std::chrono::year_month f348() { return std::chrono::year_month(); }

std::chrono::year_month f349(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f350(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f351(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f352(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f353(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f354(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f355(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f356(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f357(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f358(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f359(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month &f360(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month f361(const std::chrono::year &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f362(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f363(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f364(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f365(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::months f366(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f367(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f368(const std::chrono::months &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month f369(const std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f370(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return operator<=>(a0, a1);
}

bool f371(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return operator==(a0, a1);
}

using t80 = std::chrono::year_month_day;

std::chrono::year_month_day f372() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f373(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f374(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f375(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f376(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f377(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f378(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f379(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f380(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f381(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f382(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f383(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f384(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f385(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f386(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f387(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f388(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f389(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day &f390(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day f391(const std::chrono::month_day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f392(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f393(int a0, const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f394(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f395(const std::chrono::year_month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f396(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f397(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f398(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f399(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f400(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f401(const std::chrono::months &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day f402(const std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f403(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return operator<=>(a0, a1);
}

bool f404(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return operator==(a0, a1);
}

using t81 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f405(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f406(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f407(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f408(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f409(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f410(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f411(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f412(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f413(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f414(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f415(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f416(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f417(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f418(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f419(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last &f420(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last f421(const std::chrono::month_day_last &a0,
                                      int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f422(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f423(int a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f424(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f425(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::year_month_day_last
f426(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f427(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f428(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f429(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f430(const std::chrono::months &a0,
     const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_day_last
f431(const std::chrono::year_month_day_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f432(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f433(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return operator==(a0, a1);
}

using t82 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f434() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f435(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f436(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f437(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f438(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f439(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f440(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f441(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f442(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f443(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f444(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f445(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f446(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f447(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f448(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f449(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f450(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f451(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f452(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday &f453(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday f454(const std::chrono::month_weekday &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f455(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f456(int a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f457(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f458(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f459(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday f460(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday
f461(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f462(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday
f463(const std::chrono::months &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday f464(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f465(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return operator==(a0, a1);
}

using t83 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f466(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f467(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f468(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f469(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f470(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f471(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f472(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f473(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f474(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f475(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f476(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f477(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f478(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f479(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f480(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last &
f481(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last
f482(const std::chrono::month_weekday_last &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f483(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f484(int a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f485(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last f486(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f487(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f488(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f489(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f490(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f491(const std::chrono::months &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

template <typename T1 =
              std::chrono::__detail::__months_years_conversion_disambiguator>
std::chrono::year_month_weekday_last
f492(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::months &a1) {
  return operator+(a0, a1);
}

bool f493(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t84 = std::chrono::years;

using t85 = std::chrono::zoned_seconds;
