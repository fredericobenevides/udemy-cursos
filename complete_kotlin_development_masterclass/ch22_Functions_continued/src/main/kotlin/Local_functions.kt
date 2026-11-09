fun main() {
    listAnimals()
}

fun listAnimals() {
    fun listOneAnimal(animal: String) {
        println("I have a $animal")
    }

    val animals = listOf("cat", "dog", "giraffe")
    for (animal in animals) {
        listOneAnimal(animal)
    }
}