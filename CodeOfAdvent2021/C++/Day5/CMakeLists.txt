cmake_minimum_required(VERSION 3.8)

add_executable(Day5 "Day5.cpp")
set_property(TARGET Day5 PROPERTY CXX_STANDARD 17)
target_compile_definitions(Day5 PUBLIC PROJECT_DIR="${CMAKE_CURRENT_SOURCE_DIR}/")
