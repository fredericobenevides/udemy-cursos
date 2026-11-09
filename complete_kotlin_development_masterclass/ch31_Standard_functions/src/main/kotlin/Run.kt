fun main() {
    val newCar = NewCar().run {
        speed = 70
        drive()
        println("NewCar is driving")
        this
    }
    println(newCar)

    run {
        val car2 = NewCar()
        car2.speed
        car2.drive()
    }
//    println(car2)
}

class NewCar {
    var speed = 10
    fun drive() {
        println("Car is driving at $speed")
    }
}