fun main() {
    def array: int[50];

    loop from 0 to 50 with i {
        array[i] = 50 - i
    }

    loop from 0 to 50 with i {
        write("array[i] = ", array[i])
    }
}

main()
