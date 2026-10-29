#include <iosfwd>
#include <iostream>
#include <utility>

std::ostream &f1() { return std::cerr; }

std::istream &f2() { return std::cin; }

std::ostream &f3() { return std::clog; }

std::ostream &f4() { return std::cout; }

std::wostream &f5() { return std::wcerr; }

std::wistream &f6() { return std::wcin; }

std::wostream &f7() { return std::wclog; }

std::wostream &f8() { return std::wcout; }
