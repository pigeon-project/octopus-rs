cmake_minimum_required(VERSION 3.2)


project(octopus_crt)

include_directories(
        include
)

aux_source_directory(src DIR_SRCS)

add_library(octopus_crt
        STATIC
        ${DIR_SRCS})
