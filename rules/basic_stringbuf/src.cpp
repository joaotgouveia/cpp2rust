#include <sstream>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t1 = std::basic_stringbuf<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_stringbuf<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_stringbuf<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_stringbuf<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_stringbuf<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_stringbuf<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_stringbuf<T1, T2, T3>::off_type;

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f1() {
  return std::basic_stringbuf<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f2(std::ios_base::openmode a0) {
  return std::basic_stringbuf<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f3(std::basic_stringbuf<T1, T2, T3> &&a0) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> &f4(std::basic_stringbuf<T1, T2, T3> &a0,
                                     std::basic_stringbuf<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f5(std::basic_stringbuf<T1, T2, T3> &a0,
        std::basic_stringbuf<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f6(std::basic_stringbuf<T1, T2, T3> &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f7(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::pos_type
f8(std::basic_stringbuf<T1, T2, T3> &a0,
   typename std::basic_stringbuf<T1, T2, T3>::off_type a1,
   std::ios_base::seekdir a2, std::ios_base::openmode a3) {
  return a0.pubseekoff(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::pos_type
f9(std::basic_stringbuf<T1, T2, T3> &a0,
   typename std::basic_stringbuf<T1, T2, T3>::pos_type a1,
   std::ios_base::openmode a2) {
  return a0.pubseekpos(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f10(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.pubsync();
}

template <typename T1, typename T2, typename T3>
std::streamsize f11(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.in_avail();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f12(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.snextc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f13(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.sbumpc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f14(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.sgetc();
}

template <typename T1, typename T2, typename T3>
std::streamsize f15(std::basic_stringbuf<T1, T2, T3> &a0,
                    typename std::basic_stringbuf<T1, T2, T3>::char_type *a1,
                    std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f16(std::basic_stringbuf<T1, T2, T3> &a0,
    typename std::basic_stringbuf<T1, T2, T3>::char_type a1) {
  return a0.sputbackc(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f17(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.sungetc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f18(std::basic_stringbuf<T1, T2, T3> &a0,
    typename std::basic_stringbuf<T1, T2, T3>::char_type a1) {
  return a0.sputc(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize
f19(std::basic_stringbuf<T1, T2, T3> &a0,
    const typename std::basic_stringbuf<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.sputn(a1, a2);
}
