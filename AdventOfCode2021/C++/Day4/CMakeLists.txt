cmake_minimum_required(VERSION 3.8)

add_executable(Day4 "Day4.cpp")
set_property(TARGET Day4 PROPERTY CXX_STANDARD 17)
target_compile_definitions(Day4 PUBLIC PROJECT_DIR="${CMAKE_CURRENT_SOURCE_DIR}/")
