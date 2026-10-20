#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <compare>
#include <iosfwd>
#include <string_view>
#include <type_traits>
#include <utility>

using t29 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f108(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time f109(std::chrono::ambiguous_local_time &&a0) {
  return std::chrono::ambiguous_local_time(std::move(a0));
}

std::chrono::ambiguous_local_time &
f110(std::chrono::ambiguous_local_time &a0,
     const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::ambiguous_local_time &
f111(std::chrono::ambiguous_local_time &a0,
     std::chrono::ambiguous_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::ambiguous_local_time f112(const std::chrono::local_time<T1> &a0,
                                       const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f113(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

using t30 = std::chrono::choose;

std::chrono::choose f114() { return std::chrono::choose::earliest; }

std::chrono::choose f115() { return std::chrono::choose::latest; }

template <typename T1, typename T2, typename T3>
auto f116(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::clock_cast<T1, T2, T3>(a0);
}

const std::chrono::time_zone *f117() { return std::chrono::current_zone(); }

using t31 = std::chrono::day;

std::chrono::day f118() { return std::chrono::day(); }

std::chrono::day f119(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f120(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f121(std::chrono::day &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::day &f122(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f123(std::chrono::day &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::day &f124(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f125(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f126(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f127(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f128(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f129(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f130(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f131(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f132(const std::chrono::day &a0, const std::chrono::day &a1) {
  return a0 - a1;
}

std::chrono::day f133(const std::chrono::day &a0, const std::chrono::days &a1) {
  return a0 - a1;
}

std::chrono::day f134(const std::chrono::days &a0, const std::chrono::day &a1) {
  return a0 + a1;
}

std::chrono::day f135(const std::chrono::day &a0, const std::chrono::days &a1) {
  return a0 + a1;
}

std::strong_ordering f136(const std::chrono::day &a0,
                          const std::chrono::day &a1) {
  return a0 <=> a1;
}

bool f137(const std::chrono::day &a0, const std::chrono::day &a1) {
  return a0 == a1;
}

using t32 = std::chrono::days;

using t33 = std::chrono::file_clock;

template <typename T1> using t34 = std::chrono::file_time<T1>;

template <typename T1>
std::chrono::leap_second_info f138(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t35 = std::chrono::gps_clock;

using t36 = std::chrono::gps_clock::rep;

using t37 = std::chrono::gps_clock::period;

using t38 = std::chrono::gps_clock::duration;

using t39 = std::chrono::gps_clock::time_point;

std::chrono::gps_clock::time_point f139() {
  return std::chrono::gps_clock::now();
}

std::chrono::gps_clock f140() { return std::chrono::gps_clock(); }

std::chrono::gps_clock f141(const std::chrono::gps_clock &a0) {
  return std::chrono::gps_clock(a0);
}

std::chrono::gps_clock f142(std::chrono::gps_clock &&a0) {
  return std::chrono::gps_clock(std::move(a0));
}

std::chrono::gps_clock &f143(std::chrono::gps_clock &a0,
                             const std::chrono::gps_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::gps_clock &f144(std::chrono::gps_clock &a0,
                             std::chrono::gps_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f145(const std::chrono::gps_time<T1> &a0) {
  return std::chrono::gps_clock::to_utc(a0);
}

template <typename T1>
std::chrono::gps_time<std::common_type_t<T1, std::chrono::seconds>>
f146(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::gps_clock::from_utc(a0);
}

std::chrono::gps_clock::duration f147() {
  return std::chrono::gps_clock::duration();
}

std::chrono::gps_clock::duration
f148(const std::chrono::gps_clock::duration &a0) {
  return std::chrono::gps_clock::duration(a0);
}

std::chrono::gps_clock::duration &
f149(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f150(const std::chrono::gps_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f151(const std::chrono::gps_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f152(const std::chrono::gps_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::gps_clock::duration &f153(std::chrono::gps_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::gps_clock::duration f154(std::chrono::gps_clock::duration &a0,
                                      int a1) {
  return a0.operator++(a1);
}

std::chrono::gps_clock::duration &f155(std::chrono::gps_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::gps_clock::duration f156(std::chrono::gps_clock::duration &a0,
                                      int a1) {
  return a0.operator--(a1);
}

std::chrono::gps_clock::duration &
f157(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::gps_clock::duration &
f158(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::gps_clock::duration &
f159(std::chrono::gps_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::gps_clock::duration &
f160(std::chrono::gps_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::gps_clock::duration f161() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::gps_clock::duration f162() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::gps_clock::duration f163() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <typename T1> std::chrono::gps_clock::duration f164(const T1 &a0) {
  return std::chrono::gps_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::gps_clock::duration f165(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::gps_clock::duration(a0);
}

std::chrono::gps_clock::duration &
f166(std::chrono::gps_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::gps_clock::duration &
f167(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t40 = std::chrono::time_point<std::chrono::gps_clock>::clock;

using t41 = std::chrono::time_point<std::chrono::gps_clock>::duration;

using t42 = std::chrono::time_point<std::chrono::gps_clock>::rep;

using t43 = std::chrono::time_point<std::chrono::gps_clock>::period;

std::chrono::gps_clock::time_point f168() {
  return std::chrono::gps_clock::time_point();
}

std::chrono::gps_clock::time_point
f169(const std::chrono::time_point<std::chrono::gps_clock>::duration &a0) {
  return std::chrono::gps_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::gps_clock>::duration
f170(const std::chrono::gps_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::gps_clock::time_point &
f171(std::chrono::gps_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::gps_clock::time_point f172(std::chrono::gps_clock::time_point &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::gps_clock::time_point &
f173(std::chrono::gps_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::gps_clock::time_point f174(std::chrono::gps_clock::time_point &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::gps_clock::time_point &
f175(std::chrono::gps_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::gps_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::gps_clock::time_point &
f176(std::chrono::gps_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::gps_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::gps_clock::time_point f177() {
  return std::chrono::time_point<std::chrono::gps_clock>::min();
}

std::chrono::gps_clock::time_point f178() {
  return std::chrono::time_point<std::chrono::gps_clock>::max();
}

template <typename T1>
std::chrono::gps_clock::time_point
f179(const std::chrono::time_point<
     std::chrono::time_point<
         std::chrono::gps_clock,
         std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
     T1> &a0) {
  return std::chrono::gps_clock::time_point(a0);
}

bool f180() { return std::chrono::gps_clock::is_steady; }

using t44 = std::chrono::gps_seconds;

template <typename T1> using t45 = std::chrono::gps_time<T1>;

bool f181(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

bool f182(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

using t46 = std::chrono::last_spec;

std::chrono::last_spec f183() { return std::chrono::last_spec(); }

std::chrono::last_spec f184(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f185(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f186(std::chrono::last_spec &a0,
                             const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f187(std::chrono::last_spec &a0,
                             std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t47 = std::chrono::leap_second;

std::chrono::leap_second f188(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f189(std::chrono::leap_second &a0,
                               const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f190(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f191(const std::chrono::leap_second &a0) {
  return a0.value();
}

std::strong_ordering f192(const std::chrono::leap_second &a0,
                          const std::chrono::leap_second &a1) {
  return a0 <=> a1;
}

bool f193(const std::chrono::leap_second &a0,
          const std::chrono::leap_second &a1) {
  return a0 == a1;
}

using t48 = std::chrono::leap_second_info;

std::chrono::leap_second_info f194() { return std::chrono::leap_second_info(); }

std::chrono::leap_second_info f195(const std::chrono::leap_second_info &a0) {
  return std::chrono::leap_second_info(a0);
}

std::chrono::leap_second_info f196(std::chrono::leap_second_info &&a0) {
  return std::chrono::leap_second_info(std::move(a0));
}

std::chrono::leap_second_info &f197(std::chrono::leap_second_info &a0,
                                    const std::chrono::leap_second_info &a1) {
  return a0.operator=(a1);
}

std::chrono::leap_second_info &f198(std::chrono::leap_second_info &a0,
                                    std::chrono::leap_second_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t49 = std::chrono::local_days;

using t50 = std::chrono::local_info;

std::chrono::local_info f199() { return std::chrono::local_info(); }

std::chrono::local_info f200(const std::chrono::local_info &a0) {
  return std::chrono::local_info(a0);
}

std::chrono::local_info f201(std::chrono::local_info &&a0) {
  return std::chrono::local_info(std::move(a0));
}

std::chrono::local_info &f202(std::chrono::local_info &a0,
                              const std::chrono::local_info &a1) {
  return a0.operator=(a1);
}

std::chrono::local_info &f203(std::chrono::local_info &a0,
                              std::chrono::local_info &&a1) {
  return a0.operator=(std::move(a1));
}

int f204() { return std::chrono::local_info::unique; }

int f205() { return std::chrono::local_info::nonexistent; }

int f206() { return std::chrono::local_info::ambiguous; }

using t51 = std::chrono::local_seconds;

using t52 = std::chrono::local_t;

std::chrono::local_t f207() { return std::chrono::local_t(); }

std::chrono::local_t f208(const std::chrono::local_t &a0) {
  return std::chrono::local_t(a0);
}

std::chrono::local_t f209(std::chrono::local_t &&a0) {
  return std::chrono::local_t(std::move(a0));
}

std::chrono::local_t &f210(std::chrono::local_t &a0,
                           const std::chrono::local_t &a1) {
  return a0.operator=(a1);
}

std::chrono::local_t &f211(std::chrono::local_t &a0,
                           std::chrono::local_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t53 = std::chrono::local_time<T1>;

const std::chrono::time_zone *f212(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

std::chrono::hours f213(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f214(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}

using t54 = std::chrono::month;

std::chrono::month f215() { return std::chrono::month(); }

std::chrono::month f216(unsigned int a0) { return std::chrono::month(a0); }

std::chrono::month &f217(std::chrono::month &a0) { return a0.operator++(); }

std::chrono::month f218(std::chrono::month &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::month &f219(std::chrono::month &a0) { return a0.operator--(); }

std::chrono::month f220(std::chrono::month &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::month &f221(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::month &f222(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

unsigned int f223(const std::chrono::month &a0) {
  return static_cast<unsigned int>(a0);
}

bool f224(const std::chrono::month &a0) { return a0.ok(); }

std::chrono::month f225(const std::chrono::month &a0) {
  return std::chrono::month(a0);
}

std::chrono::month f226(std::chrono::month &&a0) {
  return std::chrono::month(std::move(a0));
}

std::chrono::month &f227(std::chrono::month &a0, const std::chrono::month &a1) {
  return a0.operator=(a1);
}

std::chrono::month &f228(std::chrono::month &a0, std::chrono::month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::months f229(const std::chrono::month &a0,
                         const std::chrono::month &a1) {
  return a0 - a1;
}

std::chrono::month f230(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return a0 - a1;
}

std::chrono::month f231(const std::chrono::months &a0,
                        const std::chrono::month &a1) {
  return a0 + a1;
}

std::chrono::month f232(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return a0 + a1;
}

std::strong_ordering f233(const std::chrono::month &a0,
                          const std::chrono::month &a1) {
  return a0 <=> a1;
}

bool f234(const std::chrono::month &a0, const std::chrono::month &a1) {
  return a0 == a1;
}

using t55 = std::chrono::month_day;

std::chrono::month_day f235() { return std::chrono::month_day(); }

std::chrono::month_day f236(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return std::chrono::month_day(a0, a1);
}

std::chrono::month f237(const std::chrono::month_day &a0) { return a0.month(); }

std::chrono::day f238(const std::chrono::month_day &a0) { return a0.day(); }

bool f239(const std::chrono::month_day &a0) { return a0.ok(); }

std::chrono::month_day f240(const std::chrono::month_day &a0) {
  return std::chrono::month_day(a0);
}

std::chrono::month_day f241(std::chrono::month_day &&a0) {
  return std::chrono::month_day(std::move(a0));
}

std::chrono::month_day &f242(std::chrono::month_day &a0,
                             const std::chrono::month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day &f243(std::chrono::month_day &a0,
                             std::chrono::month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day f244(const std::chrono::day &a0, int a1) {
  return a0 / a1;
}

std::chrono::month_day f245(const std::chrono::day &a0,
                            const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::month_day f246(int a0, const std::chrono::day &a1) {
  return a0 / a1;
}

std::chrono::month_day f247(const std::chrono::month &a0, int a1) {
  return a0 / a1;
}

std::chrono::month_day f248(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return a0 / a1;
}

std::strong_ordering f249(const std::chrono::month_day &a0,
                          const std::chrono::month_day &a1) {
  return a0 <=> a1;
}

bool f250(const std::chrono::month_day &a0, const std::chrono::month_day &a1) {
  return a0 == a1;
}

using t56 = std::chrono::month_day_last;

std::chrono::month_day_last f251(const std::chrono::month &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month f252(const std::chrono::month_day_last &a0) {
  return a0.month();
}

bool f253(const std::chrono::month_day_last &a0) { return a0.ok(); }

std::chrono::month_day_last f254(const std::chrono::month_day_last &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month_day_last f255(std::chrono::month_day_last &&a0) {
  return std::chrono::month_day_last(std::move(a0));
}

std::chrono::month_day_last &f256(std::chrono::month_day_last &a0,
                                  const std::chrono::month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day_last &f257(std::chrono::month_day_last &a0,
                                  std::chrono::month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day_last f258(std::chrono::last_spec a0, int a1) {
  return std::move(a0) / a1;
}

std::chrono::month_day_last f259(std::chrono::last_spec a0,
                                 const std::chrono::month &a1) {
  return std::move(a0) / a1;
}

std::chrono::month_day_last f260(int a0, std::chrono::last_spec a1) {
  return a0 / std::move(a1);
}

std::chrono::month_day_last f261(const std::chrono::month &a0,
                                 std::chrono::last_spec a1) {
  return a0 / std::move(a1);
}

std::strong_ordering f262(const std::chrono::month_day_last &a0,
                          const std::chrono::month_day_last &a1) {
  return a0 <=> a1;
}

bool f263(const std::chrono::month_day_last &a0,
          const std::chrono::month_day_last &a1) {
  return a0 == a1;
}

using t57 = std::chrono::month_weekday;

std::chrono::month_weekday f264(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return std::chrono::month_weekday(a0, a1);
}

std::chrono::month f265(const std::chrono::month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday_indexed f266(const std::chrono::month_weekday &a0) {
  return a0.weekday_indexed();
}

bool f267(const std::chrono::month_weekday &a0) { return a0.ok(); }

std::chrono::month_weekday f268(const std::chrono::month_weekday &a0) {
  return std::chrono::month_weekday(a0);
}

std::chrono::month_weekday f269(std::chrono::month_weekday &&a0) {
  return std::chrono::month_weekday(std::move(a0));
}

std::chrono::month_weekday &f270(std::chrono::month_weekday &a0,
                                 const std::chrono::month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday &f271(std::chrono::month_weekday &a0,
                                 std::chrono::month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday f272(const std::chrono::weekday_indexed &a0,
                                int a1) {
  return a0 / a1;
}

std::chrono::month_weekday f273(const std::chrono::weekday_indexed &a0,
                                const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::month_weekday f274(int a0,
                                const std::chrono::weekday_indexed &a1) {
  return a0 / a1;
}

std::chrono::month_weekday f275(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return a0 / a1;
}

bool f276(const std::chrono::month_weekday &a0,
          const std::chrono::month_weekday &a1) {
  return a0 == a1;
}

using t58 = std::chrono::month_weekday_last;

std::chrono::month_weekday_last f277(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return std::chrono::month_weekday_last(a0, a1);
}

std::chrono::month f278(const std::chrono::month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday_last f279(const std::chrono::month_weekday_last &a0) {
  return a0.weekday_last();
}

bool f280(const std::chrono::month_weekday_last &a0) { return a0.ok(); }

std::chrono::month_weekday_last
f281(const std::chrono::month_weekday_last &a0) {
  return std::chrono::month_weekday_last(a0);
}

std::chrono::month_weekday_last f282(std::chrono::month_weekday_last &&a0) {
  return std::chrono::month_weekday_last(std::move(a0));
}

std::chrono::month_weekday_last &
f283(std::chrono::month_weekday_last &a0,
     const std::chrono::month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday_last &f284(std::chrono::month_weekday_last &a0,
                                      std::chrono::month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday_last f285(const std::chrono::weekday_last &a0,
                                     int a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f286(const std::chrono::weekday_last &a0,
                                     const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f287(int a0,
                                     const std::chrono::weekday_last &a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f288(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return a0 / a1;
}

bool f289(const std::chrono::month_weekday_last &a0,
          const std::chrono::month_weekday_last &a1) {
  return a0 == a1;
}

using t59 = std::chrono::months;

using t60 = std::chrono::nonexistent_local_time;

std::chrono::nonexistent_local_time
f290(const std::chrono::nonexistent_local_time &a0) {
  return std::chrono::nonexistent_local_time(a0);
}

std::chrono::nonexistent_local_time
f291(std::chrono::nonexistent_local_time &&a0) {
  return std::chrono::nonexistent_local_time(std::move(a0));
}

std::chrono::nonexistent_local_time &
f292(std::chrono::nonexistent_local_time &a0,
     const std::chrono::nonexistent_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::nonexistent_local_time &
f293(std::chrono::nonexistent_local_time &a0,
     std::chrono::nonexistent_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::nonexistent_local_time f294(const std::chrono::local_time<T1> &a0,
                                         const std::chrono::local_info &a1) {
  return std::chrono::nonexistent_local_time(a0, a1);
}

const char *f295(const std::chrono::nonexistent_local_time &a0) {
  return a0.what();
}

template <typename T1, typename T2, typename T3>
auto f296(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4>
auto f297(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f298(const std::chrono::zoned_time<T1, T2> &a0,
          const std::chrono::zoned_time<T3, T4> &a1) {
  return a0 == a1;
}

std::string f299() { return std::chrono::remote_version(); }

std::chrono::steady_clock::time_point &
f300(std::chrono::steady_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::steady_clock::time_point
f301(std::chrono::steady_clock::time_point &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::steady_clock::time_point &
f302(std::chrono::steady_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::steady_clock::time_point
f303(std::chrono::steady_clock::time_point &a0, int a1) {
  return a0.operator--(a1);
}

using t61 = std::chrono::sys_days;

using t62 = std::chrono::sys_info;

std::chrono::sys_info f304() { return std::chrono::sys_info(); }

std::chrono::sys_info f305(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f306(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f307(std::chrono::sys_info &a0,
                            const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f308(std::chrono::sys_info &a0,
                            std::chrono::sys_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t63 = std::chrono::sys_seconds;

template <typename T1> using t64 = std::chrono::sys_time<T1>;

std::chrono::system_clock::time_point &
f309(std::chrono::system_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::system_clock::time_point
f310(std::chrono::system_clock::time_point &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::system_clock::time_point &
f311(std::chrono::system_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::system_clock::time_point
f312(std::chrono::system_clock::time_point &a0, int a1) {
  return a0.operator--(a1);
}

using t65 = std::chrono::tai_clock;

using t66 = std::chrono::tai_clock::rep;

using t67 = std::chrono::tai_clock::period;

using t68 = std::chrono::tai_clock::duration;

using t69 = std::chrono::tai_clock::time_point;

std::chrono::tai_clock::time_point f313() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f314() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f315(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f316(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f317(std::chrono::tai_clock &a0,
                             const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f318(std::chrono::tai_clock &a0,
                             std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f319(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f320(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

std::chrono::tai_clock::duration f321() {
  return std::chrono::tai_clock::duration();
}

std::chrono::tai_clock::duration
f322(const std::chrono::tai_clock::duration &a0) {
  return std::chrono::tai_clock::duration(a0);
}

std::chrono::tai_clock::duration &
f323(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f324(const std::chrono::tai_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f325(const std::chrono::tai_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f326(const std::chrono::tai_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::tai_clock::duration &f327(std::chrono::tai_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::tai_clock::duration f328(std::chrono::tai_clock::duration &a0,
                                      int a1) {
  return a0.operator++(a1);
}

std::chrono::tai_clock::duration &f329(std::chrono::tai_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::tai_clock::duration f330(std::chrono::tai_clock::duration &a0,
                                      int a1) {
  return a0.operator--(a1);
}

std::chrono::tai_clock::duration &
f331(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::tai_clock::duration &
f332(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::tai_clock::duration &
f333(std::chrono::tai_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::tai_clock::duration &
f334(std::chrono::tai_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::tai_clock::duration f335() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::tai_clock::duration f336() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::tai_clock::duration f337() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <typename T1> std::chrono::tai_clock::duration f338(const T1 &a0) {
  return std::chrono::tai_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::tai_clock::duration f339(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::tai_clock::duration(a0);
}

std::chrono::tai_clock::duration &
f340(std::chrono::tai_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::tai_clock::duration &
f341(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t70 = std::chrono::time_point<std::chrono::tai_clock>::clock;

using t71 = std::chrono::time_point<std::chrono::tai_clock>::duration;

using t72 = std::chrono::time_point<std::chrono::tai_clock>::rep;

using t73 = std::chrono::time_point<std::chrono::tai_clock>::period;

std::chrono::tai_clock::time_point f342() {
  return std::chrono::tai_clock::time_point();
}

std::chrono::tai_clock::time_point
f343(const std::chrono::time_point<std::chrono::tai_clock>::duration &a0) {
  return std::chrono::tai_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::tai_clock>::duration
f344(const std::chrono::tai_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::tai_clock::time_point &
f345(std::chrono::tai_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::tai_clock::time_point f346(std::chrono::tai_clock::time_point &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::tai_clock::time_point &
f347(std::chrono::tai_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::tai_clock::time_point f348(std::chrono::tai_clock::time_point &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::tai_clock::time_point &
f349(std::chrono::tai_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::tai_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::tai_clock::time_point &
f350(std::chrono::tai_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::tai_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::tai_clock::time_point f351() {
  return std::chrono::time_point<std::chrono::tai_clock>::min();
}

std::chrono::tai_clock::time_point f352() {
  return std::chrono::time_point<std::chrono::tai_clock>::max();
}

template <typename T1>
std::chrono::tai_clock::time_point
f353(const std::chrono::time_point<
     std::chrono::time_point<
         std::chrono::tai_clock,
         std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
     T1> &a0) {
  return std::chrono::tai_clock::time_point(a0);
}

bool f354() { return std::chrono::tai_clock::is_steady; }

using t74 = std::chrono::tai_seconds;

template <typename T1> using t75 = std::chrono::tai_time<T1>;

using t76 = std::chrono::time_zone;

std::string_view f355(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f356(const std::chrono::time_zone &a0,
                           const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f357(const std::chrono::time_zone &a0,
                             const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f358(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f359(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
     std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f360(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}

std::strong_ordering f361(const std::chrono::time_zone &a0,
                          const std::chrono::time_zone &a1) {
  return a0 <=> a1;
}

bool f362(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return a0 == a1;
}

using t77 = std::chrono::time_zone_link;

std::chrono::time_zone_link f363(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f364(std::chrono::time_zone_link &a0,
                                  std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f365(const std::chrono::time_zone_link &a0) {
  return a0.name();
}

std::string_view f366(const std::chrono::time_zone_link &a0) {
  return a0.target();
}

std::strong_ordering f367(const std::chrono::time_zone_link &a0,
                          const std::chrono::time_zone_link &a1) {
  return a0 <=> a1;
}

bool f368(const std::chrono::time_zone_link &a0,
          const std::chrono::time_zone_link &a1) {
  return a0 == a1;
}

using t78 = std::chrono::tzdb;

const std::chrono::time_zone *f369(const std::chrono::tzdb &a0,
                                   std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f370(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f371() { return std::chrono::tzdb(); }

std::chrono::tzdb f372(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f373(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}

using t79 = std::chrono::tzdb_list;

using t80 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f374(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f375(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f376(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f377(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f378(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f379(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t81 = std::chrono::utc_clock;

using t82 = std::chrono::utc_clock::rep;

using t83 = std::chrono::utc_clock::period;

using t84 = std::chrono::utc_clock::duration;

using t85 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f380() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f381() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f382(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f383(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f384(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f385(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f386(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f387(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

std::chrono::utc_clock::duration f388() {
  return std::chrono::utc_clock::duration();
}

std::chrono::utc_clock::duration
f389(const std::chrono::utc_clock::duration &a0) {
  return std::chrono::utc_clock::duration(a0);
}

std::chrono::utc_clock::duration &
f390(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f391(const std::chrono::utc_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f392(const std::chrono::utc_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f393(const std::chrono::utc_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::utc_clock::duration &f394(std::chrono::utc_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::utc_clock::duration f395(std::chrono::utc_clock::duration &a0,
                                      int a1) {
  return a0.operator++(a1);
}

std::chrono::utc_clock::duration &f396(std::chrono::utc_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::utc_clock::duration f397(std::chrono::utc_clock::duration &a0,
                                      int a1) {
  return a0.operator--(a1);
}

std::chrono::utc_clock::duration &
f398(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::utc_clock::duration &
f399(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::utc_clock::duration &
f400(std::chrono::utc_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::utc_clock::duration &
f401(std::chrono::utc_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::utc_clock::duration f402() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::utc_clock::duration f403() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::utc_clock::duration f404() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <typename T1> std::chrono::utc_clock::duration f405(const T1 &a0) {
  return std::chrono::utc_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::utc_clock::duration f406(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::utc_clock::duration(a0);
}

std::chrono::utc_clock::duration &
f407(std::chrono::utc_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::utc_clock::duration &
f408(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t86 = std::chrono::time_point<std::chrono::utc_clock>::clock;

using t87 = std::chrono::time_point<std::chrono::utc_clock>::duration;

using t88 = std::chrono::time_point<std::chrono::utc_clock>::rep;

using t89 = std::chrono::time_point<std::chrono::utc_clock>::period;

std::chrono::utc_clock::time_point f409() {
  return std::chrono::utc_clock::time_point();
}

std::chrono::utc_clock::time_point
f410(const std::chrono::time_point<std::chrono::utc_clock>::duration &a0) {
  return std::chrono::utc_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::utc_clock>::duration
f411(const std::chrono::utc_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::utc_clock::time_point &
f412(std::chrono::utc_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::utc_clock::time_point f413(std::chrono::utc_clock::time_point &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::utc_clock::time_point &
f414(std::chrono::utc_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::utc_clock::time_point f415(std::chrono::utc_clock::time_point &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::utc_clock::time_point &
f416(std::chrono::utc_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::utc_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::utc_clock::time_point &
f417(std::chrono::utc_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::utc_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::utc_clock::time_point f418() {
  return std::chrono::time_point<std::chrono::utc_clock>::min();
}

std::chrono::utc_clock::time_point f419() {
  return std::chrono::time_point<std::chrono::utc_clock>::max();
}

template <typename T1>
std::chrono::utc_clock::time_point
f420(const std::chrono::time_point<
     std::chrono::time_point<
         std::chrono::utc_clock,
         std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
     T1> &a0) {
  return std::chrono::utc_clock::time_point(a0);
}

bool f421() { return std::chrono::utc_clock::is_steady; }

using t90 = std::chrono::utc_seconds;

template <typename T1> using t91 = std::chrono::utc_time<T1>;

using t92 = std::chrono::weekday;

std::chrono::weekday f422() { return std::chrono::weekday(); }

std::chrono::weekday f423(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f424(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f425(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f426(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f427(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f428(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f429(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f430(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f431(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f432(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f433(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f434(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f435(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f436(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f437(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f438(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f439(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f440(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f441(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return a0 - a1;
}

std::chrono::weekday f442(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return a0 - a1;
}

std::chrono::weekday f443(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return a0 + a1;
}

std::chrono::weekday f444(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return a0 + a1;
}

bool f445(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return a0 == a1;
}

using t93 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f446() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f447(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f448(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f449(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f450(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f451(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f452(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f453(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f454(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f455(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return a0 == a1;
}

using t94 = std::chrono::weekday_last;

std::chrono::weekday_last f456(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f457(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f458(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f459(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f460(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f461(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f462(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f463(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return a0 == a1;
}

using t95 = std::chrono::weeks;

using t96 = std::chrono::year;

std::chrono::year f464() { return std::chrono::year(); }

std::chrono::year f465(int a0) { return std::chrono::year(a0); }

std::chrono::year f466() { return std::chrono::year::min(); }

std::chrono::year f467() { return std::chrono::year::max(); }

std::chrono::year &f468(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f469(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f470(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f471(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f472(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f473(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f474(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f475(const std::chrono::year &a0) { return a0.operator-(); }

bool f476(const std::chrono::year &a0) { return a0.is_leap(); }

int f477(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f478(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f479(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f480(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f481(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f482(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f483(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return a0 - a1;
}

std::chrono::year f484(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year f485(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return a0 + a1;
}

std::chrono::year f486(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return a0 + a1;
}

std::strong_ordering f487(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return a0 <=> a1;
}

bool f488(const std::chrono::year &a0, const std::chrono::year &a1) {
  return a0 == a1;
}

using t97 = std::chrono::year_month;

std::chrono::year_month f489() { return std::chrono::year_month(); }

std::chrono::year_month f490(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f491(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f492(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f493(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f494(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f495(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f496(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f497(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f498(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f499(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month &f500(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f501(std::chrono::year_month &a0,
                              const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month f502(const std::chrono::year &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month f503(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::year_month f504(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month f505(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return a0 + a1;
}

std::chrono::year_month f506(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return a0 + a1;
}

std::chrono::months f507(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return a0 - a1;
}

std::strong_ordering f508(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return a0 <=> a1;
}

bool f509(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return a0 == a1;
}

using t98 = std::chrono::year_month_day;

std::chrono::year_month_day f510() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f511(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f512(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f513(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f514(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f515(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f516(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f517(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f518(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f519(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f520(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f521(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f522(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f523(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f524(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f525(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f526(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_day &f527(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f528(std::chrono::year_month_day &a0,
                                  const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day f529(const std::chrono::month_day &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month_day f530(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_day f531(int a0, const std::chrono::month_day &a1) {
  return a0 / a1;
}

std::chrono::year_month_day f532(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return a0 / a1;
}

std::chrono::year_month_day f533(const std::chrono::year_month &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month_day f534(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return a0 / a1;
}

std::chrono::year_month_day f535(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_day f536(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return a0 + a1;
}

std::chrono::year_month_day f537(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return a0 + a1;
}

std::strong_ordering f538(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return a0 <=> a1;
}

bool f539(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return a0 == a1;
}

using t99 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f540(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f541(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f542(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f543(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f544(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f545(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f546(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f547(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f548(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f549(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f550(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f551(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f552(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f553(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_day_last &f554(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f555(std::chrono::year_month_day_last &a0,
                                       const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last f556(const std::chrono::month_day_last &a0,
                                      int a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f557(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f558(int a0,
                                      const std::chrono::month_day_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f559(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f560(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return a0 / std::move(a1);
}

std::chrono::year_month_day_last
f561(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_day_last
f562(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return a0 + a1;
}

std::chrono::year_month_day_last
f563(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return a0 + a1;
}

std::strong_ordering f564(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return a0 <=> a1;
}

bool f565(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return a0 == a1;
}

using t100 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f566() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f567(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f568(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f569(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f570(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f571(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f572(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f573(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f574(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f575(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f576(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f577(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f578(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f579(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f580(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f581(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f582(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f583(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_weekday &f584(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f585(std::chrono::year_month_weekday &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday f586(const std::chrono::month_weekday &a0,
                                     int a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f587(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f588(int a0,
                                     const std::chrono::month_weekday &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f589(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f590(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f591(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_weekday
f592(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday f593(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return a0 + a1;
}

bool f594(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return a0 == a1;
}

using t101 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f595(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f596(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f597(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f598(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f599(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f600(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f601(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f602(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f603(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f604(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f605(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f606(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f607(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f608(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_weekday_last &
f609(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f610(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last
f611(const std::chrono::month_weekday_last &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f612(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f613(int a0, const std::chrono::month_weekday_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f614(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last f615(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f616(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_weekday_last
f617(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday_last
f618(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return a0 + a1;
}

bool f619(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return a0 == a1;
}

using t102 = std::chrono::years;

using t103 = std::chrono::zoned_seconds;
