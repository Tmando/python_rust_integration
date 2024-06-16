def add_num(x,y):
    return {"result" : x + y}
def sub_num(x,y):
    return {"result" : x - y}
def mul_num(x,y):
    return {"result" : x * y}
def div_num(x,y):
    return {"result" : x / y}
def write_file(test_str):
    f = open("test.txt", "w")
    f.write(test_str)
    f.close()
