fun main(args: Array<String>) {
    val myPet = "Crocodile"
    println(myPet.length)

    val myName = "michelle"
    println(myName.capitalize())

    val myAnimal = "Giraffe"
    println(myAnimal.decapitalize())

    val myText = "     there are      too    many spaces      "
    println(myText.trim())

    println(myPet.get(3))
    println(myPet[3])
    println(myPet.substring(3))
    println(myPet.substring(3, 6))
}