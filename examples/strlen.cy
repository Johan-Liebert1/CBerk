include "./include/std.cy"

fun main() {
    def hel: str = "hello_world";
    def len: int = strlen(&hel);
    write("len string = ", len)

    def hello: str = "hello_world\0";
    def len1: int = strlen(&hello);
    def len2: int = strlen_cstr(hello as *char);
    write("len1 = ", len1, "len2 = ", len2)
}

main()
