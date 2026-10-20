#include "../common/init.h"
#include "../common/traits.h"
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

using t30 = std::hash<std::filesystem::path>;

std::size_t f136(const std::hash<std::filesystem::path> &a0,
                 const std::filesystem::path &a1) {
  return a0.operator()(a1);
}

std::hash<std::filesystem::path> f137() {
  return std::hash<std::filesystem::path>();
}

std::hash<std::filesystem::path>
f138(const std::hash<std::filesystem::path> &a0) {
  return std::hash<std::filesystem::path>(a0);
}

std::hash<std::filesystem::path> f139(std::hash<std::filesystem::path> &&a0) {
  return std::hash<std::filesystem::path>(std::move(a0));
}

std::hash<std::filesystem::path> &
f140(std::hash<std::filesystem::path> &a0,
     const std::hash<std::filesystem::path> &a1) {
  return a0.operator=(a1);
}

std::hash<std::filesystem::path> &f141(std::hash<std::filesystem::path> &a0,
                                       std::hash<std::filesystem::path> &&a1) {
  return a0.operator=(std::move(a1));
}

using t31 = std::hash<std::string_view>;

std::size_t f142(const std::hash<std::string_view> &a0,
                 const std::string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::string_view> f143() { return std::hash<std::string_view>(); }

std::hash<std::string_view> f144(const std::hash<std::string_view> &a0) {
  return std::hash<std::string_view>(a0);
}

std::hash<std::string_view> f145(std::hash<std::string_view> &&a0) {
  return std::hash<std::string_view>(std::move(a0));
}

std::hash<std::string_view> &f146(std::hash<std::string_view> &a0,
                                  const std::hash<std::string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::string_view> &f147(std::hash<std::string_view> &a0,
                                  std::hash<std::string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::hash<std::wstring_view>;

std::size_t f148(const std::hash<std::wstring_view> &a0,
                 const std::wstring_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::wstring_view> f149() { return std::hash<std::wstring_view>(); }

std::hash<std::wstring_view> f150(const std::hash<std::wstring_view> &a0) {
  return std::hash<std::wstring_view>(a0);
}

std::hash<std::wstring_view> f151(std::hash<std::wstring_view> &&a0) {
  return std::hash<std::wstring_view>(std::move(a0));
}

std::hash<std::wstring_view> &f152(std::hash<std::wstring_view> &a0,
                                   const std::hash<std::wstring_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::wstring_view> &f153(std::hash<std::wstring_view> &a0,
                                   std::hash<std::wstring_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t33 = std::hash<std::u16string_view>;

std::size_t f154(const std::hash<std::u16string_view> &a0,
                 const std::u16string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u16string_view> f155() {
  return std::hash<std::u16string_view>();
}

std::hash<std::u16string_view> f156(const std::hash<std::u16string_view> &a0) {
  return std::hash<std::u16string_view>(a0);
}

std::hash<std::u16string_view> f157(std::hash<std::u16string_view> &&a0) {
  return std::hash<std::u16string_view>(std::move(a0));
}

std::hash<std::u16string_view> &f158(std::hash<std::u16string_view> &a0,
                                     const std::hash<std::u16string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u16string_view> &f159(std::hash<std::u16string_view> &a0,
                                     std::hash<std::u16string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t34 = std::hash<std::u32string_view>;

std::size_t f160(const std::hash<std::u32string_view> &a0,
                 const std::u32string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u32string_view> f161() {
  return std::hash<std::u32string_view>();
}

std::hash<std::u32string_view> f162(const std::hash<std::u32string_view> &a0) {
  return std::hash<std::u32string_view>(a0);
}

std::hash<std::u32string_view> f163(std::hash<std::u32string_view> &&a0) {
  return std::hash<std::u32string_view>(std::move(a0));
}

std::hash<std::u32string_view> &f164(std::hash<std::u32string_view> &a0,
                                     const std::hash<std::u32string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u32string_view> &f165(std::hash<std::u32string_view> &a0,
                                     std::hash<std::u32string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t35 = std::hash<std::error_condition>;

std::size_t f166(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f167() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f168(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f169(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f170(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f171(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

using t36 = std::hash<std::monostate>;

std::size_t f172(const std::hash<std::monostate> &a0,
                 const std::monostate &a1) {
  return a0.operator()(a1);
}

std::hash<std::monostate> f173() { return std::hash<std::monostate>(); }

std::hash<std::monostate> f174(const std::hash<std::monostate> &a0) {
  return std::hash<std::monostate>(a0);
}

std::hash<std::monostate> f175(std::hash<std::monostate> &&a0) {
  return std::hash<std::monostate>(std::move(a0));
}

std::hash<std::monostate> &f176(std::hash<std::monostate> &a0,
                                const std::hash<std::monostate> &a1) {
  return a0.operator=(a1);
}

std::hash<std::monostate> &f177(std::hash<std::monostate> &a0,
                                std::hash<std::monostate> &&a1) {
  return a0.operator=(std::move(a1));
}

using t37 = std::hash<std::nullptr_t>;

std::size_t f178(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f179() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f180(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f181(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f182(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f183(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}
