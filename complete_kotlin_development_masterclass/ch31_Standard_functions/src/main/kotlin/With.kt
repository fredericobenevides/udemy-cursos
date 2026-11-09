fun main() {
    with(Car()) {
        speed = 80
        drive()
        println("Car is driving")
    }
}

class Car {
    var speed = 50
    fun drive() {
        println("Driving at $speed")
    }
}