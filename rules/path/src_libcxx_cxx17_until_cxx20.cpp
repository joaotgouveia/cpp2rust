#include <cstddef>
#include <filesystem>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

std::string f72(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f73(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

template <
    typename T1,
    std::__enable_if_t<std::filesystem::__can_convert_char<T1>::value, int> T2 =
        0,
    typename std::enable_if<
        (!(std::is_same<T1, const std::filesystem::path &>::value) &&
         !(std::is_same<T1,
                        const std::filesystem::path::string_type &>::value) &&
         !(std::is_same<T1, std::filesystem::path::__string_view>::value) &&
         !(std::is_same<T1,
                        const std::filesystem::path::value_type *>::value) &&
         !(std::is_same<T1, std::filesystem::path::value_type>::value)),
        int>::type T3 = 0>
std::filesystem::path &f74(std::filesystem::path &a0, T1 a1) {
  return a0.template operator+= <T1>(a1);
}

bool f75(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>=(a0, a1);
}

bool f76(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>(a0, a1);
}

bool f77(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<=(a0, a1);
}

bool f78(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<(a0, a1);
}

bool f79(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator!=(a0, a1);
}
