#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <filesystem>
#include <future>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t32 = std::hash<std::string_view>;

std::size_t f149(const std::hash<std::string_view> &a0,
                 const std::string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::string_view> f150() { return std::hash<std::string_view>(); }

std::hash<std::string_view> f151(const std::hash<std::string_view> &a0) {
  return std::hash<std::string_view>(a0);
}

std::hash<std::string_view> f152(std::hash<std::string_view> &&a0) {
  return std::hash<std::string_view>(std::move(a0));
}

std::hash<std::string_view> &f153(std::hash<std::string_view> &a0,
                                  const std::hash<std::string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::string_view> &f154(std::hash<std::string_view> &a0,
                                  std::hash<std::string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t33 = std::hash<std::wstring_view>;

std::size_t f155(const std::hash<std::wstring_view> &a0,
                 const std::wstring_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::wstring_view> f156() { return std::hash<std::wstring_view>(); }

std::hash<std::wstring_view> f157(const std::hash<std::wstring_view> &a0) {
  return std::hash<std::wstring_view>(a0);
}

std::hash<std::wstring_view> f158(std::hash<std::wstring_view> &&a0) {
  return std::hash<std::wstring_view>(std::move(a0));
}

std::hash<std::wstring_view> &f159(std::hash<std::wstring_view> &a0,
                                   const std::hash<std::wstring_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::wstring_view> &f160(std::hash<std::wstring_view> &a0,
                                   std::hash<std::wstring_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t34 = std::hash<std::u16string_view>;

std::size_t f161(const std::hash<std::u16string_view> &a0,
                 const std::u16string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u16string_view> f162() {
  return std::hash<std::u16string_view>();
}

std::hash<std::u16string_view> f163(const std::hash<std::u16string_view> &a0) {
  return std::hash<std::u16string_view>(a0);
}

std::hash<std::u16string_view> f164(std::hash<std::u16string_view> &&a0) {
  return std::hash<std::u16string_view>(std::move(a0));
}

std::hash<std::u16string_view> &f165(std::hash<std::u16string_view> &a0,
                                     const std::hash<std::u16string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u16string_view> &f166(std::hash<std::u16string_view> &a0,
                                     std::hash<std::u16string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t35 = std::hash<std::u32string_view>;

std::size_t f167(const std::hash<std::u32string_view> &a0,
                 const std::u32string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u32string_view> f168() {
  return std::hash<std::u32string_view>();
}

std::hash<std::u32string_view> f169(const std::hash<std::u32string_view> &a0) {
  return std::hash<std::u32string_view>(a0);
}

std::hash<std::u32string_view> f170(std::hash<std::u32string_view> &&a0) {
  return std::hash<std::u32string_view>(std::move(a0));
}

std::hash<std::u32string_view> &f171(std::hash<std::u32string_view> &a0,
                                     const std::hash<std::u32string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u32string_view> &f172(std::hash<std::u32string_view> &a0,
                                     std::hash<std::u32string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t36 = std::hash<std::error_condition>;

std::size_t f173(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f174() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f175(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f176(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f177(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f178(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

using t37 = std::hash<std::nullptr_t>;

std::size_t f179(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f180() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f181(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f182(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f183(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f184(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}
