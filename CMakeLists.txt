cmake_minimum_required(VERSION 3.8)
project(queue_sim)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/queue/mod.rs
        src/queue/statistics.rs
        src/queue/time_gen.rs
        src/event.rs
        src/main.rs
        src/schedule.rs)

add_executable(queue_sim ${SOURCE_FILES})