add_executable(aoc main.cpp)
target_include_directories(aoc PRIVATE ${CMAKE_CURRENT_LIST_DIR})
# Set clang tidy here so we dont run it on Unit tests, since catch has a lot of
# warnings from clang-tidy
set_target_properties(aoc PROPERTIES CXX_CLANG_TIDY clang-tidy)
# NOTE: in a more complicated project warnings should be in a toolchain file
# specified by the preset
target_compile_options(aoc PRIVATE -Wall -Wextra)
