#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::sentry f88(std::basic_istream<T1, T2> &a0,
                                                bool a1) {
  return typename std::basic_istream<T1, T2>::sentry(a0, a1);
}
