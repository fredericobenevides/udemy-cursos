fun main() {
    val colors = arrayListOf("blue", "red")
    println(colors)
    val noColors = arrayListOf<String>()
    println(noColors)
    colors.add("yellow")
    println(colors)
    val moreColors = arrayListOf("pink", "teal")
    colors.addAll(moreColors)
    println(colors)

    colors.remove("red")
    println(colors)

    colors.removeAll(moreColors)
    println(colors)

    colors.removeAt(1)
    println(colors)

    colors.add("red")
    colors.add("blue")
    println(colors)
    colors.remove("blue")
    println(colors)
}