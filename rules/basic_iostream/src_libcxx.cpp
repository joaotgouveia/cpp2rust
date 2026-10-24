#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
std::basic_iostream<T1, T2> f100(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_iostream<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T1,
          std::__enable_if_t<std::is_same<T1, char>::value, int> T4 = 0>
std::basic_iostream<T1, T2> &f101(std::basic_iostream<T1, T2> &a0,
                                  std::streamsize a1, T1 a2) {
  return a0.ignore(a1, a2);
}

template <typename T1, typename T2>
unsigned int f102(const std::basic_iostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f103(std::basic_iostream<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f104(std::basic_iostream<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f105(const std::basic_iostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f106(std::basic_iostream<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
unsigned int f107(const std::basic_iostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f108(std::basic_iostream<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f109(std::basic_iostream<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f110(std::basic_iostream<T1, T2> &a0, unsigned int a1,
                  unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f111(std::basic_iostream<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
