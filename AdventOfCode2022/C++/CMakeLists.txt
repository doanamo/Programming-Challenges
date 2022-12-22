cmake_minimum_required(VERSION 3.24)
project(AdventOfCode2022)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_BINARY_DIR})
list(APPEND CMAKE_PREFIX_PATH ${CMAKE_BINARY_DIR})

include(conan.cmake)
conan_cmake_configure(
    REQUIRES fmt/9.1.0
    REQUIRES scnlib/1.1.2
    REQUIRES glm/0.9.9.8
    GENERATORS cmake_find_package)
conan_cmake_autodetect(Settings)
conan_cmake_install(
    PATH_OR_REFERENCE .
    BUILD missing
    REMOTE conancenter
    SETTINGS ${Settings})

find_package(fmt)
find_package(scn)
find_package(glm)

add_executable(Day15 "Source/Day15.cpp")
target_link_libraries(Day15 fmt::fmt scn::scn glm::glm)
