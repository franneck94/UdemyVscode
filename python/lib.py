def func1() -> int:
    a1 = 1
    a2 = 1
    a3 = 1
    a4 = 1
    a5 = 1
    a6 = 1
    a = 1
    return a


def my_func2() -> int:
    ret_value = func1()
    return ret_value


def my_func3() -> int:
    res = my_func2()
    return res
