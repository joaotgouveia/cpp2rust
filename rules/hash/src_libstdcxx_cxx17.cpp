#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <filesystem>
#include <future>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t41 = std::hash<std::string_view>;

std::size_t f164(const std::hash<std::string_view> &a0,
                 const std::string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::string_view> f165() { return std::hash<std::string_view>(); }

std::hash<std::string_view> f166(const std::hash<std::string_view> &a0) {
  return std::hash<std::string_view>(a0);
}

std::hash<std::string_view> f167(std::hash<std::string_view> &&a0) {
  return std::hash<std::string_view>(std::move(a0));
}

std::hash<std::string_view> &f168(std::hash<std::string_view> &a0,
                                  const std::hash<std::string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::string_view> &f169(std::hash<std::string_view> &a0,
                                  std::hash<std::string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t42 = std::hash<std::wstring_view>;

std::size_t f170(const std::hash<std::wstring_view> &a0,
                 const std::wstring_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::wstring_view> f171() { return std::hash<std::wstring_view>(); }

std::hash<std::wstring_view> f172(const std::hash<std::wstring_view> &a0) {
  return std::hash<std::wstring_view>(a0);
}

std::hash<std::wstring_view> f173(std::hash<std::wstring_view> &&a0) {
  return std::hash<std::wstring_view>(std::move(a0));
}

std::hash<std::wstring_view> &f174(std::hash<std::wstring_view> &a0,
                                   const std::hash<std::wstring_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::wstring_view> &f175(std::hash<std::wstring_view> &a0,
                                   std::hash<std::wstring_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t43 = std::hash<std::u16string_view>;

std::size_t f176(const std::hash<std::u16string_view> &a0,
                 const std::u16string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u16string_view> f177() {
  return std::hash<std::u16string_view>();
}

std::hash<std::u16string_view> f178(const std::hash<std::u16string_view> &a0) {
  return std::hash<std::u16string_view>(a0);
}

std::hash<std::u16string_view> f179(std::hash<std::u16string_view> &&a0) {
  return std::hash<std::u16string_view>(std::move(a0));
}

std::hash<std::u16string_view> &f180(std::hash<std::u16string_view> &a0,
                                     const std::hash<std::u16string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u16string_view> &f181(std::hash<std::u16string_view> &a0,
                                     std::hash<std::u16string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t44 = std::hash<std::u32string_view>;

std::size_t f182(const std::hash<std::u32string_view> &a0,
                 const std::u32string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u32string_view> f183() {
  return std::hash<std::u32string_view>();
}

std::hash<std::u32string_view> f184(const std::hash<std::u32string_view> &a0) {
  return std::hash<std::u32string_view>(a0);
}

std::hash<std::u32string_view> f185(std::hash<std::u32string_view> &&a0) {
  return std::hash<std::u32string_view>(std::move(a0));
}

std::hash<std::u32string_view> &f186(std::hash<std::u32string_view> &a0,
                                     const std::hash<std::u32string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u32string_view> &f187(std::hash<std::u32string_view> &a0,
                                     std::hash<std::u32string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t45 = std::hash<std::error_condition>;

std::size_t f188(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f189() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f190(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f191(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f192(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f193(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

using t46 = std::hash<std::nullptr_t>;

std::size_t f194(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f195() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f196(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f197(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f198(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f199(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t47 = std::hash<std::optional<T1>>;

template <typename... T1> using t48 = std::hash<std::variant<T1...>>;
