#include <istream>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f63(std::basic_istream<T1, T2> &a0,
                                std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f64(std::basic_istream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::iostate
f65(const std::basic_istream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f66(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f67(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::iostate
f68(const std::basic_istream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f69(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f70(const std::basic_istream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f71(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f72(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f73(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::fmtflags a1,
    typename std::basic_istream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f74(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}
