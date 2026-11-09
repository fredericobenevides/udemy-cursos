import kotlin.random.Random

fun main() {
    for (i in 1..10) {
        val number = Random.nextInt(100)
        val evenOrNull = number.takeIf { it % 2 == 0 }

        println("The number is $number")
        println("The even number is $evenOrNull")
    }

    println("------------------------")

    for (i in 1..10) {
        val number = Random.nextInt(100)
        val oddOrNull = number.takeUnless { it % 2 == 0 }

        println("The number is $number")
        println("The odd number is $oddOrNull")
    }
}