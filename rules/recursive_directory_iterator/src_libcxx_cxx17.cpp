#include <filesystem>
#include <system_error>
#include <utility>

std::filesystem::recursive_directory_iterator
f1(std::filesystem::recursive_directory_iterator a0) {
  return std::filesystem::begin(std::move(a0));
}

std::filesystem::recursive_directory_iterator
f2(std::filesystem::recursive_directory_iterator a0) {
  return std::filesystem::end(std::move(a0));
}

bool f3(const std::filesystem::recursive_directory_iterator &a0,
        const std::filesystem::recursive_directory_iterator &a1) {
  return std::filesystem::operator!=(a0, a1);
}

bool f4(const std::filesystem::recursive_directory_iterator &a0,
        const std::filesystem::recursive_directory_iterator &a1) {
  return std::filesystem::operator==(a0, a1);
}

using t1 = std::filesystem::recursive_directory_iterator;

using t2 = std::filesystem::recursive_directory_iterator::value_type;

using t3 = std::filesystem::recursive_directory_iterator::difference_type;

using t4 = std::filesystem::recursive_directory_iterator::pointer;

using t5 = std::filesystem::recursive_directory_iterator::reference;

using t6 = std::filesystem::recursive_directory_iterator::iterator_category;

std::filesystem::recursive_directory_iterator f5() {
  return std::filesystem::recursive_directory_iterator();
}

std::filesystem::recursive_directory_iterator
f6(const std::filesystem::path &a0, std::filesystem::directory_options a1) {
  return std::filesystem::recursive_directory_iterator(a0, a1);
}

std::filesystem::recursive_directory_iterator
f7(const std::filesystem::path &a0, std::filesystem::directory_options a1,
   std::error_code &a2) {
  return std::filesystem::recursive_directory_iterator(a0, a1, a2);
}

std::filesystem::recursive_directory_iterator
f8(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::recursive_directory_iterator(a0, a1);
}

std::filesystem::recursive_directory_iterator
f9(const std::filesystem::recursive_directory_iterator &a0) {
  return std::filesystem::recursive_directory_iterator(a0);
}

std::filesystem::recursive_directory_iterator
f10(std::filesystem::recursive_directory_iterator &&a0) {
  return std::filesystem::recursive_directory_iterator(std::move(a0));
}

std::filesystem::recursive_directory_iterator &
f11(std::filesystem::recursive_directory_iterator &a0,
    const std::filesystem::recursive_directory_iterator &a1) {
  return a0.operator=(a1);
}

std::filesystem::recursive_directory_iterator &
f12(std::filesystem::recursive_directory_iterator &a0,
    std::filesystem::recursive_directory_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

const std::filesystem::directory_entry &
f13(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator*();
}

const std::filesystem::directory_entry *
f14(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator->();
}

std::filesystem::recursive_directory_iterator &
f15(std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator++();
}

std::filesystem::recursive_directory_iterator &
f16(std::filesystem::recursive_directory_iterator &a0, std::error_code &a1) {
  return a0.increment(a1);
}

std::filesystem::directory_options
f17(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.options();
}

int f18(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.depth();
}

void f19(std::filesystem::recursive_directory_iterator &a0) { return a0.pop(); }

void f20(std::filesystem::recursive_directory_iterator &a0,
         std::error_code &a1) {
  return a0.pop(a1);
}

bool f21(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.recursion_pending();
}

void f22(std::filesystem::recursive_directory_iterator &a0) {
  return a0.disable_recursion_pending();
}
