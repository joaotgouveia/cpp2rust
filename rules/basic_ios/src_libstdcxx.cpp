#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f27(const std::basic_ios<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f28(std::basic_ios<T1, T2> &a0,
         typename std::basic_ios<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f29(std::basic_ios<T1, T2> &a0,
         typename std::basic_ios<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f30(const std::basic_ios<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f31(std::basic_ios<T1, T2> &a0,
         typename std::basic_ios<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f32(const std::basic_ios<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags f33(std::basic_ios<T1, T2> &a0,
                            typename std::basic_ios<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f34(std::basic_ios<T1, T2> &a0,
                            typename std::basic_ios<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f35(std::basic_ios<T1, T2> &a0,
                            typename std::basic_ios<T1, T2>::fmtflags a1,
                            typename std::basic_ios<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f36(std::basic_ios<T1, T2> &a0,
         typename std::basic_ios<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}
