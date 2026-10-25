#include <algorithm>
#include <atomic>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <filesystem>
#include <format>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t49 = std::hash<std::u8string_view>;

std::size_t f202(const std::hash<std::u8string_view> &a0,
                 const std::u8string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u8string_view> f203() { return std::hash<std::u8string_view>(); }

std::hash<std::u8string_view> f204(const std::hash<std::u8string_view> &a0) {
  return std::hash<std::u8string_view>(a0);
}

std::hash<std::u8string_view> f205(std::hash<std::u8string_view> &&a0) {
  return std::hash<std::u8string_view>(std::move(a0));
}

std::hash<std::u8string_view> &f206(std::hash<std::u8string_view> &a0,
                                    const std::hash<std::u8string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u8string_view> &f207(std::hash<std::u8string_view> &a0,
                                    std::hash<std::u8string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t50 = std::hash<char8_t>;

std::size_t f208(const std::hash<char8_t> &a0, char8_t a1) {
  return a0.operator()(a1);
}

std::hash<char8_t> f209() { return std::hash<char8_t>(); }

std::hash<char8_t> f210(const std::hash<char8_t> &a0) {
  return std::hash<char8_t>(a0);
}

std::hash<char8_t> f211(std::hash<char8_t> &&a0) {
  return std::hash<char8_t>(std::move(a0));
}

std::hash<char8_t> &f212(std::hash<char8_t> &a0, const std::hash<char8_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char8_t> &f213(std::hash<char8_t> &a0, std::hash<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::size_t f214(const std::hash<std::optional<T1>> &a0,
                 const std::optional<T1> &a1)
  requires std::is_base_of<std::__optional_hash<T1>,
                           std::hash<std::optional<T1>>>::value
{
  return a0.operator()(a1);
}

template <typename... T1>
std::size_t f215(const std::hash<std::variant<T1...>> &a0,
                 const std::variant<T1...> &a1)
  requires std::is_base_of<std::__variant_hash<T1...>,
                           std::hash<std::variant<T1...>>>::value
{
  return a0.operator()(a1);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<T1>::type
f216(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.operator*();
}

template <typename T1, typename T2>
T1 *f217(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.operator->();
}

template <typename T1, typename T2>
T1 *f218(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.get();
}

template <typename T1, typename T2>
T2 &f219(std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.get_deleter();
}

template <typename T1, typename T2>
T2 const &f220(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f221(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
T1 *f222(std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.release();
}

template <typename T1, typename T2>
void f223(std::hash<std::unique_ptr<T1, T2>> &a0, T1 *a1)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.reset(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    std::__and_<std::__and_<std::is_convertible<
                                typename std::unique_ptr<T3, T4>::pointer,
                                typename std::__uniq_ptr_impl<T1, T2>::pointer>,
                            std::__not_<std::is_array<T3>>>,
                std::is_assignable<T2 &, T4 &&>>::value,
    std::unique_ptr<T1, T2> &>::type
f224(std::hash<std::unique_ptr<T1, T2>> &a0, std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value)) &&
          std::is_base_of<std::unique_ptr<T1, T2>,
                          std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1>
std::size_t f225(const std::hash<T1> &a0, T1 a1)
  requires std::is_base_of<std::__hash_enum<T1>, std::hash<T1>>::value
{
  return a0.operator()(std::move(a1));
}
