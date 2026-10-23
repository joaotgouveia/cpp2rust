#include <ios>
#include <ostream>
#include <string>
#include <utility>

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::iostate
f52(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f53(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f54(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::iostate
f55(const std::basic_ostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f56(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f57(const std::basic_ostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f58(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f59(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f60(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::fmtflags a1,
    typename std::basic_ostream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f61(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}
