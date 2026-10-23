#include <chrono>
#include <cstddef>
#include <format>
#include <string>
#include <string_view>
#include <utility>

using t52 = std::formatter<char, char>;

std::formatter<char, char> f125() { return std::formatter<char, char>(); }

std::formatter<char, char> f126(const std::formatter<char, char> &a0) {
  return std::formatter<char, char>(a0);
}

std::formatter<char, char> f127(std::formatter<char, char> &&a0) {
  return std::formatter<char, char>(std::move(a0));
}

std::formatter<char, char> &f128(std::formatter<char, char> &a0,
                                 const std::formatter<char, char> &a1) {
  return a0.operator=(a1);
}

std::formatter<char, char> &f129(std::formatter<char, char> &a0,
                                 std::formatter<char, char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t53 = std::formatter<wchar_t, wchar_t>;

std::formatter<wchar_t, wchar_t> f130() {
  return std::formatter<wchar_t, wchar_t>();
}

std::formatter<wchar_t, wchar_t>
f131(const std::formatter<wchar_t, wchar_t> &a0) {
  return std::formatter<wchar_t, wchar_t>(a0);
}

std::formatter<wchar_t, wchar_t> f132(std::formatter<wchar_t, wchar_t> &&a0) {
  return std::formatter<wchar_t, wchar_t>(std::move(a0));
}

std::formatter<wchar_t, wchar_t> &
f133(std::formatter<wchar_t, wchar_t> &a0,
     const std::formatter<wchar_t, wchar_t> &a1) {
  return a0.operator=(a1);
}

std::formatter<wchar_t, wchar_t> &f134(std::formatter<wchar_t, wchar_t> &a0,
                                       std::formatter<wchar_t, wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
typename T2::iterator f135(std::formatter<std::chrono::day, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f136(const std::formatter<std::chrono::day, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f137(std::formatter<std::chrono::month, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f138(const std::formatter<std::chrono::month, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f139(std::formatter<std::chrono::month_day, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f140(const std::formatter<std::chrono::month_day, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f141(std::formatter<std::chrono::month_day_last, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f142(const std::formatter<std::chrono::month_day_last, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f143(std::formatter<std::chrono::month_weekday, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f144(const std::formatter<std::chrono::month_weekday, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f145(std::formatter<std::chrono::month_weekday_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f146(const std::formatter<std::chrono::month_weekday_last, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f147(std::formatter<std::chrono::weekday, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f148(const std::formatter<std::chrono::weekday, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f149(std::formatter<std::chrono::weekday_indexed, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f150(const std::formatter<std::chrono::weekday_indexed, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f151(std::formatter<std::chrono::weekday_last, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f152(const std::formatter<std::chrono::weekday_last, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f153(std::formatter<std::chrono::year, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f154(const std::formatter<std::chrono::year, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f155(std::formatter<std::chrono::year_month, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f156(const std::formatter<std::chrono::year_month, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f157(std::formatter<std::chrono::year_month_day, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f158(const std::formatter<std::chrono::year_month_day, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f159(std::formatter<std::chrono::year_month_day_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f160(const std::formatter<std::chrono::year_month_day_last, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f161(std::formatter<std::chrono::year_month_weekday, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f162(const std::formatter<std::chrono::year_month_weekday, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f163(std::formatter<std::chrono::year_month_weekday_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f164(const std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f165(const std::formatter<const T1 *, T1> &a0,
                           const T1 *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f166(std::formatter<const T1 *, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f167(std::formatter<const void *, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f168(const std::formatter<const void *, T1> &a0,
                           const void *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f169(const std::formatter<T1 *, T1> &a0, T1 *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f170(std::formatter<void *, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f171(const std::formatter<void *, T1> &a0, const void *a1,
                           T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t54 = std::formatter<long double, T1>;

template <typename T1, typename T2>
typename T2::iterator f172(std::formatter<long double, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f173(const std::formatter<long double, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t55 = std::formatter<int, T1>;

template <typename T1, typename T2>
typename T2::iterator f174(std::formatter<int, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f175(const std::formatter<int, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t56 = std::formatter<long long, T1>;

template <typename T1, typename T2>
typename T2::iterator f176(std::formatter<long long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f177(const std::formatter<long long, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t57 = std::formatter<long, T1>;

template <typename T1, typename T2>
typename T2::iterator f178(std::formatter<long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f179(const std::formatter<long, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t58 = std::formatter<short, T1>;

template <typename T1, typename T2>
typename T2::iterator f180(std::formatter<short, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f181(const std::formatter<short, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t59 = std::formatter<bool, T1>;

template <typename T1, typename T2>
typename T2::iterator f182(std::formatter<bool, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f183(const std::formatter<bool, T1> &a0, bool a1,
                           T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t60 = std::formatter<unsigned char, T1>;

template <typename T1, typename T2>
typename T2::iterator f184(std::formatter<unsigned char, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f185(const std::formatter<unsigned char, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t61 = std::formatter<double, T1>;

template <typename T1, typename T2>
typename T2::iterator f186(std::formatter<double, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f187(const std::formatter<double, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t62 = std::formatter<float, T1>;

template <typename T1, typename T2>
typename T2::iterator f188(std::formatter<float, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f189(const std::formatter<float, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t63 = std::formatter<unsigned int, T1>;

template <typename T1, typename T2>
typename T2::iterator f190(std::formatter<unsigned int, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f191(const std::formatter<unsigned int, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t64 = std::formatter<__uint128_t, T1>;

template <typename T1, typename T2>
typename T2::iterator f192(std::formatter<__uint128_t, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f193(const std::formatter<__uint128_t, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t65 = std::formatter<unsigned long long, T1>;

template <typename T1, typename T2>
typename T2::iterator f194(std::formatter<unsigned long long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f195(const std::formatter<unsigned long long, T1> &a0,
                           T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t66 = std::formatter<unsigned long, T1>;

template <typename T1, typename T2>
typename T2::iterator f196(std::formatter<unsigned long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f197(const std::formatter<unsigned long, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f198(std::formatter<std::nullptr_t, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f199(const std::formatter<std::nullptr_t, T1> &a0,
                           const void *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t67 = std::formatter<signed char, T1>;

template <typename T1, typename T2>
typename T2::iterator f200(std::formatter<signed char, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f201(const std::formatter<signed char, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t68 = std::formatter<unsigned short, T1>;

template <typename T1, typename T2>
typename T2::iterator f202(std::formatter<unsigned short, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f203(const std::formatter<unsigned short, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
using t69 = std::formatter<std::chrono::hh_mm_ss<T1>, T2>;

template <typename T1, typename T2, typename T3>
typename T3::iterator f204(std::formatter<std::chrono::hh_mm_ss<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f205(const std::formatter<std::chrono::hh_mm_ss<T1>, T2> &a0, const T3 &a1,
     T4 &a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f206(std::formatter<std::chrono::local_time<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f207(const std::formatter<std::chrono::local_time<T1>, T2> &a0, const T3 &a1,
     T4 &a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f208(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f209(const std::formatter<std::chrono::sys_time<T1>, T2> &a0, const T3 &a1,
     T4 &a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2>
using t70 = std::formatter<std::basic_string_view<T1, T2>, T1>;

template <typename T1, typename T2, typename T3>
typename T3::iterator
f210(const std::formatter<std::basic_string_view<T1, T2>, T1> &a0,
     std::basic_string_view<T1, T2> a1, T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f211(std::formatter<std::basic_string_view<T1, T2>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
using t71 = std::formatter<std::chrono::duration<T1, T2>, T3>;

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f212(std::formatter<std::chrono::duration<T1, T2>, T3> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename T5::iterator
f213(const std::formatter<std::chrono::duration<T1, T2>, T3> &a0, const T4 &a1,
     T5 &a2) {
  return a0.template format<T4, T5>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t72 = std::formatter<std::basic_string<T1, T2, T3>, T1>;

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f214(const std::formatter<std::basic_string<T1, T2, T3>, T1> &a0,
     const std::basic_string<T1, T2, T3> &a1, T4 &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f215(std::formatter<std::basic_string<T1, T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}
