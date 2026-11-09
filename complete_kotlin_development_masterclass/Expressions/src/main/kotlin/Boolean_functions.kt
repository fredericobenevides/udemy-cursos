fun main() {
    var animals = arrayListOf<String>()
    if (animals.isEmpty()) {
        animals.add("dog")
    }
    println(animals)

    animals = arrayListOf("cat")
    if (animals.add("horse")) {
        println("Horse was successfully added")
    }
}