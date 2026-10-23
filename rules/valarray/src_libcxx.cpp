#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

template <typename T1> void f113(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

using t3 = std::gslice;

std::gslice f114() { return std::gslice(); }

std::gslice f115(std::size_t a0, const std::valarray<std::size_t> &a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, a1, a2);
}

std::gslice f116(std::size_t a0, const std::valarray<std::size_t> &a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, a1, std::move(a2));
}

std::gslice f117(std::size_t a0, std::valarray<std::size_t> &&a1,
                 const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, std::move(a1), a2);
}

std::gslice f118(std::size_t a0, std::valarray<std::size_t> &&a1,
                 std::valarray<std::size_t> &&a2) {
  return std::gslice(a0, std::move(a1), std::move(a2));
}

std::size_t f119(const std::gslice &a0) { return a0.start(); }

std::valarray<std::size_t> f120(const std::gslice &a0) { return a0.size(); }

std::valarray<std::size_t> f121(const std::gslice &a0) { return a0.stride(); }

std::gslice f122(const std::gslice &a0) { return std::gslice(a0); }

std::gslice f123(std::gslice &&a0) { return std::gslice(std::move(a0)); }

std::gslice &f124(std::gslice &a0, const std::gslice &a1) {
  return a0.operator=(a1);
}

std::gslice &f125(std::gslice &a0, std::gslice &&a1) {
  return a0.operator=(std::move(a1));
}

using t4 = std::slice;

std::slice f126() { return std::slice(); }

std::slice f127(std::size_t a0, std::size_t a1, std::size_t a2) {
  return std::slice(a0, a1, a2);
}

std::size_t f128(const std::slice &a0) { return a0.start(); }

std::size_t f129(const std::slice &a0) { return a0.size(); }

std::size_t f130(const std::slice &a0) { return a0.stride(); }

std::slice f131(const std::slice &a0) { return std::slice(a0); }

std::slice f132(std::slice &&a0) { return std::slice(std::move(a0)); }

std::slice &f133(std::slice &a0, const std::slice &a1) {
  return a0.operator=(a1);
}

std::slice &f134(std::slice &a0, std::slice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::valarray<T1> f135(const typename std::valarray<T1>::value_type &a0,
                       std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f136(const typename std::valarray<T1>::value_type *a0,
                       std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::valarray<T1>
f137(std::initializer_list<typename std::valarray<T1>::value_type> a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1>
f138(const std::slice_array<typename std::valarray<T1>::value_type> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1>
f139(const std::gslice_array<typename std::valarray<T1>::value_type> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1>
f140(const std::mask_array<typename std::valarray<T1>::value_type> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1>
f141(const std::indirect_array<typename std::valarray<T1>::value_type> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1> &
f142(std::valarray<T1> &a0,
     std::initializer_list<typename std::valarray<T1>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f143(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &
f144(std::valarray<T1> &a0,
     const std::slice_array<typename std::valarray<T1>::value_type> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &
f145(std::valarray<T1> &a0,
     const std::gslice_array<typename std::valarray<T1>::value_type> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &
f146(std::valarray<T1> &a0,
     const std::mask_array<typename std::valarray<T1>::value_type> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &
f147(std::valarray<T1> &a0,
     const std::indirect_array<typename std::valarray<T1>::value_type> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
const typename std::valarray<T1>::value_type &f148(const std::valarray<T1> &a0,
                                                   std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::valarray<T1>::value_type &f149(std::valarray<T1> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::slice_array<typename std::valarray<T1>::value_type>
f150(std::valarray<T1> &a0, std::slice a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::gslice_array<typename std::valarray<T1>::value_type>
f151(std::valarray<T1> &a0, const std::gslice &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::gslice_array<typename std::valarray<T1>::value_type>
f152(std::valarray<T1> &a0, std::gslice &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::mask_array<typename std::valarray<T1>::value_type>
f153(std::valarray<T1> &a0, const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::mask_array<typename std::valarray<T1>::value_type>
f154(std::valarray<T1> &a0, std::valarray<bool> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::indirect_array<typename std::valarray<T1>::value_type>
f155(std::valarray<T1> &a0, const std::valarray<std::size_t> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::indirect_array<typename std::valarray<T1>::value_type>
f156(std::valarray<T1> &a0, std::valarray<std::size_t> &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::valarray<T1> &f157(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f158(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f159(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f160(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f161(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f162(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f163(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f164(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f165(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f166(std::valarray<T1> &a0,
                        const typename std::valarray<T1>::value_type &a1) {
  return a0.operator>>=(a1);
}

template <typename T1>
typename std::valarray<T1>::value_type f167(const std::valarray<T1> &a0) {
  return a0.sum();
}

template <typename T1>
typename std::valarray<T1>::value_type f168(const std::valarray<T1> &a0) {
  return a0.min();
}

template <typename T1>
typename std::valarray<T1>::value_type f169(const std::valarray<T1> &a0) {
  return a0.max();
}

template <typename T1>
std::valarray<T1> f170(const std::valarray<T1> &a0,
                       typename std::valarray<T1>::value_type(a1)(
                           typename std::valarray<T1>::value_type)) {
  return a0.apply(a1);
}

template <typename T1>
std::valarray<T1> f171(const std::valarray<T1> &a0,
                       typename std::valarray<T1>::value_type(a1)(
                           const typename std::valarray<T1>::value_type &)) {
  return a0.apply(a1);
}

template <typename T1>
void f172(std::valarray<T1> &a0, std::size_t a1,
          typename std::valarray<T1>::value_type a2) {
  return a0.resize(a1, a2);
}
