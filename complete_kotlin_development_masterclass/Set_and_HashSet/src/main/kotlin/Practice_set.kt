fun main() {
    val colors = hashSetOf<String>()
    val colorsList = listOf("red", "green", "blue", "red")
    colors.addAll(colorsList)
    println(colors)
    colors.remove("green")
    println(colors)

    val objects = hashSetOf("laptop", "mouse", "phone", "bottle", "cup")
    val removeObjects = setOf("cup", "bottle", "phone")
    objects.removeAll(removeObjects)
    println(objects)
}