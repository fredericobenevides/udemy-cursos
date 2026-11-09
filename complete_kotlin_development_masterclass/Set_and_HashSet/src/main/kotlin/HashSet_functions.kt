fun main() {
    val numbers = hashSetOf(3, 7, 5, 4, 6)
    val newNumbers = hashSetOf(7, 2, 5, 9, 6)

    println(numbers.intersect(newNumbers))
    println(newNumbers.intersect(numbers))

    numbers.clear()
    println(numbers)
    println(numbers.isEmpty())
}