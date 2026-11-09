fun main() {
    val colors = listOf("blue", "red", "blue", "green")
    println(colors.size)
    println(colors.contains("red"))
    println(colors.contains("pink"))

    val newColors = listOf("red", "green", "teal")
    println(colors.containsAll(newColors))

    println(colors.indexOf("red"))

    println(colors.lastIndexOf("blue"))
}