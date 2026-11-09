fun main() {
    val winners = listOf("Usain", "John", "Michael", "Alex", "Bob")
    println("Michael finished on position ${winners.indexOf("Michael") + 1}")

    val requiredColors = listOf("red", "green", "blue")
    val availableColors = listOf("red", "blue")
    println("Can print: ${availableColors.containsAll(requiredColors)}")

    val ingredients = arrayListOf("Chicken", "Egg", "Mozzarella", "Pepper")
    ingredients[ingredients.indexOf("Mozzarella")] = "blue cheese"
    println(ingredients)
}