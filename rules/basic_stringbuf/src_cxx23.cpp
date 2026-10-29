#include <ios>
#include <iosfwd>
#include <sstream>
#include <utility>

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
std::basic_stringbuf<T1, T2, T3> f3(const std::basic_string<T1, T2, T3> &a0,
                                    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f4(std::basic_stringbuf<T1, T2, T3> &&a0) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f5(const T3 &a0) {
  return std::basic_stringbuf<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f6(std::ios_base::openmode a0, const T3 &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f7(std::basic_string<T1, T2, T3> &&a0,
                                    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f8(std::basic_stringbuf<T1, T2, T3> &&a0,
                                    const T3 &a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
T3 f9(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> &f10(std::basic_stringbuf<T1, T2, T3> &a0,
                                      std::basic_stringbuf<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f11(std::basic_stringbuf<T1, T2, T3> &a0,
         std::basic_stringbuf<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f12(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f13(std::basic_stringbuf<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f14(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3>
void f15(std::basic_stringbuf<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
void f16(std::basic_stringbuf<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f17(const std::basic_string<T1, T2, T4> &a0,
                                     const T3 &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f18(const std::basic_string<T1, T2, T4> &a0,
                                     std::ios_base::openmode a1, const T3 &a2) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f19(const std::basic_string<T1, T2, T4> &a0,
                                     std::ios_base::openmode a1)
  requires(!(std::is_same<T4, T3>::value))
{
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4> f20(const std::basic_stringbuf<T1, T2, T3> &a0,
                                  const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f21(std::basic_stringbuf<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1)
  requires(((!std::is_same_v<T4, T3>)) && (!(std::is_same<T4, T3>::value)))
{
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f22(std::basic_stringbuf<T1, T2, T3> &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f23(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::pos_type
f24(std::basic_stringbuf<T1, T2, T3> &a0,
    typename std::basic_stringbuf<T1, T2, T3>::off_type a1,
    std::ios_base::seekdir a2, std::ios_base::openmode a3) {
  return a0.pubseekoff(
      std::forward<typename std::basic_stringbuf<T1, T2, T3>::off_type>(a1), a2,
      a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::pos_type
f25(std::basic_stringbuf<T1, T2, T3> &a0,
    typename std::basic_stringbuf<T1, T2, T3>::pos_type a1,
    std::ios_base::openmode a2) {
  return a0.pubseekpos(
      std::forward<typename std::basic_stringbuf<T1, T2, T3>::pos_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3>
int f26(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.pubsync();
}

template <typename T1, typename T2, typename T3>
std::streamsize f27(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.in_avail();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f28(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.snextc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f29(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.sbumpc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f30(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.sgetc();
}

template <typename T1, typename T2, typename T3>
std::streamsize f31(std::basic_stringbuf<T1, T2, T3> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f32(std::basic_stringbuf<T1, T2, T3> &a0, T1 a1) {
  return a0.sputbackc(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f33(std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.sungetc();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::int_type
f34(std::basic_stringbuf<T1, T2, T3> &a0, T1 a1) {
  return a0.sputc(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::streamsize f35(std::basic_stringbuf<T1, T2, T3> &a0, T1 const *a1,
                    std::streamsize a2) {
  return a0.sputn(a1, a2);
}
