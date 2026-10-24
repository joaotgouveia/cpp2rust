#include <bitset>
#include <cstddef>
#include <ios>
#include <memory>
#include <ostream>
#include <streambuf>
#include <string_view>
#include <system_error>
#include <utility>

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::sentry
f94(std::basic_ostream<T1, T2> &a0) {
  return typename std::basic_ostream<T1, T2>::sentry(a0);
}
