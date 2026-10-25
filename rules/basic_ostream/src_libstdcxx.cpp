#include <ios>
#include <ostream>
#include <string>
#include <utility>

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::iostate
f53(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f54(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_ostream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f55(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_ostream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::iostate
f56(const std::basic_ostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f57(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_ostream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f58(const std::basic_ostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags f59(std::basic_ostream<T1, T2> &a0,
                            typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_ostream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f60(std::basic_ostream<T1, T2> &a0,
                            typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_ostream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f61(std::basic_ostream<T1, T2> &a0,
                            typename std::basic_ostream<T1, T2>::fmtflags a1,
                            typename std::basic_ostream<T1, T2>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_ostream<T1, T2>::fmtflags>(a1),
      std::forward<typename std::basic_ostream<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f62(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_ostream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::sentry
f63(std::basic_ostream<T1, T2> &a0) {
  return typename std::basic_ostream<T1, T2>::sentry(a0);
}
