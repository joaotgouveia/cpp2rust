#include <filesystem>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::filesystem::directory_iterator f1(std::filesystem::directory_iterator a0) {
  return std::filesystem::begin(std::move(a0));
}

using t1 = std::filesystem::directory_iterator;

using t2 = std::filesystem::directory_iterator::value_type;

using t3 = std::filesystem::directory_iterator::difference_type;

using t4 = std::filesystem::directory_iterator::pointer;

using t5 = std::filesystem::directory_iterator::reference;

using t6 = std::filesystem::directory_iterator::iterator_category;

std::filesystem::directory_iterator f2() {
  return std::filesystem::directory_iterator();
}

std::filesystem::directory_iterator f3(const std::filesystem::path &a0) {
  return std::filesystem::directory_iterator(a0);
}

std::filesystem::directory_iterator f4(const std::filesystem::path &a0,
                                       std::filesystem::directory_options a1) {
  return std::filesystem::directory_iterator(a0, a1);
}

std::filesystem::directory_iterator f5(const std::filesystem::path &a0,
                                       std::error_code &a1) {
  return std::filesystem::directory_iterator(a0, a1);
}

std::filesystem::directory_iterator f6(const std::filesystem::path &a0,
                                       std::filesystem::directory_options a1,
                                       std::error_code &a2) {
  return std::filesystem::directory_iterator(a0, a1, a2);
}

std::filesystem::directory_iterator
f7(const std::filesystem::directory_iterator &a0) {
  return std::filesystem::directory_iterator(a0);
}

std::filesystem::directory_iterator
f8(std::filesystem::directory_iterator &&a0) {
  return std::filesystem::directory_iterator(std::move(a0));
}

std::filesystem::directory_iterator &
f9(std::filesystem::directory_iterator &a0,
   const std::filesystem::directory_iterator &a1) {
  return a0.operator=(a1);
}

std::filesystem::directory_iterator &
f10(std::filesystem::directory_iterator &a0,
    std::filesystem::directory_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

const std::filesystem::directory_entry &
f11(const std::filesystem::directory_iterator &a0) {
  return a0.operator*();
}

const std::filesystem::directory_entry *
f12(const std::filesystem::directory_iterator &a0) {
  return a0.operator->();
}

std::filesystem::directory_iterator &
f13(std::filesystem::directory_iterator &a0) {
  return a0.operator++();
}

std::filesystem::directory_iterator &
f14(std::filesystem::directory_iterator &a0, std::error_code &a1) {
  return a0.increment(a1);
}

std::filesystem::directory_iterator
f15(std::filesystem::directory_iterator a0) {
  return std::filesystem::end(std::move(a0));
}

bool f16(const std::filesystem::directory_iterator &a0,
         const std::filesystem::directory_iterator &a1) {
  return a0 != a1;
}

bool f17(const std::filesystem::directory_iterator &a0,
         const std::filesystem::directory_iterator &a1) {
  return a0 == a1;
}
