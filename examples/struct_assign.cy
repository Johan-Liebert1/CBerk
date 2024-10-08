struct MyStruct {
    x: int,
    y: int32,
    z: int16,
    w: int8,
    hello: str,
}

fun main() {
    def a: MyStruct = MyStruct { x: 400000000, y: 2000000, z: 42069, w: 200, hello: "Hello World\n" }

    write("x = ", a.x);
    write("y = ", a.y);
    write("z = ", a.z);
    write("w = ", a.w); 
    write("hello = ", a.hello); 

    a.x = a.x + 600
    a.y = a.y + 400
    a.w = a.w + 20
    a.hello = "Goodbye World\n"

    write("x = ", a.x);
    write("y = ", a.y);
    write("z = ", a.z);
    write("w = ", a.w); 
    write("hello = ", a.hello); 

    a.x += 600
    a.y += 400
    a.w += 20

    write("x = ", a.x);
    write("y = ", a.y);
    write("z = ", a.z);
    write("w = ", a.w); 
    write("hello = ", a.hello); 
}

main()
