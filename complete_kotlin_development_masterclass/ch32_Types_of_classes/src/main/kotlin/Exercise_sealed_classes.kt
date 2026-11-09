import kotlin.random.Random

fun main() {
    for (i in 1..10) {
        val prize = lottery()
        when (prize) {
            is Vacation -> println("I'm going on holiday")
            is Car2 -> println("Wow I won a car!")
            is GiftCard -> println("Yay, a gift card")
        }
    }
}

fun lottery(): Prize {
    val number = Random.nextInt(3)
    return when(number) {
        0 -> return FranceHoliday()
        1 -> return VW()
        else -> Dollar10()
    }
}

sealed class Prize

sealed class Car2: Prize()

sealed class  Vacation: Prize()

sealed class GiftCard: Prize()

class VW: Car2()

class FranceHoliday: Vacation()

class Dollar10: GiftCard()