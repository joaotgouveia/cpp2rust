# Builds rulegenerator's checkusedrules against this cpp2rust checkout, so that
# it matches rules exactly as the cpp2rust being tested does.
cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(CheckUsedRules)

if (NOT RULEGENERATOR_DIR)
  message(FATAL_ERROR "Set RULEGENERATOR_DIR to the rulegenerator checkout.")
endif()
file(REAL_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../.." Cpp2Rust_DIR)
set(utils_dir "${RULEGENERATOR_DIR}/targetgenerator-support/utils")

include("${Cpp2Rust_DIR}/cmake/rust-toolchain.cmake")

if (NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

find_package(LLVM REQUIRED CONFIG)
find_package(Clang REQUIRED CONFIG)
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}" "${CLANG_CMAKE_DIR}")
include(AddLLVM)
include(AddClang)

add_subdirectory("${Cpp2Rust_DIR}/cpp2rust" cpp2rust SYSTEM)

# Translations cpp2rust performs without rules (std::move, std::endl, ...),
# which checkusedrules never reports.
set(bt_subdir "cpp2rust_builtin_translations")
set(bt_src_dir "${utils_dir}/${bt_subdir}")
set(bt_part "${CMAKE_BINARY_DIR}/builtin-translations-part.json")
set(bt_index_dir "${CMAKE_BINARY_DIR}/builtin-translations-index")
file(GLOB bt_srcs CONFIGURE_DEPENDS
     "${bt_src_dir}/src.cpp" "${bt_src_dir}/src_cxx*.cpp")
add_custom_command(
  OUTPUT "${bt_part}"
  COMMAND $<TARGET_FILE:cpp-rule-indexer> --dir "${bt_src_dir}" -o "${bt_part}"
  DEPENDS ${bt_srcs}
  VERBATIM)
add_custom_command(
  OUTPUT "${bt_index_dir}/cpp.json"
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${bt_index_dir}"
  COMMAND $<TARGET_FILE:cpp-rule-indexer> -o "${bt_index_dir}" "${bt_part}"
  DEPENDS "${bt_part}"
  VERBATIM)
add_custom_target(builtin-translations-index ALL
                  DEPENDS "${bt_index_dir}/cpp.json")

add_clang_executable(checkusedrules "${utils_dir}/checkusedrules.cpp")
add_dependencies(checkusedrules builtin-translations-index)
target_link_libraries(checkusedrules PRIVATE cpp2rust_core clang-cpp)
target_include_directories(checkusedrules SYSTEM PRIVATE
  "${Cpp2Rust_DIR}" ${CLANG_INCLUDE_DIRS} ${LLVM_INCLUDE_DIRS})
target_compile_definitions(checkusedrules PRIVATE
  ${LLVM_DEFINITIONS} "CLANG_EXECUTABLE=\"${CMAKE_CXX_COMPILER}\""
  "BUILTIN_TRANSLATIONS_ID=\"${bt_subdir}\""
  "BUILTIN_TRANSLATIONS_INDEX=\"${bt_index_dir}\""
  "SUPPORT_RULES_ID=\"support\"")
llvm_map_components_to_libnames(llvm_libs support)
target_link_libraries(checkusedrules PRIVATE ${llvm_libs})
