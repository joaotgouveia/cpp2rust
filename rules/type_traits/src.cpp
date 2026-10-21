<<<<<<< HEAD
#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1> auto f1() -> decltype(std::declval<T1>()) {
  return std::declval<T1>();
}

using t1 = std::false_type;

using t2 = std::true_type;
=======
// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <type_traits>

template <typename Ref, typename T1 = typename std::remove_reference<Ref>::type>
using t1 = typename std::remove_reference<Ref>::type;

template <typename T1> using t2 = typename std::enable_if<true, T1>::type;

template <typename T1> using t3 = typename std::add_lvalue_reference<T1>::type;
>>>>>>> master
