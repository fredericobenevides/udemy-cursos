fun main() {
    var currentState = GameState.STARTED
    for (i in 1..30) {
        println("${i}. $currentState")
        currentState = changeState(currentState)
    }
}

fun changeState(currentState: GameState): GameState {
    return when(currentState) {
        GameState.STARTED -> GameState.PLAYING
        GameState.PLAYING -> GameState.GAMEOVER
        GameState.GAMEOVER -> GameState.STARTED
    }
}

enum class GameState {
    STARTED,
    PLAYING,
    GAMEOVER
}