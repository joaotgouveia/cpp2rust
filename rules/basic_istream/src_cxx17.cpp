#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_istream<T1, T2> f62(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_istream<T1, T2>(a0);
}
