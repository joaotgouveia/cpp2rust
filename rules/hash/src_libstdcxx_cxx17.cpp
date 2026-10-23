#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <filesystem>
#include <future>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t40 = std::hash<std::string_view>;

std::size_t f154(const std::hash<std::string_view> &a0,
                 const std::string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::string_view> f155() { return std::hash<std::string_view>(); }

std::hash<std::string_view> f156(const std::hash<std::string_view> &a0) {
  return std::hash<std::string_view>(a0);
}

std::hash<std::string_view> f157(std::hash<std::string_view> &&a0) {
  return std::hash<std::string_view>(std::move(a0));
}

std::hash<std::string_view> &f158(std::hash<std::string_view> &a0,
                                  const std::hash<std::string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::string_view> &f159(std::hash<std::string_view> &a0,
                                  std::hash<std::string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t41 = std::hash<std::wstring_view>;

std::size_t f160(const std::hash<std::wstring_view> &a0,
                 const std::wstring_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::wstring_view> f161() { return std::hash<std::wstring_view>(); }

std::hash<std::wstring_view> f162(const std::hash<std::wstring_view> &a0) {
  return std::hash<std::wstring_view>(a0);
}

std::hash<std::wstring_view> f163(std::hash<std::wstring_view> &&a0) {
  return std::hash<std::wstring_view>(std::move(a0));
}

std::hash<std::wstring_view> &f164(std::hash<std::wstring_view> &a0,
                                   const std::hash<std::wstring_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::wstring_view> &f165(std::hash<std::wstring_view> &a0,
                                   std::hash<std::wstring_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t42 = std::hash<std::u16string_view>;

std::size_t f166(const std::hash<std::u16string_view> &a0,
                 const std::u16string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u16string_view> f167() {
  return std::hash<std::u16string_view>();
}

std::hash<std::u16string_view> f168(const std::hash<std::u16string_view> &a0) {
  return std::hash<std::u16string_view>(a0);
}

std::hash<std::u16string_view> f169(std::hash<std::u16string_view> &&a0) {
  return std::hash<std::u16string_view>(std::move(a0));
}

std::hash<std::u16string_view> &f170(std::hash<std::u16string_view> &a0,
                                     const std::hash<std::u16string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u16string_view> &f171(std::hash<std::u16string_view> &a0,
                                     std::hash<std::u16string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t43 = std::hash<std::u32string_view>;

std::size_t f172(const std::hash<std::u32string_view> &a0,
                 const std::u32string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u32string_view> f173() {
  return std::hash<std::u32string_view>();
}

std::hash<std::u32string_view> f174(const std::hash<std::u32string_view> &a0) {
  return std::hash<std::u32string_view>(a0);
}

std::hash<std::u32string_view> f175(std::hash<std::u32string_view> &&a0) {
  return std::hash<std::u32string_view>(std::move(a0));
}

std::hash<std::u32string_view> &f176(std::hash<std::u32string_view> &a0,
                                     const std::hash<std::u32string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u32string_view> &f177(std::hash<std::u32string_view> &a0,
                                     std::hash<std::u32string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t44 = std::hash<std::error_condition>;

std::size_t f178(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f179() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f180(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f181(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f182(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f183(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

using t45 = std::hash<std::nullptr_t>;

std::size_t f184(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f185() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f186(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f187(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f188(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f189(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1> using t46 = std::hash<std::variant<T1...>>;
