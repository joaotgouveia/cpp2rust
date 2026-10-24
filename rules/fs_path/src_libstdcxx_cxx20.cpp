#include <algorithm>
#include <barrier>
#include <filesystem>
#include <ios>
#include <iosfwd>
#include <istream>
#include <string_view>
#include <system_error>
#include <utility>

std::u8string f106(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f107(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f108(std::filesystem::path &a0,
                                           const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f109(std::filesystem::path &a0,
                                           const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value))
{
  return a0.template operator/= <T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f110(std::filesystem::path &a0,
                                           const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value) &&
           !(std::is_same<T1, std::filesystem::path::string_type>::value))
{
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path2<T1 *> &f111(std::filesystem::path &a0, T1 a1)
  requires(
      !(std::is_same<T1, const std::filesystem::path &>::value) &&
      !(std::is_same<T1, const std::filesystem::path::string_type &>::value) &&
      !(std::is_same<T1, const std::filesystem::path::value_type *>::value) &&
      !(std::is_same<T1, std::filesystem::path::value_type>::value) &&
      !(std::is_same<T1, std::basic_string_view<
                             std::filesystem::path::value_type>>::value))
{
  return a0.template operator+= <T1>(std::move(a1));
}

std::strong_ordering f112(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return operator<=>(a0, a1);
}
