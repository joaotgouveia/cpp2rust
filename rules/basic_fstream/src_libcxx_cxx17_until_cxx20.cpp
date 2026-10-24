#include <filesystem>
#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::enable_if_t<std::is_same_v<T3, std::filesystem::path>>,
    typename std::enable_if<(!(std::is_same<T3, std::string>::value)),
                            int>::type T5 = 0>
std::basic_fstream<T1, T2> f128(const T3 &a0, unsigned int a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}
