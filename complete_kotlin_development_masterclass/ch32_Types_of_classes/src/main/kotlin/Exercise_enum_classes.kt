fun main() {
    val olympics = Olympics()
    println(olympics.getMedal(2))
    println(olympics.getMedal(7))
    println(olympics.getPosition(Medal.GOLD))
    println(olympics.getPosition(Medal.BRONZE))
}

enum class Medal(val position: Int) {
    GOLD(1),
    SILVER(2),
    BRONZE(3),
    NONE(4),
}

class Olympics {
    fun getMedal(position: Int): Medal {
        return when(position) {
            1 -> Medal.GOLD
            2 -> Medal.SILVER
            3 -> Medal.BRONZE
            else -> Medal.NONE
        }
    }

    fun getPosition(medal: Medal) = medal.position

}