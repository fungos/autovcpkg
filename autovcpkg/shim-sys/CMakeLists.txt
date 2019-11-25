cmake_minimum_required(VERSION 3.5)
project(shim CXX C)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_LIST_DIR})
include(AutoVcpkg)

vcpkg_install(${AUTO_VCPKG_PACKAGES})

include(${CMAKE_TOOLCHAIN_FILE})

# debugging stuff
#find_package(Crc32c REQUIRED)
#add_library(shim test.c)
#include_directories("${AUTO_VCPKG_ROOT}/installed/x64-windows/include")
#target_link_libraries(shim crc32c)

add_custom_target(shim "echo")