#include <chrono>
#include <cstddef>
#include <format>
#include <string>
#include <string_view>
#include <utility>

using t50 = std::formatter<char, char>;

std::formatter<char, char> f120() { return std::formatter<char, char>(); }

std::formatter<char, char> f121(const std::formatter<char, char> &a0) {
  return std::formatter<char, char>(a0);
}

std::formatter<char, char> f122(std::formatter<char, char> &&a0) {
  return std::formatter<char, char>(std::move(a0));
}

std::formatter<char, char> &f123(std::formatter<char, char> &a0,
                                 const std::formatter<char, char> &a1) {
  return a0.operator=(a1);
}

std::formatter<char, char> &f124(std::formatter<char, char> &a0,
                                 std::formatter<char, char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t51 = std::formatter<wchar_t, wchar_t>;

std::formatter<wchar_t, wchar_t> f125() {
  return std::formatter<wchar_t, wchar_t>();
}

std::formatter<wchar_t, wchar_t>
f126(const std::formatter<wchar_t, wchar_t> &a0) {
  return std::formatter<wchar_t, wchar_t>(a0);
}

std::formatter<wchar_t, wchar_t> f127(std::formatter<wchar_t, wchar_t> &&a0) {
  return std::formatter<wchar_t, wchar_t>(std::move(a0));
}

std::formatter<wchar_t, wchar_t> &
f128(std::formatter<wchar_t, wchar_t> &a0,
     const std::formatter<wchar_t, wchar_t> &a1) {
  return a0.operator=(a1);
}

std::formatter<wchar_t, wchar_t> &f129(std::formatter<wchar_t, wchar_t> &a0,
                                       std::formatter<wchar_t, wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
typename T2::iterator f130(std::formatter<std::chrono::day, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f131(const std::formatter<std::chrono::day, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f132(std::formatter<std::chrono::month, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f133(const std::formatter<std::chrono::month, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f134(std::formatter<std::chrono::month_day, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f135(const std::formatter<std::chrono::month_day, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f136(std::formatter<std::chrono::month_day_last, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f137(const std::formatter<std::chrono::month_day_last, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f138(std::formatter<std::chrono::month_weekday, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f139(const std::formatter<std::chrono::month_weekday, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f140(std::formatter<std::chrono::month_weekday_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f141(const std::formatter<std::chrono::month_weekday_last, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f142(std::formatter<std::chrono::weekday, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f143(const std::formatter<std::chrono::weekday, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f144(std::formatter<std::chrono::weekday_indexed, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f145(const std::formatter<std::chrono::weekday_indexed, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f146(std::formatter<std::chrono::weekday_last, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f147(const std::formatter<std::chrono::weekday_last, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f148(std::formatter<std::chrono::year, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f149(const std::formatter<std::chrono::year, T1> &a0,
                           const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f150(std::formatter<std::chrono::year_month, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f151(const std::formatter<std::chrono::year_month, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f152(std::formatter<std::chrono::year_month_day, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f153(const std::formatter<std::chrono::year_month_day, T1> &a0, const T2 &a1,
     T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f154(std::formatter<std::chrono::year_month_day_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f155(const std::formatter<std::chrono::year_month_day_last, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f156(std::formatter<std::chrono::year_month_weekday, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f157(const std::formatter<std::chrono::year_month_weekday, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator
f158(std::formatter<std::chrono::year_month_weekday_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f159(const std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
     const T2 &a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f160(const std::formatter<const T1 *, T1> &a0,
                           const T1 *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f161(std::formatter<const T1 *, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f162(std::formatter<const void *, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f163(const std::formatter<const void *, T1> &a0,
                           const void *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f164(const std::formatter<T1 *, T1> &a0, T1 *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f165(std::formatter<void *, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f166(const std::formatter<void *, T1> &a0, const void *a1,
                           T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t52 = std::formatter<long double, T1>;

template <typename T1, typename T2>
typename T2::iterator f167(std::formatter<long double, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f168(const std::formatter<long double, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t53 = std::formatter<int, T1>;

template <typename T1, typename T2>
typename T2::iterator f169(std::formatter<int, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f170(const std::formatter<int, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t54 = std::formatter<long long, T1>;

template <typename T1, typename T2>
typename T2::iterator f171(std::formatter<long long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f172(const std::formatter<long long, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t55 = std::formatter<long, T1>;

template <typename T1, typename T2>
typename T2::iterator f173(std::formatter<long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f174(const std::formatter<long, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t56 = std::formatter<short, T1>;

template <typename T1, typename T2>
typename T2::iterator f175(std::formatter<short, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f176(const std::formatter<short, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t57 = std::formatter<bool, T1>;

template <typename T1, typename T2>
typename T2::iterator f177(std::formatter<bool, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f178(const std::formatter<bool, T1> &a0, bool a1,
                           T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t58 = std::formatter<unsigned char, T1>;

template <typename T1, typename T2>
typename T2::iterator f179(std::formatter<unsigned char, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f180(const std::formatter<unsigned char, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t59 = std::formatter<double, T1>;

template <typename T1, typename T2>
typename T2::iterator f181(std::formatter<double, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f182(const std::formatter<double, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t60 = std::formatter<float, T1>;

template <typename T1, typename T2>
typename T2::iterator f183(std::formatter<float, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f184(const std::formatter<float, T1> &a0, T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t61 = std::formatter<unsigned int, T1>;

template <typename T1, typename T2>
typename T2::iterator f185(std::formatter<unsigned int, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f186(const std::formatter<unsigned int, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t62 = std::formatter<__uint128_t, T1>;

template <typename T1, typename T2>
typename T2::iterator f187(std::formatter<__uint128_t, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f188(const std::formatter<__uint128_t, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t63 = std::formatter<unsigned long long, T1>;

template <typename T1, typename T2>
typename T2::iterator f189(std::formatter<unsigned long long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f190(const std::formatter<unsigned long long, T1> &a0,
                           T2 a1, T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t64 = std::formatter<unsigned long, T1>;

template <typename T1, typename T2>
typename T2::iterator f191(std::formatter<unsigned long, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f192(const std::formatter<unsigned long, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f193(std::formatter<std::nullptr_t, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f194(const std::formatter<std::nullptr_t, T1> &a0,
                           const void *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t65 = std::formatter<signed char, T1>;

template <typename T1, typename T2>
typename T2::iterator f195(std::formatter<signed char, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f196(const std::formatter<signed char, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1> using t66 = std::formatter<unsigned short, T1>;

template <typename T1, typename T2>
typename T2::iterator f197(std::formatter<unsigned short, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f198(const std::formatter<unsigned short, T1> &a0, T2 a1,
                           T3 &a2) {
  return a0.template format<T2, T3>(a1, a2);
}

template <typename T1, typename T2>
using t67 = std::formatter<std::chrono::hh_mm_ss<T1>, T2>;

template <typename T1, typename T2, typename T3>
typename T3::iterator f199(std::formatter<std::chrono::hh_mm_ss<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f200(const std::formatter<std::chrono::hh_mm_ss<T1>, T2> &a0, const T3 &a1,
     T4 &a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f201(std::formatter<std::chrono::local_time<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f202(const std::formatter<std::chrono::local_time<T1>, T2> &a0, const T3 &a1,
     T4 &a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f203(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f204(const std::formatter<std::chrono::sys_time<T1>, T2> &a0, const T3 &a1,
     T4 &a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2>
using t68 = std::formatter<std::basic_string_view<T1, T2>, T1>;

template <typename T1, typename T2, typename T3>
typename T3::iterator
f205(const std::formatter<std::basic_string_view<T1, T2>, T1> &a0,
     std::basic_string_view<T1, T2> a1, T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f206(std::formatter<std::basic_string_view<T1, T2>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f207(std::formatter<std::chrono::duration<T1, T2>, T3> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename T5::iterator
f208(const std::formatter<std::chrono::duration<T1, T2>, T3> &a0, const T4 &a1,
     T5 &a2) {
  return a0.template format<T4, T5>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t69 = std::formatter<std::basic_string<T1, T2, T3>, T1>;

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f209(const std::formatter<std::basic_string<T1, T2, T3>, T1> &a0,
     const std::basic_string<T1, T2, T3> &a1, T4 &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f210(std::formatter<std::basic_string<T1, T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}
