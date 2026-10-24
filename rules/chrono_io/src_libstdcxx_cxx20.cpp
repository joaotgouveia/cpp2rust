#include <barrier>
#include <chrono>
#include <iosfwd>
#include <istream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f1(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::day &a2,
   std::basic_string<T1, T2, T3> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f2(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::month &a2,
   std::basic_string<T1, T2, T3> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f3(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::month_day &a2,
   std::basic_string<T1, T2, T3> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f4(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::weekday &a2,
   std::basic_string<T1, T2, T3> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f5(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::year &a2,
   std::basic_string<T1, T2, T3> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f6(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::year_month &a2,
   std::basic_string<T1, T2, T3> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f7(std::basic_istream<T1, T2> &a0, const T1 *a1,
                               std::chrono::year_month_day &a2,
                               std::basic_string<T1, T2, T3> *a3,
                               std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istream<T1, T2> &
f8(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::sys_time<T3> &a2,
   std::basic_string<T1, T2, T4> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istream<T1, T2> &
f9(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::gps_time<T3> &a2,
   std::basic_string<T1, T2, T4> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istream<T1, T2> &f10(std::basic_istream<T1, T2> &a0, const T1 *a1,
                                std::chrono::local_time<T3> &a2,
                                std::basic_string<T1, T2, T4> *a3,
                                std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istream<T1, T2> &
f11(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::tai_time<T3> &a2,
    std::basic_string<T1, T2, T4> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istream<T1, T2> &
f12(std::basic_istream<T1, T2> &a0, const T1 *a1, std::chrono::utc_time<T3> &a2,
    std::basic_string<T1, T2, T4> *a3, std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istream<T1, T2> &f13(std::basic_istream<T1, T2> &a0, const T1 *a1,
                                std::chrono::file_time<T3> &a2,
                                std::basic_string<T1, T2, T4> *a3,
                                std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::basic_istream<T1, T2> &f14(std::basic_istream<T1, T2> &a0, const T1 *a1,
                                std::chrono::duration<T3, T4> &a2,
                                std::basic_string<T1, T2, T5> *a3,
                                std::chrono::minutes *a4) {
  return std::chrono::from_stream<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1>
auto f15(std::chrono::local_time<T1> a0, const std::string *a1,
         const std::chrono::seconds *a2) {
  return std::chrono::local_time_format<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::day &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f17(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::local_info &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f18(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f19(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_day &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f20(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_day_last &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f21(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_weekday &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f22(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_weekday_last &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f23(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::sys_info &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f24(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::sys_days &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f25(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::weekday &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f26(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::weekday_indexed &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f27(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::weekday_last &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f28(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f29(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f30(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month_day &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f31(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month_day_last &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f32(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month_weekday &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f33(std::basic_ostream<T1, T2> &a0,
    const std::chrono::year_month_weekday_last &a1) {
  return std::chrono::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f34(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::hh_mm_ss<T3> &a1) {
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f35(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::sys_time<T3> &a1)
  requires(
      (!std::chrono::treat_as_floating_point_v<typename T3::rep>) &&
      std::ratio_less_v<typename T3::period,
                        std::chrono::duration<long, std::ratio<86400>>::period>)
{
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f36(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::gps_time<T3> &a1) {
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f37(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::local_time<T3> &a1) {
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f38(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::tai_time<T3> &a1) {
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f39(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::utc_time<T3> &a1) {
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f40(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::file_time<T3> &a1) {
  return std::chrono::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f41(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f42(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2>
auto f43(const T1 *a0, T2 &a1)
  requires(std::chrono::__detail::__parsable<T2, T1>)
{
  return std::chrono::parse<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f44(const std::basic_string<T1, T2, T3> &a0, T4 &a1)
  requires(std::chrono::__detail::__parsable<T4, T1, T2>)
{
  return std::chrono::parse<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f45(const T1 *a0, T4 &a1, std::chrono::minutes &a2)
  requires(
      std::chrono::__detail::__parsable<T4, T1, T2, T3, std::chrono::minutes>)
{
  return std::chrono::parse<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f46(const std::basic_string<T1, T2, T3> &a0, T5 &a1,
         std::chrono::minutes &a2)
  requires(
      std::chrono::__detail::__parsable<T5, T1, T2, T4, std::chrono::minutes>)
{
  return std::chrono::parse<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f47(const std::basic_string<T1, T2, T3> &a0, T5 &a1,
         std::basic_string<T1, T2, T3> &a2)
  requires(std::chrono::__detail::__parsable<T5, T1, T2, T4>)
{
  return std::chrono::parse<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f48(const std::basic_string<T1, T2, T3> &a0, T5 &a1,
         std::basic_string<T1, T2, T3> &a2, std::chrono::minutes &a3)
  requires(
      std::chrono::__detail::__parsable<T5, T1, T2, T4, std::chrono::minutes>)
{
  return std::chrono::parse<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f49(const T1 *a0, T5 &a1, std::basic_string<T1, T2, T3> &a2)
  requires(std::chrono::__detail::__parsable<T5, T1, T2, T4>)
{
  return std::chrono::parse<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f50(const T1 *a0, T5 &a1, std::basic_string<T1, T2, T3> &a2,
         std::chrono::minutes &a3)
  requires(
      std::chrono::__detail::__parsable<T5, T1, T2, T4, std::chrono::minutes>)
{
  return std::chrono::parse<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}
