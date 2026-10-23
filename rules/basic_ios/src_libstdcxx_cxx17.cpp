#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_ios<T1, T2> f37(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_ios<T1, T2>(a0);
}
