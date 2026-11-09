fun main() {
    var acceptedColors = hashSetOf("white", "black", "grey")
    val myColors = hashSetOf("blue", "red", "black", "green")
    acceptedColors.retainAll(myColors)
    println("I can only wear $acceptedColors")

    acceptedColors = hashSetOf("white", "black", "grey")
    acceptedColors.add("red")
    acceptedColors.retainAll(myColors)
    println("Now I can wear $acceptedColors")
}