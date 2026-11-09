fun main() {
    println("Please enter a number")
    val numberInput = readLine()?:""
//    val myByte = numberInput.toByte()
//    println(myByte)
//    println(myByte::class.java)
//
//    val myShort = myByte.toShort()
//    println(myShort)
//    println(myShort::class.java)

//    val myInt = numberInput.toInt()
//    println(myInt::class.java)
//
//    val myLong = numberInput.toLong()
//    println(myLong::class.java)

//    val myFloat = numberInput.toFloat()
//    println(myFloat::class.java)
//
//    val myDouble = numberInput.toFloat()
//    println(myDouble::class.java)

    val pi = 3.14159
    println("$pi is ${pi::class.java}")

    val piFloat = pi.toFloat()
    println("$piFloat is ${piFloat::class.java}")

    val piInt = pi.toInt()
    println("$piInt is ${piInt::class.java}")

    val piString = pi.toString()
    println("$piString is ${piString::class.java}")
}