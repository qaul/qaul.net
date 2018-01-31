
function(add_test_program program_name)
    add_executable(${program_name} ${program_name}.c)
    target_link_libraries(${program_name} libqaulutils)
    add_test(NAME ${program_name} COMMAND ${program_name})
endfunction(add_test_program)

add_test_program(defines_test)
add_test_program(logging_test)
