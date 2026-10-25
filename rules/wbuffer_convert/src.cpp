#include <ios>
#include <iosfwd>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::wbuffer_convert<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::wbuffer_convert<T1, T2, T3>::state_type;

template <typename T1, typename T2, typename T3>
std::wbuffer_convert<T1, T2, T3> f1() {
  return {};
}

template <typename T1, typename T2, typename T3>
std::streambuf *f2(const std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
std::streambuf *f3(std::wbuffer_convert<T1, T2, T3> &a0, std::streambuf *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::state_type
f4(const std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.state();
}

template <typename T1, typename T2, typename T3>
std::locale f5(std::wbuffer_convert<T1, T2, T3> &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f6(const std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::pos_type
f7(std::wbuffer_convert<T1, T2, T3> &a0,
   typename std::wbuffer_convert<T1, T2, T3>::off_type a1,
   std::ios_base::seekdir a2, std::ios_base::openmode a3) {
  return a0.pubseekoff(
      std::forward<typename std::wbuffer_convert<T1, T2, T3>::off_type>(a1), a2,
      a3);
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::pos_type
f8(std::wbuffer_convert<T1, T2, T3> &a0,
   typename std::wbuffer_convert<T1, T2, T3>::pos_type a1,
   std::ios_base::openmode a2) {
  return a0.pubseekpos(
      std::forward<typename std::wbuffer_convert<T1, T2, T3>::pos_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3>
int f9(std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.pubsync();
}

template <typename T1, typename T2, typename T3>
std::streamsize f10(std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.in_avail();
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::int_type
f11(std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.snextc();
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::int_type
f12(std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.sbumpc();
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::int_type
f13(std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.sgetc();
}

template <typename T1, typename T2, typename T3>
std::streamsize f14(std::wbuffer_convert<T1, T2, T3> &a0, T2 *a1,
                    std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::int_type
f15(std::wbuffer_convert<T1, T2, T3> &a0, T2 a1) {
  return a0.sputbackc(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::int_type
f16(std::wbuffer_convert<T1, T2, T3> &a0) {
  return a0.sungetc();
}

template <typename T1, typename T2, typename T3>
typename std::wbuffer_convert<T1, T2, T3>::int_type
f17(std::wbuffer_convert<T1, T2, T3> &a0, T2 a1) {
  return a0.sputc(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::streamsize f18(std::wbuffer_convert<T1, T2, T3> &a0, T2 const *a1,
                    std::streamsize a2) {
  return a0.sputn(a1, a2);
}
