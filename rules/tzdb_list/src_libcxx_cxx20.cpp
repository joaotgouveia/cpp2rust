#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <string.h>
#include <string_view>
#include <utility>

const std::chrono::time_zone *f1() { return std::chrono::current_zone(); }

const std::chrono::tzdb &f2() { return std::chrono::get_tzdb(); }

std::chrono::tzdb_list &f3() { return std::chrono::get_tzdb_list(); }

const std::chrono::time_zone *f4(std::string_view a0) {
  return std::chrono::locate_zone(std::move(a0));
}

const std::chrono::tzdb &f5() { return std::chrono::reload_tzdb(); }

std::string f6() { return std::chrono::remote_version(); }

using t1 = std::chrono::tzdb_list;

using t2 = std::chrono::tzdb_list::const_iterator;

const std::chrono::tzdb &f7(const std::chrono::tzdb_list &a0) {
  return a0.front();
}

std::chrono::tzdb_list::const_iterator
f8(std::chrono::tzdb_list &a0, std::chrono::tzdb_list::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

std::chrono::tzdb_list::const_iterator f9(const std::chrono::tzdb_list &a0) {
  return a0.begin();
}

std::chrono::tzdb_list::const_iterator f10(const std::chrono::tzdb_list &a0) {
  return a0.end();
}

std::chrono::tzdb_list::const_iterator f11(const std::chrono::tzdb_list &a0) {
  return a0.cbegin();
}

std::chrono::tzdb_list::const_iterator f12(const std::chrono::tzdb_list &a0) {
  return a0.cend();
}

using t3 = std::__forward_list_const_iterator<
    std::__forward_list_node<std::chrono::tzdb, void *> *>::iterator_category;

using t4 = std::__forward_list_const_iterator<
    std::__forward_list_node<std::chrono::tzdb, void *> *>::value_type;

using t5 = std::__forward_list_const_iterator<
    std::__forward_list_node<std::chrono::tzdb, void *> *>::reference;

using t6 = std::__forward_list_const_iterator<
    std::__forward_list_node<std::chrono::tzdb, void *> *>::difference_type;

using t7 = std::__forward_list_const_iterator<
    std::__forward_list_node<std::chrono::tzdb, void *> *>::pointer;

std::chrono::tzdb_list::const_iterator f13() {
  return std::chrono::tzdb_list::const_iterator();
}

std::chrono::tzdb_list::const_iterator &
f14(std::chrono::tzdb_list::const_iterator &a0) {
  return a0.operator++();
}

std::chrono::tzdb_list::const_iterator
f15(std::chrono::tzdb_list::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

bool f16(const std::chrono::tzdb_list::const_iterator &a0,
         const std::chrono::tzdb_list::const_iterator &a1) {
  return a0 != a1;
}

bool f17(const std::chrono::tzdb_list::const_iterator &a0,
         const std::chrono::tzdb_list::const_iterator &a1) {
  return a0 == a1;
}
