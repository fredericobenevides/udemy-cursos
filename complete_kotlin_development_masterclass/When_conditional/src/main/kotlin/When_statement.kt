fun main() {
    val animal = "dog"
    val action = when(animal) {
        "cat" -> "feed it"
        "dog" -> {
            println("Yey I met a dog!")
            "pet it"
        }
        else -> "google it"
    }
    println("When you meet a $animal you should $action")

    val month = "February"
    val days = when(month) {
        "January", "March", "May" -> 31
        "February" -> 28
        else -> 30
    }
    println("The month of $month has $days days")
}