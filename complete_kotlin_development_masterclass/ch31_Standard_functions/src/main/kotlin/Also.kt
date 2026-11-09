fun main() {
    Car().apply {
        speed = 80
        drive()
    }
        .also {car ->
            println("Car is running")
            println("Car speed is ${car.speed}")
        }
}