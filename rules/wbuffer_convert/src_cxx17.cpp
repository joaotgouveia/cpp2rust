#include <ios>
#include <iosfwd>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::wbuffer_convert<T1, T2, T3>
f19(std::streambuf *a0, T1 *a1,
    typename std::wbuffer_convert<T1, T2, T3>::state_type a2) {
  return std::wbuffer_convert<T1, T2, T3>(a0, a1, std::move(a2));
}
