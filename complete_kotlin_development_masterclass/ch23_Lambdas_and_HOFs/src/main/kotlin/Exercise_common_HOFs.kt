fun main() {
    val numbers = listOf(34, 67, 76, 3435, 345, 67, 23, 5, 3, 56, 324, 5, 43, 90)
    val subset = numbers.map {
        if (it % 2 == 0)
            it / 2
        else
            it * 2
    }.filter { it > 25 }
    println(subset)
}