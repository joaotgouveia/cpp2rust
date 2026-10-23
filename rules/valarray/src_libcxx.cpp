#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f122(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f123(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator!= <T1>(a0, a1)) {
  return std::operator!= <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f124(const T1 &a0, const T2 &a1)
    -> decltype(std::operator!= <T1, T2>(a0, a1)) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f125(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f126(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator== <T1>(a0, a1)) {
  return std::operator== <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f127(const T1 &a0, const T2 &a1)
    -> decltype(std::operator== <T1, T2>(a0, a1)) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1> void f128(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f129(const T1 &a0) -> decltype(std::abs<T1>(a0)) {
  return std::abs<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f130(const T1 &a0) -> decltype(std::acos<T1>(a0)) {
  return std::acos<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f131(const T1 &a0) -> decltype(std::asin<T1>(a0)) {
  return std::asin<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f132(const T1 &a0) -> decltype(std::atan<T1>(a0)) {
  return std::atan<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f133(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f134(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f135(const T1 &a0, const T2 &a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f136(const T1 &a0) -> decltype(std::cos<T1>(a0)) {
  return std::cos<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f137(const T1 &a0) -> decltype(std::cosh<T1>(a0)) {
  return std::cosh<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f138(const T1 &a0) -> decltype(std::exp<T1>(a0)) {
  return std::exp<T1>(a0);
}

using t3 = std::gslice;

std::gslice f139() { return std::gslice(); }

std::gslice f140(std::size_t a0, const std::valarray<std::size_t> &a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, a1, a2);
}

std::gslice f141(std::size_t a0, const std::valarray<std::size_t> &a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, a1, std::move(a2));
}

std::gslice f142(std::size_t a0, std::valarray<std::size_t> &&a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, std::move(a1), a2);
}

std::gslice f143(std::size_t a0, std::valarray<std::size_t> &&a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, std::move(a1), std::move(a2));
}

std::size_t f144(const std::gslice &a0) { return a0.start(); }

std::valarray<std::size_t> f145(const std::gslice &a0) { return a0.size(); }

std::valarray<std::size_t> f146(const std::gslice &a0) { return a0.stride(); }

std::gslice f147(const std::gslice &a0) { return std::gslice(a0); }

std::gslice f148(std::gslice &&a0) { return std::gslice(std::move(a0)); }

std::gslice &f149(std::gslice &a0, const std::gslice &a1) {
  return a0.operator=(a1);
}

std::gslice &f150(std::gslice &a0, std::gslice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f151(const T1 &a0) -> decltype(std::log<T1>(a0)) {
  return std::log<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f152(const T1 &a0) -> decltype(std::log10<T1>(a0)) {
  return std::log10<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f153(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f154(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator% <T1>(a0, a1)) {
  return std::operator% <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f155(const T1 &a0, const T2 &a1)
    -> decltype(std::operator% <T1, T2>(a0, a1)) {
  return std::operator% <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f156(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f157(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator& <T1>(a0, a1)) {
  return std::operator& <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f158(const T1 &a0, const T2 &a1)
    -> decltype(std::operator& <T1, T2>(a0, a1)) {
  return std::operator& <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f159(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f160(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator&& <T1>(a0, a1)) {
  return std::operator&& <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f161(const T1 &a0, const T2 &a1)
    -> decltype(std::operator&& <T1, T2>(a0, a1)) {
  return std::operator&& <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f162(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f163(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator* <T1>(a0, a1)) {
  return std::operator* <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f164(const T1 &a0, const T2 &a1)
    -> decltype(std::operator* <T1, T2>(a0, a1)) {
  return std::operator* <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f165(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f166(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator+ <T1>(a0, a1)) {
  return std::operator+ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f167(const T1 &a0, const T2 &a1)
    -> decltype(std::operator+ <T1, T2>(a0, a1)) {
  return std::operator+ <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f168(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f169(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator- <T1>(a0, a1)) {
  return std::operator- <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f170(const T1 &a0, const T2 &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f171(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f172(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator/ <T1>(a0, a1)) {
  return std::operator/ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f173(const T1 &a0, const T2 &a1)
    -> decltype(std::operator/ <T1, T2>(a0, a1)) {
  return std::operator/ <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f174(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f175(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator< <T1>(a0, a1)) {
  return std::operator< <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f176(const T1 &a0, const T2 &a1)
    -> decltype(std::operator< <T1, T2>(a0, a1)) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f177(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f178(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator<< <T1>(a0, a1)) {
  return std::operator<< <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f179(const T1 &a0, const T2 &a1)
    -> decltype(std::operator<< <T1, T2>(a0, a1)) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f180(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f181(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator<= <T1>(a0, a1)) {
  return std::operator<= <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f182(const T1 &a0, const T2 &a1)
    -> decltype(std::operator<= <T1, T2>(a0, a1)) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f183(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f184(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator><T1>(a0, a1)) {
  return std::operator><T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f185(const T1 &a0, const T2 &a1)
    -> decltype(std::operator><T1, T2>(a0, a1)) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f186(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f187(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator>= <T1>(a0, a1)) {
  return std::operator>= <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f188(const T1 &a0, const T2 &a1)
    -> decltype(std::operator>= <T1, T2>(a0, a1)) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f189(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f190(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator>> <T1>(a0, a1)) {
  return std::operator>> <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f191(const T1 &a0, const T2 &a1)
    -> decltype(std::operator>> <T1, T2>(a0, a1)) {
  return std::operator>> <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f192(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f193(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator^ <T1>(a0, a1)) {
  return std::operator^ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f194(const T1 &a0, const T2 &a1)
    -> decltype(std::operator^ <T1, T2>(a0, a1)) {
  return std::operator^ <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f195(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f196(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator| <T1>(a0, a1)) {
  return std::operator| <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f197(const T1 &a0, const T2 &a1)
    -> decltype(std::operator| <T1, T2>(a0, a1)) {
  return std::operator| <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f198(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f199(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::operator|| <T1>(a0, a1)) {
  return std::operator|| <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f200(const T1 &a0, const T2 &a1)
    -> decltype(std::operator|| <T1, T2>(a0, a1)) {
  return std::operator|| <T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f201(const typename T1::value_type &a0, const T1 &a1)
    -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f202(const T1 &a0, const typename T1::value_type &a1)
    -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__is_val_expr<T1>::value && std::__is_val_expr<T2>::value, int>
        T3 = 0>
auto f203(const T1 &a0, const T2 &a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f204(const T1 &a0) -> decltype(std::sin<T1>(a0)) {
  return std::sin<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f205(const T1 &a0) -> decltype(std::sinh<T1>(a0)) {
  return std::sinh<T1>(a0);
}

using t4 = std::slice;

std::slice f206() { return std::slice(); }

std::slice f207(std::size_t a0, std::size_t a1, std::size_t a2) {
  return std::slice(a0, a1, a2);
}

std::size_t f208(const std::slice &a0) { return a0.start(); }

std::size_t f209(const std::slice &a0) { return a0.size(); }

std::size_t f210(const std::slice &a0) { return a0.stride(); }

std::slice f211(const std::slice &a0) { return std::slice(a0); }

std::slice f212(std::slice &&a0) { return std::slice(std::move(a0)); }

std::slice &f213(std::slice &a0, const std::slice &a1) {
  return a0.operator=(a1);
}

std::slice &f214(std::slice &a0, std::slice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f215(const T1 &a0) -> decltype(std::sqrt<T1>(a0)) {
  return std::sqrt<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f216(const T1 &a0) -> decltype(std::tan<T1>(a0)) {
  return std::tan<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__is_val_expr<T1>::value, int> T2 = 0>
auto f217(const T1 &a0) -> decltype(std::tanh<T1>(a0)) {
  return std::tanh<T1>(a0);
}

template <typename T1> std::valarray<T1> f218(const T1 *a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
auto f219(const std::valarray<T1> &a0, std::gslice &&a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::gslice_array<T1> f220(std::valarray<T1> &a0, std::gslice &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
auto f221(const std::valarray<T1> &a0, const std::valarray<bool> &a1)
    -> decltype(a0.operator[](a1)) {
  return a0.operator[](a1);
}

template <typename T1>
auto f222(const std::valarray<T1> &a0, std::valarray<bool> &&a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::mask_array<T1> f223(std::valarray<T1> &a0, std::valarray<bool> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
auto f224(const std::valarray<T1> &a0, std::valarray<std::size_t> &&a1)
    -> decltype(a0.operator[](std::move(a1))) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::indirect_array<T1> f225(std::valarray<T1> &a0,
                             std::valarray<std::size_t> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::valarray<T1> f226(const std::valarray<T1> &a0, T1(a1)(T1)) {
  return a0.apply(a1);
}

template <typename T1>
std::valarray<T1> f227(const std::valarray<T1> &a0, T1(a1)(const T1 &)) {
  return a0.apply(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f228(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f229(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f230(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f231(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f232(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f233(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f234(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f235(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f236(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f237(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
