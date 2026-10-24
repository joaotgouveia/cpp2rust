#include <filesystem>
#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::enable_if_t<std::is_same_v<T3, std::filesystem::path>>>
std::basic_fstream<T1, T2> f130(const T3 &a0, unsigned int a1)
  requires(!(std::is_same<T3, std::string>::value))
{
  return std::basic_fstream<T1, T2>(a0, a1);
}
