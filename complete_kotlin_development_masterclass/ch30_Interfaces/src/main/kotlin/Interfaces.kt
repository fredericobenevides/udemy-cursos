fun main() {
    val myOven: Oven = Bosch()
    myOven.turnOn()
    myOven.cook(150)
    myOven.turnOff()
}

interface Oven {
    val temperature: Int

    fun turnOn()
    fun turnOff()
    fun cook(temp: Int) {
        println("Cooking at $temp degrees")
    }
}

class Bosch: Oven {
    override val temperature = 180

    override fun turnOn() {
        println("Turrning on")
    }

    override fun turnOff() {
        println("Turning off")
    }
}