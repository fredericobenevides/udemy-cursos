fun main() {
    val count = hashMapOf(Pair(1, "one"), Pair(2, "two"), Pair(3, "three"))
    count[5] = "five"
    println(count)

    val countMore = mapOf(Pair(20, "twenty"), Pair(30, "thirty"))
    count.putAll(countMore)
    println(count)

    count.remove(20)
    println(count)

    count.replace(2, "twotwo")
    println(count)

    count.clear()
    println(count)
}