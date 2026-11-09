fun main() {
}

open class Airplane {
    protected val type = "Airbus"

    fun fly() {
        println("lying")
    }
}

class MyAirplane: Airplane() {
    fun takeOff() {
        println(type)
        fly()
    }
}

class MyCar() {
    fun start() {
        val airplane = Airplane()
//        println(airplane.type)
        airplane.fly()
    }
}