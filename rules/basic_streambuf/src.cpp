#include <ios>
#include <streambuf>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_streambuf<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_streambuf<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_streambuf<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_streambuf<T1, T2>::int_type;

template <typename T1, typename T2>
using t5 = typename std::basic_streambuf<T1, T2>::pos_type;

template <typename T1, typename T2>
using t6 = typename std::basic_streambuf<T1, T2>::off_type;

template <typename T1, typename T2>
std::locale f1(std::basic_streambuf<T1, T2> &a0, const std::locale &a1) {
  return a0.pubimbue(a1);
}

template <typename T1, typename T2>
std::locale f2(const std::basic_streambuf<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f3(std::basic_streambuf<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.pubsetbuf(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::pos_type
f4(std::basic_streambuf<T1, T2> &a0,
   typename std::basic_streambuf<T1, T2>::off_type a1,
   std::ios_base::seekdir a2, std::ios_base::openmode a3) {
  return a0.pubseekoff(std::move(a1), a2, a3);
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::pos_type
f5(std::basic_streambuf<T1, T2> &a0,
   typename std::basic_streambuf<T1, T2>::pos_type a1,
   std::ios_base::openmode a2) {
  return a0.pubseekpos(std::move(a1), a2);
}

template <typename T1, typename T2> int f6(std::basic_streambuf<T1, T2> &a0) {
  return a0.pubsync();
}

template <typename T1, typename T2>
std::streamsize f7(std::basic_streambuf<T1, T2> &a0) {
  return a0.in_avail();
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::int_type
f8(std::basic_streambuf<T1, T2> &a0) {
  return a0.snextc();
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::int_type
f9(std::basic_streambuf<T1, T2> &a0) {
  return a0.sbumpc();
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::int_type
f10(std::basic_streambuf<T1, T2> &a0) {
  return a0.sgetc();
}

template <typename T1, typename T2>
std::streamsize f11(std::basic_streambuf<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.sgetn(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::int_type
f12(std::basic_streambuf<T1, T2> &a0, T1 a1) {
  return a0.sputbackc(std::move(a1));
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::int_type
f13(std::basic_streambuf<T1, T2> &a0) {
  return a0.sungetc();
}

template <typename T1, typename T2>
typename std::basic_streambuf<T1, T2>::int_type
f14(std::basic_streambuf<T1, T2> &a0, T1 a1) {
  return a0.sputc(std::move(a1));
}

template <typename T1, typename T2>
std::streamsize f15(std::basic_streambuf<T1, T2> &a0, const T1 *a1,
                    std::streamsize a2) {
  return a0.sputn(a1, a2);
}
