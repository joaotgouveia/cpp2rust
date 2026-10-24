#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f134(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f135(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f136(const T1 &a0, const T2 &a1)
    -> decltype(std::operator!= <T1, T2>(a0, a1)) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f137(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f138(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f139(const T1 &a0, const T2 &a1)
    -> decltype(std::operator== <T1, T2>(a0, a1)) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1> void f140(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1> using t3 = std::__val_expr<T1>;

template <typename T1> using t4 = typename std::__val_expr<T1>::value_type;

template <typename T1>
std::__val_expr<T1> f141(const typename std::remove_reference<T1>::type &a0) {
  return std::__val_expr<T1>(a0);
}

template <typename T1>
auto f142(const std::__val_expr<T1> &a0, std::slice a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
auto f143(const std::__val_expr<T1> &a0, const std::gslice &a1)
    -> decltype(a0.operator[](a1)) {
  return a0.operator[](a1);
}

template <typename T1>
auto f144(const std::__val_expr<T1> &a0, const std::valarray<bool> &a1)
    -> decltype(a0.operator[](a1)) {
  return a0.operator[](a1);
}

template <typename T1>
auto f145(const std::__val_expr<T1> &a0, const std::valarray<std::size_t> &a1)
    -> decltype(a0.operator[](a1)) {
  return a0.operator[](a1);
}

template <typename T1>
auto f146(const std::__val_expr<T1> &a0) -> decltype(a0.operator+()) {
  return a0.operator+();
}

template <typename T1>
auto f147(const std::__val_expr<T1> &a0) -> decltype(a0.operator-()) {
  return a0.operator-();
}

template <typename T1>
auto f148(const std::__val_expr<T1> &a0) -> decltype(a0.operator~()) {
  return a0.operator~();
}

template <typename T1>
auto f149(const std::__val_expr<T1> &a0) -> decltype(a0.operator!()) {
  return a0.operator!();
}

template <typename T1> std::size_t f150(const std::__val_expr<T1> &a0) {
  return a0.size();
}

template <typename T1>
auto f151(const std::__val_expr<T1> &a0, int a1) -> decltype(a0.shift(a1)) {
  return a0.shift(a1);
}

template <typename T1>
auto f152(const std::__val_expr<T1> &a0, int a1) -> decltype(a0.cshift(a1)) {
  return a0.cshift(a1);
}

template <typename T1>
auto f153(const std::__val_expr<T1> &a0,
          typename std::__val_expr<T1>::value_type(a1)(
              typename std::__val_expr<T1>::value_type))
    -> decltype(a0.apply(a1)) {
  return a0.apply(a1);
}

template <typename T1>
auto f154(const std::__val_expr<T1> &a0,
          typename std::__val_expr<T1>::value_type(a1)(
              const typename std::__val_expr<T1>::value_type &))
    -> decltype(a0.apply(a1)) {
  return a0.apply(a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f155(const T1 &a0) -> decltype(std::abs<T1>(a0)) {
  return std::abs<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f156(const T1 &a0) -> decltype(std::acos<T1>(a0)) {
  return std::acos<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f157(const T1 &a0) -> decltype(std::asin<T1>(a0)) {
  return std::asin<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f158(const T1 &a0) -> decltype(std::atan<T1>(a0)) {
  return std::atan<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f159(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f160(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f161(const T1 &a0, const T2 &a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f162(const T1 &a0) -> decltype(std::cos<T1>(a0)) {
  return std::cos<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f163(const T1 &a0) -> decltype(std::cosh<T1>(a0)) {
  return std::cosh<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f164(const T1 &a0) -> decltype(std::exp<T1>(a0)) {
  return std::exp<T1>(a0);
}

using t5 = std::gslice;

std::gslice f165() { return std::gslice(); }

std::gslice f166(std::size_t a0, const std::valarray<std::size_t> &a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, a1, a2);
}

std::gslice f167(std::size_t a0, const std::valarray<std::size_t> &a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, a1, std::move(a2));
}

std::gslice f168(std::size_t a0, std::valarray<std::size_t> &&a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, std::move(a1), a2);
}

std::gslice f169(std::size_t a0, std::valarray<std::size_t> &&a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, std::move(a1), std::move(a2));
}

std::size_t f170(const std::gslice &a0) { return a0.start(); }

std::valarray<std::size_t> f171(const std::gslice &a0) { return a0.size(); }

std::valarray<std::size_t> f172(const std::gslice &a0) { return a0.stride(); }

std::gslice f173(const std::gslice &a0) { return std::gslice(a0); }

std::gslice f174(std::gslice &&a0) { return std::gslice(std::move(a0)); }

std::gslice &f175(std::gslice &a0, const std::gslice &a1) {
  return a0.operator=(a1);
}

std::gslice &f176(std::gslice &a0, std::gslice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f177(const T1 &a0) -> decltype(std::log<T1>(a0)) {
  return std::log<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f178(const T1 &a0) -> decltype(std::log10<T1>(a0)) {
  return std::log10<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f179(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f180(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f181(const T1 &a0, const T2 &a1)
    -> decltype(std::operator% <T1, T2>(a0, a1)) {
  return std::operator% <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f182(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f183(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f184(const T1 &a0, const T2 &a1)
    -> decltype(std::operator& <T1, T2>(a0, a1)) {
  return std::operator& <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f185(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f186(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f187(const T1 &a0, const T2 &a1)
    -> decltype(std::operator&& <T1, T2>(a0, a1)) {
  return std::operator&& <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f188(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f189(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f190(const T1 &a0, const T2 &a1)
    -> decltype(std::operator* <T1, T2>(a0, a1)) {
  return std::operator* <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f191(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f192(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f193(const T1 &a0, const T2 &a1)
    -> decltype(std::operator+ <T1, T2>(a0, a1)) {
  return std::operator+ <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f194(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f195(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f196(const T1 &a0, const T2 &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f197(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f198(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f199(const T1 &a0, const T2 &a1)
    -> decltype(std::operator/ <T1, T2>(a0, a1)) {
  return std::operator/ <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f200(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f201(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f202(const T1 &a0, const T2 &a1)
    -> decltype(std::operator< <T1, T2>(a0, a1)) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f203(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f204(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f205(const T1 &a0, const T2 &a1)
    -> decltype(std::operator<< <T1, T2>(a0, a1)) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f206(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f207(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f208(const T1 &a0, const T2 &a1)
    -> decltype(std::operator<= <T1, T2>(a0, a1)) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f209(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f210(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f211(const T1 &a0, const T2 &a1)
    -> decltype(std::operator><T1, T2>(a0, a1)) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f212(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f213(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f214(const T1 &a0, const T2 &a1)
    -> decltype(std::operator>= <T1, T2>(a0, a1)) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f215(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f216(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f217(const T1 &a0, const T2 &a1)
    -> decltype(std::operator>> <T1, T2>(a0, a1)) {
  return std::operator>> <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f218(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f219(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f220(const T1 &a0, const T2 &a1)
    -> decltype(std::operator^ <T1, T2>(a0, a1)) {
  return std::operator^ <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f221(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f222(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f223(const T1 &a0, const T2 &a1)
    -> decltype(std::operator| <T1, T2>(a0, a1)) {
  return std::operator| <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f224(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f225(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f226(const T1 &a0, const T2 &a1)
    -> decltype(std::operator|| <T1, T2>(a0, a1)) {
  return std::operator|| <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f227(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f228(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f229(const T1 &a0, const T2 &a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f230(const T1 &a0) -> decltype(std::sin<T1>(a0)) {
  return std::sin<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f231(const T1 &a0) -> decltype(std::sinh<T1>(a0)) {
  return std::sinh<T1>(a0);
}

using t6 = std::slice;

std::slice f232() { return std::slice(); }

std::slice f233(std::size_t a0, std::size_t a1, std::size_t a2) {
  return std::slice(a0, a1, a2);
}

std::size_t f234(const std::slice &a0) { return a0.start(); }

std::size_t f235(const std::slice &a0) { return a0.size(); }

std::size_t f236(const std::slice &a0) { return a0.stride(); }

std::slice f237(const std::slice &a0) { return std::slice(a0); }

std::slice f238(std::slice &&a0) { return std::slice(std::move(a0)); }

std::slice &f239(std::slice &a0, const std::slice &a1) {
  return a0.operator=(a1);
}

std::slice &f240(std::slice &a0, std::slice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f241(const T1 &a0) -> decltype(std::sqrt<T1>(a0)) {
  return std::sqrt<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f242(const T1 &a0) -> decltype(std::tan<T1>(a0)) {
  return std::tan<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f243(const T1 &a0) -> decltype(std::tanh<T1>(a0)) {
  return std::tanh<T1>(a0);
}

template <typename T1> std::valarray<T1> f244(const T1 *a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
auto f245(const std::valarray<T1> &a0, std::gslice &&a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::gslice_array<T1> f246(std::valarray<T1> &a0, std::gslice &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
auto f247(const std::valarray<T1> &a0, const std::valarray<bool> &a1)
    -> decltype(a0.operator[](a1)) {
  return a0.operator[](a1);
}

template <typename T1>
auto f248(const std::valarray<T1> &a0, std::valarray<bool> &&a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::mask_array<T1> f249(std::valarray<T1> &a0, std::valarray<bool> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
auto f250(const std::valarray<T1> &a0, std::valarray<std::size_t> &&a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::indirect_array<T1> f251(std::valarray<T1> &a0,
                             std::valarray<std::size_t> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::valarray<T1> f252(const std::valarray<T1> &a0, T1(a1)(T1)) {
  return a0.apply(a1);
}

template <typename T1>
std::valarray<T1> f253(const std::valarray<T1> &a0, T1(a1)(const T1 &)) {
  return a0.apply(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f254(std::valarray<T1> &a0, const std::__val_expr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
