#include <algorithm>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::_If_fs_path<T3>>
std::basic_fstream<T1, T2>
f110(const T3 &a0, typename std::basic_fstream<T1, T2>::openmode a1)
  requires(!(std::is_same<T3, std::string>::value))
{
  return std::basic_fstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::_If_fs_path<T3, void>
f111(std::basic_fstream<T1, T2> &a0, const T3 &a1,
     typename std::basic_fstream<T1, T2>::openmode a2)
  requires(!(std::is_same<T3, std::string>::value))
{
  return a0.template open<T3>(a1, a2);
}
