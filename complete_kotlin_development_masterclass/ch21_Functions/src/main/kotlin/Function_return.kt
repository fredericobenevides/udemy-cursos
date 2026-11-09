fun main() {
    val r = 63
    val area = calculateCircleArea(r)
    println("A circle with the radius $r has an area of $area")

    val people = listOf("Anna", "Bob", "William")
    for (person in people) {
        val message = personalizedGreeting(person)
        println(message)
    }
}

fun calculateCircleArea(radius: Int) = 3.1415 * radius * radius

fun personalizedGreeting(person: String): String {
    val greeting = when(person[0]) {
        'A' -> "Are you ok $person"
        'B' -> "What's up $person"
        else -> "Hi $person"
    }
    return greeting
}