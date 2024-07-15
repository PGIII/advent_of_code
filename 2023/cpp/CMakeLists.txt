cmake_minimum_required(VERSION 3.25.0)
project(program CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_COLOR_DIAGNOSTICS ON)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

include(FetchContent)

# inclue catch2 for unit testing
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG v3.0.1 # or a later release
)

FetchContent_MakeAvailable(Catch2)
list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)

# Module name should be the base name with extension. This Expects the tests to
# be located in MODULE_NAME.test.cpp Eg. source file: hi.cpp testfile:
# hi.test.cpp add test with AddModuleTest(hi)
function(AddModuleTest MODULE_NAME)
  add_executable(
    ${MODULE_NAME}_test ${CMAKE_CURRENT_LIST_DIR}/${MODULE_NAME}.test.cpp
                        ${CMAKE_CURRENT_LIST_DIR}/${MODULE_NAME}.cpp)
  target_include_directories(${MODULE_NAME}_test
                             PRIVATE ${CMAKE_CURRENT_LIST_DIR})
  target_link_libraries(${MODULE_NAME}_test PRIVATE Catch2::Catch2WithMain)
  catch_discover_tests(${MODULE_NAME}_test)
endfunction()

# This should be used for modules that are header only
function(AddHeaderModuleTest MODULE_NAME)
  add_executable(${MODULE_NAME}_test
                 ${CMAKE_CURRENT_LIST_DIR}/${MODULE_NAME}.test.cpp)
  target_include_directories(${MODULE_NAME}_test
                             PRIVATE ${CMAKE_CURRENT_LIST_DIR})
  target_link_libraries(${MODULE_NAME}_test PRIVATE Catch2::Catch2WithMain)
  catch_discover_tests(${MODULE_NAME}_test)
endfunction()

# Both of these will have a CMakelists that tells us what needs to be built
add_subdirectory(src)
add_subdirectory(test)
