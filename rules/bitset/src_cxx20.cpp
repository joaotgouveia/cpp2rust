#include <algorithm>
#include <barrier>
#include <bitset>
#include <ios>
#include <string>
#include <utility>

template <std::size_t T1, typename T2>
std::basic_string<T2, std::char_traits<T2>, std::allocator<T2>>
f36(const std::bitset<T1> &a0, T2 a1, T2 a2)
  requires(!(std::is_same<T2, char>::value))
{
  return a0.template to_string<T2>(std::move(a1), std::move(a2));
}
