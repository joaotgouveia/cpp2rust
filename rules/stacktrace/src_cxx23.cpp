#include <compare>
#include <iosfwd>
#include <stacktrace>
#include <utility>

std::ostream &f1(std::ostream &a0, const std::stacktrace_entry &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
std::ostream &f2(std::ostream &a0, const std::basic_stacktrace<T1> &a1) {
  return std::operator<<(a0, a1);
}

using t1 = std::pmr::stacktrace;

using t2 = std::stacktrace;

using t3 = std::stacktrace_entry;

using t4 = std::stacktrace_entry::native_handle_type;

std::stacktrace_entry f3() { return std::stacktrace_entry(); }

std::stacktrace_entry f4(const std::stacktrace_entry &a0) {
  return std::stacktrace_entry(a0);
}

std::stacktrace_entry &f5(std::stacktrace_entry &a0,
                          const std::stacktrace_entry &a1) {
  return a0.operator=(a1);
}

std::stacktrace_entry::native_handle_type f6(const std::stacktrace_entry &a0) {
  return a0.native_handle();
}

bool f7(const std::stacktrace_entry &a0) { return static_cast<bool>(a0); }

std::string f8(const std::stacktrace_entry &a0) { return a0.description(); }

std::string f9(const std::stacktrace_entry &a0) { return a0.source_file(); }

unsigned int f10(const std::stacktrace_entry &a0) { return a0.source_line(); }

std::strong_ordering f11(const std::stacktrace_entry &a0,
                         const std::stacktrace_entry &a1) {
  return operator<=>(a0, a1);
}

bool f12(const std::stacktrace_entry &a0, const std::stacktrace_entry &a1) {
  return operator==(a0, a1);
}

template <typename T1>
void f13(std::basic_stacktrace<T1> &a0, std::basic_stacktrace<T1> &a1) {
  return std::swap(a0, a1);
}

std::string f14(const std::stacktrace_entry &a0) { return std::to_string(a0); }

template <typename T1> std::string f15(const std::basic_stacktrace<T1> &a0) {
  return std::to_string(a0);
}
