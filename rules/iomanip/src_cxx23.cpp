#include <barrier>
#include <iomanip>
#include <istream>
#include <utility>

template <typename T1> using t1 = std::_Get_money<T1>;

template <typename T1> using t2 = std::_Get_time<T1>;

template <typename T1> using t3 = std::_Put_money<T1>;

template <typename T1> using t4 = std::_Put_time<T1>;

using t5 = std::_Resetiosflags;

std::_Resetiosflags f1() { return std::_Resetiosflags(); }

std::_Resetiosflags f2(const std::_Resetiosflags &a0) {
  return std::_Resetiosflags(a0);
}

std::_Resetiosflags f3(std::_Resetiosflags &&a0) {
  return std::_Resetiosflags(std::move(a0));
}

std::_Resetiosflags &f4(std::_Resetiosflags &a0,
                        const std::_Resetiosflags &a1) {
  return a0.operator=(a1);
}

std::_Resetiosflags &f5(std::_Resetiosflags &a0, std::_Resetiosflags &&a1) {
  return a0.operator=(std::move(a1));
}

using t6 = std::_Setbase;

std::_Setbase f6() { return std::_Setbase(); }

std::_Setbase f7(const std::_Setbase &a0) { return std::_Setbase(a0); }

std::_Setbase f8(std::_Setbase &&a0) { return std::_Setbase(std::move(a0)); }

std::_Setbase &f9(std::_Setbase &a0, const std::_Setbase &a1) {
  return a0.operator=(a1);
}

std::_Setbase &f10(std::_Setbase &a0, std::_Setbase &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t7 = std::_Setfill<T1>;

using t8 = std::_Setiosflags;

std::_Setiosflags f11() { return std::_Setiosflags(); }

std::_Setiosflags f12(const std::_Setiosflags &a0) {
  return std::_Setiosflags(a0);
}

std::_Setiosflags f13(std::_Setiosflags &&a0) {
  return std::_Setiosflags(std::move(a0));
}

std::_Setiosflags &f14(std::_Setiosflags &a0, const std::_Setiosflags &a1) {
  return a0.operator=(a1);
}

std::_Setiosflags &f15(std::_Setiosflags &a0, std::_Setiosflags &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::_Setprecision;

std::_Setprecision f16() { return std::_Setprecision(); }

std::_Setprecision f17(const std::_Setprecision &a0) {
  return std::_Setprecision(a0);
}

std::_Setprecision f18(std::_Setprecision &&a0) {
  return std::_Setprecision(std::move(a0));
}

std::_Setprecision &f19(std::_Setprecision &a0, const std::_Setprecision &a1) {
  return a0.operator=(a1);
}

std::_Setprecision &f20(std::_Setprecision &a0, std::_Setprecision &&a1) {
  return a0.operator=(std::move(a1));
}

using t10 = std::_Setw;

std::_Setw f21() { return std::_Setw(); }

std::_Setw f22(const std::_Setw &a0) { return std::_Setw(a0); }

std::_Setw f23(std::_Setw &&a0) { return std::_Setw(std::move(a0)); }

std::_Setw &f24(std::_Setw &a0, const std::_Setw &a1) {
  return a0.operator=(a1);
}

std::_Setw &f25(std::_Setw &a0, std::_Setw &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::_Get_money<T1> f26(T1 &a0, bool a1) {
  return std::get_money<T1>(a0, a1);
}

template <typename T1> std::_Get_time<T1> f27(std::tm *a0, const T1 *a1) {
  return std::get_time<T1>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f28(std::basic_ostream<T1, T2> &a0,
                                std::_Resetiosflags a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f29(std::basic_ostream<T1, T2> &a0,
                                std::_Setbase a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f30(std::basic_ostream<T1, T2> &a0,
                                std::_Setiosflags a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f31(std::basic_ostream<T1, T2> &a0,
                                std::_Setprecision a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f32(std::basic_ostream<T1, T2> &a0, std::_Setw a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f33(std::basic_ostream<T1, T2> &a0,
                                std::_Put_time<T1> a1) {
  return std::operator<< <T1, T2>(a0, std::forward<std::_Put_time<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f34(std::basic_ostream<T1, T2> &a0,
                                std::_Setfill<T1> a1) {
  return std::operator<< <T1, T2>(a0, std::forward<std::_Setfill<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f35(std::basic_ostream<T1, T2> &a0,
                                std::_Put_money<T3> a1) {
  return std::operator<< <T1, T2, T3>(a0,
                                      std::forward<std::_Put_money<T3>>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f36(std::basic_istream<T1, T2> &a0,
                                std::_Resetiosflags a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f37(std::basic_istream<T1, T2> &a0,
                                std::_Setbase a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f38(std::basic_istream<T1, T2> &a0,
                                std::_Setiosflags a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f39(std::basic_istream<T1, T2> &a0,
                                std::_Setprecision a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f40(std::basic_istream<T1, T2> &a0, std::_Setw a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f41(std::basic_istream<T1, T2> &a0,
                                std::_Get_time<T1> a1) {
  return std::operator>> <T1, T2>(a0, std::forward<std::_Get_time<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f42(std::basic_istream<T1, T2> &a0,
                                std::_Setfill<T1> a1) {
  return std::operator>> <T1, T2>(a0, std::forward<std::_Setfill<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f43(std::basic_istream<T1, T2> &a0,
                                std::_Get_money<T3> a1) {
  return std::operator>>
      <T1, T2, T3>(a0, std::forward<std::_Get_money<T3>>(a1));
}

template <typename T1> std::_Put_money<T1> f44(const T1 &a0, bool a1) {
  return std::put_money<T1>(a0, a1);
}

template <typename T1> std::_Put_time<T1> f45(const std::tm *a0, const T1 *a1) {
  return std::put_time<T1>(a0, a1);
}

template <typename T1> auto f46(const T1 *a0, T1 a1, T1 a2) {
  return std::quoted<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
auto f47(std::basic_string_view<T1, T2> a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2>(std::forward<std::basic_string_view<T1, T2>>(a0),
                             std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
auto f48(const std::basic_string<T1, T2, T3> &a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2, T3>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
auto f49(std::basic_string<T1, T2, T3> &a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2, T3>(a0, std::move(a1), std::move(a2));
}

std::_Resetiosflags f50(std::ios_base::fmtflags a0) {
  return std::resetiosflags(a0);
}

std::_Setbase f51(int a0) { return std::setbase(a0); }

template <typename T1> std::_Setfill<T1> f52(T1 a0) {
  return std::setfill<T1>(std::move(a0));
}

std::_Setiosflags f53(std::ios_base::fmtflags a0) {
  return std::setiosflags(a0);
}

std::_Setprecision f54(int a0) { return std::setprecision(a0); }

std::_Setw f55(int a0) { return std::setw(a0); }
