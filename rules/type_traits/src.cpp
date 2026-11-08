// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <type_traits>

template <typename Ref, typename T1 = typename std::remove_reference<Ref>::type>
using t1 = typename std::remove_reference<Ref>::type;

template <typename T1> using t2 = typename std::enable_if<true, T1>::type;

template <typename T1> using t3 = typename std::add_lvalue_reference<T1>::type;

template <typename... T1> bool f2() { return std::conjunction_v<T1...>; }

template <typename... T1> bool f4() { return std::disjunction_v<T1...>; }

using t15 = std::false_type;

using t28 = std::true_type;
