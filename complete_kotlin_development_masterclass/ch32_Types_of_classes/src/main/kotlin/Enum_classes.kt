fun main() {
    val color = Colors.RED
    when (color) {
        Colors.RED -> println("You chose red")
        Colors.BLUE -> println("You chose blue")
        Colors.GREEN -> println("You chose green")
    }

    println(Colors.RED.timesUsed)
    println(Colors.GREEN.name)
    println(Colors.BLUE.ordinal)
}

enum class Colors(val timesUsed: Int) {
    RED(34),
    GREEN(12),
    BLUE(45)
}