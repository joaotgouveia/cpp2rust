#include <iomanip>
#include <istream>
#include <ostream>
#include <utility>

template <typename T1> using t1 = std::_Get_money<T1>;

template <typename T1> using t2 = std::_Get_time<T1>;

template <typename T1> using t3 = std::_Put_money<T1>;

template <typename T1> using t4 = std::_Put_time<T1>;

using t5 = std::_Resetiosflags;

std::_Resetiosflags f4() { return std::_Resetiosflags(); }

std::_Resetiosflags f5(const std::_Resetiosflags &a0) {
  return std::_Resetiosflags(a0);
}

std::_Resetiosflags f6(std::_Resetiosflags &&a0) {
  return std::_Resetiosflags(std::move(a0));
}

std::_Resetiosflags &f7(std::_Resetiosflags &a0,
                        const std::_Resetiosflags &a1) {
  return a0.operator=(a1);
}

std::_Resetiosflags &f8(std::_Resetiosflags &a0, std::_Resetiosflags &&a1) {
  return a0.operator=(std::move(a1));
}

using t6 = std::_Setbase;

std::_Setbase f9() { return std::_Setbase(); }

std::_Setbase f10(const std::_Setbase &a0) { return std::_Setbase(a0); }

std::_Setbase f11(std::_Setbase &&a0) { return std::_Setbase(std::move(a0)); }

std::_Setbase &f12(std::_Setbase &a0, const std::_Setbase &a1) {
  return a0.operator=(a1);
}

std::_Setbase &f13(std::_Setbase &a0, std::_Setbase &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t7 = std::_Setfill<T1>;

using t8 = std::_Setiosflags;

std::_Setiosflags f14() { return std::_Setiosflags(); }

std::_Setiosflags f15(const std::_Setiosflags &a0) {
  return std::_Setiosflags(a0);
}

std::_Setiosflags f16(std::_Setiosflags &&a0) {
  return std::_Setiosflags(std::move(a0));
}

std::_Setiosflags &f17(std::_Setiosflags &a0, const std::_Setiosflags &a1) {
  return a0.operator=(a1);
}

std::_Setiosflags &f18(std::_Setiosflags &a0, std::_Setiosflags &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::_Setprecision;

std::_Setprecision f19() { return std::_Setprecision(); }

std::_Setprecision f20(const std::_Setprecision &a0) {
  return std::_Setprecision(a0);
}

std::_Setprecision f21(std::_Setprecision &&a0) {
  return std::_Setprecision(std::move(a0));
}

std::_Setprecision &f22(std::_Setprecision &a0, const std::_Setprecision &a1) {
  return a0.operator=(a1);
}

std::_Setprecision &f23(std::_Setprecision &a0, std::_Setprecision &&a1) {
  return a0.operator=(std::move(a1));
}

using t10 = std::_Setw;

std::_Setw f24() { return std::_Setw(); }

std::_Setw f25(const std::_Setw &a0) { return std::_Setw(a0); }

std::_Setw f26(std::_Setw &&a0) { return std::_Setw(std::move(a0)); }

std::_Setw &f27(std::_Setw &a0, const std::_Setw &a1) {
  return a0.operator=(a1);
}

std::_Setw &f28(std::_Setw &a0, std::_Setw &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::_Get_money<T1> f29(T1 &a0, bool a1) {
  return std::get_money<T1>(a0, a1);
}

template <typename T1> std::_Get_time<T1> f30(std::tm *a0, const T1 *a1) {
  return std::get_time<T1>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f31(std::basic_ostream<T1, T2> &a0,
                                std::_Resetiosflags a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f32(std::basic_ostream<T1, T2> &a0,
                                std::_Setbase a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f33(std::basic_ostream<T1, T2> &a0,
                                std::_Setiosflags a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f34(std::basic_ostream<T1, T2> &a0,
                                std::_Setprecision a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f35(std::basic_ostream<T1, T2> &a0, std::_Setw a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f36(std::basic_ostream<T1, T2> &a0,
                                std::_Put_time<T1> a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f37(std::basic_ostream<T1, T2> &a0,
                                std::_Setfill<T1> a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f38(std::basic_ostream<T1, T2> &a0,
                                std::_Put_money<T3> a1) {
  return std::operator<< <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f39(std::basic_istream<T1, T2> &a0,
                                std::_Resetiosflags a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f40(std::basic_istream<T1, T2> &a0,
                                std::_Setbase a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f41(std::basic_istream<T1, T2> &a0,
                                std::_Setiosflags a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f42(std::basic_istream<T1, T2> &a0,
                                std::_Setprecision a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f43(std::basic_istream<T1, T2> &a0, std::_Setw a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f44(std::basic_istream<T1, T2> &a0,
                                std::_Get_time<T1> a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f45(std::basic_istream<T1, T2> &a0,
                                std::_Setfill<T1> a1) {
  return std::operator>> <T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f46(std::basic_istream<T1, T2> &a0,
                                std::_Get_money<T3> a1) {
  return std::operator>> <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1> std::_Put_money<T1> f47(const T1 &a0, bool a1) {
  return std::put_money<T1>(a0, a1);
}

template <typename T1> std::_Put_time<T1> f48(const std::tm *a0, const T1 *a1) {
  return std::put_time<T1>(a0, a1);
}

std::_Resetiosflags f49(std::ios_base::fmtflags a0) {
  return std::resetiosflags(a0);
}

std::_Setbase f50(int a0) { return std::setbase(a0); }

template <typename T1> std::_Setfill<T1> f51(T1 a0) {
  return std::setfill<T1>(std::move(a0));
}

std::_Setiosflags f52(std::ios_base::fmtflags a0) {
  return std::setiosflags(a0);
}

std::_Setprecision f53(int a0) { return std::setprecision(a0); }

std::_Setw f54(int a0) { return std::setw(a0); }
