#include <ostream>
#include <regex>
#include <string>
#include <utility>

using t24 = std::regex_constants::error_type;

std::regex_constants::error_type f70() {
  return std::regex_constants::error_collate;
}

std::regex_constants::error_type f71() {
  return std::regex_constants::error_ctype;
}

std::regex_constants::error_type f72() {
  return std::regex_constants::error_escape;
}

std::regex_constants::error_type f73() {
  return std::regex_constants::error_backref;
}

std::regex_constants::error_type f74() {
  return std::regex_constants::error_brack;
}

std::regex_constants::error_type f75() {
  return std::regex_constants::error_paren;
}

std::regex_constants::error_type f76() {
  return std::regex_constants::error_brace;
}

std::regex_constants::error_type f77() {
  return std::regex_constants::error_badbrace;
}

std::regex_constants::error_type f78() {
  return std::regex_constants::error_range;
}

std::regex_constants::error_type f79() {
  return std::regex_constants::error_space;
}

std::regex_constants::error_type f80() {
  return std::regex_constants::error_badrepeat;
}

std::regex_constants::error_type f81() {
  return std::regex_constants::error_complexity;
}

std::regex_constants::error_type f82() {
  return std::regex_constants::error_stack;
}

using t25 = std::regex_constants::match_flag_type;

std::regex_constants::match_flag_type f83() {
  return std::regex_constants::match_default;
}

std::regex_constants::match_flag_type f84() {
  return std::regex_constants::match_not_bol;
}

std::regex_constants::match_flag_type f85() {
  return std::regex_constants::match_not_eol;
}

std::regex_constants::match_flag_type f86() {
  return std::regex_constants::match_not_bow;
}

std::regex_constants::match_flag_type f87() {
  return std::regex_constants::match_not_eow;
}

std::regex_constants::match_flag_type f88() {
  return std::regex_constants::match_any;
}

std::regex_constants::match_flag_type f89() {
  return std::regex_constants::match_not_null;
}

std::regex_constants::match_flag_type f90() {
  return std::regex_constants::match_continuous;
}

std::regex_constants::match_flag_type f91() {
  return std::regex_constants::match_prev_avail;
}

std::regex_constants::match_flag_type f92() {
  return std::regex_constants::format_default;
}

std::regex_constants::match_flag_type f93() {
  return std::regex_constants::format_sed;
}

std::regex_constants::match_flag_type f94() {
  return std::regex_constants::format_no_copy;
}

std::regex_constants::match_flag_type f95() {
  return std::regex_constants::format_first_only;
}

std::regex_constants::match_flag_type
f96(std::regex_constants::match_flag_type a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator&(a0, a1);
}

std::regex_constants::syntax_option_type
f97(std::regex_constants::syntax_option_type a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator&(a0, a1);
}

std::regex_constants::match_flag_type &
f98(std::regex_constants::match_flag_type &a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator&=(a0, a1);
}

std::regex_constants::syntax_option_type &
f99(std::regex_constants::syntax_option_type &a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator&=(a0, a1);
}

std::regex_constants::match_flag_type
f100(std::regex_constants::match_flag_type a0,
     std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator^(a0, a1);
}

std::regex_constants::syntax_option_type
f101(std::regex_constants::syntax_option_type a0,
     std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator^(a0, a1);
}

std::regex_constants::match_flag_type &
f102(std::regex_constants::match_flag_type &a0,
     std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator^=(a0, a1);
}

std::regex_constants::syntax_option_type &
f103(std::regex_constants::syntax_option_type &a0,
     std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator^=(a0, a1);
}

std::regex_constants::match_flag_type
f104(std::regex_constants::match_flag_type a0,
     std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator|(a0, a1);
}

std::regex_constants::syntax_option_type
f105(std::regex_constants::syntax_option_type a0,
     std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator|(a0, a1);
}

std::regex_constants::match_flag_type &
f106(std::regex_constants::match_flag_type &a0,
     std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator|=(a0, a1);
}

std::regex_constants::syntax_option_type &
f107(std::regex_constants::syntax_option_type &a0,
     std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator|=(a0, a1);
}

std::regex_constants::match_flag_type
f108(std::regex_constants::match_flag_type a0) {
  return std::regex_constants::operator~(a0);
}

std::regex_constants::syntax_option_type
f109(std::regex_constants::syntax_option_type a0) {
  return std::regex_constants::operator~(a0);
}

using t26 = std::regex_constants::syntax_option_type;

std::regex_constants::syntax_option_type f110() {
  return std::regex_constants::icase;
}

std::regex_constants::syntax_option_type f111() {
  return std::regex_constants::nosubs;
}

std::regex_constants::syntax_option_type f112() {
  return std::regex_constants::optimize;
}

std::regex_constants::syntax_option_type f113() {
  return std::regex_constants::collate;
}

std::regex_constants::syntax_option_type f114() {
  return std::regex_constants::ECMAScript;
}

std::regex_constants::syntax_option_type f115() {
  return std::regex_constants::basic;
}

std::regex_constants::syntax_option_type f116() {
  return std::regex_constants::extended;
}

std::regex_constants::syntax_option_type f117() {
  return std::regex_constants::awk;
}

std::regex_constants::syntax_option_type f118() {
  return std::regex_constants::grep;
}

std::regex_constants::syntax_option_type f119() {
  return std::regex_constants::egrep;
}

std::regex_constants::syntax_option_type f120() {
  return std::regex_constants::multiline;
}

using t27 = std::regex_error;

std::regex_error f121(std::regex_constants::error_type a0) {
  return std::regex_error(a0);
}

std::regex_error f122(const std::regex_error &a0) {
  return std::regex_error(a0);
}

std::regex_constants::error_type f123(const std::regex_error &a0) {
  return a0.code();
}

std::regex_error &f124(std::regex_error &a0, const std::regex_error &a1) {
  return a0.operator=(a1);
}

const char *f125(const std::regex_error &a0) { return a0.what(); }

template <typename T1, typename T2>
bool f126(const T1 *a0, const T1 *a1, const std::basic_regex<T1, T2> &a2,
          std::regex_constants::match_flag_type a3) {
  return std::regex_search<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f127(const T2 *a0, const T2 *a1, std::match_results<const T2 *, T1> &a2,
          const std::basic_regex<T2, T3> &a3,
          std::regex_constants::match_flag_type a4) {
  return std::regex_search<T1, T2, T3>(a0, a1, a2, a3, a4);
}
