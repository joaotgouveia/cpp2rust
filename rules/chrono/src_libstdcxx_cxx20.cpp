#include <chrono>
#include <compare>
#include <iosfwd>
#include <limits>
#include <string_view>
#include <type_traits>
#include <utility>

using t29 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f96(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time f97(std::chrono::ambiguous_local_time &&a0) {
  return std::chrono::ambiguous_local_time(std::move(a0));
}

std::chrono::ambiguous_local_time &
f98(std::chrono::ambiguous_local_time &a0,
    const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::ambiguous_local_time &f99(std::chrono::ambiguous_local_time &a0,
                                       std::chrono::ambiguous_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::ambiguous_local_time f100(const std::chrono::local_time<T1> &a0,
                                       const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f101(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

using t30 = std::chrono::choose;

std::chrono::choose f102() { return std::chrono::choose::earliest; }

std::chrono::choose f103() { return std::chrono::choose::latest; }

template <typename T1, typename T2, typename T3>
auto f104(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::clock_cast<T1, T2, T3>(a0);
}

const std::chrono::time_zone *f105() { return std::chrono::current_zone(); }

using t31 = std::chrono::day;

std::chrono::day f106() { return std::chrono::day(); }

std::chrono::day f107(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f108(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f109(std::chrono::day &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::day &f110(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f111(std::chrono::day &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::day &f112(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f113(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f114(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f115(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f116(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f117(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f118(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f119(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f120(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator-(a0, a1);
}

std::chrono::day f121(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::day f122(const std::chrono::days &a0, const std::chrono::day &a1) {
  return operator+(a0, a1);
}

std::chrono::day f123(const std::chrono::day &a0, const std::chrono::days &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f124(const std::chrono::day &a0,
                          const std::chrono::day &a1) {
  return operator<=>(a0, a1);
}

bool f125(const std::chrono::day &a0, const std::chrono::day &a1) {
  return operator==(a0, a1);
}

using t32 = std::chrono::days;

using t33 = std::chrono::file_clock;

template <typename T1> using t34 = std::chrono::file_time<T1>;

template <typename T1>
std::chrono::leap_second_info f126(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t35 = std::chrono::gps_clock;

using t36 = std::chrono::gps_clock::rep;

using t37 = std::chrono::gps_clock::period;

using t38 = std::chrono::gps_clock::duration;

using t39 = std::chrono::gps_clock::time_point;

std::chrono::gps_clock::time_point f127() {
  return std::chrono::gps_clock::now();
}

std::chrono::gps_clock f128() { return std::chrono::gps_clock(); }

std::chrono::gps_clock f129(const std::chrono::gps_clock &a0) {
  return std::chrono::gps_clock(a0);
}

std::chrono::gps_clock f130(std::chrono::gps_clock &&a0) {
  return std::chrono::gps_clock(std::move(a0));
}

std::chrono::gps_clock &f131(std::chrono::gps_clock &a0,
                             const std::chrono::gps_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::gps_clock &f132(std::chrono::gps_clock &a0,
                             std::chrono::gps_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f133(const std::chrono::gps_time<T1> &a0) {
  return std::chrono::gps_clock::to_utc(a0);
}

template <typename T1>
std::chrono::gps_time<std::common_type_t<T1, std::chrono::seconds>>
f134(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::gps_clock::from_utc(a0);
}

std::chrono::gps_clock::duration f135() {
  return std::chrono::gps_clock::duration();
}

std::chrono::gps_clock::duration
f136(const std::chrono::gps_clock::duration &a0) {
  return std::chrono::gps_clock::duration(a0);
}

std::chrono::gps_clock::duration &
f137(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f138(const std::chrono::gps_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f139(const std::chrono::gps_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f140(const std::chrono::gps_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::gps_clock::duration &f141(std::chrono::gps_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::gps_clock::duration f142(std::chrono::gps_clock::duration &a0,
                                      int a1) {
  return a0.operator++(a1);
}

std::chrono::gps_clock::duration &f143(std::chrono::gps_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::gps_clock::duration f144(std::chrono::gps_clock::duration &a0,
                                      int a1) {
  return a0.operator--(a1);
}

std::chrono::gps_clock::duration &
f145(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::gps_clock::duration &
f146(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::gps_clock::duration &
f147(std::chrono::gps_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::gps_clock::duration &
f148(std::chrono::gps_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::gps_clock::duration f149() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::gps_clock::duration f150() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::gps_clock::duration f151() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::is_convertible<const T1 &,
                                std::chrono::gps_clock::rep>::value &&
                std::integral_constant<
                    bool, std::integral_constant<bool, false>::value ||
                              std::integral_constant<
                                  bool, !std::chrono::treat_as_floating_point<
                                            T1>::value>::value>::value>::value,
        void>::type>
std::chrono::gps_clock::duration f152(const T1 &a0) {
  return std::chrono::gps_clock::duration(a0);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f153(std::chrono::gps_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f154(std::chrono::gps_clock::duration &a0,
     const std::chrono::gps_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t40 = std::chrono::time_point<std::chrono::gps_clock>::clock;

using t41 = std::chrono::time_point<std::chrono::gps_clock>::duration;

using t42 = std::chrono::time_point<std::chrono::gps_clock>::rep;

using t43 = std::chrono::time_point<std::chrono::gps_clock>::period;

std::chrono::gps_clock::time_point f155() {
  return std::chrono::gps_clock::time_point();
}

std::chrono::gps_clock::time_point
f156(const std::chrono::time_point<std::chrono::gps_clock>::duration &a0) {
  return std::chrono::gps_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::gps_clock>::duration
f157(const std::chrono::gps_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::gps_clock::time_point &
f158(std::chrono::gps_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::gps_clock::time_point f159(std::chrono::gps_clock::time_point &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::gps_clock::time_point &
f160(std::chrono::gps_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::gps_clock::time_point f161(std::chrono::gps_clock::time_point &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::gps_clock::time_point &
f162(std::chrono::gps_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::gps_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::gps_clock::time_point &
f163(std::chrono::gps_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::gps_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::gps_clock::time_point f164() {
  return std::chrono::time_point<std::chrono::gps_clock>::min();
}

std::chrono::gps_clock::time_point f165() {
  return std::chrono::time_point<std::chrono::gps_clock>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool, std::is_convertible<T1, typename std::chrono::gps_clock::
                                              duration>::value>::value,
        void>::type>
std::chrono::gps_clock::time_point
f166(const std::chrono::time_point<
     std::chrono::time_point<
         std::chrono::gps_clock,
         std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
     T1> &a0) {
  return std::chrono::gps_clock::time_point(a0);
}

bool f167() { return std::chrono::gps_clock::is_steady; }

using t44 = std::chrono::gps_seconds;

template <typename T1> using t45 = std::chrono::gps_time<T1>;

bool f168(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

bool f169(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

using t46 = std::chrono::last_spec;

std::chrono::last_spec f170() { return std::chrono::last_spec(); }

std::chrono::last_spec f171(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f172(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f173(std::chrono::last_spec &a0,
                             const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f174(std::chrono::last_spec &a0,
                             std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t47 = std::chrono::leap_second;

std::chrono::leap_second f175(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f176(std::chrono::leap_second &a0,
                               const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f177(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f178(const std::chrono::leap_second &a0) {
  return a0.value();
}

std::strong_ordering f179(const std::chrono::leap_second &a0,
                          const std::chrono::leap_second &a1) {
  return operator<=>(a0, a1);
}

bool f180(const std::chrono::leap_second &a0,
          const std::chrono::leap_second &a1) {
  return operator==(a0, a1);
}

using t48 = std::chrono::leap_second_info;

std::chrono::leap_second_info f181() { return std::chrono::leap_second_info(); }

std::chrono::leap_second_info f182(const std::chrono::leap_second_info &a0) {
  return std::chrono::leap_second_info(a0);
}

std::chrono::leap_second_info f183(std::chrono::leap_second_info &&a0) {
  return std::chrono::leap_second_info(std::move(a0));
}

std::chrono::leap_second_info &f184(std::chrono::leap_second_info &a0,
                                    const std::chrono::leap_second_info &a1) {
  return a0.operator=(a1);
}

std::chrono::leap_second_info &f185(std::chrono::leap_second_info &a0,
                                    std::chrono::leap_second_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t49 = std::chrono::local_days;

using t50 = std::chrono::local_info;

std::chrono::local_info f186() { return std::chrono::local_info(); }

std::chrono::local_info f187(const std::chrono::local_info &a0) {
  return std::chrono::local_info(a0);
}

std::chrono::local_info f188(std::chrono::local_info &&a0) {
  return std::chrono::local_info(std::move(a0));
}

std::chrono::local_info &f189(std::chrono::local_info &a0,
                              const std::chrono::local_info &a1) {
  return a0.operator=(a1);
}

std::chrono::local_info &f190(std::chrono::local_info &a0,
                              std::chrono::local_info &&a1) {
  return a0.operator=(std::move(a1));
}

int f191() { return std::chrono::local_info::unique; }

int f192() { return std::chrono::local_info::nonexistent; }

int f193() { return std::chrono::local_info::ambiguous; }

using t51 = std::chrono::local_seconds;

using t52 = std::chrono::local_t;

std::chrono::local_t f194() { return std::chrono::local_t(); }

std::chrono::local_t f195(const std::chrono::local_t &a0) {
  return std::chrono::local_t(a0);
}

std::chrono::local_t f196(std::chrono::local_t &&a0) {
  return std::chrono::local_t(std::move(a0));
}

std::chrono::local_t &f197(std::chrono::local_t &a0,
                           const std::chrono::local_t &a1) {
  return a0.operator=(a1);
}

std::chrono::local_t &f198(std::chrono::local_t &a0,
                           std::chrono::local_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t53 = std::chrono::local_time<T1>;

const std::chrono::time_zone *f199(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

std::chrono::hours f200(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f201(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}

using t54 = std::chrono::month;

std::chrono::month f202() { return std::chrono::month(); }

std::chrono::month f203(unsigned int a0) { return std::chrono::month(a0); }

std::chrono::month &f204(std::chrono::month &a0) { return a0.operator++(); }

std::chrono::month f205(std::chrono::month &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::month &f206(std::chrono::month &a0) { return a0.operator--(); }

std::chrono::month f207(std::chrono::month &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::month &f208(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::month &f209(std::chrono::month &a0,
                         const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

unsigned int f210(const std::chrono::month &a0) {
  return static_cast<unsigned int>(a0);
}

bool f211(const std::chrono::month &a0) { return a0.ok(); }

std::chrono::month f212(const std::chrono::month &a0) {
  return std::chrono::month(a0);
}

std::chrono::month f213(std::chrono::month &&a0) {
  return std::chrono::month(std::move(a0));
}

std::chrono::month &f214(std::chrono::month &a0, const std::chrono::month &a1) {
  return a0.operator=(a1);
}

std::chrono::month &f215(std::chrono::month &a0, std::chrono::month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::months f216(const std::chrono::month &a0,
                         const std::chrono::month &a1) {
  return operator-(a0, a1);
}

std::chrono::month f217(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator-(a0, a1);
}

std::chrono::month f218(const std::chrono::months &a0,
                        const std::chrono::month &a1) {
  return operator+(a0, a1);
}

std::chrono::month f219(const std::chrono::month &a0,
                        const std::chrono::months &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f220(const std::chrono::month &a0,
                          const std::chrono::month &a1) {
  return operator<=>(a0, a1);
}

bool f221(const std::chrono::month &a0, const std::chrono::month &a1) {
  return operator==(a0, a1);
}

using t55 = std::chrono::month_day;

std::chrono::month_day f222() { return std::chrono::month_day(); }

std::chrono::month_day f223(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return std::chrono::month_day(a0, a1);
}

std::chrono::month f224(const std::chrono::month_day &a0) { return a0.month(); }

std::chrono::day f225(const std::chrono::month_day &a0) { return a0.day(); }

bool f226(const std::chrono::month_day &a0) { return a0.ok(); }

std::chrono::month_day f227(const std::chrono::month_day &a0) {
  return std::chrono::month_day(a0);
}

std::chrono::month_day f228(std::chrono::month_day &&a0) {
  return std::chrono::month_day(std::move(a0));
}

std::chrono::month_day &f229(std::chrono::month_day &a0,
                             const std::chrono::month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day &f230(std::chrono::month_day &a0,
                             std::chrono::month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day f231(const std::chrono::day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f232(const std::chrono::day &a0,
                            const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f233(int a0, const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f234(const std::chrono::month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::month_day f235(const std::chrono::month &a0,
                            const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::strong_ordering f236(const std::chrono::month_day &a0,
                          const std::chrono::month_day &a1) {
  return operator<=>(a0, a1);
}

bool f237(const std::chrono::month_day &a0, const std::chrono::month_day &a1) {
  return operator==(a0, a1);
}

using t56 = std::chrono::month_day_last;

std::chrono::month_day_last f238(const std::chrono::month &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month f239(const std::chrono::month_day_last &a0) {
  return a0.month();
}

bool f240(const std::chrono::month_day_last &a0) { return a0.ok(); }

std::chrono::month_day_last f241(const std::chrono::month_day_last &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month_day_last f242(std::chrono::month_day_last &&a0) {
  return std::chrono::month_day_last(std::move(a0));
}

std::chrono::month_day_last &f243(std::chrono::month_day_last &a0,
                                  const std::chrono::month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day_last &f244(std::chrono::month_day_last &a0,
                                  std::chrono::month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day_last f245(std::chrono::last_spec a0, int a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f246(std::chrono::last_spec a0,
                                 const std::chrono::month &a1) {
  return operator/(std::move(a0), a1);
}

std::chrono::month_day_last f247(int a0, std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::month_day_last f248(const std::chrono::month &a0,
                                 std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::strong_ordering f249(const std::chrono::month_day_last &a0,
                          const std::chrono::month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f250(const std::chrono::month_day_last &a0,
          const std::chrono::month_day_last &a1) {
  return operator==(a0, a1);
}

using t57 = std::chrono::month_weekday;

std::chrono::month_weekday f251(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return std::chrono::month_weekday(a0, a1);
}

std::chrono::month f252(const std::chrono::month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday_indexed f253(const std::chrono::month_weekday &a0) {
  return a0.weekday_indexed();
}

bool f254(const std::chrono::month_weekday &a0) { return a0.ok(); }

std::chrono::month_weekday f255(const std::chrono::month_weekday &a0) {
  return std::chrono::month_weekday(a0);
}

std::chrono::month_weekday f256(std::chrono::month_weekday &&a0) {
  return std::chrono::month_weekday(std::move(a0));
}

std::chrono::month_weekday &f257(std::chrono::month_weekday &a0,
                                 const std::chrono::month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday &f258(std::chrono::month_weekday &a0,
                                 std::chrono::month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday f259(const std::chrono::weekday_indexed &a0,
                                int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f260(const std::chrono::weekday_indexed &a0,
                                const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f261(int a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday f262(const std::chrono::month &a0,
                                const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

bool f263(const std::chrono::month_weekday &a0,
          const std::chrono::month_weekday &a1) {
  return operator==(a0, a1);
}

using t58 = std::chrono::month_weekday_last;

std::chrono::month_weekday_last f264(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return std::chrono::month_weekday_last(a0, a1);
}

std::chrono::month f265(const std::chrono::month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday_last f266(const std::chrono::month_weekday_last &a0) {
  return a0.weekday_last();
}

bool f267(const std::chrono::month_weekday_last &a0) { return a0.ok(); }

std::chrono::month_weekday_last
f268(const std::chrono::month_weekday_last &a0) {
  return std::chrono::month_weekday_last(a0);
}

std::chrono::month_weekday_last f269(std::chrono::month_weekday_last &&a0) {
  return std::chrono::month_weekday_last(std::move(a0));
}

std::chrono::month_weekday_last &
f270(std::chrono::month_weekday_last &a0,
     const std::chrono::month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday_last &f271(std::chrono::month_weekday_last &a0,
                                      std::chrono::month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday_last f272(const std::chrono::weekday_last &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f273(const std::chrono::weekday_last &a0,
                                     const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f274(int a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::month_weekday_last f275(const std::chrono::month &a0,
                                     const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

bool f276(const std::chrono::month_weekday_last &a0,
          const std::chrono::month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t59 = std::chrono::months;

using t60 = std::chrono::nonexistent_local_time;

std::chrono::nonexistent_local_time
f277(const std::chrono::nonexistent_local_time &a0) {
  return std::chrono::nonexistent_local_time(a0);
}

std::chrono::nonexistent_local_time
f278(std::chrono::nonexistent_local_time &&a0) {
  return std::chrono::nonexistent_local_time(std::move(a0));
}

std::chrono::nonexistent_local_time &
f279(std::chrono::nonexistent_local_time &a0,
     const std::chrono::nonexistent_local_time &a1) {
  return a0.operator=(a1);
}

std::chrono::nonexistent_local_time &
f280(std::chrono::nonexistent_local_time &a0,
     std::chrono::nonexistent_local_time &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::nonexistent_local_time f281(const std::chrono::local_time<T1> &a0,
                                         const std::chrono::local_info &a1) {
  return std::chrono::nonexistent_local_time(a0, a1);
}

const char *f282(const std::chrono::nonexistent_local_time &a0) {
  return a0.what();
}

template <typename T1, typename T2, typename T3>
auto f283(const std::chrono::time_point<T1, T2> &a0,
          const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f284(const std::chrono::duration<T1, T2> &a0,
          const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f285(const std::chrono::zoned_time<T1, T2> &a0,
          const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

std::string f286() { return std::chrono::remote_version(); }

std::chrono::steady_clock::time_point &
f287(std::chrono::steady_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::steady_clock::time_point
f288(std::chrono::steady_clock::time_point &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::steady_clock::time_point &
f289(std::chrono::steady_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::steady_clock::time_point
f290(std::chrono::steady_clock::time_point &a0, int a1) {
  return a0.operator--(a1);
}

using t61 = std::chrono::sys_days;

using t62 = std::chrono::sys_info;

std::chrono::sys_info f291() { return std::chrono::sys_info(); }

std::chrono::sys_info f292(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f293(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f294(std::chrono::sys_info &a0,
                            const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f295(std::chrono::sys_info &a0,
                            std::chrono::sys_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t63 = std::chrono::sys_seconds;

template <typename T1> using t64 = std::chrono::sys_time<T1>;

std::chrono::system_clock::time_point &
f296(std::chrono::system_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::system_clock::time_point
f297(std::chrono::system_clock::time_point &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::system_clock::time_point &
f298(std::chrono::system_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::system_clock::time_point
f299(std::chrono::system_clock::time_point &a0, int a1) {
  return a0.operator--(a1);
}

using t65 = std::chrono::tai_clock;

using t66 = std::chrono::tai_clock::rep;

using t67 = std::chrono::tai_clock::period;

using t68 = std::chrono::tai_clock::duration;

using t69 = std::chrono::tai_clock::time_point;

std::chrono::tai_clock::time_point f300() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f301() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f302(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f303(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f304(std::chrono::tai_clock &a0,
                             const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f305(std::chrono::tai_clock &a0,
                             std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f306(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f307(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

std::chrono::tai_clock::duration f308() {
  return std::chrono::tai_clock::duration();
}

std::chrono::tai_clock::duration
f309(const std::chrono::tai_clock::duration &a0) {
  return std::chrono::tai_clock::duration(a0);
}

std::chrono::tai_clock::duration &
f310(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f311(const std::chrono::tai_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f312(const std::chrono::tai_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f313(const std::chrono::tai_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::tai_clock::duration &f314(std::chrono::tai_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::tai_clock::duration f315(std::chrono::tai_clock::duration &a0,
                                      int a1) {
  return a0.operator++(a1);
}

std::chrono::tai_clock::duration &f316(std::chrono::tai_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::tai_clock::duration f317(std::chrono::tai_clock::duration &a0,
                                      int a1) {
  return a0.operator--(a1);
}

std::chrono::tai_clock::duration &
f318(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::tai_clock::duration &
f319(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::tai_clock::duration &
f320(std::chrono::tai_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::tai_clock::duration &
f321(std::chrono::tai_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::tai_clock::duration f322() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::tai_clock::duration f323() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::tai_clock::duration f324() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::is_convertible<const T1 &,
                                std::chrono::tai_clock::rep>::value &&
                std::integral_constant<
                    bool, std::integral_constant<bool, false>::value ||
                              std::integral_constant<
                                  bool, !std::chrono::treat_as_floating_point<
                                            T1>::value>::value>::value>::value,
        void>::type>
std::chrono::tai_clock::duration f325(const T1 &a0) {
  return std::chrono::tai_clock::duration(a0);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f326(std::chrono::tai_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f327(std::chrono::tai_clock::duration &a0,
     const std::chrono::tai_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t70 = std::chrono::time_point<std::chrono::tai_clock>::clock;

using t71 = std::chrono::time_point<std::chrono::tai_clock>::duration;

using t72 = std::chrono::time_point<std::chrono::tai_clock>::rep;

using t73 = std::chrono::time_point<std::chrono::tai_clock>::period;

std::chrono::tai_clock::time_point f328() {
  return std::chrono::tai_clock::time_point();
}

std::chrono::tai_clock::time_point
f329(const std::chrono::time_point<std::chrono::tai_clock>::duration &a0) {
  return std::chrono::tai_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::tai_clock>::duration
f330(const std::chrono::tai_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::tai_clock::time_point &
f331(std::chrono::tai_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::tai_clock::time_point f332(std::chrono::tai_clock::time_point &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::tai_clock::time_point &
f333(std::chrono::tai_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::tai_clock::time_point f334(std::chrono::tai_clock::time_point &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::tai_clock::time_point &
f335(std::chrono::tai_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::tai_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::tai_clock::time_point &
f336(std::chrono::tai_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::tai_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::tai_clock::time_point f337() {
  return std::chrono::time_point<std::chrono::tai_clock>::min();
}

std::chrono::tai_clock::time_point f338() {
  return std::chrono::time_point<std::chrono::tai_clock>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool, std::is_convertible<T1, typename std::chrono::tai_clock::
                                              duration>::value>::value,
        void>::type>
std::chrono::tai_clock::time_point
f339(const std::chrono::time_point<
     std::chrono::time_point<
         std::chrono::tai_clock,
         std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
     T1> &a0) {
  return std::chrono::tai_clock::time_point(a0);
}

bool f340() { return std::chrono::tai_clock::is_steady; }

using t74 = std::chrono::tai_seconds;

template <typename T1> using t75 = std::chrono::tai_time<T1>;

using t76 = std::chrono::time_zone;

std::string_view f341(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f342(const std::chrono::time_zone &a0,
                           const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f343(const std::chrono::time_zone &a0,
                             const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f344(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f345(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
     std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f346(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}

std::strong_ordering f347(const std::chrono::time_zone &a0,
                          const std::chrono::time_zone &a1) {
  return operator<=>(a0, a1);
}

bool f348(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return operator==(a0, a1);
}

using t77 = std::chrono::time_zone_link;

std::chrono::time_zone_link f349(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f350(std::chrono::time_zone_link &a0,
                                  std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f351(const std::chrono::time_zone_link &a0) {
  return a0.name();
}

std::string_view f352(const std::chrono::time_zone_link &a0) {
  return a0.target();
}

std::strong_ordering f353(const std::chrono::time_zone_link &a0,
                          const std::chrono::time_zone_link &a1) {
  return operator<=>(a0, a1);
}

bool f354(const std::chrono::time_zone_link &a0,
          const std::chrono::time_zone_link &a1) {
  return operator==(a0, a1);
}

using t78 = std::chrono::tzdb;

const std::chrono::time_zone *f355(const std::chrono::tzdb &a0,
                                   std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f356(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f357() { return std::chrono::tzdb(); }

std::chrono::tzdb f358(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f359(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}

using t79 = std::chrono::tzdb_list;

using t80 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f360(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f361(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f362(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f363(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f364(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f365(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t81 = std::chrono::utc_clock;

using t82 = std::chrono::utc_clock::rep;

using t83 = std::chrono::utc_clock::period;

using t84 = std::chrono::utc_clock::duration;

using t85 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f366() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f367() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f368(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f369(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f370(std::chrono::utc_clock &a0,
                             const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f371(std::chrono::utc_clock &a0,
                             std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f372(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f373(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

std::chrono::utc_clock::duration f374() {
  return std::chrono::utc_clock::duration();
}

std::chrono::utc_clock::duration
f375(const std::chrono::utc_clock::duration &a0) {
  return std::chrono::utc_clock::duration(a0);
}

std::chrono::utc_clock::duration &
f376(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f377(const std::chrono::utc_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f378(const std::chrono::utc_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f379(const std::chrono::utc_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::utc_clock::duration &f380(std::chrono::utc_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::utc_clock::duration f381(std::chrono::utc_clock::duration &a0,
                                      int a1) {
  return a0.operator++(a1);
}

std::chrono::utc_clock::duration &f382(std::chrono::utc_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::utc_clock::duration f383(std::chrono::utc_clock::duration &a0,
                                      int a1) {
  return a0.operator--(a1);
}

std::chrono::utc_clock::duration &
f384(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::utc_clock::duration &
f385(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::utc_clock::duration &
f386(std::chrono::utc_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::utc_clock::duration &
f387(std::chrono::utc_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::utc_clock::duration f388() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::utc_clock::duration f389() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::utc_clock::duration f390() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::is_convertible<const T1 &,
                                std::chrono::utc_clock::rep>::value &&
                std::integral_constant<
                    bool, std::integral_constant<bool, false>::value ||
                              std::integral_constant<
                                  bool, !std::chrono::treat_as_floating_point<
                                            T1>::value>::value>::value>::value,
        void>::type>
std::chrono::utc_clock::duration f391(const T1 &a0) {
  return std::chrono::utc_clock::duration(a0);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f392(std::chrono::utc_clock::duration &a0,
     const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f393(std::chrono::utc_clock::duration &a0,
     const std::chrono::utc_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t86 = std::chrono::time_point<std::chrono::utc_clock>::clock;

using t87 = std::chrono::time_point<std::chrono::utc_clock>::duration;

using t88 = std::chrono::time_point<std::chrono::utc_clock>::rep;

using t89 = std::chrono::time_point<std::chrono::utc_clock>::period;

std::chrono::utc_clock::time_point f394() {
  return std::chrono::utc_clock::time_point();
}

std::chrono::utc_clock::time_point
f395(const std::chrono::time_point<std::chrono::utc_clock>::duration &a0) {
  return std::chrono::utc_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::utc_clock>::duration
f396(const std::chrono::utc_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::utc_clock::time_point &
f397(std::chrono::utc_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::utc_clock::time_point f398(std::chrono::utc_clock::time_point &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::utc_clock::time_point &
f399(std::chrono::utc_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::utc_clock::time_point f400(std::chrono::utc_clock::time_point &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::utc_clock::time_point &
f401(std::chrono::utc_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::utc_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::utc_clock::time_point &
f402(std::chrono::utc_clock::time_point &a0,
     const std::chrono::time_point<std::chrono::utc_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::utc_clock::time_point f403() {
  return std::chrono::time_point<std::chrono::utc_clock>::min();
}

std::chrono::utc_clock::time_point f404() {
  return std::chrono::time_point<std::chrono::utc_clock>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool, std::is_convertible<T1, typename std::chrono::utc_clock::
                                              duration>::value>::value,
        void>::type>
std::chrono::utc_clock::time_point
f405(const std::chrono::time_point<
     std::chrono::time_point<
         std::chrono::utc_clock,
         std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
     T1> &a0) {
  return std::chrono::utc_clock::time_point(a0);
}

bool f406() { return std::chrono::utc_clock::is_steady; }

using t90 = std::chrono::utc_seconds;

template <typename T1> using t91 = std::chrono::utc_time<T1>;

using t92 = std::chrono::weekday;

std::chrono::weekday f407() { return std::chrono::weekday(); }

std::chrono::weekday f408(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f409(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f410(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f411(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f412(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f413(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f414(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f415(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f416(std::chrono::weekday &a0,
                           const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f417(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f418(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f419(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f420(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f421(const std::chrono::weekday &a0,
                               std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f422(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f423(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f424(std::chrono::weekday &a0,
                           const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f425(std::chrono::weekday &a0,
                           std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::days f426(const std::chrono::weekday &a0,
                       const std::chrono::weekday &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f427(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator-(a0, a1);
}

std::chrono::weekday f428(const std::chrono::days &a0,
                          const std::chrono::weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::weekday f429(const std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return operator+(a0, a1);
}

bool f430(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return operator==(a0, a1);
}

using t93 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f431() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f432(const std::chrono::weekday &a0,
                                  unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f433(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f434(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f435(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f436(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f437(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f438(std::chrono::weekday_indexed &a0,
                                   const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f439(std::chrono::weekday_indexed &a0,
                                   std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

bool f440(const std::chrono::weekday_indexed &a0,
          const std::chrono::weekday_indexed &a1) {
  return operator==(a0, a1);
}

using t94 = std::chrono::weekday_last;

std::chrono::weekday_last f441(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f442(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f443(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f444(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f445(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f446(std::chrono::weekday_last &a0,
                                const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f447(std::chrono::weekday_last &a0,
                                std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

bool f448(const std::chrono::weekday_last &a0,
          const std::chrono::weekday_last &a1) {
  return operator==(a0, a1);
}

using t95 = std::chrono::weeks;

using t96 = std::chrono::year;

std::chrono::year f449() { return std::chrono::year(); }

std::chrono::year f450(int a0) { return std::chrono::year(a0); }

std::chrono::year f451() { return std::chrono::year::min(); }

std::chrono::year f452() { return std::chrono::year::max(); }

std::chrono::year &f453(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f454(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f455(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f456(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f457(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f458(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f459(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f460(const std::chrono::year &a0) { return a0.operator-(); }

bool f461(const std::chrono::year &a0) { return a0.is_leap(); }

int f462(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f463(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f464(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f465(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f466(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f467(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::years f468(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return operator-(a0, a1);
}

std::chrono::year f469(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year f470(const std::chrono::years &a0,
                       const std::chrono::year &a1) {
  return operator+(a0, a1);
}

std::chrono::year f471(const std::chrono::year &a0,
                       const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f472(const std::chrono::year &a0,
                          const std::chrono::year &a1) {
  return operator<=>(a0, a1);
}

bool f473(const std::chrono::year &a0, const std::chrono::year &a1) {
  return operator==(a0, a1);
}

using t97 = std::chrono::year_month;

std::chrono::year_month f474() { return std::chrono::year_month(); }

std::chrono::year_month f475(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f476(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f477(const std::chrono::year_month &a0) {
  return a0.month();
}

std::chrono::year_month &f478(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f479(std::chrono::year_month &a0,
                              const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f480(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f481(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f482(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f483(std::chrono::year_month &a0,
                              const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f484(std::chrono::year_month &a0,
                              std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month f485(const std::chrono::year &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f486(const std::chrono::year &a0,
                             const std::chrono::month &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month f487(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month f488(const std::chrono::years &a0,
                             const std::chrono::year_month &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month f489(const std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::chrono::months f490(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return operator-(a0, a1);
}

std::strong_ordering f491(const std::chrono::year_month &a0,
                          const std::chrono::year_month &a1) {
  return operator<=>(a0, a1);
}

bool f492(const std::chrono::year_month &a0,
          const std::chrono::year_month &a1) {
  return operator==(a0, a1);
}

using t98 = std::chrono::year_month_day;

std::chrono::year_month_day f493() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f494(const std::chrono::year &a0,
                                 const std::chrono::month &a1,
                                 const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f495(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f496(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f497(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f498(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f499(std::chrono::year_month_day &a0,
                                  const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f500(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f501(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f502(const std::chrono::year_month_day &a0) {
  return a0.day();
}

std::chrono::sys_days f503(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f504(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f505(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f506(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f507(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f508(std::chrono::year_month_day &a0,
                                  const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f509(std::chrono::year_month_day &a0,
                                  std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_day f510(const std::chrono::month_day &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f511(const std::chrono::month_day &a0,
                                 const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f512(int a0, const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f513(const std::chrono::year &a0,
                                 const std::chrono::month_day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f514(const std::chrono::year_month &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f515(const std::chrono::year_month &a0,
                                 const std::chrono::day &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day f516(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day f517(const std::chrono::years &a0,
                                 const std::chrono::year_month_day &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day f518(const std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f519(const std::chrono::year_month_day &a0,
                          const std::chrono::year_month_day &a1) {
  return operator<=>(a0, a1);
}

bool f520(const std::chrono::year_month_day &a0,
          const std::chrono::year_month_day &a1) {
  return operator==(a0, a1);
}

using t99 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f521(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f522(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f523(std::chrono::year_month_day_last &a0,
                                       const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f524(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f525(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f526(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f527(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f528(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f529(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f530(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f531(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f532(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f533(std::chrono::year_month_day_last &a0,
     const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f534(std::chrono::year_month_day_last &a0,
                                       std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_day_last f535(const std::chrono::month_day_last &a0,
                                      int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f536(const std::chrono::month_day_last &a0,
                                      const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f537(int a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f538(const std::chrono::year &a0,
                                      const std::chrono::month_day_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_day_last f539(const std::chrono::year_month &a0,
                                      std::chrono::last_spec a1) {
  return operator/(a0, std::move(a1));
}

std::chrono::year_month_day_last
f540(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_day_last
f541(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_day_last
f542(const std::chrono::year_month_day_last &a0, const std::chrono::years &a1) {
  return operator+(a0, a1);
}

std::strong_ordering f543(const std::chrono::year_month_day_last &a0,
                          const std::chrono::year_month_day_last &a1) {
  return operator<=>(a0, a1);
}

bool f544(const std::chrono::year_month_day_last &a0,
          const std::chrono::year_month_day_last &a1) {
  return operator==(a0, a1);
}

using t100 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f545() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f546(const std::chrono::year &a0,
                                     const std::chrono::month &a1,
                                     const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f547(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f548(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f549(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f550(std::chrono::year_month_weekday &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f551(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f552(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f553(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f554(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f555(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f556(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f557(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f558(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday
f559(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f560(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f561(std::chrono::year_month_weekday &a0,
     const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f562(std::chrono::year_month_weekday &a0,
                                      std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_weekday f563(const std::chrono::month_weekday &a0,
                                     int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f564(const std::chrono::month_weekday &a0,
                                     const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f565(int a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f566(const std::chrono::year &a0,
                                     const std::chrono::month_weekday &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f567(const std::chrono::year_month &a0,
                                     const std::chrono::weekday_indexed &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday f568(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday
f569(const std::chrono::years &a0, const std::chrono::year_month_weekday &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday f570(const std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

bool f571(const std::chrono::year_month_weekday &a0,
          const std::chrono::year_month_weekday &a1) {
  return operator==(a0, a1);
}

using t101 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f572(const std::chrono::year &a0,
                                          const std::chrono::month &a1,
                                          const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f573(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f574(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f575(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f576(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f577(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f578(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f579(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f580(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f581(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f582(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f583(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f584(std::chrono::year_month_weekday_last &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f585(std::chrono::year_month_weekday_last &a0,
     std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::year_month_weekday_last
f586(const std::chrono::month_weekday_last &a0, int a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f587(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f588(int a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f589(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last f590(const std::chrono::year_month &a0,
                                          const std::chrono::weekday_last &a1) {
  return operator/(a0, a1);
}

std::chrono::year_month_weekday_last
f591(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator-(a0, a1);
}

std::chrono::year_month_weekday_last
f592(const std::chrono::years &a0,
     const std::chrono::year_month_weekday_last &a1) {
  return operator+(a0, a1);
}

std::chrono::year_month_weekday_last
f593(const std::chrono::year_month_weekday_last &a0,
     const std::chrono::years &a1) {
  return operator+(a0, a1);
}

bool f594(const std::chrono::year_month_weekday_last &a0,
          const std::chrono::year_month_weekday_last &a1) {
  return operator==(a0, a1);
}

using t102 = std::chrono::years;

using t103 = std::chrono::zoned_seconds;
