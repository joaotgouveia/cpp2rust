#include <istream>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f64(std::basic_istream<T1, T2> &a0,
                                std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f65(std::basic_istream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::iostate
f66(const std::basic_istream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f67(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f68(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::iostate
f69(const std::basic_istream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f70(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f71(const std::basic_istream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f72(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f73(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::fmtflags
f74(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::fmtflags a1,
    typename std::basic_istream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f75(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
using t8 = typename std::basic_istream<T1, T2>::sentry::traits_type;

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::sentry f76(std::basic_istream<T1, T2> &a0,
                                                bool a1) {
  return typename std::basic_istream<T1, T2>::sentry(a0, a1);
}
