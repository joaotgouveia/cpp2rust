#include <ios>
#include <iosfwd>
#include <span>
#include <spanstream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::basic_spanbuf<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_spanbuf<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_spanbuf<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_spanbuf<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_spanbuf<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_spanbuf<T1, T2>::traits_type;

template <typename T1, typename T2> std::basic_spanbuf<T1, T2> f1() {
  return std::basic_spanbuf<T1, T2>();
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> f2(std::ios_base::openmode a0) {
  return std::basic_spanbuf<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> f3(std::span<T1> a0, std::ios_base::openmode a1) {
  return std::basic_spanbuf<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> f4(std::basic_spanbuf<T1, T2> &&a0) {
  return std::basic_spanbuf<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> &f5(std::basic_spanbuf<T1, T2> &a0,
                               std::basic_spanbuf<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f6(std::basic_spanbuf<T1, T2> &a0, std::basic_spanbuf<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::span<T1> f7(const std::basic_spanbuf<T1, T2> &a0) {
  return a0.span();
}

template <typename T1, typename T2>
void f8(std::basic_spanbuf<T1, T2> &a0, std::span<T1> a1) {
  return a0.span(a1);
}

template <typename T1, typename T2>
std::locale f9(std::basic_spanbuf<T1, T2> &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

template <typename T1, typename T2>
std::locale f10(const std::basic_spanbuf<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::pos_type
f11(std::basic_spanbuf<T1, T2> &a0,
    typename std::basic_spanbuf<T1, T2>::off_type a1, std::ios_base::seekdir a2,
    std::ios_base::openmode a3) {
  return a0.pubseekoff(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::pos_type
f12(std::basic_spanbuf<T1, T2> &a0,
    typename std::basic_spanbuf<T1, T2>::pos_type a1,
    std::ios_base::openmode a2) {
  return a0.pubseekpos(a1, a2);
}

template <typename T1, typename T2> int f13(std::basic_spanbuf<T1, T2> &a0) {
  return a0.pubsync();
}

template <typename T1, typename T2>
std::streamsize f14(std::basic_spanbuf<T1, T2> &a0) {
  return a0.in_avail();
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::int_type
f15(std::basic_spanbuf<T1, T2> &a0) {
  return a0.snextc();
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::int_type
f16(std::basic_spanbuf<T1, T2> &a0) {
  return a0.sbumpc();
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::int_type
f17(std::basic_spanbuf<T1, T2> &a0) {
  return a0.sgetc();
}

template <typename T1, typename T2>
std::streamsize f18(std::basic_spanbuf<T1, T2> &a0,
                    typename std::basic_spanbuf<T1, T2>::char_type *a1,
                    std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::int_type
f19(std::basic_spanbuf<T1, T2> &a0,
    typename std::basic_spanbuf<T1, T2>::char_type a1) {
  return a0.sputbackc(a1);
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::int_type
f20(std::basic_spanbuf<T1, T2> &a0) {
  return a0.sungetc();
}

template <typename T1, typename T2>
typename std::basic_spanbuf<T1, T2>::int_type
f21(std::basic_spanbuf<T1, T2> &a0,
    typename std::basic_spanbuf<T1, T2>::char_type a1) {
  return a0.sputc(a1);
}

template <typename T1, typename T2>
std::streamsize f22(std::basic_spanbuf<T1, T2> &a0,
                    const typename std::basic_spanbuf<T1, T2>::char_type *a1,
                    std::streamsize a2) {
  return a0.sputn(a1, a2);
}
