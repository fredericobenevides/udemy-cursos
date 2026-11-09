fun main() {
    val count = mapOf(Pair(1, "One"), Pair(2, "two"), Pair(3, "three"))
    println(count)

    val count2 = mapOf<Int, String>()
    println(count2)

    println(count[2])

    println(count.keys)
    println(count.values)
}
