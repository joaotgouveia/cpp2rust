#include <algorithm>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::_If_fs_path<T3>>
std::basic_ofstream<T1, T2>
f71(const T3 &a0, typename std::basic_ofstream<T1, T2>::openmode a1)
  requires(!(std::is_same<T3, std::string>::value))
{
  return std::basic_ofstream<T1, T2>(a0, a1);
}
