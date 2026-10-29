#include <algorithm>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <filesystem>
#include <format>
#include <optional>
#include <stacktrace>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t1 = std::hash<std::filesystem::path>;

std::size_t f1(const std::hash<std::filesystem::path> &a0,
               const std::filesystem::path &a1) {
  return a0.operator()(a1);
}

std::hash<std::filesystem::path> f2() {
  return std::hash<std::filesystem::path>();
}

std::hash<std::filesystem::path>
f3(const std::hash<std::filesystem::path> &a0) {
  return std::hash<std::filesystem::path>(a0);
}

std::hash<std::filesystem::path> f4(std::hash<std::filesystem::path> &&a0) {
  return std::hash<std::filesystem::path>(std::move(a0));
}

std::hash<std::filesystem::path> &
f5(std::hash<std::filesystem::path> &a0,
   const std::hash<std::filesystem::path> &a1) {
  return a0.operator=(a1);
}

std::hash<std::filesystem::path> &f6(std::hash<std::filesystem::path> &a0,
                                     std::hash<std::filesystem::path> &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::hash<std::string_view>;

std::size_t f7(const std::hash<std::string_view> &a0,
               const std::string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::string_view> f8() { return std::hash<std::string_view>(); }

std::hash<std::string_view> f9(const std::hash<std::string_view> &a0) {
  return std::hash<std::string_view>(a0);
}

std::hash<std::string_view> f10(std::hash<std::string_view> &&a0) {
  return std::hash<std::string_view>(std::move(a0));
}

std::hash<std::string_view> &f11(std::hash<std::string_view> &a0,
                                 const std::hash<std::string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::string_view> &f12(std::hash<std::string_view> &a0,
                                 std::hash<std::string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::hash<std::wstring_view>;

std::size_t f13(const std::hash<std::wstring_view> &a0,
                const std::wstring_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::wstring_view> f14() { return std::hash<std::wstring_view>(); }

std::hash<std::wstring_view> f15(const std::hash<std::wstring_view> &a0) {
  return std::hash<std::wstring_view>(a0);
}

std::hash<std::wstring_view> f16(std::hash<std::wstring_view> &&a0) {
  return std::hash<std::wstring_view>(std::move(a0));
}

std::hash<std::wstring_view> &f17(std::hash<std::wstring_view> &a0,
                                  const std::hash<std::wstring_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::wstring_view> &f18(std::hash<std::wstring_view> &a0,
                                  std::hash<std::wstring_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t4 = std::hash<std::u16string_view>;

std::size_t f19(const std::hash<std::u16string_view> &a0,
                const std::u16string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u16string_view> f20() {
  return std::hash<std::u16string_view>();
}

std::hash<std::u16string_view> f21(const std::hash<std::u16string_view> &a0) {
  return std::hash<std::u16string_view>(a0);
}

std::hash<std::u16string_view> f22(std::hash<std::u16string_view> &&a0) {
  return std::hash<std::u16string_view>(std::move(a0));
}

std::hash<std::u16string_view> &f23(std::hash<std::u16string_view> &a0,
                                    const std::hash<std::u16string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u16string_view> &f24(std::hash<std::u16string_view> &a0,
                                    std::hash<std::u16string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t5 = std::hash<std::u8string_view>;

std::size_t f25(const std::hash<std::u8string_view> &a0,
                const std::u8string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u8string_view> f26() { return std::hash<std::u8string_view>(); }

std::hash<std::u8string_view> f27(const std::hash<std::u8string_view> &a0) {
  return std::hash<std::u8string_view>(a0);
}

std::hash<std::u8string_view> f28(std::hash<std::u8string_view> &&a0) {
  return std::hash<std::u8string_view>(std::move(a0));
}

std::hash<std::u8string_view> &f29(std::hash<std::u8string_view> &a0,
                                   const std::hash<std::u8string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u8string_view> &f30(std::hash<std::u8string_view> &a0,
                                   std::hash<std::u8string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t6 = std::hash<std::u32string_view>;

std::size_t f31(const std::hash<std::u32string_view> &a0,
                const std::u32string_view &a1) {
  return a0.operator()(a1);
}

std::hash<std::u32string_view> f32() {
  return std::hash<std::u32string_view>();
}

std::hash<std::u32string_view> f33(const std::hash<std::u32string_view> &a0) {
  return std::hash<std::u32string_view>(a0);
}

std::hash<std::u32string_view> f34(std::hash<std::u32string_view> &&a0) {
  return std::hash<std::u32string_view>(std::move(a0));
}

std::hash<std::u32string_view> &f35(std::hash<std::u32string_view> &a0,
                                    const std::hash<std::u32string_view> &a1) {
  return a0.operator=(a1);
}

std::hash<std::u32string_view> &f36(std::hash<std::u32string_view> &a0,
                                    std::hash<std::u32string_view> &&a1) {
  return a0.operator=(std::move(a1));
}

using t7 = std::hash<std::bitset<0>>;

std::size_t f37(const std::hash<std::bitset<0>> &a0, const std::bitset<0> &a1) {
  return a0.operator()(a1);
}

std::hash<std::bitset<0>> f38() { return std::hash<std::bitset<0>>(); }

std::hash<std::bitset<0>> f39(const std::hash<std::bitset<0>> &a0) {
  return std::hash<std::bitset<0>>(a0);
}

std::hash<std::bitset<0>> f40(std::hash<std::bitset<0>> &&a0) {
  return std::hash<std::bitset<0>>(std::move(a0));
}

std::hash<std::bitset<0>> &f41(std::hash<std::bitset<0>> &a0,
                               const std::hash<std::bitset<0>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::bitset<0>> &f42(std::hash<std::bitset<0>> &a0,
                               std::hash<std::bitset<0>> &&a1) {
  return a0.operator=(std::move(a1));
}

using t8 = std::hash<std::error_code>;

std::size_t f43(const std::hash<std::error_code> &a0,
                const std::error_code &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_code> f44() { return std::hash<std::error_code>(); }

std::hash<std::error_code> f45(const std::hash<std::error_code> &a0) {
  return std::hash<std::error_code>(a0);
}

std::hash<std::error_code> f46(std::hash<std::error_code> &&a0) {
  return std::hash<std::error_code>(std::move(a0));
}

std::hash<std::error_code> &f47(std::hash<std::error_code> &a0,
                                const std::hash<std::error_code> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_code> &f48(std::hash<std::error_code> &a0,
                                std::hash<std::error_code> &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::hash<std::error_condition>;

std::size_t f49(const std::hash<std::error_condition> &a0,
                const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f50() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition> f51(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f52(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f53(std::hash<std::error_condition> &a0,
    const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f54(std::hash<std::error_condition> &a0,
                                     std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

using t10 = std::hash<std::monostate>;

std::size_t f55(const std::hash<std::monostate> &a0, const std::monostate &a1) {
  return a0.operator()(a1);
}

std::hash<std::monostate> f56() { return std::hash<std::monostate>(); }

std::hash<std::monostate> f57(const std::hash<std::monostate> &a0) {
  return std::hash<std::monostate>(a0);
}

std::hash<std::monostate> f58(std::hash<std::monostate> &&a0) {
  return std::hash<std::monostate>(std::move(a0));
}

std::hash<std::monostate> &f59(std::hash<std::monostate> &a0,
                               const std::hash<std::monostate> &a1) {
  return a0.operator=(a1);
}

std::hash<std::monostate> &f60(std::hash<std::monostate> &a0,
                               std::hash<std::monostate> &&a1) {
  return a0.operator=(std::move(a1));
}

using t11 = std::hash<std::stacktrace_entry>;

std::size_t f61(const std::hash<std::stacktrace_entry> &a0,
                const std::stacktrace_entry &a1) {
  return a0.operator()(a1);
}

std::hash<std::stacktrace_entry> f62() {
  return std::hash<std::stacktrace_entry>();
}

std::hash<std::stacktrace_entry>
f63(const std::hash<std::stacktrace_entry> &a0) {
  return std::hash<std::stacktrace_entry>(a0);
}

std::hash<std::stacktrace_entry> f64(std::hash<std::stacktrace_entry> &&a0) {
  return std::hash<std::stacktrace_entry>(std::move(a0));
}

std::hash<std::stacktrace_entry> &
f65(std::hash<std::stacktrace_entry> &a0,
    const std::hash<std::stacktrace_entry> &a1) {
  return a0.operator=(a1);
}

std::hash<std::stacktrace_entry> &f66(std::hash<std::stacktrace_entry> &a0,
                                      std::hash<std::stacktrace_entry> &&a1) {
  return a0.operator=(std::move(a1));
}

using t12 = std::hash<std::thread::id>;

std::size_t f67(const std::hash<std::thread::id> &a0,
                const std::thread::id &a1) {
  return a0.operator()(a1);
}

std::hash<std::thread::id> f68() { return std::hash<std::thread::id>(); }

std::hash<std::thread::id> f69(const std::hash<std::thread::id> &a0) {
  return std::hash<std::thread::id>(a0);
}

std::hash<std::thread::id> f70(std::hash<std::thread::id> &&a0) {
  return std::hash<std::thread::id>(std::move(a0));
}

std::hash<std::thread::id> &f71(std::hash<std::thread::id> &a0,
                                const std::hash<std::thread::id> &a1) {
  return a0.operator=(a1);
}

std::hash<std::thread::id> &f72(std::hash<std::thread::id> &a0,
                                std::hash<std::thread::id> &&a1) {
  return a0.operator=(std::move(a1));
}

using t13 = std::hash<std::type_index>;

using t14 = std::hash<std::type_index>::result_type;

using t15 = std::hash<std::type_index>::argument_type;

std::size_t f73(const std::hash<std::type_index> &a0,
                const std::type_index &a1) {
  return a0.operator()(a1);
}

std::hash<std::type_index> f74() { return std::hash<std::type_index>(); }

std::hash<std::type_index> f75(const std::hash<std::type_index> &a0) {
  return std::hash<std::type_index>(a0);
}

std::hash<std::type_index> f76(std::hash<std::type_index> &&a0) {
  return std::hash<std::type_index>(std::move(a0));
}

std::hash<std::type_index> &f77(std::hash<std::type_index> &a0,
                                const std::hash<std::type_index> &a1) {
  return a0.operator=(a1);
}

std::hash<std::type_index> &f78(std::hash<std::type_index> &a0,
                                std::hash<std::type_index> &&a1) {
  return a0.operator=(std::move(a1));
}

using t16 = std::hash<char>;

std::size_t f79(const std::hash<char> &a0, char a1) {
  return a0.operator()(a1);
}

std::hash<char> f80() { return std::hash<char>(); }

std::hash<char> f81(const std::hash<char> &a0) { return std::hash<char>(a0); }

std::hash<char> f82(std::hash<char> &&a0) {
  return std::hash<char>(std::move(a0));
}

std::hash<char> &f83(std::hash<char> &a0, const std::hash<char> &a1) {
  return a0.operator=(a1);
}

std::hash<char> &f84(std::hash<char> &a0, std::hash<char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t17 = std::hash<long double>;

std::size_t f85(const std::hash<long double> &a0, long double a1) {
  return a0.operator()(a1);
}

std::hash<long double> f86() { return std::hash<long double>(); }

std::hash<long double> f87(const std::hash<long double> &a0) {
  return std::hash<long double>(a0);
}

std::hash<long double> f88(std::hash<long double> &&a0) {
  return std::hash<long double>(std::move(a0));
}

std::hash<long double> &f89(std::hash<long double> &a0,
                            const std::hash<long double> &a1) {
  return a0.operator=(a1);
}

std::hash<long double> &f90(std::hash<long double> &a0,
                            std::hash<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t18 = std::hash<int>;

std::size_t f91(const std::hash<int> &a0, int a1) { return a0.operator()(a1); }

std::hash<int> f92() { return std::hash<int>(); }

std::hash<int> f93(const std::hash<int> &a0) { return std::hash<int>(a0); }

std::hash<int> f94(std::hash<int> &&a0) {
  return std::hash<int>(std::move(a0));
}

std::hash<int> &f95(std::hash<int> &a0, const std::hash<int> &a1) {
  return a0.operator=(a1);
}

std::hash<int> &f96(std::hash<int> &a0, std::hash<int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t19 = std::hash<long long>;

std::size_t f97(const std::hash<long long> &a0, long long a1) {
  return a0.operator()(a1);
}

std::hash<long long> f98() { return std::hash<long long>(); }

std::hash<long long> f99(const std::hash<long long> &a0) {
  return std::hash<long long>(a0);
}

std::hash<long long> f100(std::hash<long long> &&a0) {
  return std::hash<long long>(std::move(a0));
}

std::hash<long long> &f101(std::hash<long long> &a0,
                           const std::hash<long long> &a1) {
  return a0.operator=(a1);
}

std::hash<long long> &f102(std::hash<long long> &a0,
                           std::hash<long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t20 = std::hash<long>;

std::size_t f103(const std::hash<long> &a0, long a1) {
  return a0.operator()(a1);
}

std::hash<long> f104() { return std::hash<long>(); }

std::hash<long> f105(const std::hash<long> &a0) { return std::hash<long>(a0); }

std::hash<long> f106(std::hash<long> &&a0) {
  return std::hash<long>(std::move(a0));
}

std::hash<long> &f107(std::hash<long> &a0, const std::hash<long> &a1) {
  return a0.operator=(a1);
}

std::hash<long> &f108(std::hash<long> &a0, std::hash<long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t21 = std::hash<short>;

std::size_t f109(const std::hash<short> &a0, short a1) {
  return a0.operator()(a1);
}

std::hash<short> f110() { return std::hash<short>(); }

std::hash<short> f111(const std::hash<short> &a0) {
  return std::hash<short>(a0);
}

std::hash<short> f112(std::hash<short> &&a0) {
  return std::hash<short>(std::move(a0));
}

std::hash<short> &f113(std::hash<short> &a0, const std::hash<short> &a1) {
  return a0.operator=(a1);
}

std::hash<short> &f114(std::hash<short> &a0, std::hash<short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t22 = std::hash<wchar_t>;

std::size_t f115(const std::hash<wchar_t> &a0, wchar_t a1) {
  return a0.operator()(a1);
}

std::hash<wchar_t> f116() { return std::hash<wchar_t>(); }

std::hash<wchar_t> f117(const std::hash<wchar_t> &a0) {
  return std::hash<wchar_t>(a0);
}

std::hash<wchar_t> f118(std::hash<wchar_t> &&a0) {
  return std::hash<wchar_t>(std::move(a0));
}

std::hash<wchar_t> &f119(std::hash<wchar_t> &a0, const std::hash<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::hash<wchar_t> &f120(std::hash<wchar_t> &a0, std::hash<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t23 = std::hash<bool>;

std::size_t f121(const std::hash<bool> &a0, bool a1) {
  return a0.operator()(a1);
}

std::hash<bool> f122() { return std::hash<bool>(); }

std::hash<bool> f123(const std::hash<bool> &a0) { return std::hash<bool>(a0); }

std::hash<bool> f124(std::hash<bool> &&a0) {
  return std::hash<bool>(std::move(a0));
}

std::hash<bool> &f125(std::hash<bool> &a0, const std::hash<bool> &a1) {
  return a0.operator=(a1);
}

std::hash<bool> &f126(std::hash<bool> &a0, std::hash<bool> &&a1) {
  return a0.operator=(std::move(a1));
}

using t24 = std::hash<unsigned char>;

std::size_t f127(const std::hash<unsigned char> &a0, unsigned char a1) {
  return a0.operator()(a1);
}

std::hash<unsigned char> f128() { return std::hash<unsigned char>(); }

std::hash<unsigned char> f129(const std::hash<unsigned char> &a0) {
  return std::hash<unsigned char>(a0);
}

std::hash<unsigned char> f130(std::hash<unsigned char> &&a0) {
  return std::hash<unsigned char>(std::move(a0));
}

std::hash<unsigned char> &f131(std::hash<unsigned char> &a0,
                               const std::hash<unsigned char> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned char> &f132(std::hash<unsigned char> &a0,
                               std::hash<unsigned char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t25 = std::hash<double>;

std::size_t f133(const std::hash<double> &a0, double a1) {
  return a0.operator()(a1);
}

std::hash<double> f134() { return std::hash<double>(); }

std::hash<double> f135(const std::hash<double> &a0) {
  return std::hash<double>(a0);
}

std::hash<double> f136(std::hash<double> &&a0) {
  return std::hash<double>(std::move(a0));
}

std::hash<double> &f137(std::hash<double> &a0, const std::hash<double> &a1) {
  return a0.operator=(a1);
}

std::hash<double> &f138(std::hash<double> &a0, std::hash<double> &&a1) {
  return a0.operator=(std::move(a1));
}

using t26 = std::hash<float>;

std::size_t f139(const std::hash<float> &a0, float a1) {
  return a0.operator()(a1);
}

std::hash<float> f140() { return std::hash<float>(); }

std::hash<float> f141(const std::hash<float> &a0) {
  return std::hash<float>(a0);
}

std::hash<float> f142(std::hash<float> &&a0) {
  return std::hash<float>(std::move(a0));
}

std::hash<float> &f143(std::hash<float> &a0, const std::hash<float> &a1) {
  return a0.operator=(a1);
}

std::hash<float> &f144(std::hash<float> &a0, std::hash<float> &&a1) {
  return a0.operator=(std::move(a1));
}

using t27 = std::hash<unsigned int>;

std::size_t f145(const std::hash<unsigned int> &a0, unsigned int a1) {
  return a0.operator()(a1);
}

std::hash<unsigned int> f146() { return std::hash<unsigned int>(); }

std::hash<unsigned int> f147(const std::hash<unsigned int> &a0) {
  return std::hash<unsigned int>(a0);
}

std::hash<unsigned int> f148(std::hash<unsigned int> &&a0) {
  return std::hash<unsigned int>(std::move(a0));
}

std::hash<unsigned int> &f149(std::hash<unsigned int> &a0,
                              const std::hash<unsigned int> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned int> &f150(std::hash<unsigned int> &a0,
                              std::hash<unsigned int> &&a1) {
  return a0.operator=(std::move(a1));
}

using t28 = std::hash<unsigned long long>;

std::size_t f151(const std::hash<unsigned long long> &a0,
                 unsigned long long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long long> f152() { return std::hash<unsigned long long>(); }

std::hash<unsigned long long> f153(const std::hash<unsigned long long> &a0) {
  return std::hash<unsigned long long>(a0);
}

std::hash<unsigned long long> f154(std::hash<unsigned long long> &&a0) {
  return std::hash<unsigned long long>(std::move(a0));
}

std::hash<unsigned long long> &f155(std::hash<unsigned long long> &a0,
                                    const std::hash<unsigned long long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long long> &f156(std::hash<unsigned long long> &a0,
                                    std::hash<unsigned long long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t29 = std::hash<unsigned long>;

std::size_t f157(const std::hash<unsigned long> &a0, unsigned long a1) {
  return a0.operator()(a1);
}

std::hash<unsigned long> f158() { return std::hash<unsigned long>(); }

std::hash<unsigned long> f159(const std::hash<unsigned long> &a0) {
  return std::hash<unsigned long>(a0);
}

std::hash<unsigned long> f160(std::hash<unsigned long> &&a0) {
  return std::hash<unsigned long>(std::move(a0));
}

std::hash<unsigned long> &f161(std::hash<unsigned long> &a0,
                               const std::hash<unsigned long> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned long> &f162(std::hash<unsigned long> &a0,
                               std::hash<unsigned long> &&a1) {
  return a0.operator=(std::move(a1));
}

using t30 = std::hash<std::nullptr_t>;

std::size_t f163(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f164() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f165(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f166(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f167(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f168(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t31 = std::hash<char16_t>;

std::size_t f169(const std::hash<char16_t> &a0, char16_t a1) {
  return a0.operator()(a1);
}

std::hash<char16_t> f170() { return std::hash<char16_t>(); }

std::hash<char16_t> f171(const std::hash<char16_t> &a0) {
  return std::hash<char16_t>(a0);
}

std::hash<char16_t> f172(std::hash<char16_t> &&a0) {
  return std::hash<char16_t>(std::move(a0));
}

std::hash<char16_t> &f173(std::hash<char16_t> &a0,
                          const std::hash<char16_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char16_t> &f174(std::hash<char16_t> &a0, std::hash<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t32 = std::hash<signed char>;

std::size_t f175(const std::hash<signed char> &a0, signed char a1) {
  return a0.operator()(a1);
}

std::hash<signed char> f176() { return std::hash<signed char>(); }

std::hash<signed char> f177(const std::hash<signed char> &a0) {
  return std::hash<signed char>(a0);
}

std::hash<signed char> f178(std::hash<signed char> &&a0) {
  return std::hash<signed char>(std::move(a0));
}

std::hash<signed char> &f179(std::hash<signed char> &a0,
                             const std::hash<signed char> &a1) {
  return a0.operator=(a1);
}

std::hash<signed char> &f180(std::hash<signed char> &a0,
                             std::hash<signed char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t33 = std::hash<unsigned short>;

std::size_t f181(const std::hash<unsigned short> &a0, unsigned short a1) {
  return a0.operator()(a1);
}

std::hash<unsigned short> f182() { return std::hash<unsigned short>(); }

std::hash<unsigned short> f183(const std::hash<unsigned short> &a0) {
  return std::hash<unsigned short>(a0);
}

std::hash<unsigned short> f184(std::hash<unsigned short> &&a0) {
  return std::hash<unsigned short>(std::move(a0));
}

std::hash<unsigned short> &f185(std::hash<unsigned short> &a0,
                                const std::hash<unsigned short> &a1) {
  return a0.operator=(a1);
}

std::hash<unsigned short> &f186(std::hash<unsigned short> &a0,
                                std::hash<unsigned short> &&a1) {
  return a0.operator=(std::move(a1));
}

using t34 = std::hash<char8_t>;

std::size_t f187(const std::hash<char8_t> &a0, char8_t a1) {
  return a0.operator()(a1);
}

std::hash<char8_t> f188() { return std::hash<char8_t>(); }

std::hash<char8_t> f189(const std::hash<char8_t> &a0) {
  return std::hash<char8_t>(a0);
}

std::hash<char8_t> f190(std::hash<char8_t> &&a0) {
  return std::hash<char8_t>(std::move(a0));
}

std::hash<char8_t> &f191(std::hash<char8_t> &a0, const std::hash<char8_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char8_t> &f192(std::hash<char8_t> &a0, std::hash<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t35 = std::hash<char32_t>;

std::size_t f193(const std::hash<char32_t> &a0, char32_t a1) {
  return a0.operator()(a1);
}

std::hash<char32_t> f194() { return std::hash<char32_t>(); }

std::hash<char32_t> f195(const std::hash<char32_t> &a0) {
  return std::hash<char32_t>(a0);
}

std::hash<char32_t> f196(std::hash<char32_t> &&a0) {
  return std::hash<char32_t>(std::move(a0));
}

std::hash<char32_t> &f197(std::hash<char32_t> &a0,
                          const std::hash<char32_t> &a1) {
  return a0.operator=(a1);
}

std::hash<char32_t> &f198(std::hash<char32_t> &a0, std::hash<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <std::size_t T1> using t36 = std::hash<std::bitset<T1>>;

template <typename T1> using t37 = std::hash<T1 *>;

template <typename T1> std::size_t f199(const std::hash<T1 *> &a0, T1 *a1) {
  return a0.operator()(a1);
}

template <typename T1>
using t38 = std::hash<std::basic_string<char, std::char_traits<char>, T1>>;

template <typename T1>
std::size_t
f200(const std::hash<std::basic_string<char, std::char_traits<char>, T1>> &a0,
     const std::basic_string<char, std::char_traits<char>, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1>
using t39 =
    std::hash<std::basic_string<wchar_t, std::char_traits<wchar_t>, T1>>;

template <typename T1>
std::size_t
f201(const std::hash<std::basic_string<wchar_t, std::char_traits<wchar_t>, T1>>
         &a0,
     const std::basic_string<wchar_t, std::char_traits<wchar_t>, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1>
using t40 =
    std::hash<std::basic_string<char16_t, std::char_traits<char16_t>, T1>>;

template <typename T1>
std::size_t f202(
    const std::hash<std::basic_string<char16_t, std::char_traits<char16_t>, T1>>
        &a0,
    const std::basic_string<char16_t, std::char_traits<char16_t>, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1>
using t41 =
    std::hash<std::basic_string<char8_t, std::char_traits<char8_t>, T1>>;

template <typename T1>
std::size_t
f203(const std::hash<std::basic_string<char8_t, std::char_traits<char8_t>, T1>>
         &a0,
     const std::basic_string<char8_t, std::char_traits<char8_t>, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1>
using t42 =
    std::hash<std::basic_string<char32_t, std::char_traits<char32_t>, T1>>;

template <typename T1>
std::size_t f204(
    const std::hash<std::basic_string<char32_t, std::char_traits<char32_t>, T1>>
        &a0,
    const std::basic_string<char32_t, std::char_traits<char32_t>, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t43 = std::hash<std::coroutine_handle<T1>>;

template <typename T1>
std::size_t f205(const std::hash<std::coroutine_handle<T1>> &a0,
                 const std::coroutine_handle<T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t44 = std::hash<std::basic_stacktrace<T1>>;

template <typename T1>
std::size_t f206(const std::hash<std::basic_stacktrace<T1>> &a0,
                 const std::basic_stacktrace<T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t45 = std::hash<std::optional<T1>>;

template <typename T1>
std::size_t f207(const std::hash<std::optional<T1>> &a0,
                 const std::optional<T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t46 = std::hash<std::shared_ptr<T1>>;

template <typename T1>
std::size_t f208(const std::hash<std::shared_ptr<T1>> &a0,
                 const std::shared_ptr<T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t47 = std::hash<std::vector<bool, T1>>;

template <typename T1>
std::size_t f209(const std::hash<std::vector<bool, T1>> &a0,
                 const std::vector<bool, T1> &a1) {
  return a0.operator()(a1);
}

template <typename... T1> using t48 = std::hash<std::variant<T1...>>;

template <typename... T1>
std::size_t f210(const std::hash<std::variant<T1...>> &a0,
                 const std::variant<T1...> &a1) {
  return a0.operator()(a1);
}

template <typename T1, typename T2>
using t49 = std::hash<std::unique_ptr<T1, T2>>;

template <typename T1, typename T2>
typename std::add_lvalue_reference<T1>::type
f211(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.operator*();
}

template <typename T1, typename T2>
T1 *f212(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.operator->();
}

template <typename T1, typename T2>
T1 *f213(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.get();
}

template <typename T1, typename T2>
T2 &f214(std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.get_deleter();
}

template <typename T1, typename T2>
T2 const &f215(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f216(const std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
T1 *f217(std::hash<std::unique_ptr<T1, T2>> &a0)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.release();
}

template <typename T1, typename T2>
void f218(std::hash<std::unique_ptr<T1, T2>> &a0, T1 *a1)
  requires std::is_base_of<std::unique_ptr<T1, T2>,
                           std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.reset(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> &f219(std::hash<std::unique_ptr<T1, T2>> &a0,
                              std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value)) &&
          std::is_base_of<std::unique_ptr<T1, T2>,
                          std::hash<std::unique_ptr<T1, T2>>>::value
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1> using t50 = std::hash<T1>;

template <typename T1> std::size_t f220(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}
